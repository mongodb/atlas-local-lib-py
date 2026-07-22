"""Smoke test for the compiled extension module.

Confirms the Rust-Python boundary is wired up and importable. Replace/extend
with real tests (e.g. get_or_create, exception mapping) as the API lands.
"""

import atlas_local_lib_py


def test_module_imports():
    assert atlas_local_lib_py is not None


def test_sum_as_string():
    assert atlas_local_lib_py.sum_as_string(2, 3) == "5"
