use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

#[pymethods]
impl LocalDeployment {
    /// Returns all existing Atlas Local deployments.
    ///
    /// Returns an empty Python list if no deployments exist.
    #[staticmethod]
    fn list(py: Python<'_>) -> PyResult<Vec<Py<LocalDeployment>>> {
        let deployments =
            runtime_block_on(py, |client| async move { client.list_deployments().await })?;

        deployments
            .into_iter()
            .map(|deployment| Py::new(py, LocalDeployment::from(deployment)))
            .collect()
    }
}
