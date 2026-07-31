use atlas_local::client::{
    CreateDeploymentError as RsCreateDeploymentError,
    DeleteDeploymentError as RsDeleteDeploymentError,
    GetConnectionStringError as RsGetConnectionStringError,
    GetDeploymentError as RsGetDeploymentError, GetLogsError as RsGetLogsError,
    PauseDeploymentError as RsPauseDeploymentError, StartDeploymentError as RsStartDeploymentError,
    StopDeploymentError as RsStopDeploymentError,
    UnpauseDeploymentError as RsUnpauseDeploymentError,
    WatchDeploymentError as RsWatchDeploymentError,
};

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Base class
create_exception!(
    atlas_local,
    AtlasLocalError,
    PyException,
    "Base class for every error raised by atlas_local."
);

// Operations
create_exception!(
    atlas_local,
    DeploymentError,
    AtlasLocalError,
    "Base class for errors about a local Atlas deployment."
);
create_exception!(
    atlas_local,
    CreateDeploymentError,
    DeploymentError,
    "Creating a local deployment failed."
);
create_exception!(
    atlas_local,
    GetDeploymentError,
    DeploymentError,
    "Retrieving or listing local deployments failed."
);
create_exception!(
    atlas_local,
    DeleteDeploymentError,
    DeploymentError,
    "Deleting a local deployment failed."
);
create_exception!(
    atlas_local,
    WatchDeploymentError,
    DeploymentError,
    "Waiting for a deployment to become healthy failed."
);
create_exception!(
    atlas_local,
    GetConnectionStringError,
    DeploymentError,
    "Building the connection string for a deployment failed."
);
create_exception!(
    atlas_local,
    GetLogsError,
    DeploymentError,
    "Reading the logs of a deployment failed."
);
create_exception!(
    atlas_local,
    PauseDeploymentError,
    DeploymentError,
    "Pausing a deployment failed."
);
create_exception!(
    atlas_local,
    StartDeploymentError,
    DeploymentError,
    "Starting a deployment failed."
);
create_exception!(
    atlas_local,
    StopDeploymentError,
    DeploymentError,
    "Stopping a deployment failed."
);
create_exception!(
    atlas_local,
    UnpauseDeploymentError,
    DeploymentError,
    "Unpausing a deployment failed."
);
create_exception!(
    atlas_local,
    UnhealthyDeploymentError,
    WatchDeploymentError,
    "The deployment is unhealthy."
);
create_exception!(
    atlas_local,
    DeploymentTimeoutError,
    WatchDeploymentError,
    "The deployment did not become healthy within the timeout."
);

#[allow(dead_code)]
pub(crate) trait IntoPyErr {
    fn into_pyerr(self) -> PyErr;
}

impl IntoPyErr for RsStartDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsStartDeploymentError::ContainerStart(message) => {
                PyErr::new::<StartDeploymentError, _>(message)
            }
            RsStartDeploymentError::GetDeployment(err) => {
                PyErr::new::<StartDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
        }
    }
}

impl IntoPyErr for RsStopDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsStopDeploymentError::ContainerStop(message) => {
                PyErr::new::<StopDeploymentError, _>(message)
            }
            RsStopDeploymentError::GetDeployment(err) => {
                PyErr::new::<StopDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
        }
    }
}

impl IntoPyErr for RsPauseDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsPauseDeploymentError::ContainerPause(message) => {
                PyErr::new::<PauseDeploymentError, _>(message)
            }
            RsPauseDeploymentError::GetDeployment(err) => {
                PyErr::new::<PauseDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
        }
    }
}

impl IntoPyErr for RsUnpauseDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsUnpauseDeploymentError::ContainerUnpause(message) => {
                PyErr::new::<UnpauseDeploymentError, _>(message)
            }
            RsUnpauseDeploymentError::GetDeployment(err) => {
                PyErr::new::<UnpauseDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
        }
    }
}

impl IntoPyErr for RsGetDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsGetDeploymentError::ContainerInspect(err) => {
                PyErr::new::<GetDeploymentError, _>(format!("Failed to inspect container: {err}"))
            }
            RsGetDeploymentError::IntoDeployment(err) => PyErr::new::<GetDeploymentError, _>(
                format!("Failed to convert container into deployment: {err}"),
            ),
        }
    }
}

impl IntoPyErr for RsCreateDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsCreateDeploymentError::CreateContainer(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!("Failed to create container: {err}"))
            }
            RsCreateDeploymentError::PullImage(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!("Failed to pull image: {err}"))
            }
            RsCreateDeploymentError::ContainerAlreadyExists(message) => {
                PyErr::new::<CreateDeploymentError, _>(message)
            }
            RsCreateDeploymentError::ContainerInspect(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!(
                    "Failed to inspect container: {err}"
                ))
            }
            RsCreateDeploymentError::UnhealthyDeployment(message) => {
                PyErr::new::<CreateDeploymentError, _>(message)
            }
            RsCreateDeploymentError::GetDeploymentError(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
            RsCreateDeploymentError::WatchDeployment(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!("Failed to watch deployment: {err}"))
            }
            RsCreateDeploymentError::ReceiveDeployment(err) => {
                PyErr::new::<CreateDeploymentError, _>(format!(
                    "Failed to receive deployment: {err}"
                ))
            }
            RsCreateDeploymentError::InvalidImage(message) => {
                PyErr::new::<CreateDeploymentError, _>(message)
            }
        }
    }
}

impl IntoPyErr for RsGetConnectionStringError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsGetConnectionStringError::GetDeployment(err) => {
                PyErr::new::<GetConnectionStringError, _>(format!(
                    "Failed to get deployment: {err}"
                ))
            }
            RsGetConnectionStringError::GetMongodbUsername(err) => {
                PyErr::new::<GetConnectionStringError, _>(format!(
                    "Failed to get MongoDB username: {err}"
                ))
            }
            RsGetConnectionStringError::GetMongodbPassword(err) => {
                PyErr::new::<GetConnectionStringError, _>(format!(
                    "Failed to get MongoDB password: {err}"
                ))
            }
            RsGetConnectionStringError::MissingPortBinding => {
                PyErr::new::<GetConnectionStringError, _>("The deployment has no port binding.")
            }
        }
    }
}

impl IntoPyErr for RsGetLogsError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsGetLogsError::ContainerLogs(message) => PyErr::new::<GetLogsError, _>(message),
        }
    }
}

impl IntoPyErr for RsDeleteDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsDeleteDeploymentError::ContainerStop(err) => {
                PyErr::new::<DeleteDeploymentError, _>(format!("Failed to stop container: {err}"))
            }
            RsDeleteDeploymentError::ContainerRemove(err) => {
                PyErr::new::<DeleteDeploymentError, _>(format!("Failed to remove container: {err}"))
            }
            RsDeleteDeploymentError::GetDeployment(err) => {
                PyErr::new::<DeleteDeploymentError, _>(format!("Failed to get deployment: {err}"))
            }
        }
    }
}

impl IntoPyErr for RsWatchDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsWatchDeploymentError::ContainerInspect(err) => {
                PyErr::new::<WatchDeploymentError, _>(format!("Failed to inspect container: {err}"))
            }
            RsWatchDeploymentError::Timeout { deployment_name } => {
                PyErr::new::<DeploymentTimeoutError, _>(format!(
                    "Timeout while waiting for deployment {deployment_name} to become healthy."
                ))
            }
            RsWatchDeploymentError::UnhealthyDeployment {
                deployment_name,
                status,
            } => PyErr::new::<UnhealthyDeploymentError, _>(format!(
                "Deployment {deployment_name} is unhealthy. Status: {status:?}"
            )),
        }
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("AtlasLocalError", module.py().get_type::<AtlasLocalError>())?;
    module.add("DeploymentError", module.py().get_type::<DeploymentError>())?;
    module.add(
        "CreateDeploymentError",
        module.py().get_type::<CreateDeploymentError>(),
    )?;
    module.add(
        "GetDeploymentError",
        module.py().get_type::<GetDeploymentError>(),
    )?;
    module.add(
        "DeleteDeploymentError",
        module.py().get_type::<DeleteDeploymentError>(),
    )?;
    module.add(
        "WatchDeploymentError",
        module.py().get_type::<WatchDeploymentError>(),
    )?;
    module.add(
        "GetConnectionStringError",
        module.py().get_type::<GetConnectionStringError>(),
    )?;
    module.add(
        "GetLogsError",
        module.py().get_type::<GetLogsError>(),
    )?;
    module.add(
        "PauseDeploymentError",
        module.py().get_type::<PauseDeploymentError>(),
    )?;
    module.add(
        "StartDeploymentError",
        module.py().get_type::<StartDeploymentError>(),
    )?;
    module.add(
        "StopDeploymentError",
        module.py().get_type::<StopDeploymentError>(),
    )?;
    module.add(
        "UnpauseDeploymentError",
        module.py().get_type::<UnpauseDeploymentError>(),
    )?;
    module.add(
        "UnhealthyDeploymentError",
        module.py().get_type::<UnhealthyDeploymentError>(),
    )?;
    module.add(
        "DeploymentTimeoutError",
        module.py().get_type::<DeploymentTimeoutError>(),
    )?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use atlas_local::{DockerError as RsDockerError};

    #[test]
    fn test_create_deployment_server_error_into_pyerr() {
        let error = RsCreateDeploymentError::CreateContainer(RsDockerError::ServerError);

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<CreateDeploymentError>(py));
            assert_eq!(py_error.value(py).to_string(), "Failed to create container: internal server error");
        });
    }

    #[test]
    fn test_create_deployment_other_error_into_pyerr() {
        let error = RsCreateDeploymentError::CreateContainer(RsDockerError::Other { status_code: Some(503), message: ("Service Unavailable").to_string() });

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<CreateDeploymentError>(py));
            assert_eq!(py_error.value(py).to_string(), "Failed to create container: docker error (status Some(503)): Service Unavailable");
        });
    }

    #[test]
    fn test_container_unpause_error_into_pyerr() {
        let error = RsUnpauseDeploymentError::ContainerUnpause("Failed to unpause container".to_string());

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<UnpauseDeploymentError>(py));
            assert_eq!(py_error.value(py).to_string(), "Failed to unpause container");
        });
    }

    #[test]
    fn test_watch_deployment_unhealthy_error_into_pyerr() {
        let error = RsWatchDeploymentError::UnhealthyDeployment { deployment_name: "test_deployment".to_string(), status: (atlas_local::ContainerHealthStatus::Unhealthy) };

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<WatchDeploymentError>(py));
            assert_eq!(py_error.value(py).to_string(), "Deployment test_deployment is unhealthy. Status: Unhealthy");
        });
    }
}