# Miilo OWL widgets

Compact, theme-aware Miilo controls for Omalauncher. Animated agents blink while working and close their eyes with drifting z's while resting. One heading per tile; details and controls appear on tap.

| Widget | Tile | On tap |
| --- | --- | --- |
| Feed · 3×3 | Agents and one current task | Select an agent, its reported assignments, or a task |
| Needs you · 3×2 | Highest-priority review and its system | Inspect the queue, evidence, explanation, and supported actions |
| Work in motion · 3×2 | Queued/working/waiting distribution; failures stay visible | Filter systems, browse phases, and inspect a task |

All colors come from Omalauncher (`accent`, `background`, `foreground`, `muted`, `red`). There are no fixed provider colors. Light and dark theme changes are handled by the launcher. Native tile sizing is intentionally small; users can enlarge tiles on the home screen.

## Install and register what to observe

Run **inside Omalauncher's Linux environment**, with Python 3.9+ installed:

```sh
./install.sh
```

The installer copies the shared `miilo-owl` helper to `~/.local/bin`, installs the three widgets under `~/.config/omalauncher/widgets`, and starts setup. Existing widget files receive a one-time `.before-miilo-1.2` backup; existing connection config is preserved. Ensure `~/.local/bin` is in the launcher's Linux PATH. To install without interactive setup, use `./install.sh --no-setup`.

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

The helper uses only Python's standard library. No Miilo Cloud dependency, CLI installation, package download, or account creation is required by the installer. Users still need network access and valid credentials for their chosen Miilo backend(s).

## Validation

```sh
make test
make assemble
```

Tests cover shared onboarding, selecting/removing two endpoints, provider isolation with colliding IDs (including 20 factories / 1,000 tasks), cache invalidation, credentials, action restrictions, revision conflicts, authenticated localhost explanations/actions, redirects, and installation into paths containing spaces. CI runs helper/installer tests and checks generated OWL does not drift.

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

This uses official `owl-check` to produce live tile **and full-screen** PNGs under `~/.config/omalauncher/widgets/.preview`. Review those in your chosen light and dark themes, including system font scaling, long names, scroll, and confirmation behavior. No native-device result is claimed for version 1.2 yet: no Android device was attached during development.

## Source layout

- `widgets/*.inc`: small OWL source fragments (OWL itself has no imports).
- `tools/assemble.py`: generates the three installable `.owl` files, each under 300 lines.
- `helper/miilo_owl`: configuration, transport, aggregation, onboarding, and operations in separate modules.
- `tests`: helper, HTTP, installer, and runtime fixtures.
- `tools/reference.html`, `tools/render_reference.py`: browser layout references from evaluated OWL trees; not native screenshots.
- Numbered original PNGs under `previews`: before images, retained for comparison.

Edit fragments, then run `make assemble`; do not edit generated OWL directly.

Design reference: [Omalauncher widget guidance](https://github.com/SimonSchubert/omalauncher/blob/main/app/src/main/assets/widgets/README.md). Community context: [Building Calm in Complex Interfaces](https://dev.to/khanh_nguyen_04b1205f5a91/building-calm-in-complex-interfaces-18n8) — consistent hierarchy and details on demand. The implemented safety boundary comes from Miilo's SDK/API contracts and the tests, not from community examples.
