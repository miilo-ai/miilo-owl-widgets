# Connect a physical phone

The widget displays selected observations. It does not install or sign you in to
Codex, Cursor or OpenCode, and it does not require an SSH key for the observed host.

## Start with the right source

- Work on the phone: a local source can only read that environment's sessions.
  Android/PRoot local collection needs separate qualification; Linux/macOS support
  does not establish it.
- Work on a laptop/server: run collection there. Ask its operator for a compatible
  Miilo observation API URL and Miilo credential. Tailscale alone does not expose
  the local snapshot file as an API or enroll any source.
- Just trying the design: sample previews are fixtures, not authenticated agents.

The [CLI phone and remote guide](https://github.com/miilo-ai/miilo-cli/blob/main/docs/PHONE_AND_REMOTE.md)
explains where each component runs and the separate credential boundaries.

## Connection checklist

1. Connect the phone to the approved private network if the endpoint requires it.
   Test reachability from Omalauncher's Linux environment, not only its browser.
2. Install the verified ARM64 Linux widget bundle inside that environment, following
   the README. The installer does not discover private server credentials.
3. Open **Set up** (or run `miilo-owl setup`), add a named observation endpoint,
   and enter its Miilo bearer token privately. A Codex login or model API key is
   not a replacement for that token.
4. Select returned providers explicitly. An empty list may mean no provider was
   registered upstream; a failed/unauthorized response is not empty success.
5. Keep actions disabled initially. Open a system and verify a natural source
   update and freshness before considering provider-supported actions.
6. Configure **Chat model** separately if desired. Its OpenAI-compatible `/v1`
   URL and model credential belong in chat setup, not observation connections.
7. Report the installed versions, selected source, connectivity result and actual
   update. Missing access is a handoff to the operator, not an instruction to
   change server configuration or rummage through credential/system directories.

Keep credentials out of chat, shell history, screenshots and widget source. The
user enters secrets through the hidden prompt or an approved private file path.
Do not copy a computer's SSH identity or Codex login cache onto the phone.

## Emulator versus phone

An emulator can use the computer's existing route or explicit port forwarding.
A physical phone needs its own tested route and authorized endpoint access.
`localhost` refers to the device/environment making the request. Do not reuse a
host's loopback URL as if it were a remote address.

A recording can combine sample tasks with a real model explanation. That proves
only the demonstrated paths, not agent sign-in or arbitrary remote approvals.
Real new installs stay empty until actual sources are selected and reachable.

## Running a coding agent is optional and separate

If you want a coding agent to run inside the phone's Linux environment, follow
that agent's official installation and sign-in instructions. User-completed login
and platform support must be checked separately. Miilo setup must not launch new
agent work, change `/etc`, create a server, or grant SSH access implicitly.

Stop on missing permissions or repeated authentication failures. Ask the operator
for the right endpoint/credential instead of using privileged workarounds.
