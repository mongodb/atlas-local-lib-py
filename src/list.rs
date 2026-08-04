use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

#[pymethods]
impl LocalDeployment {
    /// Returns all existing Atlas Local deployments.
    ///
    /// Returns an empty Python list if no deployments exist.
    #[staticmethod]
    fn list(py: Python<'_>) -> PyResult<Vec<Py<LocalDeployment>>> {
        let context = get_context()?;
        let client = context.client()?;

        let deployments = py
            .detach(|| {
                context
                    .runtime
                    .block_on(client.list_deployments())
            })
            .into_pyresult()?;

        deployments
            .into_iter()
            .map(|deployment| Py::new(py, LocalDeployment::from(deployment)))
            .collect()
    }
}
