# Architecture boundary enforcement

Domain/application owns ports and policy; infrastructure implements those ports.
Executable/runtime composition chooses concrete implementations. Core use cases
must run with fake ports. Compatibility entry points remain explicit at the edge.
The constitution and architecture policy describe the enforced source scope.

Projection, graph, artifact and action policy are pure domain modules. Snapshot and action use cases depend on Transport, SnapshotStore, StoreLease and Clock. Files, credentials, process helpers and HTTP live in adapters. Compatibility exports preserve existing callers. Forty helper tests and warnings-denied lint passed.
