"""Smoke test for the compiled extension module.

Confirms the Rust-Python boundary is wired up and importable. Replace/extend
with real tests (e.g. get_or_create, exception mapping) as the API lands.
"""

import atlas_local


def test_module_imports():
    assert atlas_local is not None

