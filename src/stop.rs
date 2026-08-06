use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

fn stop_deployment(py: Python<'_>, container_id_or_name: &str) -> PyResult<()> {
    let context = get_context()?;
    let client = context.client()?;

    py.detach(|| {
        context
            .runtime
            .block_on(client.stop_deployment(container_id_or_name))
    })
    .into_pyresult()
}

#[pymethods]
impl LocalDeployment {
    /// Stop a running deployment, making it unavailable for connections.
    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        stop_deployment(py, &self.inner().container_id)
    }

    /// Stop a running deployment by name or container ID.
    #[staticmethod]
    fn stop_deployment(py: Python<'_>, container_id_or_name: String) -> PyResult<()> {
        stop_deployment(py, &container_id_or_name)
    }
}
