use atlas_local::{
    DockerError as RsDockerError,
    bollard::{Docker, query_parameters::InspectContainerOptions},
    client::{
        CreateDeploymentError as RsCreateDeploymentError,
        GetDeploymentError as RsGetDeploymentError,
    },
    models::{CreateDeploymentOptions, Deployment},
};

use pyo3::prelude::*;

use crate::create_deployment_options::{CreateArgs, build_create_deployment_options};
use crate::deployment::LocalDeployment;
use crate::docker_with_embedding_provider_endpoint::{
    DockerWithEmbeddingProviderEndpoint, embedding_provider_endpoint_from_inspect_response,
};
use crate::exceptions::IntoPyErr;
use crate::option_conflicts::ensure_options_match;
use crate::runtime::runtime_block_on_with_embedding_provider_endpoint;

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
    docker: &Docker,
    name: &str,
) -> Result<Option<(Deployment, Option<String>)>, RsGetDeploymentError> {
    let response = match docker
        .inspect_container(name, None::<InspectContainerOptions>)
        .await
    {
        Ok(response) => response,
        Err(error) => match RsDockerError::from(error) {
            RsDockerError::NotFound => return Ok(None),
            error => return Err(RsGetDeploymentError::ContainerInspect(error)),
        },
    };

    let embedding_provider_endpoint = embedding_provider_endpoint_from_inspect_response(&response);
    let deployment =
        Deployment::try_from(response).map_err(RsGetDeploymentError::IntoDeployment)?;

    Ok(Some((deployment, embedding_provider_endpoint)))
}

async fn get_or_create_deployment(
    client: atlas_local::Client<DockerWithEmbeddingProviderEndpoint>,
    docker: Docker,
    name: String,
    options: CreateDeploymentOptions,
    embedding_provider_endpoint: Option<String>,
) -> Result<atlas_local::models::Deployment, GetOrCreateDeploymentError> {
    if let Some((deployment, existing_embedding_provider_endpoint)) =
        find_deployment(&docker, &name)
            .await
            .map_err(GetOrCreateDeploymentError::Get)?
    {
        ensure_options_match(
            &name,
            &deployment,
            &options,
            embedding_provider_endpoint.as_deref(),
            existing_embedding_provider_endpoint.as_deref(),
        )
        .map_err(GetOrCreateDeploymentError::OptionsMismatch)?;

        return Ok(deployment);
    }

    match client.create_deployment(options.clone()).await {
        Ok(deployment) => Ok(deployment),

        Err(error @ RsCreateDeploymentError::ContainerAlreadyExists(_)) => {
            //Deployment might have been created by another process after we checked for it, so we try to get it again and check the options.
            match find_deployment(&docker, &name)
                .await
                .map_err(GetOrCreateDeploymentError::Get)?
            {
                Some((deployment, existing_embedding_provider_endpoint)) => {
                    ensure_options_match(
                        &name,
                        &deployment,
                        &options,
                        embedding_provider_endpoint.as_deref(),
                        existing_embedding_provider_endpoint.as_deref(),
                    )
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
        do_not_track=None,
        embedding_provider_endpoint=None
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
        embedding_provider_endpoint: Option<String>,
    ) -> PyResult<Self> {
        let (options, embedding_provider_endpoint) = build_create_deployment_options(
            py,
            CreateArgs {
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
                embedding_provider_endpoint,
                do_not_track,
            },
        )?;

        let requested_embedding_provider_endpoint = embedding_provider_endpoint.clone();
        let deployment = runtime_block_on_with_embedding_provider_endpoint(
            py,
            embedding_provider_endpoint,
            move |client, docker| async move {
                get_or_create_deployment(
                    client,
                    docker,
                    name,
                    options,
                    requested_embedding_provider_endpoint,
                )
                .await
            },
        )?;

        Ok(LocalDeployment::from(deployment))
    }
}
