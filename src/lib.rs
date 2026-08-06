use pyo3::prelude::*;

mod create;
mod create_deployment_options;
mod deployment;
mod exceptions;
mod get;
mod list;
mod runtime;

#[pymodule]
mod atlas_local {
    use pyo3::prelude::*;

    #[pymodule_export]
    pub use crate::deployment::LocalDeployment;

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        crate::exceptions::register(module)
    }
}
