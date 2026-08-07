use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn run_unpause(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    runtime_block_on(py, |client| async move {
        client.unpause_deployment(container_id_or_name).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Unpause a paused deployment.
    fn unpause(&self, py: Python<'_>) -> PyResult<()> {
        run_unpause(py, &self.inner().container_id)
    }

    /// Unpause a paused deployment by name or container ID.
    #[staticmethod]
    fn unpause_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        run_unpause(py, &container_id_or_name)
    }
}
