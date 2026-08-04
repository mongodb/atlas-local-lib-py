use atlas_local::models::{BindingType, Deployment as RsDeployment, MongodbType};

use pyo3::prelude::*;

#[pyclass(module = "atlas_local", name = "LocalDeployment", frozen)]
pub struct LocalDeployment {
    inner: RsDeployment,
}

impl From<RsDeployment> for LocalDeployment {
    fn from(inner: RsDeployment) -> Self {
        Self { inner }
    }
}

impl LocalDeployment {
    pub fn inner(&self) -> &RsDeployment {
        &self.inner
    }
}

#[pymethods]
impl LocalDeployment {
    #[getter]
    fn container_id(&self) -> &str {
        &self.inner.container_id
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    #[getter]
    fn local_seed_location(&self) -> Option<&str> {
        self.inner.local_seed_location.as_deref()
    }

    #[getter]
    fn mongodb_initdb_database(&self) -> Option<&str> {
        self.inner.mongodb_initdb_database.as_deref()
    }

    #[getter]
    fn mongodb_initdb_root_username(&self) -> Option<&str> {
        self.inner.mongodb_initdb_root_username.as_deref()
    }

    #[getter]
    fn mongodb_initdb_root_password(&self) -> Option<&str> {
        self.inner.mongodb_initdb_root_password.as_deref()
    }

    #[getter]
    fn voyage_api_key(&self) -> Option<&str> {
        self.inner.voyage_api_key.as_deref()
    }

    #[getter]
    fn state(&self) -> String {
        self.inner.state.to_string()
    }

    #[getter]
    fn mongodb_version(&self) -> String {
        self.inner.mongodb_version.to_string()
    }

    #[getter]
    fn mongodb_type(&self) -> &'static str {
        match self.inner.mongodb_type {
            MongodbType::Community => "community",
            MongodbType::Enterprise => "enterprise",
        }
    }

    #[getter]
    fn mongodb_load_sample_data(&self) -> Option<bool> {
        self.inner.mongodb_load_sample_data
    }

    // Uncomment when new atlas-local version is released with the new fields in the Deployment struct.
    //#[getter]
    //fn image(&self) -> Option <&str> {
    //    self.inner.image.as_deref()
    //}

    //#[getter]
    //fn image_tag(&self) -> Option<String> {
    //    self.inner.image_tag.clone()
    //}

    #[getter]
    fn port_bindings(&self) -> Option<String> {
        self.inner
            .port_bindings
            .as_ref()
            .map(|binding| match &binding.binding_type {
                BindingType::Loopback => "127.0.0.1".to_owned(),
                BindingType::AnyInterface => "0.0.0.0".to_owned(),
                BindingType::Specific { ip } => ip.to_string(),
            })
    }

    #[getter]
    fn do_not_track(&self) -> bool {
        self.inner.do_not_track
    }

    fn __repr__(&self) -> String {
        format!(
            "LocalDeployment(name={:?}, container_id={:?}, state={:?}, mongodb_version={:?})",
            self.inner.name,
            self.inner.container_id,
            self.inner.state.to_string(),
            self.inner.mongodb_version.to_string(),
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
