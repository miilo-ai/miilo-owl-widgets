# Computer pairing acceptance

Date: October 19, 2026. Feature branch; not a release or marketplace approval.

- macOS: explicit TLS host using an isolated profile containing three previously
  selected real assistant sessions. No new agent work was started.
- Linux/Omarchy: native helper paired across the private network. Both comparison
  codes matched before owner-side approval. Fresh reads returned three providers
  and three work items. The credential survived an actual host restart.
- Android 14 ARM64 emulator: native helper paired independently, and ran inside
  Omalauncher's Linux environment. The actual launcher displayed three sleeping
  robots; clicking a robot opened its work list and then the actual task detail.
  Explain displayed the supplied summary and its observation/evidence limitations.
  Action authority remained disabled. This is emulator evidence, not a physical
  handset qualification.
- Revoking the Android test credential blocked its subsequent reads while the
  Linux device continued reading. Android then paired with a fresh credential.
- Stopping the isolated collector caused stale reads to fail and return no current
  providers. Restarting it restored reads. Local production profiles were untouched.
- Small client/host clock skew initially rejected a valid invitation. A regression
  test now permits up to 30 seconds in the client's future-expiry sanity check;
  host-side invitation expiry remains five minutes.
- Idle paired sessions initially disappeared under the local historical-session
  filter. Explicit paired selection now remains visible with its actual idle state;
  local discovery's historical filtering is unchanged.

Automated evidence includes the full helper Rust suite, paired-origin/action and
fixed-error-disclosure policy tests, observation-only explanations, mobile settings
navigation through the actual OWL evaluator, session visibility, architecture and
no-Python guards. The native plugin separately records its Qt/package checks.
Credentials, private invitations and captures are not committed.
