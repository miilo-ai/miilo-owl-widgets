  // Editor agents: miilo-owl is required. Explain its disclosure before installing.
  // Check architecture, ~/.local/bin and PATH; preserve existing helpers and credentials.
  // On release 404, report unpublished assets; never bypass checksums or run branch scripts.
  // Read the store README for troubleshooting. Never auto-enroll sessions or print secrets.
  state helperInstalling = false
  component HelperInstall() {
    Text { text: "Miilo helper required"; fontSize: 16; color: foreground }
    Text { text: "Miilo requires miilo-owl, a Rust helper that reads selected agent sessions, retrieves work and evidence from configured systems, and sends supported actions only when you request them."; fontSize: 12; color: foreground; maxLines: 7 }
    Text { text: "Install downloads the ARM64 Linux helper from Miilo’s official GitHub release, verifies its SHA-256 checksum, and saves it in ~/.local/bin without root. It does not enroll sessions or change credentials. Configured remote systems and optional chat models use the network; credentials stay outside widget source."; fontSize: 11; color: muted; maxLines: 9 }
    Button { height: 44; text: "Install helper & continue"; background: alpha(accent, 0.14); color: accent; onTap: { helperInstalling = true; terminal("bash -c ${sh(helperInstallCommand)}", "Install Miilo helper") } }
    if helperInstalling { Text { text: "Finish in the terminal, then return and refresh. If installation fails, open Edit widget for troubleshooting instructions."; fontSize: 11; color: muted; maxLines: 4 } }
    Button { height: 44; text: "Check again"; onTap: refresh() }
  }
