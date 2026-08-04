import atlas_local
import pytest


def test_cannot_be_instantiated_directly():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment()
