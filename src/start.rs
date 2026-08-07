use std::time::Duration;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use atlas_local::models::StartDeploymentOptions;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

fn parse_timeout(seconds: Option<i64>) -> PyResult<Option<Duration>> {
    seconds
        .map(|seconds| {
            u64::try_from(seconds)
                .map(Duration::from_secs)
                .map_err(|_| {
                    PyValueError::new_err(
                        "wait_until_healthy_timeout must be non-negative number of seconds",
                    )
                })
        })
        .transpose()
}

fn run_start(
    py: Python<'_>,
    container_id_or_name: &str,
    wait_until_healthy: bool,
    wait_until_healthy_timeout: Option<i64>,
) -> PyResult<()> {
    let options = StartDeploymentOptions {
        wait_until_healthy: Some(wait_until_healthy),
        wait_until_healthy_timeout: parse_timeout(wait_until_healthy_timeout)?,
    };

    runtime_block_on(py, |client| async move {
        client.start_deployment(container_id_or_name, options).await
    })
}

#[pymethods]
impl LocalDeployment {
    /// Start a stopped or paused deployment.
    #[pyo3(signature = (wait_until_healthy=true, wait_until_healthy_timeout=None))]
    fn start(
        &self,
        py: Python<'_>,
        wait_until_healthy: bool,
        wait_until_healthy_timeout: Option<i64>,
    ) -> PyResult<()> {
        run_start(
            py,
            &self.inner().container_id,
            wait_until_healthy,
            wait_until_healthy_timeout,
        )
    }

    /// Start a stopped or paused deployment by name or container ID.
    #[staticmethod]
    #[pyo3(signature = (
        container_id_or_name,
        wait_until_healthy=true,
        wait_until_healthy_timeout=None
    ))]
    fn start_deployment(
        py: Python<'_>,
        container_id_or_name: String,
        wait_until_healthy: bool,
        wait_until_healthy_timeout: Option<i64>,
    ) -> PyResult<()> {
        run_start(
            py,
            &container_id_or_name,
            wait_until_healthy,
            wait_until_healthy_timeout,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeout_is_converted_to_seconds() {
        assert_eq!(
            parse_timeout(Some(30)).unwrap(),
            Some(Duration::from_secs(30))
        );
    }

    #[test]
    fn test_missing_timeout_leaves_the_default_to_atlas_local() {
        assert_eq!(parse_timeout(None).unwrap(), None);
    }

    #[test]
    fn test_negative_timeout_is_rejected() {
        Python::initialize();
        Python::attach(|py| {
            let error = parse_timeout(Some(-1)).unwrap_err();

            assert!(error.is_instance_of::<PyValueError>(py));
            assert_eq!(
                error.value(py).to_string(),
                "wait_until_healthy_timeout must be non-negative number of seconds"
            );
        });
    }
}
