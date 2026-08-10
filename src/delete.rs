use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn run_delete(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    runtime_block_on(py, |client| async move {
        client.delete_deployment(container_id_or_name).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Delete a deployment.
    fn delete(&self, py: Python<'_>) -> PyResult<()> {
        run_delete(py, &self.inner().container_id)
    }

    /// Delete a deployment by name or container ID.
    #[staticmethod]
    fn delete_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        run_delete(py, &container_id_or_name)
    }
}
