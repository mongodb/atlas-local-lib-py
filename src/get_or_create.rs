use atlas_local::{
    DockerError as RsDockerError,
    client::{
        CreateDeploymentError as RsCreateDeploymentError,
        GetDeploymentError as RsGetDeploymentError,
    },
};

use pyo3::prelude::*;

use crate::create_deployment_options::{CreateArgs, build_create_deployment_options};
use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyErr;
use crate::option_conflicts::ensure_options_match;
use crate::runtime::runtime_block_on;

pub(crate) enum GetOrCreateDeploymentError {
    Get(RsGetDeploymentError),
    Create(RsCreateDeploymentError),
    OptionsMismatch(PyErr),
}

impl IntoPyErr for GetOrCreateDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            Self::Get(error) => error.into_pyerr(),
            Self::Create(error) => error.into_pyerr(),
            Self::OptionsMismatch(error) => error,
        }
    }
}

async fn find_deployment(
    client: &atlas_local::Client,
    name: &str,
) -> Result<Option<atlas_local::models::Deployment>, RsGetDeploymentError> {
    match client.get_deployment(name).await {
        Ok(deployment) => Ok(Some(deployment)),

        Err(RsGetDeploymentError::ContainerInspect(RsDockerError::NotFound)) => Ok(None),

        Err(error) => Err(error),
    }
}

async fn get_or_create_deployment(
    client: atlas_local::Client,
    name: String,
    options: atlas_local::models::CreateDeploymentOptions,
) -> Result<atlas_local::models::Deployment, GetOrCreateDeploymentError> {
    if let Some(deployment) = find_deployment(&client, &name)
        .await
        .map_err(GetOrCreateDeploymentError::Get)?
    {
        ensure_options_match(&name, &deployment, &options)
            .map_err(GetOrCreateDeploymentError::OptionsMismatch)?;

        return Ok(deployment);
    }

    match client.create_deployment(options.clone()).await {
        Ok(deployment) => Ok(deployment),

        Err(error @ RsCreateDeploymentError::ContainerAlreadyExists(_)) => {
            //Deployment might have been created by another process after we checked for it, so we try to get it again and check the options.
            match find_deployment(&client, &name)
                .await
                .map_err(GetOrCreateDeploymentError::Get)?
            {
                Some(deployment) => {
                    ensure_options_match(&name, &deployment, &options)
                        .map_err(GetOrCreateDeploymentError::OptionsMismatch)?;

                    Ok(deployment)
                }
                None => Err(GetOrCreateDeploymentError::Create(error)),
            }
        }

        Err(error) => Err(GetOrCreateDeploymentError::Create(error)),
    }
}

#[pymethods]
impl LocalDeployment {
    /// Retrieve a deployment by name, creating it if it does not exist.
    #[staticmethod]
    #[pyo3(signature = (
        name,
        image=None,
        image_tag=None,
        skip_pull_image=None,
        load_sample_data=None,
        port=None,
        ip=None,
        wait_until_healthy=None,
        wait_until_healthy_timeout=None,
        local_seed_location=None,
        mongodb_initdb_root_username=None,
        mongodb_initdb_root_password=None,
        voyage_api_key=None,
        do_not_track=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn get_or_create(
        py: Python<'_>,
        name: String,
        image: Option<String>,
        image_tag: Option<String>,
        skip_pull_image: Option<bool>,
        load_sample_data: Option<bool>,
        port: Option<u16>,
        ip: Option<String>,
        wait_until_healthy: Option<bool>,
        wait_until_healthy_timeout: Option<i64>,
        local_seed_location: Option<String>,
        mongodb_initdb_root_username: Option<String>,
        mongodb_initdb_root_password: Option<String>,
        voyage_api_key: Option<String>,
        do_not_track: Option<bool>,
    ) -> PyResult<Self> {
        let options = build_create_deployment_options(CreateArgs {
            name: Some(name.clone()),
            image,
            image_tag,
            skip_pull_image,
            load_sample_data,
            port,
            ip,
            wait_until_healthy,
            wait_until_healthy_timeout,
            local_seed_location,
            mongodb_initdb_root_username,
            mongodb_initdb_root_password,
            voyage_api_key,
            do_not_track,
        })?;

        let deployment = runtime_block_on(py, move |client| {
            get_or_create_deployment(client, name, options)
        })?;

        Ok(LocalDeployment::from(deployment))
    }
}
