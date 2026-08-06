use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::exceptions::IntoPyResult;
use crate::runtime::get_context;

#[pymethods]
impl LocalDeployment {
    /// Retrieve a deployment by name or container ID.
    #[staticmethod]
    fn get(py: Python<'_>, container_id_or_name: String) -> PyResult<Self> {
        let context = get_context()?;
        let client = context.client()?;

        let deployment = py
            .detach(|| {
                context
                    .runtime
                    .block_on(client.get_deployment(&container_id_or_name))
            })
            .into_pyresult()?;

        Ok(LocalDeployment::from(deployment))
    }
}
