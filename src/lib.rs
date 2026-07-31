use pyo3::prelude::*;

mod exceptions;

#[pymodule]
mod atlas_local {
    use pyo3::prelude::*;

    #[pyfunction]
    fn test_error(name: &str) -> PyResult<String> {
        use crate::exceptions::IntoPyErr;


        fn some_call(name: &str) -> Result<String, ::atlas_local::client::StartDeploymentError> {
            Err(::atlas_local::client::StartDeploymentError::ContainerStart(format!(
                "no such container: {name}"
            )))
        }

        let started = some_call(name).map_err(IntoPyErr::into_pyerr)?;
        Ok(started)
    }

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        crate::exceptions::register(module)
    }
}
