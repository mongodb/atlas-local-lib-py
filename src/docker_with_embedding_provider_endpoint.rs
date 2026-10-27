use atlas_local::{
    DockerError,
    bollard::{
        Docker,
        models::{ContainerCreateBody, ContainerCreateResponse, ContainerInspectResponse},
        query_parameters::{
            CreateContainerOptions, InspectContainerOptions, StartContainerOptions,
        },
    },
    docker::{
        DockerCreateContainer, DockerInspectContainer, DockerPullImage, DockerStartContainer,
    },
};

pub(crate) const EMBEDDING_PROVIDER_ENDPOINT_ENV: &str = "EMBEDDING_PROVIDER_ENDPOINT";

#[derive(Clone)]
pub(crate) struct DockerWithEmbeddingProviderEndpoint {
    docker: Docker,
    embedding_provider_endpoint: Option<String>,
}

impl DockerWithEmbeddingProviderEndpoint {
    pub(crate) fn new(docker: Docker, embedding_provider_endpoint: Option<String>) -> Self {
        Self {
            docker,
            embedding_provider_endpoint,
        }
    }
}

fn inject_embedding_provider_endpoint(
    env: &mut Option<Vec<String>>,
    embedding_provider_endpoint: Option<&str>,
) {
    let Some(endpoint) = embedding_provider_endpoint else {
        return;
    };

    let env = env.get_or_insert_with(Vec::new);
    env.retain(|entry| {
        entry
            .split_once('=')
            .is_none_or(|(name, _)| name != EMBEDDING_PROVIDER_ENDPOINT_ENV)
    });
    env.push(format!("{EMBEDDING_PROVIDER_ENDPOINT_ENV}={endpoint}"));
}

pub(crate) fn embedding_provider_endpoint_from_inspect_response(
    response: &ContainerInspectResponse,
) -> Option<String> {
    response
        .config
        .as_ref()?
        .env
        .as_ref()?
        .iter()
        .rev()
        .find_map(|entry| {
            let (name, value) = entry.split_once('=')?;
            (name == EMBEDDING_PROVIDER_ENDPOINT_ENV).then(|| value.to_owned())
        })
}

impl DockerPullImage for DockerWithEmbeddingProviderEndpoint {
    async fn pull_image(&self, image: &str, tag: &str) -> Result<(), DockerError> {
        DockerPullImage::pull_image(&self.docker, image, tag).await
    }
}

impl DockerCreateContainer for DockerWithEmbeddingProviderEndpoint {
    async fn create_container(
        &self,
        options: Option<CreateContainerOptions>,
        mut config: ContainerCreateBody,
    ) -> Result<ContainerCreateResponse, DockerError> {
        inject_embedding_provider_endpoint(
            &mut config.env,
            self.embedding_provider_endpoint.as_deref(),
        );

        DockerCreateContainer::create_container(&self.docker, options, config).await
    }
}

impl DockerStartContainer for DockerWithEmbeddingProviderEndpoint {
    async fn start_container(
        &self,
        container_id: &str,
        options: Option<StartContainerOptions>,
    ) -> Result<(), DockerError> {
        DockerStartContainer::start_container(&self.docker, container_id, options).await
    }
}

impl DockerInspectContainer for DockerWithEmbeddingProviderEndpoint {
    async fn inspect_container(
        &self,
        container_id: &str,
        options: Option<InspectContainerOptions>,
    ) -> Result<ContainerInspectResponse, DockerError> {
        DockerInspectContainer::inspect_container(&self.docker, container_id, options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_local::bollard::models::ContainerConfig;

    #[test]
    fn test_embedding_provider_endpoint_is_injected_into_container_environment() {
        let mut env = Some(vec!["VOYAGE_API_KEY=test-key".to_owned()]);

        inject_embedding_provider_endpoint(
            &mut env,
            Some("https://api.voyageai.com/v1/embeddings"),
        );

        assert_eq!(
            env,
            Some(vec![
                "VOYAGE_API_KEY=test-key".to_owned(),
                "EMBEDDING_PROVIDER_ENDPOINT=https://api.voyageai.com/v1/embeddings".to_owned(),
            ])
        );
    }

    #[test]
    fn test_embedding_provider_endpoint_replaces_existing_environment_value() {
        let mut env = Some(vec![
            "EMBEDDING_PROVIDER_ENDPOINT=https://old.example/v1/embeddings".to_owned(),
            "VOYAGE_API_KEY=test-key".to_owned(),
        ]);

        inject_embedding_provider_endpoint(&mut env, Some("https://new.example/v1/embeddings"));

        assert_eq!(
            env,
            Some(vec![
                "VOYAGE_API_KEY=test-key".to_owned(),
                "EMBEDDING_PROVIDER_ENDPOINT=https://new.example/v1/embeddings".to_owned(),
            ])
        );
    }

    #[test]
    fn test_missing_embedding_provider_endpoint_leaves_environment_unchanged() {
        let mut env = Some(vec!["VOYAGE_API_KEY=test-key".to_owned()]);

        inject_embedding_provider_endpoint(&mut env, None);

        assert_eq!(env, Some(vec!["VOYAGE_API_KEY=test-key".to_owned()]));
    }

    #[test]
    fn test_embedding_provider_endpoint_is_read_from_container_environment() {
        let response = ContainerInspectResponse {
            config: Some(ContainerConfig {
                env: Some(vec![
                    "EMBEDDING_PROVIDER_ENDPOINT=https://old.example/v1/embeddings".to_owned(),
                    "EMBEDDING_PROVIDER_ENDPOINT=https://new.example/v1/embeddings".to_owned(),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            embedding_provider_endpoint_from_inspect_response(&response).as_deref(),
            Some("https://new.example/v1/embeddings")
        );
    }
}
