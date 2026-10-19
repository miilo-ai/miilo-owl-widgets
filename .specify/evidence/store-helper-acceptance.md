# Store helper acceptance, 2026-10-18

Version remains 1.5.0 because that version has not yet been tagged or published.

## Android emulator

- Archived the existing configuration, credentials, helper and widget state before
  replacing anything. Private backup locations and connection values are excluded.
- A missing helper displays the required dependency, purpose, permissions, network
  use, credential boundary, local installation location and explicit install action.
- Rendering the disclosure does not download or execute the installer.
- Tapping Install opens the launcher terminal. An unpublished release returns 404
  and leaves no executable or enrolled connection. Added an actionable error message.
- A temporary acceptance-only copy pinned to the already published 1.4.0 bundle
  downloaded over HTTPS, verified SHA-256, installed the helper and successfully
  opened the empty connection setup terminal after installer cleanup.
- The shipping installer is pinned to 1.5.0. Repeat the real-download check against
  1.5.0 after publication; the temporary 1.4.0 test does not qualify that artifact.

## Automated checks

Installer success, corrupt checksum, failed download, existing helper preservation
and unsupported architecture are exercised with isolated files and fake downloads.
The OWL evaluator verifies disclosure visibility and explicit terminal invocation.
Settings, compact layout, orbit rendering and session visibility regressions pass.
All three generated widgets pass the official store compiler. Architecture and
Python-free gates pass.

## Marketplace

A local submission candidate is staged separately. Official review accepts the
widget but flags running the embedded installer as code and requires reviewer
inspection. The candidate still needs a preview image and maintainer approval.
No marketplace submission or release publication was performed.
