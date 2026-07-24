"""Print the runtime Python dependencies for this package, including
transitive dependencies.

This script is used by generate-third-party.sh and the license CI check
to make sure we only look at real runtime dependencies. It excludes the
package itself, as well as build and development tools that may happen
to be installed in the environment.

"""

from importlib import metadata

from packaging.requirements import Requirement

ROOT = "atlas-local-lib-py"


def runtime_requires(dist_name):
    try:
        dist = metadata.distribution(dist_name)
    except metadata.PackageNotFoundError:
        return []
    out = []
    for spec in dist.requires or []:
        req = Requirement(spec)
        # Skip deps that only apply under an extra or a non-matching marker.
        if req.marker is not None and not req.marker.evaluate({"extra": ""}):
            continue
        out.append(req.name)
    return out


def closure(root):
    seen = set()
    stack = [root]
    while stack:
        name = stack.pop()
        for dep in runtime_requires(name):
            key = dep.lower().replace("_", "-")
            if key not in seen:
                seen.add(key)
                stack.append(dep)
    return sorted(seen)


if __name__ == "__main__":
    try:
        metadata.distribution(ROOT)
    except metadata.PackageNotFoundError as exc:
        raise SystemExit(
            f"Package '{ROOT}' is not installed; install it (e.g. `pip install .`) "
            "before running this script."
        ) from exc
    for name in closure(ROOT):
        print(name)
