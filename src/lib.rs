use pyo3::prelude::*;

mod connection_string;
mod create;
mod create_deployment_options;
mod delete;
mod deployment;
mod exceptions;
mod get;
mod get_or_create;
mod list;
mod option_conflicts;
mod logs;
mod pause;
mod runtime;
mod start;
mod stop;
mod unpause;

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
