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

### Get or create a deployment

Use `get_or_create` to reuse an existing deployment with matching options, or create a new one if it doesn't exist.

```python
deployment = LocalDeployment.get_or_create(name="my-local-deployment")
```

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
