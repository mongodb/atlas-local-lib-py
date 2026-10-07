#!/usr/bin/env bash
# Runs from the apix-action Dependabot workflow after a dependency bump.
# Only edit files here: the workflow stages the working tree and creates the
# signed commit itself.
set -Eeou pipefail

# Pinned to the version CI uses to verify the notice, so the regenerated file
# matches the checked-in copy. Bump both together (see .github/workflows/license.yml).
CARGO_ABOUT_VERSION="0.9.1"

if ! command -v cargo-about >/dev/null 2>&1; then
  # 0.9.x only ships the binary behind the `cli` feature.
  cargo install --locked --version "${CARGO_ABOUT_VERSION}" --features cli cargo-about
fi

# A throwaway venv: the runner's system Python is externally managed (PEP 668),
# so `pip install .` would refuse there. Kept outside the repo so the workflow's
# commit never picks it up.
VENV="$(mktemp -d)/venv"
python3 -m venv "${VENV}"
"${VENV}/bin/pip" install --upgrade pip
"${VENV}/bin/pip" install . pip-licenses packaging
export PATH="${VENV}/bin:${PATH}"

# Same script CI runs; regenerates both the Rust and Python notice sections.
PYTHON="${VENV}/bin/python" ./scripts/generate-third-party.sh
