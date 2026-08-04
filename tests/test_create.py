import pytest
import atlas_local


def test_negative_timeout_raises_value_error():
    with pytest.raises(ValueError):
        atlas_local.LocalDeployment.create(wait_until_healthy_timeout=-1)


def test_invalid_image_tag_raises_value_error():
    with pytest.raises(ValueError):
        atlas_local.LocalDeployment.create(image_tag="not-a-tag")


def test_unknown_kwarg_raises_type_error():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment.create(nonexistent_option=True)
