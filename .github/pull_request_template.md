<!--
The pull request title becomes the squashed commit message and feeds the
changelog, so it has to follow conventional commits:

    <type>(<optional scope>): <description>

Accepted types: build, chore, ci, docs, feat, fix, perf, refactor, revert,
style, test, ops. Add `!` after the type for a breaking change.
-->

## Summary

<!-- What changes and why. Link the Jira ticket if there is one. -->
_Jira ticket:_

## Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (a change that may break existing functionality)
- [ ] Documentation fix or enhancement
- [ ] Build, CI or tooling change
- [ ] Other (please describe the change in the Summary section)


## Public API

<!-- Delete this section if the Python surface is untouched. -->

- [ ] Public API changes are covered by appropriate tests
- [ ] Errors use the most specific existing exception, or introduce a new one when appropriate
- [ ] `README.md` and relevant examples are up to date


## Checklist

- [ ] I have signed the [MongoDB CLA](https://www.mongodb.com/legal/contributor-agreement)
- [ ] I have read [CONTRIBUTING.md](../CONTRIBUTING.md)
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] I have run `cargo fmt` and `ruff format .`
- [ ] `cargo clippy --all-targets -- -D warnings` and `ruff check .` pass
- [ ] `cargo test` and `pytest` pass
- [ ] `pytest -m integration` passes, or the change cannot affect it
- [ ] `LICENSE-3RD-PARTY.txt` is regenerated, or no dependency changed

## Testing

<!--
How this was verified beyond the suites above: commands run, output, or the
scenario exercised by hand.
-->
