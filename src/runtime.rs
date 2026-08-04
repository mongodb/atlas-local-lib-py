use std::sync::{Mutex, OnceLock};

use pyo3::prelude::*;

use crate::exceptions::DockerConnectionError;

pub(crate) struct PythonContext {
    pub(crate) runtime: tokio::runtime::Runtime,
    client: Mutex<Option<atlas_local::Client>>,
}

static CONTEXT: OnceLock<Result<PythonContext, String>> = OnceLock::new();

pub(crate) fn get_context() -> PyResult<&'static PythonContext> {
    match CONTEXT.get_or_init(|| {
        tokio::runtime::Runtime::new()
            .map(|runtime| PythonContext {
                runtime,
                client: Mutex::new(None),
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

impl PythonContext {
    pub(crate) fn client(&self) -> PyResult<atlas_local::Client> {
        let mut cached_client = self
            .client
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(client) = cached_client.as_ref() {
            return Ok(client.clone());
        }

        let new_client = atlas_local::Client::connect_with_defaults().map_err(|error| {
            DockerConnectionError::new_err(format!(
                "Could not connect to Docker.\n\
                     Make sure Docker is installed and running: \
                     https://docs.docker.com/get-docker/\n\
                     Details: {error}"
            ))
        })?;

        *cached_client = Some(new_client.clone());

        Ok(new_client)
    }
}
