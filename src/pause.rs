use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

fn pause_deployment(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    let context = get_context()?;
    let client = context.client()?;

    py.detach(|| {
        context
            .runtime
            .block_on(client.pause_deployment(container_id_or_name))
    })
    .into_pyresult()
}

#[pymethods]
impl LocalDeployment {
    /// Pause a running deployment.
    fn pause(&self, py: Python<'_>) -> PyResult<()> {
        pause_deployment(py, &self.inner().container_id)
    }

    /// Pause a running deployment by name or container ID.
    #[staticmethod]
    fn pause_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        pause_deployment(py, &container_id_or_name)
    }
}
