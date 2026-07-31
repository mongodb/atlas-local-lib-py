import atlas_local

def test_exception_hierarchy():
    assert issubclass(
        atlas_local.StartDeploymentError,
        atlas_local.DeploymentError,
    )
    assert issubclass(
        atlas_local.DeploymentError,
        atlas_local.AtlasLocalError,
    )
    assert issubclass(
        atlas_local.UnhealthyDeploymentError,
        atlas_local.WatchDeploymentError,
    )
    assert issubclass(
        atlas_local.DeploymentTimeoutError,
        atlas_local.WatchDeploymentError,
    )

def test_exceptions_are_exported():
    assert atlas_local.AtlasLocalError
    assert atlas_local.DeploymentError
    assert atlas_local.CreateDeploymentError
    assert atlas_local.StopDeploymentError
    assert atlas_local.StartDeploymentError
    assert atlas_local.UnpauseDeploymentError
    assert atlas_local.PauseDeploymentError
    assert atlas_local.GetDeploymentError
    assert atlas_local.DeleteDeploymentError
    assert atlas_local.WatchDeploymentError
    assert atlas_local.GetConnectionStringError
    assert atlas_local.GetLogsError
    assert atlas_local.UnhealthyDeploymentError
    assert atlas_local.DeploymentTimeoutError
