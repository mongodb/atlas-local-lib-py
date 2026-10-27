import inspect

import atlas_local
import pytest


def test_negative_timeout_raises_value_error():
    with pytest.raises(ValueError):
        atlas_local.LocalDeployment.create(wait_until_healthy_timeout=-1)


def test_invalid_image_tag_raises_value_error():
    with pytest.raises(ValueError):
        atlas_local.LocalDeployment.create(image_tag="not-a-tag")


def test_unknown_kwarg_raises_type_error():
    with pytest.raises(TypeError):
        atlas_local.LocalDeployment.create(nonexistent_option=True)


def test_embedding_provider_endpoint_is_appended_to_create_signatures():
    methods = (
        atlas_local.LocalDeployment.create,
        atlas_local.LocalDeployment.get_or_create,
    )

    for method in methods:
        parameters = list(inspect.signature(method).parameters)
        assert parameters[-2:] == ["do_not_track", "embedding_provider_endpoint"]
