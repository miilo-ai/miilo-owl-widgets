# Miilo OWL widgets

Compact, theme-aware Miilo controls for Omalauncher. Animated agents blink while working and close their eyes with drifting z's while resting. One heading per tile; details and controls appear on tap.

Application logic, installation, assembly, validation orchestration, and tests are Rust. Run `make test` and `cargo xtask assemble`; Python is not required. OWL remains the shared declarative UI format. Its official Android compiler and the Linux Quickshell renderer are upstream platform tools.

The Linux dashboard, native Omarchy bar and desktop tile, and existing-session connection flow are documented in [Miilo on Linux](docs/desktop.md).

| Widget | Collapsed | Expanded |
| --- | --- | --- |
| Miilo · 3×2 | Responsive animated robot grid, per-system colors, and attention marks | Search and filter systems/agents, inspect tasks, evidence and diffs, and confirm advertised actions |

The installer enables one Miilo widget. The focused attention/board sources remain available for manual use; existing installations of those are preserved as `.disabled` files.

All colors come from Omalauncher’s semantic palette. There are no fixed provider colors. Light and dark theme changes are handled by the launcher. Native tile sizing is intentionally small; users can enlarge tiles on the home screen.

## Install and register what to observe

Download and verify the **ARM64 Linux release bundle**, extract it, then run **inside Omalauncher's Linux environment**. No Python or Rust toolchain is needed on the device:

```sh
./install.sh
```

The installer copies the shared `miilo-owl` helper to `~/.local/bin`, installs the Miilo widget under `~/.config/omalauncher/widgets`, and starts setup. Existing widget files receive a one-time `.before-miilo-1.3` backup; existing connection config is preserved. Ensure `~/.local/bin` is in the launcher's Linux PATH. To install without interactive setup, use `./install.sh --no-setup`.

Setup lets you:

1. Add a named Miilo API endpoint (home, office, customer, etc.).
2. Enter its bearer token through a hidden prompt.
3. Discover its registered providers and explicitly choose which to observe.
4. Opt in to supported provider actions with explicit confirmation, or keep observation read-only.
5. Repeat for other endpoints; later update or remove any of them.

One endpoint may expose many factories/providers. Up to eight independent endpoints are supported. Identical provider/task IDs on different endpoints remain separate. New providers are **not** silently added to the selection. All widget sources use the same configuration; their `Set up` control reopens this flow.

This setup registers **observation endpoints and provider selections locally**. Factory/provider adapters must already be registered in the Miilo backend. It does not invent credentials or create an upstream factory integration. If an endpoint has no providers, setup asks you to register them in Miilo first. Existing per-widget endpoint/token state from version 1.1 is no longer used; run setup once to create the shared configuration.

## Expanded controls

- `Filter` switches between selected systems. Expanded views start with collapsed system groups. Search narrows work, agent names and system context; work search includes completed records. Lists show five entries at first, with explicit “more” controls.
- Select an agent to see work associated through its provider-scoped attempts.
- Select a task to replace the list with its revision-bound explanation, linked evidence, and provider links. Back restores your search/group context. Agent detail includes a Name control; display names are local preferences and never change provider IDs.
- Provider-advertised, available **read/bounded or explicitly confirmed privileged** actions with no input form can be proposed, then explicitly confirmed. The helper re-reads the task and rejects a changed revision before posting. The backend also validates access and revision.
- Strong-auth, privileged, destructive, expired, unavailable, or form-dependent actions remain in the provider interface. CE currently has observation/explanations but no action-proxy route; leave action opt-in disabled for CE.
- Accepted receipts are shown as accepted, never as completed. Mutation requests are not automatically retried. Idempotency keys are stable for the same task/action/revision.
- Explanation results are text. Speech playback and microphone capture are not implemented in these widgets.

No demo approvals or fabricated evidence are included in the installed widgets.

## Credentials, connectivity, and limits

Credentials are stored in `~/.config/miilo-owl/config.json` with owner-only permissions (600), not in OWL, shell arguments, or screenshots. This is a local file, **not an encrypted credential vault**. Use narrowly scoped, revocable tokens and a trusted device. HTTPS is preferred; private HTTP endpoints are allowed for local/Tailscale setups. Redirects are refused so bearer tokens cannot be forwarded to another host.

Each widget checks the helper every 15 seconds while visible. A shared eight-second cache and file lock avoid triple fetches. Each HTTP request has an eight-second timeout and two-megabyte response limit. Endpoint reads are bounded to four concurrent requests. This is periodic observation, not an SSE client or an offline store. The private cache holds only the short-lived last aggregate; failed endpoints are explicitly reported and are not represented as healthy empty systems. Actions and explanations always fetch fresh observations.

The helper is a statically linked Rust binary (about 2.2 MiB for ARM64 Linux), with Rustls HTTPS and bundled public trust roots. The installer copies the bundled binary; it does not download a runtime or install packages. No Miilo Cloud dependency, separate CLI installation, or account creation is required by the installer. Private CAs are not yet configurable; use a publicly trusted HTTPS certificate or your trusted local/Tailscale HTTP endpoint. Users still need network access and valid credentials for their chosen Miilo backend(s).

## Validation

```sh
make test
make assemble
```

Tests cover shared onboarding, selecting/removing two endpoints, provider isolation with colliding IDs (including 20 factories / 1,000 tasks), cache invalidation, credentials, action restrictions, revision conflicts, authenticated localhost explanations/actions, redirects, and installation into paths containing spaces. CI runs Rust boundary tests, black-box CLI/HTTP/installer journeys, formatting/lint checks, and generated OWL drift checks. An ARM64 job builds the static helper, runs the same tests against it, and uploads a checksummed install bundle. These workflows are prepared; a remote CI pass is not implied by local results.

The real [Omalauncher compiler](https://github.com/SimonSchubert/omalauncher/tree/main/owl) validates OWL syntax. The runtime inspector evaluates both tiles and expanded screens across light/dark themes, multiple sizes, crowded/empty/disconnected/partial/multi-system snapshots, and UI selections. It also executes the actual propose/confirm handlers against a recording host: the first tap must not dispatch an action.

```sh
OWL_CHECKER_CLASSPATH=/path/to/owl.jar \
KOTLIN_COMPILER_CLASSPATH=/path/to/kotlin/compiler-and-dependencies \
make check-owl
```

The Kotlin inspector is a real-language evaluator, **not a Compose screenshot renderer**. Native launcher validation is a separate required step. With the launcher visible on the phone, run inside its Linux terminal:

```sh
./tools/check-device.sh
```

This uses official `owl-check` to validate installed widgets and, when supported, produce live tile **and full-screen** PNGs under `~/.config/omalauncher/widgets/.preview`. Widgets containing Terminal may be skipped by the automatic preview renderer; use on-screen inspection/ADB screenshots for those. Review your chosen light and dark themes, including system font scaling, long names, scroll, and confirmation behavior. Version 1.3 has been installed in an ARM64 Android 14 emulator running Omalauncher 0.7.2 and its Arch/PRoot environment. The helper runs and opens the in-widget onboarding terminal without Python. Real-provider connectivity and physical-phone interaction are separate checks; no tower connection is implied by this smoke test.

## Source layout

- `widgets/*.inc`: small OWL source fragments (OWL itself has no imports).
- `cargo xtask assemble`: generates the three installable `.owl` files, assembled from small source fragments.
- `helper/src`: Rust configuration, transport, projection, graph, aggregation, onboarding, and operations modules. `Transport` and `Console` ports keep HTTP and terminal details outside the application logic.
- `helper/tests`: Rust boundary, HTTP, concurrency, and installer tests of the compiled binary.
- `xtask/src`: Rust assembly, fixture generation, official OWL validation orchestration, reference exports, and release-policy tooling.
- `tools/Dockerfile.helper`, `tools/build-linux.sh`: pinned ARM64 Linux build environment, capped at two CPUs/2 GB during compilation.
- `tools/package.sh`: packages the static helper, widgets and installer with SHA-256 checksums. No Python runtime is shipped.
- `tests`: helper, HTTP, installer, and runtime fixtures.
- `tools/reference.html`, `cargo xtask render-reference`: browser layout references from evaluated OWL trees; not native screenshots.
- Numbered original PNGs under `previews`: before images, retained for comparison.

Edit fragments, then run `make assemble`; do not edit generated OWL directly.

Design reference: [Omalauncher widget guidance](https://github.com/SimonSchubert/omalauncher/blob/main/app/src/main/assets/widgets/README.md). Community context: [Building Calm in Complex Interfaces](https://dev.to/khanh_nguyen_04b1205f5a91/building-calm-in-complex-interfaces-18n8) — consistent hierarchy and details on demand. The implemented safety boundary comes from Miilo's SDK/API contracts and the tests, not from community examples.

## Native helper development and migration

```sh
make test                 # Rust boundary, compiled CLI, installer and release-policy tests
make build                # Build for your development machine
make build-linux          # Docker: static aarch64-unknown-linux-musl, including on Apple Silicon
make package              # dist/miilo-owl-widgets-1.3-linux-aarch64.tar.gz + SHA256SUMS
```

The Linux binary runs inside Arch/PRoot. It is not an Android/Bionic APK or a macOS executable. A source checkout intentionally requires building the helper first; it does not silently fall back to Python. For local installer tests, set `MIILO_OWL_BINARY=/absolute/path/to/miilo-owl`. Release packaging rejects a non-static/non-ARM64 artifact.

Existing version-1 configuration and the CLI commands (`setup`, `snapshot`, `refresh`, `explain ITEM REVISION`, `action ITEM ACTION REVISION`) remain compatible. Opaque task/action/revision keys match the old helper, including Unicode. Existing credentials and provider selections are preserved. Older Python helper files may remain from previous installations but are not used by the replaced executable. Python is not removed from the device; other software may need it.

The installer leaves connections unconfigured on a fresh device. For live data, register the desired factories/providers in your Miilo backend, ensure that API is reachable from the device, then use **Set up** to add the endpoint, enter a token privately, and select providers. Emulator access depends on the host's routing: this Mac's emulator was verified to reach the tower through the Mac's Tailscale connection, without an emulator-specific VPN. Test that route before adding another Tailscale client; a physical phone needs its own reachable path. Avoid placing real credentials in ADB commands or shell history.

Validation evidence and remaining device/provider checks: [native helper validation](docs/native-helper-validation.md).

### Unified feed

Miilo Feed is the primary view: expand a system to browse five work items at a time, filter Needs you across systems, search work or agents, and open a task for evidence and confirmed actions. Animated robots remain visible above each expanded system's work and act as tabs that filter workflows by assigned agent. All work includes unassigned tasks; agent details remain available on demand. The installer exposes only Miilo and preserves older focused widgets as .disabled files. The inline toolbar uses ← for Back, ⚙ for setup, and ↻ for refresh. Labels on consequential actions remain explicit.

Controls default to right-handed placement. Open ⚙ and tap Controls: right hand to switch to left-hand placement; the toolbar and work-list More control follow that preference. Setup/refresh/back use borderless icons with 44 dp touch areas. Omalauncher's title/close row is launcher-owned and has no widget action slot, so icons sit directly below it. Robot tabs replace the redundant Agents count button for small teams.

The collapsed Miilo tile is robot-first: no task titles or workflow progress bar. Theme-colored `?!` marks actionable work and tight red `!!` marks failures; a muted connection marker distinguishes reported offline agents. Activity is conveyed through robot animation without a bulb. Resting agents retain closed eyes and drifting Zs. Tapping a robot opens its assigned workflows. Additional agents remain discoverable in the expanded view. New installations default to a compact 3×2 tile; existing user sizing is retained by the launcher.

For visual testing, setup offers Show/Hide preview robots. This adds only tile-local sleeping and working examples, labeled Preview; it does not register systems or alter snapshots/actions. Preview mode shows one real robot plus two examples. It is disabled by default in the repository; enabled temporarily in the emulator at the user's request.

Robot tile layout responds to both width and height. Equal grid cells use bounded 30–100 dp robots, responsive punctuation anchored near the head, and a fixed 28 dp two-line label area. Very short tiles omit names. Up to two rows render when there is room; overflow is represented by +N and remains accessible in the expanded view.

The tile has a muted bottom-right miilo wordmark, overlaid independently of the centered robot group. Awake robots occasionally glance toward occupied adjacent cells (left/right/up/down), with a smooth two-second eye movement staggered across a seventeen-second cycle. Sleeping eyes stay closed, and lone robots have no glance target. These movements are decorative and do not imply inter-agent communication.

Robot colors: open setup (⚙), select an automation under Robot colors, then choose Automatic, a theme palette color, or Shuffle. Choices persist locally under the scoped actor key; semantic palette positions adapt to the current launcher theme. Automatic assignment is deterministic from identity, and Shuffle chooses a different palette position. Color preferences never alter provider records. Blocked, failed, or actionable assigned work also triggers an occasional brief wistful eye/mouth expression; ordinary waiting does not.

Newcomer empty state: when no systems are configured, the tile centers a blinking welcome robot above “Tap to add a system”. Tapping that group opens setup directly. It uses the same theme palette and scales down to compact tiles; it is not a provider or agent record.

Theme behavior: saved Green/Red/etc. choices keep their color family, while the current theme supplies each shade. Robot ink also blends slightly with the theme foreground for contrast. Sleep Zs are larger, bold, foreground-weighted and use a stronger fade envelope so they remain visible against light/dark themed tiles.


### Files, changes, and evidence

Task details show evidence and linked artifacts inline. The Rust helper preserves the SDK `artifacts` collection and scopes task/attempt/evidence links to each endpoint and provider. Files and change sets include provider-supplied previews (`fileDiff`, `preview`, `delta`, or `diff` within extensions), bounded to 8,000 characters with an explicit shortened-preview label. Empty previews display “Diff not supplied”; absent task artifacts display “No files or change sets supplied”. Missing evidence references are counted explicitly. Only HTTP(S) artifact links without embedded credentials are actionable; local paths are labels, never executed or fetched.

No SDK protocol change is required. Tests cover real authenticated HTTP snapshot output, colliding identities across connections/providers, unrelated attempts, missing references, Unicode preview limits, and rendered filenames/diffs/empty-preview states.

### Approval controls

Supported provider actions appear above evidence in task details. Approve, Reject, Retry, or any other supported advertised action first opens a Confirm/Cancel prompt bound to the task revision. No action runs on the first tap. Pending requests hide action controls; stale proposals cannot confirm; dispatch rechecks the current provider revision, preserves idempotency, and never retries a timed-out mutation. Receipts report the actual provider outcome.

Actions are opt-in per connection in setup. With observation-only configuration, details show “Actions disabled for this connection” and an Action settings button. When the provider publishes no actions, details say so. Other unsupported actions explain why and offer their safe provider URL when available. The widget supports no-input read/bounded actions and privileged actions requiring explicit confirmation, including standard empty-object SDK schemas. Destructive actions, strong confirmation, and actions requiring form input stay in the provider interface. Enabling the local option never grants upstream permission or invents provider actions.

Search toolbar behavior and launcher-owned close icon: see [launcher-header.md](docs/launcher-header.md).

Search shares one compact row with cog/refresh (and a system filter for multiple systems). Tap it to expand a focused input; submit or ✓ restores the controls without clearing the query. The stock launcher does not expose blur callbacks, so keyboard dismissal alone leaves editing active. This works with the default OWL6 install; no custom launcher is needed.

## Release automation

See [.github/RELEASING.md](.github/RELEASING.md) for private-repository rehearsals, manifest-driven tags, credential-missing skips and activation. Publication stays disabled until explicitly requested/configured.

## Standalone local sessions (candidate 1.4)

Native Omarchy imports use the public CLI's `miilo-local` binary and SDK runtime.
Install candidate CLI 0.2 alongside the helper, or provide `MIILO_LOCAL_BINARY`
when running `tools/install-desktop.sh`. The helper reads a private version-2
local snapshot file, without a CE process, Miilo account or HTTP listener. Existing
remote endpoints remain configured and can be mixed with this local source.
Local source selection/future-group opt-in belongs to the observer configuration.
The widget follows only that consented projection; it cannot grant action authority.

Stop/restart the user-owned `miilo-local-desktop.service` to stop/restart native
observation. A cache older than 15 seconds fails closed and the source is shown
unavailable; it is not a sleeping healthy robot. Explain uses the same projected
revision and SDK-grounded evidence. Local transcripts advertise no approval or
retry controls. Remote factory controls continue to require current capability,
confirmation, revision and provider receipt.

Actual Omarchy theme/focus/keyboard and service install/update/removal acceptance
is tracked in `.specify`; passing macOS helper tests is not native Linux sign-off.

## Architecture checks

The [engineering constitution](.specify/memory/constitution.md) defines the
boundaries. Run `python3 tools/check_architecture.py` and
`python3 tools/test_architecture.py`; CI qualification runs both. These are
development checks, not runtime dependencies.

Text chat helper checkpoint and setup: [launcher chat](docs/launcher-chat.md). Graphical chat and voice acceptance remain pending.
