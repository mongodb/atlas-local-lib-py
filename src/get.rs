use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

#[pymethods]
impl LocalDeployment {
    /// Retrieve a deployment by name or container ID.
    #[staticmethod]
    fn get(py: Python<'_>, container_id_or_name: String) -> PyResult<Self> {
        let deployment = runtime_block_on(py, |client| async move {
            client.get_deployment(&container_id_or_name).await
        })?;

        Ok(LocalDeployment::from(deployment))
    }
}
