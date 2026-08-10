//! Comparison between the options a caller asked for and the deployment that
//! already exists under that name.

use atlas_local::models::{CreateDeploymentOptions, Deployment, MongoDBPortBinding};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn compare<T: PartialEq>(
    field: &str,
    requested: Option<&T>,
    existing: Option<&T>,
    conflicts: &mut Vec<String>,
) {
    let Some(requested) = requested else {
        return;
    };

    if existing != Some(requested) {
        conflicts.push(field.to_owned());
    }
}

fn compare_port_binding(
    requested: Option<&MongoDBPortBinding>,
    existing: Option<&MongoDBPortBinding>,
    conflicts: &mut Vec<String>,
) {
    let Some(requested) = requested else {
        return;
    };

    let Some(existing) = existing else {
        conflicts.push("port/ip".to_owned());
        return;
    };

    // Only compare the port when the caller explicitly provided one.
    if requested.port.is_some() && requested.port != existing.port {
        conflicts.push("port".to_owned());
    }

    if requested.binding_type != existing.binding_type {
        conflicts.push("ip (the deployment is bound to a different address)".to_owned());
    }
}

pub(crate) fn ensure_options_match(
    name: &str,
    deployment: &Deployment,
    options: &CreateDeploymentOptions,
) -> PyResult<()> {
    let mut conflicts = Vec::new();

    compare(
        "image",
        options.image.as_ref(),
        deployment.image.as_ref(),
        &mut conflicts,
    );

    compare(
        "image_tag",
        options.image_tag.as_ref(),
        deployment.image_tag.as_ref(),
        &mut conflicts,
    );

    compare_port_binding(
        options.mongodb_port_binding.as_ref(),
        deployment.port_bindings.as_ref(),
        &mut conflicts,
    );

    compare(
        "local_seed_location",
        options.local_seed_location.as_ref(),
        deployment.local_seed_location.as_ref(),
        &mut conflicts,
    );

    compare(
        "load_sample_data",
        options.load_sample_data.as_ref(),
        deployment.mongodb_load_sample_data.as_ref(),
        &mut conflicts,
    );

    compare(
        "mongodb_initdb_root_username",
        options.mongodb_initdb_root_username.as_ref(),
        deployment.mongodb_initdb_root_username.as_ref(),
        &mut conflicts,
    );

    compare(
        "mongodb_initdb_root_password",
        options.mongodb_initdb_root_password.as_ref(),
        deployment.mongodb_initdb_root_password.as_ref(),
        &mut conflicts,
    );

    compare(
        "voyage_api_key",
        options.voyage_api_key.as_ref(),
        deployment.voyage_api_key.as_ref(),
        &mut conflicts,
    );

    compare(
        "do_not_track",
        options.do_not_track.as_ref(),
        Some(&deployment.do_not_track),
        &mut conflicts,
    );

    if conflicts.is_empty() {
        return Ok(());
    }

    Err(PyValueError::new_err(format!(
        "Deployment {name:?} already exists and does not match the requested \
         options: {}. Delete it first or omit the conflicting options.",
        conflicts.join(", ")
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_local::models::{BindingType, ImageTag, MongodbType, State};
    use std::net::{IpAddr, Ipv4Addr};

    fn deployment_fixture() -> Deployment {
        Deployment {
            container_id: "a-container-id".to_owned(),
            name: Some("a-deployment".to_owned()),
            state: State::Running,
            port_bindings: Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                },
            )),
            image: Some("an-image".to_owned()),
            image_tag: Some(ImageTag::Latest),
            mongodb_type: MongodbType::Community,
            mongodb_version: "8.0.0".parse().unwrap(),
            creation_source: None,
            local_seed_location: Some("/seed".to_owned()),
            mongodb_initdb_database: None,
            mongodb_initdb_root_password_file: None,
            mongodb_initdb_root_password: Some("a-password".to_owned()),
            mongodb_initdb_root_username_file: None,
            mongodb_initdb_root_username: Some("a-username".to_owned()),
            mongodb_load_sample_data: Some(true),
            voyage_api_key: Some("a-key".to_owned()),
            mongot_log_file: None,
            runner_log_file: None,
            do_not_track: true,
            telemetry_base_url: None,
        }
    }

    fn requested_options_matching() -> CreateDeploymentOptions {
        CreateDeploymentOptions {
            name: Some("a-deployment".to_owned()),
            image: Some("an-image".to_owned()),
            image_tag: Some(ImageTag::Latest),
            mongodb_port_binding: Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                },
            )),
            local_seed_location: Some("/seed".to_owned()),
            load_sample_data: Some(true),
            mongodb_initdb_root_username: Some("a-username".to_owned()),
            mongodb_initdb_root_password: Some("a-password".to_owned()),
            voyage_api_key: Some("a-key".to_owned()),
            do_not_track: Some(true),
            ..Default::default()
        }
    }

    fn requested_options_not_matching() -> CreateDeploymentOptions {
        CreateDeploymentOptions {
            name: Some("another-deployment".to_owned()),
            image: Some("another-image".to_owned()),
            image_tag: Some(ImageTag::Preview),
            mongodb_port_binding: Some(MongoDBPortBinding::new(
                Some(27018),
                BindingType::AnyInterface,
            )),
            local_seed_location: Some("/another_seed".to_owned()),
            load_sample_data: Some(false),
            mongodb_initdb_root_username: Some("another-username".to_owned()),
            mongodb_initdb_root_password: Some("another-password".to_owned()),
            voyage_api_key: Some("another-key".to_owned()),
            do_not_track: Some(false),
            ..Default::default()
        }
    }

    #[test]
    fn test_fully_matching_deployment_accepted() {
        Python::initialize();
        Python::attach(|_| {
            let result = ensure_options_match(
                "a-deployment",
                &deployment_fixture(),
                &requested_options_matching(),
            );

            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_not_matching_deployment_rejected() {
        Python::initialize();
        Python::attach(|py| {
            let error = ensure_options_match(
                "a-deployment",
                &deployment_fixture(),
                &requested_options_not_matching(),
            )
            .unwrap_err();

            assert_eq!(
                error.value(py).to_string(),
                "Deployment \"a-deployment\" already exists and does not match the requested \
                 options: image, image_tag, port, ip (the deployment is bound to a different \
                 address), local_seed_location, load_sample_data, mongodb_initdb_root_username, \
                 mongodb_initdb_root_password, voyage_api_key, do_not_track. Delete it first or \
                 omit the conflicting options."
            );
        });
    }

    fn conflicts_of(
        requested: Option<MongoDBPortBinding>,
        existing: Option<MongoDBPortBinding>,
    ) -> Vec<String> {
        let mut conflicts = Vec::new();

        compare_port_binding(requested.as_ref(), existing.as_ref(), &mut conflicts);

        conflicts
    }

    #[test]
    fn test_option_not_requested_is_ignored() {
        let mut conflicts = Vec::new();

        compare("image", None, Some(&"an-image"), &mut conflicts);

        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_option_missing_from_deployment_is_conflict() {
        let mut conflicts = Vec::new();

        compare("image", Some(&"an-image"), None, &mut conflicts);

        assert_eq!(conflicts, vec!["image".to_owned()]);
    }

    #[test]
    fn test_binding_without_port_does_not_compare_port() {
        let conflicts = conflicts_of(
            Some(MongoDBPortBinding::new(None, BindingType::AnyInterface)),
            Some(MongoDBPortBinding::new(
                Some(27018),
                BindingType::AnyInterface,
            )),
        );

        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_a_deployment_without_a_binding_is_a_conflict() {
        let conflicts = conflicts_of(
            Some(MongoDBPortBinding::new(Some(27017), BindingType::Loopback)),
            None,
        );

        assert_eq!(conflicts, vec!["port/ip".to_owned()]);
    }

    #[test]
    fn test_different_specific_ip_reported() {
        let conflicts = conflicts_of(
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                },
            )),
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Specific {
                    ip: IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8)),
                },
            )),
        );

        assert_eq!(
            conflicts,
            vec!["ip (the deployment is bound to a different address)".to_owned()]
        );
    }

    #[test]
    fn test_binding_not_requested_ignored() {
        let conflicts = conflicts_of(
            None,
            Some(MongoDBPortBinding::new(
                Some(27017),
                BindingType::Loopback,
            )),
        );

        assert!(conflicts.is_empty());
    }


}
