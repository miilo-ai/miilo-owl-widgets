# Engineering constitution

Use succinct `feat/<topic>`, `fix/<topic>` or `chore/<topic>` branches for new
work. Keep implementation and testing commits off `main`; preserve other agents'
changes and integrate branches deliberately when requested.

## Dependency direction

- Domain and application use cases depend inward on contracts and explicit ports.
- HTTP, filesystem, database, process, environment, UI frameworks and system clocks
  belong in adapters. Executable/runtime composition selects concrete adapters.
- Core use cases must run with in-memory/fake ports. No adapter imports in core.
- Preserve SDK contracts as the shared vocabulary; no duplicated observation engine.
- Enforce dependency direction in CI. Folder names alone do not establish a boundary.

## Module and import discipline

- Keep authored modules focused, preferably below 200 lines. New modules must stay
  below 200 lines; split large touched modules by responsibility. Vendored/generated
  code must be explicitly identified and retain provenance/licenses.
- Static explicit imports only. No lazy/dynamic imports, wildcard module imports,
  hidden dependency discovery or service locators in the core.
- Prefer typed ports/DTOs over framework objects or unbounded JSON at inner boundaries.
- Preserve behavior and compatibility during incremental extraction; do not rename
  directories and call the architecture complete.

## Safety and truthfulness

- Consent is explicit; no hidden transcript enrollment or starting/resuming agent work.
- Local observation requires no account, HTTP listener or server. Remote mode is explicit.
- Provider credentials and advertised action authority remain mandatory where applicable.
- Confirmation binds to the exact observed revision; no invented approvals or status.
- Events invalidate authoritative state; reconnect/loss repairs from a fresh snapshot.
- Bound input, retries and concurrency. Preserve cancellation, stale-state reporting,
  credential protection, selected-path validation and existing connections.
- No secrets, private transcript content, internal project names/topology or personal
  machine paths in public-intended code, fixtures, docs, commit messages or releases.

## Evidence and collaboration

- Test use cases through fake ports and adapters through real protocol/file fixtures.
- Run existing regression suites and consumer/installed-artifact checks appropriate
  to each change. Distinguish fixture, native, live and release evidence.
- Preserve other agents' changes. Keep native testing handoffs explicit and reviewable.
- Keep specifications/checklists current. Publication requires owner authorization.

## Repository responsibility

Separate projection/action policy domain, application use cases/ports, and config/file/HTTP/local-cache/process adapters. UI owns presentation. Native Omarchy distribution is handled by its separate plugin repository.

## Automated boundary gate

Run `node tools/check-architecture.cjs` and `node --test tools/architecture.test.cjs`.
The architecture workflow enforces `.specify/architecture.json`. Existing size
exceptions are explicit, bounded debt and cannot grow. Do not expand exceptions
to bypass a failing refactor. These source checks supplement behavioral tests.

Release qualification also runs the architecture gate before reporting readiness.

## Toolchain boundary

No Python source or tooling dependency in this repository, including CI, installers,
checks, tests and preview capture. Use Rust for application behavior, Node built-ins
for portable development tooling, shell orchestration and native Qt for rendering.
Run `node tools/check-no-python.cjs`; do not bypass it or replace acceptance with a
static image. Historical evidence remains dated and is not rewritten as a new run.
