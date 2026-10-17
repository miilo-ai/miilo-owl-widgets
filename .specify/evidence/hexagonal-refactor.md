# Architecture boundary checkpoint

Domain/application code depends on explicit ports; executable/runtime adapters
compose concrete infrastructure. `.specify/architecture.json` and its CI workflow
check inward dependencies and module sizes. Negative tests prove reversed imports
and silent size growth are rejected. Empty layer matches fail instead of passing.

Behavioral tests and native consumer qualification remain separate from these
lexical source checks. This checkpoint does not certify release, native UI, hosted
identity or every uninspected file. No release has been made.

## Implemented boundary

Projection, graph, artifact and action policy are pure domain modules. Snapshot and action use cases depend on Transport, SnapshotStore, StoreLease and Clock. Files, credentials, process helpers and HTTP live in adapters. Compatibility exports preserve existing callers. Forty helper tests and warnings-denied lint passed.
