use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

fn unpause_deployment(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    let context = get_context()?;
    let client = context.client()?;

    py.detach(|| {
        context
            .runtime
            .block_on(client.unpause_deployment(container_id_or_name))
    })
    .into_pyresult()
}

#[pymethods]
impl LocalDeployment {
    /// Unpause a paused deployment.
    fn unpause(&self, py: Python<'_>) -> PyResult<()> {
        unpause_deployment(py, &self.inner().container_id)
    }

    /// Unpause a paused deployment by name or container ID.
    #[staticmethod]
    fn unpause_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        unpause_deployment(py, &container_id_or_name)
    }
}
