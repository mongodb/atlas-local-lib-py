#!/usr/bin/env bash
#
# Checks that every third-party Python runtime dependency is covered by an
# allowed license.
#
# Override the interpreter with PYTHON=/path/to/python.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTHON="${PYTHON:-python3}"
# Allowed licenses for the Python layer. Must mirror deny.toml `licenses.allow`
# and about.toml `accepted`.
PY_ALLOWED_LICENSES="\
MIT License;MIT;\
Apache Software License;Apache License 2.0;Apache 2.0;Apache-2.0;Apache-2.0 WITH LLVM-exception;\
BSD Zero Clause License;0BSD;\
The Unlicense (Unlicense);Unlicense;\
Boost Software License 1.0 (BSL-1.0);BSL-1.0;\
Unicode-3.0"

set -euo pipefail

cd "${ROOT}"

for tool in pip-licenses "${PYTHON}"; do
    command -v "${tool}" >/dev/null 2>&1 || { echo "Required tool not found: ${tool}" >&2; exit 1; }
done

deps="$("${PYTHON}" scripts/py_runtime_deps.py | tr '\n' ' ')"
if [ -z "${deps}" ]; then
    echo "No third-party Python runtime dependencies to check."
    exit 0
fi

echo "Checking: ${deps}"
pip-licenses --python "${PYTHON}" --packages ${deps} --allow-only "${PY_ALLOWED_LICENSES}"
