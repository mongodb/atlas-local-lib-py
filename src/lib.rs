use pyo3::prelude::*;

mod deployment;
mod exceptions;

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
