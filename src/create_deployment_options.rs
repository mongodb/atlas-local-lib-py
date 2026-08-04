use std::time::Duration;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use atlas_local::models::{
    BindingType,
    CreationSource,
    CreateDeploymentOptions,
    ImageTag,
    MongoDBPortBinding,
};

pub(crate) struct CreateArgs {
    pub name: Option<String>,
    pub image: Option<String>,
    pub skip_pull_image: Option<bool>,
    pub image_tag: Option<String>,
    pub load_sample_data: Option<bool>,
    pub mongodb_port_binding: Option<u16>,
    pub wait_until_healthy: Option<bool>,
    pub wait_until_healthy_timeout: Option<i64>,
    pub local_seed_location: Option<String>,
    pub mongodb_initdb_root_username: Option<String>,
    pub mongodb_initdb_root_password: Option<String>,
    pub voyage_api_key: Option<String>,
    pub do_not_track: Option<bool>,
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
                        "wait_until_healthy_timeout must be non-negative",
                    )
                })
        })
        .transpose()?;

    let mongodb_port_binding = args.mongodb_port_binding.map(|port| {
        MongoDBPortBinding::new(Some(port), BindingType::Loopback)
    });

    let creation_source = Some(CreationSource::Unknown("PYTHON".to_owned()));

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

    fn empty_args() -> CreateArgs {
        CreateArgs {
            name: None,
            image: None,
            skip_pull_image: None,
            image_tag: None,
            load_sample_data: None,
            mongodb_port_binding: None,
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
    fn test_port_binding_is_bound_to_loopback() {
        let args = CreateArgs {
            mongodb_port_binding: Some(27017),
            ..empty_args()
        };

        let options = build_create_deployment_options(args).unwrap();

        assert_eq!(
            options.mongodb_port_binding,
            Some(MongoDBPortBinding::new(Some(27017), BindingType::Loopback))
        );
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
                "wait_until_healthy_timeout must be non-negative"
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
            mongodb_port_binding: Some(27017),
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
        assert_eq!(options.mongodb_initdb_root_username.as_deref(), Some("user"));
        assert_eq!(options.mongodb_initdb_root_password.as_deref(), Some("pass"));
        assert_eq!(options.voyage_api_key.as_deref(), Some("key"));
        assert_eq!(options.local_seed_location.as_deref(), Some("/seed"));
        assert_eq!(options.wait_until_healthy_timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.skip_pull_image, Some(true));
        // Marca de telemetría: si cambia, el equipo pierde la atribución de uso.
        assert_eq!(options.creation_source, Some(CreationSource::Unknown("PYTHON".into())));
    }

}
