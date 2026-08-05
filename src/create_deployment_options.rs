use std::time::Duration;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use atlas_local::models::{
    BindingType, CreateDeploymentOptions, CreationSource, ImageTag, MongoDBPortBinding,
};

pub(crate) struct CreateArgs {
    pub name: Option<String>,
    pub image: Option<String>,
    pub skip_pull_image: Option<bool>,
    pub image_tag: Option<String>,
    pub load_sample_data: Option<bool>,
    pub port: Option<u16>,
    pub ip: Option<String>,
    pub wait_until_healthy: Option<bool>,
    pub wait_until_healthy_timeout: Option<i64>,
    pub local_seed_location: Option<String>,
    pub mongodb_initdb_root_username: Option<String>,
    pub mongodb_initdb_root_password: Option<String>,
    pub voyage_api_key: Option<String>,
    pub do_not_track: Option<bool>,
}

fn port_binding(port: Option<u16>, ip: Option<&str>) -> PyResult<Option<MongoDBPortBinding>> {
    if port.is_none() && ip.is_none() {
        return Ok(None);
    }

    let binding_type = match ip {
        None | Some("127.0.0.1") => BindingType::Loopback,
        Some("0.0.0.0") => BindingType::AnyInterface,
        Some(ip) => BindingType::Specific {
            ip: ip.parse().map_err(|_| {
                PyValueError::new_err(format!(
                    "ip must be an IP address such as '127.0.0.1' or '0.0.0.0', got {ip:?}"
                ))
            })?,
        },
    };

    Ok(Some(MongoDBPortBinding::new(port, binding_type)))
}

fn tool_identifier(in_jupyter: bool) -> CreationSource {
    let tool = if in_jupyter {
        "PYTHON_LIB_JUPYTER"
    } else {
        "PYTHON_LIB"
    };

    CreationSource::Unknown(tool.to_owned())
}

fn running_in_jupyter() -> bool {
    std::env::var_os("JPY_PARENT_PID").is_some()
}

pub(crate) fn build_create_deployment_options(
    args: CreateArgs,
) -> PyResult<CreateDeploymentOptions> {
    let image_tag = args
        .image_tag
        .as_deref()
        .map(ImageTag::try_from)
        .transpose()
        .map_err(PyValueError::new_err)?;

    let wait_until_healthy_timeout = args
        .wait_until_healthy_timeout
        .map(|seconds| {
            u64::try_from(seconds)
                .map(Duration::from_secs)
                .map_err(|_| {
                    PyValueError::new_err(
                        "wait_until_healthy_timeout must be non-negative number of seconds",
                    )
                })
        })
        .transpose()?;

    let mongodb_port_binding = port_binding(args.port, args.ip.as_deref())?;

    let creation_source = Some(tool_identifier(running_in_jupyter()));

    Ok(CreateDeploymentOptions {
        name: args.name,
        image: args.image,
        skip_pull_image: args.skip_pull_image,
        image_tag,
        load_sample_data: args.load_sample_data,
        mongodb_port_binding,
        wait_until_healthy: args.wait_until_healthy,
        wait_until_healthy_timeout,
        creation_source,
        local_seed_location: args.local_seed_location,
        mongodb_initdb_root_username: args.mongodb_initdb_root_username,
        mongodb_initdb_root_password: args.mongodb_initdb_root_password,
        voyage_api_key: args.voyage_api_key,
        do_not_track: args.do_not_track,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn empty_args() -> CreateArgs {
        CreateArgs {
            name: None,
            image: None,
            skip_pull_image: None,
            image_tag: None,
            load_sample_data: None,
            port: None,
            ip: None,
            wait_until_healthy: None,
            wait_until_healthy_timeout: None,
            local_seed_location: None,
            mongodb_initdb_root_username: None,
            mongodb_initdb_root_password: None,
            voyage_api_key: None,
            do_not_track: None,
        }
    }

    #[test]
    fn test_leaving_binding_to_atlas_local() {
        let options = build_create_deployment_options(empty_args()).unwrap();

        assert_eq!(options.mongodb_port_binding, None);
    }

    #[test]
    fn test_port_defaults_to_loopback() {
        let args = CreateArgs {
            port: Some(27017),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(Some(27017), BindingType::Loopback))
        );
    }

    #[test]
    fn test_loopback_address_maps_to_loopback() {
        let args = CreateArgs {
            port: Some(27017),
            ip: Some("127.0.0.1".into()),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(Some(27017), BindingType::Loopback))
        );
    }

    #[test]
    fn test_unspecified_address() {
        let args = CreateArgs {
            port: Some(27017),
            ip: Some("0.0.0.0".into()),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::AnyInterface
            ))
        );
    }

    #[test]
    fn test_other_address() {
        let args = CreateArgs {
            port: Some(27017),
            ip: Some("1.2.3.4".into()),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                }
            ))
        );
    }

    #[test]
    fn test_ip_without_port() {
        let args = CreateArgs {
            ip: Some("0.0.0.0".into()),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(None, BindingType::AnyInterface))
        );
    }

    #[test]
    fn test_invalid_ip_is_rejected() {
        let args = CreateArgs {
            ip: Some("AnyInterface".into()),
            ..empty_args()
        };

        Python::initialize();
        Python::attach(|py| {
            let error = build_create_deployment_options(args).unwrap_err();
            assert!(error.is_instance_of::<PyValueError>(py));
            assert_eq!(
                error.value(py).to_string(),
                "ip must be an IP address such as '127.0.0.1' or '0.0.0.0', got \"AnyInterface\""
            );
        });
    }

    #[test]
    fn test_negative_timeout_is_rejected() {
        let args = CreateArgs {
            wait_until_healthy_timeout: Some(-1),
            ..empty_args()
        };

        Python::initialize();
        Python::attach(|py| {
            let error = build_create_deployment_options(args).unwrap_err();
            assert!(error.is_instance_of::<PyValueError>(py));
            assert_eq!(
                error.value(py).to_string(),
                "wait_until_healthy_timeout must be non-negative number of seconds"
            );
        });
    }

    #[test]
    fn test_all_fields_are_mapped() {
        let args = CreateArgs {
            name: Some("n".into()),
            image: Some("i".into()),
            image_tag: Some("latest".into()),
            skip_pull_image: Some(true),
            load_sample_data: Some(true),
            port: Some(27017),
            ip: Some("1.2.3.4".into()),
            wait_until_healthy: Some(true),
            wait_until_healthy_timeout: Some(30),
            local_seed_location: Some("/seed".into()),
            mongodb_initdb_root_username: Some("user".into()),
            mongodb_initdb_root_password: Some("pass".into()),
            voyage_api_key: Some("key".into()),
            do_not_track: Some(true),
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(options.name.as_deref(), Some("n"));
        assert_eq!(options.image.as_deref(), Some("i"));
        assert_eq!(options.image_tag, Some(ImageTag::Latest));
        assert_eq!(
            options.mongodb_initdb_root_username.as_deref(),
            Some("user")
        );
        assert_eq!(
            options.mongodb_initdb_root_password.as_deref(),
            Some("pass")
        );
        assert_eq!(options.voyage_api_key.as_deref(), Some("key"));
        assert_eq!(options.local_seed_location.as_deref(), Some("/seed"));
        assert_eq!(
            options.wait_until_healthy_timeout,
            Some(Duration::from_secs(30))
        );
        assert_eq!(options.skip_pull_image, Some(true));
        assert_eq!(options.load_sample_data, Some(true));
        assert_eq!(options.wait_until_healthy, Some(true));
        assert_eq!(options.do_not_track, Some(true));
        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                }
            ))
        );
        assert_eq!(
            options.creation_source,
            Some(tool_identifier(running_in_jupyter()))
        );
    }

    #[test]
    fn test_tool_identifier() {
        assert_eq!(
            tool_identifier(false),
            CreationSource::Unknown("PYTHON_LIB".to_owned())
        );
        assert_eq!(
            tool_identifier(true),
            CreationSource::Unknown("PYTHON_LIB_JUPYTER".to_owned())
        );
    }
}
