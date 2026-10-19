# Standalone consumer evidence — macOS arm64

> Historical validation record. Results apply only to the revision and environment recorded below; they do not describe current release readiness.


38 helper tests, 5 xtask tests, warnings-denied helper/xtask clippy and 36 desktop
OWL structural scenarios passed. New local transport tests use a private cache,
no HTTP provider, preserve existing remote endpoints, refuse stale reads/actions,
and bind explanations to the current revision. Native Linux service/theme/focus
interaction is not proved by these structural fixtures. Generated OWL is 1.4.0.
The TCP fixtures now explicitly make accepted sockets blocking on macOS.

Installed public CLI/optimized macOS archive → helper fixture journey passed:
no CE, import, native-file update, grounded explanation and stale-source rejection.
The helper cache signature now incorporates actual local revision/freshness so
its own cache cannot extend a stopped source's lifetime.
