use atlas_local::DockerError as RsDockerError;
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

impl IntoPyErr for RsCreateDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::Timeout {
                deployment_name,
            }) => PyErr::new::<DeploymentTimeoutError, _>(format!(
                "The deployment {deployment_name} was created but did not become healthy within \
                 the configured timeout.\n\
                 Increase `wait_until_healthy_timeout`, or inspect the deployment logs for additional details."
            )),
            // Both paths mean the same thing, so they get the same type and text.
            RsCreateDeploymentError::UnhealthyDeployment(deployment_name)
            | RsCreateDeploymentError::WatchDeployment(
                RsWatchDeploymentError::UnhealthyDeployment {
                    deployment_name, ..
                },
            ) => PyErr::new::<UnhealthyDeploymentError, _>(format!(
                "The deployment {deployment_name} was created but ended up in an unhealthy \
                 state.\n\
                 Inspect the deployment logs for additional details."
            )),
            RsCreateDeploymentError::WatchDeployment(RsWatchDeploymentError::ContainerInspect(
                _,
            ))
            | RsCreateDeploymentError::GetDeploymentError(_) => {
                PyErr::new::<CreateDeploymentError, _>(
                    "The deployment was created but its status could not be verified",
                )
            }
            RsCreateDeploymentError::CreateContainer(_) => PyErr::new::<CreateDeploymentError, _>(
                "Docker failed to create the deployment's container",
            ),
            RsCreateDeploymentError::PullImage(_) => PyErr::new::<CreateDeploymentError, _>(
                "Failed to pull the deployment's image.\n\
                     Check your network connection and that the `image` and `image_tag` exist.",
            ),
            RsCreateDeploymentError::ContainerAlreadyExists(name) => {
                PyErr::new::<CreateDeploymentError, _>(format!(
                    "A deployment named {name:?} already exists.\n\
                     Delete it first, or use `get_or_create()` to reuse it."
                ))
            }
            RsCreateDeploymentError::ContainerInspect(_) => PyErr::new::<CreateDeploymentError, _>(
                "Failed to inspect the deployment's container",
            ),
            RsCreateDeploymentError::ReceiveDeployment(_) => {
                PyErr::new::<CreateDeploymentError, _>(
                    "Failed to receive confirmation of the created deployment",
                )
            }
            RsCreateDeploymentError::InvalidImage(image) => {
                PyErr::new::<CreateDeploymentError, _>(format!(
                    "The image {image:?} must not include a tag.\n\
                     Use the `image_tag` field to specify a tag."
                ))
            }
        }
    }
}

impl IntoPyErr for RsDeleteDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            RsDeleteDeploymentError::ContainerStop(_) => {
                "Failed to stop the deployment's container"
            }
            RsDeleteDeploymentError::ContainerRemove(_) => {
                "Failed to remove the deployment's container"
            }
            RsDeleteDeploymentError::GetDeployment(_) => "Failed to verify the deployment's status",
        };

        PyErr::new::<DeleteDeploymentError, _>(message)
    }
}

impl IntoPyErr for RsGetConnectionStringError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            RsGetConnectionStringError::GetDeployment(_) => {
                "Failed to verify the deployment's status"
            }
            RsGetConnectionStringError::GetMongodbUsername(_) => {
                "Failed to retrieve the deployment's MongoDB username"
            }
            RsGetConnectionStringError::GetMongodbPassword(_) => {
                "Failed to retrieve the deployment's MongoDB password"
            }
            RsGetConnectionStringError::MissingPortBinding => {
                "The deployment does not have a published MongoDB port.\n\
                 If it is stopped, call `start()` first; if it is paused, call `unpause()`.\n\
                 If it was created without a published port, recreate it with the `port` \
                 argument to make it accessible from the host."
            }
        };

        PyErr::new::<GetConnectionStringError, _>(message)
    }
}

impl IntoPyErr for RsGetDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            RsGetDeploymentError::ContainerInspect(RsDockerError::NotFound) => {
                "No local Atlas deployment found with that name or container ID.\n\
                 Use `list()` to see the existing deployments."
            }
            RsGetDeploymentError::ContainerInspect(_) => {
                "Failed to inspect the deployment's container"
            }
            RsGetDeploymentError::IntoDeployment(_) => {
                "The container is not a local Atlas deployment"
            }
        };

        PyErr::new::<GetDeploymentError, _>(message)
    }
}

impl IntoPyErr for RsGetLogsError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsGetLogsError::ContainerLogs(_) => {
                PyErr::new::<GetLogsError, _>("Failed to retrieve the deployment's logs")
            }
        }
    }
}

impl IntoPyErr for RsWatchDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsWatchDeploymentError::Timeout { deployment_name } => {
                PyErr::new::<DeploymentTimeoutError, _>(format!(
                    "Deployment {deployment_name} did not become healthy within the configured timeout.\n\
                     Increase `wait_until_healthy_timeout`, or inspect the deployment logs for additional details."
                ))
            }
            RsWatchDeploymentError::UnhealthyDeployment {
                deployment_name, ..
            } => PyErr::new::<UnhealthyDeploymentError, _>(format!(
                "Deployment {deployment_name} is unhealthy.\n\
                 Inspect the deployment logs for additional details."
            )),
            RsWatchDeploymentError::ContainerInspect(_) => PyErr::new::<WatchDeploymentError, _>(
                "Failed to inspect the deployment's container",
            ),
        }
    }
}

impl IntoPyErr for RsStartDeploymentError {
    fn into_pyerr(self) -> PyErr {
        match self {
            RsStartDeploymentError::ContainerStart(_) => PyErr::new::<StartDeploymentError, _>(
                "Failed to start the deployment.\n\
                     A paused deployment cannot be started; use `unpause()` or `stop()` first.",
            ),
            RsStartDeploymentError::GetDeployment(_) => {
                PyErr::new::<StartDeploymentError, _>("Failed to verify the deployment's status")
            }
            RsStartDeploymentError::WatchDeployment(error) => error.into_pyerr(),
        }
    }
}

impl IntoPyErr for RsStopDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            // Stopping an already stopped or paused deployment is not an error.
            RsStopDeploymentError::ContainerStop(_) => "Failed to stop the deployment's container",
            RsStopDeploymentError::GetDeployment(_) => "Failed to verify the deployment's status",
        };

        PyErr::new::<StopDeploymentError, _>(message)
    }
}

impl IntoPyErr for RsPauseDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            RsPauseDeploymentError::ContainerPause(_) => {
                "Failed to pause the deployment.\n\
                 Only a running deployment can be paused; use `unpause()` if it is already paused."
            }
            RsPauseDeploymentError::GetDeployment(_) => "Failed to verify the deployment's status",
        };

        PyErr::new::<PauseDeploymentError, _>(message)
    }
}

impl IntoPyErr for RsUnpauseDeploymentError {
    fn into_pyerr(self) -> PyErr {
        let message = match self {
            RsUnpauseDeploymentError::ContainerUnpause(_) => {
                "Failed to unpause the deployment.\n\
                 Only a paused deployment can be unpaused; use `start()` if it is stopped."
            }
            RsUnpauseDeploymentError::GetDeployment(_) => {
                "Failed to verify the deployment's status"
            }
        };

        PyErr::new::<UnpauseDeploymentError, _>(message)
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
    fn test_create_deployment_container_error_into_pyerr() {
        let errors = [
            RsCreateDeploymentError::CreateContainer(RsDockerError::ServerError),
            RsCreateDeploymentError::CreateContainer(RsDockerError::Other {
                status_code: Some(503),
                message: "Service Unavailable".to_string(),
            }),
        ];

        Python::initialize();
        Python::attach(|py| {
            for error in errors {
                let py_error = error.into_pyerr();
                assert!(py_error.is_instance_of::<CreateDeploymentError>(py));
                assert_eq!(
                    py_error.value(py).to_string(),
                    "Docker failed to create the deployment's container"
                );
            }
        });
    }

    #[test]
    fn test_deployment_not_found_into_pyerr() {
        let error = RsGetDeploymentError::ContainerInspect(RsDockerError::NotFound);

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<GetDeploymentError>(py));
            assert_eq!(
                py_error.value(py).to_string(),
                "No local Atlas deployment found with that name or container ID.\n\
                 Use `list()` to see the existing deployments."
            );
        });
    }

    #[test]
    fn test_start_timeout_maps_to_timeout_error() {
        let error = RsStartDeploymentError::WatchDeployment(RsWatchDeploymentError::Timeout {
            deployment_name: "test_deployment".to_owned(),
        });

        Python::initialize();
        Python::attach(|py| {
            let py_error = error.into_pyerr();
            assert!(py_error.is_instance_of::<DeploymentTimeoutError>(py));
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
                "Failed to unpause the deployment.\n\
                 Only a paused deployment can be unpaused; use `start()` if it is stopped."
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
                "Deployment test_deployment is unhealthy.\n\
                 Inspect the deployment logs for additional details."
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
                "The deployment test_deployment was created but did not become healthy within \
                 the configured timeout.\n\
                 Increase `wait_until_healthy_timeout`, or inspect the deployment logs for additional details."
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
                "The deployment test_deployment was created but ended up in an unhealthy \
                 state.\n\
                 Inspect the deployment logs for additional details."
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
                RsStartDeploymentError::ContainerStart("error".into())
                    .into_pyerr()
                    .is_instance_of::<StartDeploymentError>(py)
            );
            assert!(
                RsStopDeploymentError::ContainerStop("error".into())
                    .into_pyerr()
                    .is_instance_of::<StopDeploymentError>(py)
            );
            assert!(
                RsGetLogsError::ContainerLogs("error".into())
                    .into_pyerr()
                    .is_instance_of::<GetLogsError>(py)
            );
        });
    }
}
