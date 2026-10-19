# Python-free presentation tooling — 2026-10-17

> Historical validation record. Results apply only to the revision and environment recorded below; they do not describe current release readiness.


## Scope and boundaries

The runtime remains Rust plus OWL/QML. Portable repository tools use Node 22
built-ins with static imports and no npm dependencies. Isolated preview capture
uses a native C++17/Qt 6 harness with CMake. Domain/provider logic was not moved
into the plugin. Authored replacement modules remain below 200 lines.

The previous Python drivers and tests have been replaced and removed from the
working tree. Historical evidence remains unchanged and dated. The constitution
and CI now run tools/check-no-python.cjs to reject reintroduction.

## Completed validation

- [x] Source and active toolchain contain no Python files or dependencies.
- [x] Real Qt Orbit capture and work-row selection pass in both repositories.
- [x] Original robot presentation, attention marks and deterministic motion remain.
- [x] Tokyo Night recording rebuilt from 241 native Qt frames, without PySide.
- [x] Changes preserve existing local design and incoming main-branch edits.

Qt 6.11.2 on macOS rendered the isolated fixtures. Font-alias and inherited-palette
advisories were printed; they did not prevent drawing or interaction. This is
not actual Omarchy shell, physical Android or live-provider acceptance.

No release, tag, marketplace publication or deployment was performed.

OWL-specific checks: helper and xtask Rust suites passed; eight Node architecture/
SDK access tests and one toolchain regression passed; 36 desktop scenarios,
motion checks, 54 exact portrait draw comparisons and official syntax checking
for all three generated widgets passed. Generation, formatting, workflow policy
and whitespace checks passed. Missing private SDK access still reports
"No release has been made"; token files stay owner-only and masked.
