use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

fn delete_deployment(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    let context = get_context()?;
    let client = context.client()?;

    py.detach(|| {
        context
            .runtime
            .block_on(client.delete_deployment(container_id_or_name))
    })
    .into_pyresult()
}

#[pymethods]
impl LocalDeployment {
    /// Delete a deployment.
    fn delete(&self, py: Python<'_>) -> PyResult<()> {
        delete_deployment(py, &self.inner().container_id)
    }

    /// Delete a deployment by name or container ID.
    #[staticmethod]
    fn delete_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        delete_deployment(py, &container_id_or_name)
    }
}
