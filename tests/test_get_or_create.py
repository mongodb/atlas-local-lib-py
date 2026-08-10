"""
These tests need a running Docker daemon.
Run them locally with: pytest -m integration
"""

import time
import uuid

import atlas_local
import pymongo
import pytest

pytestmark = pytest.mark.integration

LocalDeployment = atlas_local.LocalDeployment

SECONDS_TO_RETURN_AN_EXISTING_DEPLOYMENT = 5


@pytest.fixture
def name():
    deployment_name = f"test-get-or-create-{uuid.uuid4().hex[:8]}"

    yield deployment_name

    try:
        LocalDeployment.get(deployment_name)
    except atlas_local.GetDeploymentError:
        return  # Never created, or already deleted by the test.

    LocalDeployment.delete_deployment(deployment_name)


def timed(call):
    started = time.perf_counter()
    result = call()

    return result, time.perf_counter() - started


# Drop this helper to use deployment.connection_string() once implemented.
def connection_string(deployment):
    """Builds a direct connection URI from the published port binding."""
    ip, _, port = deployment.port_bindings.partition("/")

    return f"mongodb://{ip}:{port}/?directConnection=true"


def test_re_running(name):
    created, creation_seconds = timed(
        lambda: LocalDeployment.get_or_create(name=name, load_sample_data=True)
    )

    assert LocalDeployment.get(name).state == "running"

    existing, retrieval_seconds = timed(
        lambda: LocalDeployment.get_or_create(name=name, load_sample_data=True)
    )

    # Re-running the cell must return the same container, not a second one.
    assert existing.container_id == created.container_id
    assert [listed.name for listed in LocalDeployment.list()].count(name) == 1

    # The second call should be fast, since it returns an existing deployment.
    assert retrieval_seconds < SECONDS_TO_RETURN_AN_EXISTING_DEPLOYMENT, (
        f"returning an existing deployment took {retrieval_seconds:.1f}s "
        f"(creating it took {creation_seconds:.1f}s)"
    )

    # Check that the sample data survives.
    client = pymongo.MongoClient(connection_string(existing))
    try:
        assert "sample_mflix" in client.list_database_names()
        assert client["sample_mflix"]["movies"].estimated_document_count() > 0
    finally:
        client.close()
