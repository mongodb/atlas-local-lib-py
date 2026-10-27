use std::future::Future;
use std::sync::{Mutex, OnceLock};

use pyo3::marker::Ungil;
use pyo3::prelude::*;

use crate::docker_with_embedding_provider_endpoint::DockerWithEmbeddingProviderEndpoint;
use crate::exceptions::{DockerConnectionError, IntoPyErr, IntoPyResult};

pub(crate) struct PythonContext {
    pub(crate) runtime: tokio::runtime::Runtime,
    client: Mutex<Option<atlas_local::Client>>,
    docker: Mutex<Option<atlas_local::bollard::Docker>>,
}

static CONTEXT: OnceLock<Result<PythonContext, String>> = OnceLock::new();

pub(crate) fn get_context() -> PyResult<&'static PythonContext> {
    match CONTEXT.get_or_init(|| {
        tokio::runtime::Runtime::new()
            .map(|runtime| PythonContext {
                runtime,
                client: Mutex::new(None),
                docker: Mutex::new(None),
            })
            .map_err(|error| format!("{error} (kind: {:?})", error.kind()))
    }) {
        Ok(context) => Ok(context),
        Err(error) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Tokio runtime failed to initialize on first use and will not be retried \
             for the lifetime of this process; the process must be restarted. \
             Original error: {error}"
        ))),
    }
}

pub(crate) fn runtime_block_on<T, E, F, Fut>(py: Python<'_>, operation: F) -> PyResult<T>
where
    F: FnOnce(atlas_local::Client) -> Fut + Ungil + Send,
    Fut: Future<Output = Result<T, E>>,
    T: Ungil + Send,
    E: IntoPyErr + Ungil + Send,
{
    let context = get_context()?;
    let client = context.client()?;

    py.detach(|| context.runtime.block_on(operation(client)))
        .into_pyresult()
}

pub(crate) fn runtime_block_on_with_embedding_provider_endpoint<T, E, F, Fut>(
    py: Python<'_>,
    embedding_provider_endpoint: Option<String>,
    operation: F,
) -> PyResult<T>
where
    F: FnOnce(
            atlas_local::Client<DockerWithEmbeddingProviderEndpoint>,
            atlas_local::bollard::Docker,
        ) -> Fut
        + Ungil
        + Send,
    Fut: Future<Output = Result<T, E>>,
    T: Ungil + Send,
    E: IntoPyErr + Ungil + Send,
{
    let context = get_context()?;
    let docker = context.docker()?;
    let client = atlas_local::Client::new(DockerWithEmbeddingProviderEndpoint::new(
        docker.clone(),
        embedding_provider_endpoint,
    ));

    py.detach(|| context.runtime.block_on(operation(client, docker)))
        .into_pyresult()
}

fn docker_connection_error(error: impl std::fmt::Display) -> PyErr {
    DockerConnectionError::new_err(format!(
        "Could not connect to Docker.\n\
         Make sure Docker is installed and running: \
         https://docs.docker.com/get-docker/\n\
         Details: {error}"
    ))
}

impl PythonContext {
    pub(crate) fn docker(&self) -> PyResult<atlas_local::bollard::Docker> {
        let mut cached_docker = self
            .docker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(docker) = cached_docker.as_ref() {
            return Ok(docker.clone());
        }

        let new_docker = atlas_local::bollard::Docker::connect_with_defaults()
            .map_err(docker_connection_error)?;

        *cached_docker = Some(new_docker.clone());

        Ok(new_docker)
    }

    pub(crate) fn client(&self) -> PyResult<atlas_local::Client> {
        let mut cached_client = self
            .client
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(client) = cached_client.as_ref() {
            return Ok(client.clone());
        }

        let new_client = atlas_local::Client::new(self.docker()?);

        *cached_client = Some(new_client.clone());

        Ok(new_client)
    }
}
