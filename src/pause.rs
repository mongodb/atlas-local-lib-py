use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn run_pause(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    runtime_block_on(py, |client| async move {
        client.pause_deployment(container_id_or_name).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Pause a running deployment.
    fn pause(&self, py: Python<'_>) -> PyResult<()> {
        run_pause(py, &self.inner().container_id)
    }

    /// Pause a running deployment by name or container ID.
    #[staticmethod]
    fn pause_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        run_pause(py, &container_id_or_name)
    }
}
