import atlas_local
import pytest


def test_negative_timeout_raises_value_error():
    with pytest.raises(ValueError, match="must be non-negative"):
        atlas_local.LocalDeployment.start_deployment(
            "deployment", wait_until_healthy_timeout=-1
        )


def test_missing_container_id_or_name_raises_type_error():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment.start_deployment()


def test_unknown_kwarg_raises_type_error():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment.start_deployment(
            "deployment", nonexistent_option=True
        )
