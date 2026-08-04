use pyo3::prelude::*;

use crate::create_deployment_options::{CreateArgs, build_create_deployment_options};
use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

#[pymethods]
impl LocalDeployment {
    #[staticmethod]
    #[pyo3(signature = (
        name=None,
        image=None,
        image_tag=None,
        skip_pull_image=None,
        load_sample_data=None,
        mongodb_port_binding=None,
        wait_until_healthy=None,
        wait_until_healthy_timeout=None,
        local_seed_location=None,
        mongodb_initdb_root_username=None,
        mongodb_initdb_root_password=None,
        voyage_api_key=None,
        do_not_track=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn create(
        py: Python<'_>,
        name: Option<String>,
        image: Option<String>,
        image_tag: Option<String>,
        skip_pull_image: Option<bool>,
        load_sample_data: Option<bool>,
        mongodb_port_binding: Option<u16>,
        wait_until_healthy: Option<bool>,
        wait_until_healthy_timeout: Option<i64>,
        local_seed_location: Option<String>,
        mongodb_initdb_root_username: Option<String>,
        mongodb_initdb_root_password: Option<String>,
        voyage_api_key: Option<String>,
        do_not_track: Option<bool>,
    ) -> PyResult<Self> {
        let options = build_create_deployment_options(CreateArgs {
            name,
            image,
            image_tag,
            skip_pull_image,
            load_sample_data,
            mongodb_port_binding,
            wait_until_healthy,
            wait_until_healthy_timeout,
            local_seed_location,
            mongodb_initdb_root_username,
            mongodb_initdb_root_password,
            voyage_api_key,
            do_not_track,
        })?;

        let context = get_context()?;
        let client = context.client()?;

        // Release the GIL: creating a deployment can pull an image and wait for
        // the container to become healthy.
        // `create_deployment` must be called inside the Tokio runtime because it
        // spawns the deployment task. The returned progress value is then awaited
        // until the deployment completes.
        let deployment = py
            .detach(|| {
                context
                    .runtime
                    .block_on(async { client.create_deployment(options).await })
            })
            .into_pyresult()?;

        Ok(LocalDeployment::from(deployment))
    }
}
