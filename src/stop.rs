use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn run_stop(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    runtime_block_on(py, |client| async move {
        client.stop_deployment(container_id_or_name).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Stop a running deployment.
    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        run_stop(py, &self.inner().container_id)
    }

    /// Stop a running deployment by name or container ID.
    #[staticmethod]
    fn stop_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        run_stop(py, &container_id_or_name)
    }
}
