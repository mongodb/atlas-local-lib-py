# atlas-local-lib-py

Python library for managing [MongoDB Atlas Local](https://www.mongodb.com/docs/atlas/cli/current/atlas-cli-deploy-local/) deployments. It wraps the [atlas-local](https://github.com/mongodb/atlas-local-lib) Rust library and requires a running Docker daemon.

## Requirements

- Python >= 3.10
- Docker running locally

## Installation

```bash
pip install atlas-local-lib-py
```

## Usage

### Create a deployment

```python
from atlas_local import LocalDeployment

deployment = LocalDeployment.create(name="my-local-deployment")

print(deployment.container_id)
print(deployment.state)
```

When using a Voyage API key that needs a non-default gateway, pass the full
embeddings URL as `embedding_provider_endpoint`:

```python
deployment = LocalDeployment.create(
    name="my-local-deployment",
    voyage_api_key="<key>",
    embedding_provider_endpoint="https://api.voyageai.com/v1/embeddings",
)
```

Use `https://ai.mongodb.com/v1/embeddings` for Atlas-issued production keys
and `https://api.voyageai.com/v1/embeddings` for keys created directly with
Voyage AI. For an Atlas staging key, provide its gateway, such as
`https://ai-stage.mongodb.com/v1/embeddings`.

### Get or create a deployment

Use `get_or_create` to reuse an existing deployment with matching options, or create a new one if it doesn't exist.

```python
deployment = LocalDeployment.get_or_create(name="my-local-deployment")
```

Creation options such as `voyage_api_key` and `embedding_provider_endpoint` are
also accepted by `get_or_create`. If a deployment with the same name already
exists but was created with a different endpoint, `get_or_create` raises a
`ValueError` instead of reusing it.

### Get an existing deployment

```python
deployment = LocalDeployment.get("my-local-deployment")
```

### List deployments

```python
for deployment in LocalDeployment.list():
    print(deployment.name, deployment.state)
```

### Manage the deployment lifecycle

```python
deployment.stop()
deployment.start()
deployment.pause()
deployment.unpause()
deployment.delete()
```

Each lifecycle method is also available as a static method that takes a container ID or name, for example `LocalDeployment.stop_deployment("my-local-deployment")`.

## Error handling

Operations raise exceptions rooted at `atlas_local.AtlasLocalError`, with more specific subclasses such as `CreateDeploymentError`, `GetDeploymentError`, `DeleteDeploymentError`, `StartDeploymentError`, `StopDeploymentError`, `PauseDeploymentError`, and `UnpauseDeploymentError`.

```python
from atlas_local import AtlasLocalError, LocalDeployment

try:
    deployment = LocalDeployment.get("nonexistent-deployment")
except AtlasLocalError as error:
    print(f"Failed to get deployment: {error}")
```

## Development

Build the extension in editable mode:

```bash
pip install maturin
maturin develop --extras test
```

Run the test suite:

```bash
pytest
```

Integration tests require a running Docker daemon and are opt-in:

```bash
pytest -m integration
```
