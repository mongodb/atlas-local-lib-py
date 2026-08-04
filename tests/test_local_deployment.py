import pytest
import atlas_local

def test_cannot_be_instantiated_directly():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment()
