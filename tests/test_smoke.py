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

def test_class_module_is_public_name():
    assert atlas_local.LocalDeployment.__module__ == "atlas_local"
