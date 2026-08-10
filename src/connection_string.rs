use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn run_connection_string(py: Python<'_>, container_id_or_name: String) -> PyResult<String> {
    runtime_block_on(py, |client| async move {
        client.get_connection_string(container_id_or_name).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Get the connection string for a deployment.
    fn connection_string(&self, py: Python<'_>) -> PyResult<String> {
        run_connection_string(py, self.inner().container_id.clone())
    }

    /// Get the connection string for a deployment by name or container ID.
    #[staticmethod]
    fn connection_string_deployment(
        py: Python<'_>,
        container_id_or_name: String,
    ) -> PyResult<String> {
        run_connection_string(py, container_id_or_name)
    }
}
