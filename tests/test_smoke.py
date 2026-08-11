"""Smoke test for the compiled extension module.

Confirms the Rust-Python boundary is wired up and importable.
"""

import atlas_local


def test_module_imports():
    assert atlas_local is not None


def test_methods_are_exposed():
    deployment = atlas_local.LocalDeployment
    assert callable(deployment.create)
    assert callable(deployment.get)
    assert callable(deployment.list)
    assert callable(deployment.start)
    assert callable(deployment.stop)
    assert callable(deployment.pause)
    assert callable(deployment.unpause)
    assert callable(deployment.delete)
    assert callable(deployment.get_or_create)
    assert callable(deployment.connection_string)
    assert callable(deployment.get_connection_string)
    assert callable(deployment.logs)
    assert callable(deployment.get_logs)
    assert callable(deployment.start_deployment)
    assert callable(deployment.stop_deployment)
    assert callable(deployment.pause_deployment)
    assert callable(deployment.unpause_deployment)
    assert callable(deployment.delete_deployment)


def test_class_module_is_public_name():
    assert atlas_local.LocalDeployment.__module__ == "atlas_local"
