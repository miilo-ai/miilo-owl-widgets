# Miilo OWL widgets

Compact, theme-aware Miilo controls for Omalauncher. Animated agents blink while working and close their eyes with drifting z's while resting. One heading per tile; details and controls appear on tap.

| Widget | Tile | On tap |
| --- | --- | --- |
| Feed · 3×3 | Agents and one current task | Select an agent, its reported assignments, or a task |
| Needs you · 3×2 | Highest-priority review and its system | Inspect the queue, evidence, explanation, and supported actions |
| Work in motion · 3×2 | Queued/working/waiting distribution; failures stay visible | Filter systems, browse phases, and inspect a task |

All colors come from Omalauncher (`accent`, `background`, `foreground`, `muted`, `red`). There are no fixed provider colors. Light and dark theme changes are handled by the launcher. Native tile sizing is intentionally small; users can enlarge tiles on the home screen.

## Install and register what to observe

Download and verify the **ARM64 Linux release bundle**, extract it, then run **inside Omalauncher's Linux environment**. No Python or Rust toolchain is needed on the device:

```sh
./install.sh
```

The installer copies the shared `miilo-owl` helper to `~/.local/bin`, installs the three widgets under `~/.config/omalauncher/widgets`, and starts setup. Existing widget files receive a one-time `.before-miilo-1.3` backup; existing connection config is preserved. Ensure `~/.local/bin` is in the launcher's Linux PATH. To install without interactive setup, use `./install.sh --no-setup`.

Setup lets you:

1. Add a named Miilo API endpoint (home, office, customer, etc.).
2. Enter its bearer token through a hidden prompt.
3. Discover its registered providers and explicitly choose which to observe.
4. Opt in to supported bounded provider actions, or keep observation read-only.
5. Repeat for other endpoints; later update or remove any of them.

One endpoint may expose many factories/providers. Up to eight independent endpoints are supported. Identical provider/task IDs on different endpoints remain separate. New providers are **not** silently added to the selection. All three widgets use the same configuration; their `Set up` control reopens this flow.

This setup registers **observation endpoints and provider selections locally**. Factory/provider adapters must already be registered in the Miilo backend. It does not invent credentials or create an upstream factory integration. If an endpoint has no providers, setup asks you to register them in Miilo first. Existing per-widget endpoint/token state from version 1.1 is no longer used; run setup once to create the shared configuration.

## Expanded controls

- `Filter` switches between all selected systems and one system; large lists have “more” controls.
- Select an agent to see work associated through its provider-scoped attempts.
- Select a task for a revision-bound explanation, linked evidence, and provider links.
- Provider-advertised, available **read/bounded** actions with no input form can be proposed, then explicitly confirmed. The helper re-reads the task and rejects a changed revision before posting. The backend also validates access and revision.
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
- `tools/assemble.py`: generates the three installable `.owl` files, each under 300 lines.
- `helper/src`: Rust configuration, transport, projection, graph, aggregation, onboarding, and operations modules. `Transport` and `Console` ports keep HTTP and terminal details outside the application logic.
- `helper/tests`: Rust boundary tests; `tests/test_*.py`: development-only black-box tests of the compiled binary.
- `tools/Dockerfile.helper`, `tools/build-linux.sh`: pinned ARM64 Linux build environment, capped at two CPUs/2 GB during compilation.
- `tools/package.sh`: packages the static helper, widgets and installer with SHA-256 checksums. No Python runtime is shipped.
- `tests`: helper, HTTP, installer, and runtime fixtures.
- `tools/reference.html`, `tools/render_reference.py`: browser layout references from evaluated OWL trees; not native screenshots.
- Numbered original PNGs under `previews`: before images, retained for comparison.

Edit fragments, then run `make assemble`; do not edit generated OWL directly.

Design reference: [Omalauncher widget guidance](https://github.com/SimonSchubert/omalauncher/blob/main/app/src/main/assets/widgets/README.md). Community context: [Building Calm in Complex Interfaces](https://dev.to/khanh_nguyen_04b1205f5a91/building-calm-in-complex-interfaces-18n8) — consistent hierarchy and details on demand. The implemented safety boundary comes from Miilo's SDK/API contracts and the tests, not from community examples.

## Native helper development and migration

```sh
make test                 # Rust tests + compiled CLI journeys; Python is a dev test tool only
make build                # Build for your development machine
make build-linux          # Docker: static aarch64-unknown-linux-musl, including on Apple Silicon
make package              # dist/miilo-owl-widgets-1.3-linux-aarch64.tar.gz + SHA256SUMS
```

The Linux binary runs inside Arch/PRoot. It is not an Android/Bionic APK or a macOS executable. A source checkout intentionally requires building the helper first; it does not silently fall back to Python. For local installer tests, set `MIILO_OWL_BINARY=/absolute/path/to/miilo-owl`. Release packaging rejects a non-static/non-ARM64 artifact.

Existing version-1 configuration and the CLI commands (`setup`, `snapshot`, `refresh`, `explain ITEM REVISION`, `action ITEM ACTION REVISION`) remain compatible. Opaque task/action/revision keys match the old helper, including Unicode. Existing credentials and provider selections are preserved. Older Python helper files may remain from previous installations but are not used by the replaced executable. Python is not removed from the device; other software may need it.

The installer leaves connections unconfigured on a fresh device. For live data, register the desired factories/providers in your Miilo backend, ensure that API is reachable from the device, then use **Set up** to add the endpoint, enter a token privately, and select providers. The emulator does not automatically inherit the Mac's Tailscale connectivity. Avoid placing real credentials in ADB commands or shell history.

Validation evidence and remaining device/provider checks: [native helper validation](docs/native-helper-validation.md).
