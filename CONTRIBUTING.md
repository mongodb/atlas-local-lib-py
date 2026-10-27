# Contributing

`atlas-local-lib-py` is a Python extension module written in Rust with
[PyO3](https://pyo3.rs) and built with [maturin](https://www.maturin.rs). It
wraps the [atlas-local](https://github.com/mongodb/atlas-local-lib) crate.

## Prerequisites

- **Rust** 1.85 or later, since the crate uses edition 2024
- **Python** 3.10 or later
- **Docker**, installed and running, to build deployments and run the
  integration tests

## Quick start

1. **Get the code.** Fork the repository on GitHub, then:

   ```bash
   git clone https://github.com/mongodb/atlas-local-lib-py.git
   cd atlas-local-lib-py
   ```

2. **Create a branch** off `main`:

   ```bash
   git checkout -b amazing-feature origin/main
   ```

3. **Set up the environment:**

   ```bash
   python -m venv .venv
   source .venv/bin/activate

   pip install maturin
   maturin develop --extras test
   ```

4. **Check that everything works:**

   ```bash
   docker info    # the daemon has to be reachable
   cargo test
   pytest
   ```

`maturin develop` compiles the crate and installs it into the active virtual
environment in editable mode. Re-run `maturin develop` after changing Rust code so that the extension is
rebuilt. If you are using a notebook, restart the kernel after rebuilding so
that it loads the updated extension.


## Development workflow

### Making changes

1. Create a branch for your feature or bug fix
2. Make your changes in small, logical commits
3. Add tests for any new functionality
4. Re-run `maturin develop` after touching Rust code, so that Python picks up
   the rebuilt extension
5. Before committing, run the complete validation suite:

```bash
# Format your code
cargo fmt
ruff format .

# Run linting
cargo clippy --all-targets -- -D warnings
ruff check .

# Run all tests
cargo test
pytest
pytest -m integration    # needs a Docker daemon
```

CI runs the same checks in their read-only form (`cargo fmt --all -- --check`
and `ruff format --check .`)  and fails on any diff or warning. `clippy` runs
with `-D warnings`, so an unused import is enough to fail the build.


Integration tests are deselected by default through `addopts` in
`pyproject.toml`, so `pytest` alone is safe on a machine without Docker.
Integration tests create real containers with unique names and delete them
afterwards; if one fails halfway, its fixture still cleans up.



## Third-party licenses

`LICENSE-3RD-PARTY.txt` is generated from the dependency tree and checked in.
CI regenerates it and fails if the committed copy is stale, so after changing
any dependency:

```bash
PYTHON="$(which python)" ./scripts/generate-third-party.sh
```

Commit the result. `cargo deny check licenses` and
`./scripts/check-python-licenses.sh` enforce the license policy itself.

## Pull requests

Titles must follow [conventional commits](https://www.conventionalcommits.org),
because the title becomes the squashed commit message and feeds the changelog:

```
<type>(<optional scope>): <description>
```

Accepted types are `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`,
`refactor`, `revert`, `style`, `test` and `ops`. Add `!` after the type for a
breaking change.

## Supported platforms

The CI matrix currently tests Ubuntu, macOS, and Windows with Python 3.10
and the latest supported Python version.


## Changing the public API

Changes to `#[pymethods]` affect the Python API. When modifying it:

- Preserve parameter names, keyword arguments, and default values. Verify the
  exposed signature from Python, for example:
  `inspect.signature(LocalDeployment.create)`.
- Map Rust errors to the most specific appropriate Python exception.
- Add or update tests for the public behavior and error cases.
- Update `README.md` and relevant examples or notebooks.
- Consider whether the change is breaking and mention it in the PR description.


## Examples

`examples/` holds runnable notebooks. They are documentation, so they need to
work top to bottom on a clean machine: no leftover state between cells, and a
cleanup cell at the end. Re-run them after changing anything they touch.


## Reporting bugs

When reporting a bug, include the operating system, Python and Rust versions,
Docker version, reproduction steps, and the complete error message.

## Reporting Security Issues

Do not report security vulnerabilities through public GitHub issues or pull
requests. Follow the [MongoDB vulnerability reporting instructions](https://www.mongodb.com/docs/manual/tutorial/create-a-vulnerability-report/)
instead.

---

Thank you for contributing to the MongoDB Atlas Local Python Library!
