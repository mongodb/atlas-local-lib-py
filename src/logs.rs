use chrono::{DateTime, Utc};

use pyo3::exceptions::PyValueError;

use atlas_local::models::{LogsOptions, Tail};

use pyo3::prelude::*;

use crate::deployment::LocalDeployment;
use crate::runtime::runtime_block_on;

struct LogsArgs {
    stdout: bool,
    stderr: bool,
    tail: Option<i64>,
    timestamps: bool,
    since: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}

fn parse_tail(lines: Option<i64>) -> PyResult<Tail> {
    let Some(lines) = lines else {
        return Ok(Tail::All);
    };

    u64::try_from(lines)
        .map(Tail::Number)
        .map_err(|_| PyValueError::new_err("tail must be a non-negative number of lines"))
}

fn build_logs_options(args: LogsArgs) -> PyResult<LogsOptions> {
    Ok(LogsOptions {
        stdout: args.stdout,
        stderr: args.stderr,
        tail: Some(parse_tail(args.tail)?),
        timestamps: args.timestamps,
        since: args.since,
        until: args.until,
    })
}

fn run_get_logs(
    py: Python<'_>,
    container_id_or_name: &str,
    args: LogsArgs,
) -> PyResult<Vec<String>> {
    let options = build_logs_options(args)?;

    let logs = runtime_block_on(py, |client| async move {
        client.get_logs(container_id_or_name, Some(options)).await
    })?;

    Ok(logs
        .into_iter()
        .map(|log| log.as_str_lossy().into_owned())
        .collect())
}

#[pymethods]
impl LocalDeployment {
    /// Read the container logs of a deployment.
    #[pyo3(signature = (
        stdout=true,
        stderr=true,
        tail=None,
        timestamps=false,
        since=None,
        until=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn logs(
        &self,
        py: Python<'_>,
        stdout: bool,
        stderr: bool,
        tail: Option<i64>,
        timestamps: bool,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> PyResult<Vec<String>> {
        run_get_logs(
            py,
            &self.inner().container_id,
            LogsArgs {
                stdout,
                stderr,
                tail,
                timestamps,
                since,
                until,
            },
        )
    }

    /// Read the container logs of a deployment by name or container ID.
    #[staticmethod]
    #[pyo3(signature = (
        container_id_or_name,
        stdout=true,
        stderr=true,
        tail=None,
        timestamps=false,
        since=None,
        until=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn get_logs(
        py: Python<'_>,
        container_id_or_name: String,
        stdout: bool,
        stderr: bool,
        tail: Option<i64>,
        timestamps: bool,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> PyResult<Vec<String>> {
        run_get_logs(
            py,
            &container_id_or_name,
            LogsArgs {
                stdout,
                stderr,
                tail,
                timestamps,
                since,
                until,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn default_args() -> LogsArgs {
        LogsArgs {
            stdout: true,
            stderr: true,
            tail: None,
            timestamps: false,
            since: None,
            until: None,
        }
    }

    #[test]
    fn test_missing_tail_returns_every_line() {
        let options = build_logs_options(default_args()).unwrap();

        assert_eq!(options.tail, Some(Tail::All));
    }

    #[test]
    fn test_negative_tail_is_rejected() {
        let args = LogsArgs {
            tail: Some(-1),
            ..default_args()
        };

        Python::initialize();
        Python::attach(|py| {
            let error = build_logs_options(args).unwrap_err();

            assert!(error.is_instance_of::<PyValueError>(py));
            assert_eq!(
                error.value(py).to_string(),
                "tail must be a non-negative number of lines"
            );
        });
    }

    #[test]
    fn test_all_fields_are_mapped() {
        let since = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let until = Utc.with_ymd_and_hms(2026, 1, 1, 1, 0, 0).unwrap();

        let options = build_logs_options(LogsArgs {
            stdout: false,
            stderr: true,
            tail: Some(10),
            timestamps: true,
            since: Some(since),
            until: Some(until),
        })
        .unwrap();

        assert!(!options.stdout);
        assert!(options.stderr);
        assert_eq!(options.tail, Some(Tail::Number(10)));
        assert!(options.timestamps);
        assert_eq!(options.since, Some(since));
        assert_eq!(options.until, Some(until));
    }
}
