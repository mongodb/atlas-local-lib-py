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

create_exception!(
    atlas_local,
    DockerConnectionError,
    AtlasLocalError,
    "Could not connect to Docker. Make sure Docker is installed and running: https://docs.docker.com/get-docker/"
);

#[allow(dead_code)]
pub(crate) trait IntoPyResult<T> {
    fn into_pyresult(self) -> Result<T, PyErr>;
}

impl<T, E: IntoPyErr> IntoPyResult<T> for Result<T, E> {
    fn into_pyresult(self) -> Result<T, PyErr> {
        self.map_err(IntoPyErr::into_pyerr)
    }
}

pub(crate) trait IntoPyErr {
    fn into_pyerr(self) -> PyErr;
}

macro_rules! into_pyerr {
    ($rust_error:ty => $python_error:ty) => {
        impl IntoPyErr for $rust_error {
            fn into_pyerr(self) -> PyErr {
                PyErr::new::<$python_error, _>(self.to_string())
            }
        }
    };
}

into_pyerr!(RsStartDeploymentError => StartDeploymentError);
into_pyerr!(RsStopDeploymentError => StopDeploymentError);
into_pyerr!(RsPauseDeploymentError => PauseDeploymentError);
into_pyerr!(RsUnpauseDeploymentError => UnpauseDeploymentError);
into_pyerr!(RsGetDeploymentError => GetDeploymentError);
into_pyerr!(RsGetConnectionStringError => GetConnectionStringError);
into_pyerr!(RsGetLogsError => GetLogsError);
into_pyerr!(RsDeleteDeploymentError => DeleteDeploymentError);

impl IntoPyErr for RsCreateDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = self.to_string();

        match self {
            RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::Timeout {
                ..
            }) => PyErr::new::<DeploymentTimeoutError, _>(message),

            RsCreateDeploymentError::WatchDeployment(
                RsWatchDeploymentError::UnhealthyDeployment { .. },
            ) => PyErr::new::<UnhealthyDeploymentError, _>(message),

            RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::ContainerInspect(
                _,
            )) => PyErr::new::<CreateDeploymentError, _>(message),

            RsCreateDeploymentError::CreateContainer(_)
            | RsCreateDeploymentError::PullImage(_)
            | RsCreateDeploymentError::ContainerAlreadyExists(_)
            | RsCreateDeploymentError::ContainerInspect(_)
            | RsCreateDeploymentError::UnhealthyDeployment(_)
            | RsCreateDeploymentError::GetDeploymentError(_)
            | RsCreateDeploymentError::ReceiveDeployment(_)
            | RsCreateDeploymentError::InvalidImage(_) => {
                PyErr::new::<CreateDeploymentError, _>(message)
            }
        }
    }
}

impl IntoPyErr for RsWatchDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = self.to_string();

        match self {
            RsWatchDeploymentError::Timeout { .. } => {
                PyErr::new::<DeploymentTimeoutError, _>(message)
            }
            RsWatchDeploymentError::UnhealthyDeployment { .. } => {
                PyErr::new::<UnhealthyDeploymentError, _>(message)
            }
            RsWatchDeploymentError::ContainerInspect(_) => {
                PyErr::new::<WatchDeploymentError, _>(message)
            }
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
    module.add("GetLogsError", module.py().get_type::<GetLogsError>())?;
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
    module.add(
        "DockerConnectionError",
        module.py().get_type::<DockerConnectionError>(),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_local::DockerError as RsDockerError;

    #[test]
    fn test_create_deployment_server_error_into_pyerr() {
        let error = RsCreateDeploymentError::CreateContainer(RsDockerError::ServerError);

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<CreateDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Failed to create container: internal server error"
            );
        });
    }

    #[test]
    fn test_create_deployment_other_error_into_pyerr() {
        let error = RsCreateDeploymentError::CreateContainer(RsDockerError::Other {
            status_code: Some(503),
            message: ("Service Unavailable").to_string(),
        });

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<CreateDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Failed to create container: docker error (status Some(503)): Service Unavailable"
            );
        });
    }

    #[test]
    fn test_container_unpause_error_into_pyerr() {
        let error =
            RsUnpauseDeploymentError::ContainerUnpause("container is not paused".to_string());

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<UnpauseDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Failed to unpause container: container is not paused"
            );
        });
    }

    #[test]
    fn test_watch_deployment_unhealthy_error_into_pyerr() {
        let error = RsWatchDeploymentError::UnhealthyDeployment {
            deployment_name: "test_deployment".to_string(),
            status: (atlas_local::ContainerHealthStatus::Unhealthy),
        };

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<UnhealthyDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Deployment test_deployment is not healthy [status: unhealthy]"
            );
        });
    }

    #[test]
    fn test_create_timeout_maps_to_timeout_error() {
        let error = RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::Timeout {
            deployment_name: "test_deployment".to_owned(),
        });

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();

            assert!(py_error.is_instance_of::<DeploymentTimeoutError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Error when waiting for deployment to become healthy: \
                Timeout while waiting for container test_deployment to become healthy"
            );
        });
    }

    #[test]
    fn test_create_unhealthy_maps_to_unhealthy_error() {
        let error =
            RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::UnhealthyDeployment {
                deployment_name: "test_deployment".to_owned(),
                status: atlas_local::ContainerHealthStatus::Unhealthy,
            });

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();

            assert!(py_error.is_instance_of::<UnhealthyDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "Error when waiting for deployment to become healthy: \
                Deployment test_deployment is not healthy [status: unhealthy]"
            );
        });
    }

    #[test]
    fn test_macro_generated_mappings() {
        Python::initialize();
        Python::attach(|py| {
            let docker_error = || RsDockerError::ServerError;

            assert!(
                RsGetDeploymentError::ContainerInspect(docker_error())
                    .into_pyerr()
                    .is_instance_of::<GetDeploymentError>(py)
            );
            assert!(
                RsStartDeploymentError::ContainerStart("boom".into())
                    .into_pyerr()
                    .is_instance_of::<StartDeploymentError>(py)
            );
            assert!(
                RsStopDeploymentError::ContainerStop("boom".into())
                    .into_pyerr()
                    .is_instance_of::<StopDeploymentError>(py)
            );
            assert!(
                RsGetLogsError::ContainerLogs("boom".into())
                    .into_pyerr()
                    .is_instance_of::<GetLogsError>(py)
            );
        });
    }
}
