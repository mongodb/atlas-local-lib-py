"""
These tests need a running Docker daemon.
Run them locally with: pytest -m integration
"""

import uuid

import atlas_local
import pytest

pytestmark = pytest.mark.integration

LocalDeployment = atlas_local.LocalDeployment


@pytest.fixture
def deployment():
    name = f"test-identifiers-{uuid.uuid4().hex[:8]}"
    created = LocalDeployment.create(name=name)

    yield created

    try:
        LocalDeployment.get(created.container_id)
    except atlas_local.GetDeploymentError:
        return  # Already deleted by the test.

    LocalDeployment.delete_deployment(created.container_id)


def exercise_static_methods(container_id_or_name):
    LocalDeployment.pause_deployment(container_id_or_name)
    LocalDeployment.unpause_deployment(container_id_or_name)
    LocalDeployment.stop_deployment(container_id_or_name)
    LocalDeployment.start_deployment(container_id_or_name)
    LocalDeployment.delete_deployment(container_id_or_name)


@pytest.mark.parametrize("identifier", ["name", "container_id"])
def test_static_methods_accept_container_id_or_name(deployment, identifier):
    container_id_or_name = getattr(deployment, identifier)

    # Static methods should accept both name and container_id.
    exercise_static_methods(container_id_or_name)

    with pytest.raises(atlas_local.GetDeploymentError):
        LocalDeployment.get(container_id_or_name)


def state_of(name):
    return LocalDeployment.get(name).state


def test_full_lifecycle(deployment):
    name = deployment.name

    assert deployment.state == "running"
    assert LocalDeployment.get(name) == deployment
    assert name in [listed.name for listed in LocalDeployment.list()]

    deployment.stop()
    assert state_of(name) == "exited"

    deployment.start()
    assert state_of(name) == "running"

    deployment.pause()
    assert state_of(name) == "paused"

    deployment.unpause()
    assert state_of(name) == "running"
    logs = deployment.logs()

    deployment.delete()
    with pytest.raises(atlas_local.GetDeploymentError):
        LocalDeployment.get(name)
    assert name not in [listed.name for listed in LocalDeployment.list()]
