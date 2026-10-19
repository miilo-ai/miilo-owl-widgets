  component SettingsEntry(label, detail, section) {
    Box {
      fill: width; padding: 10; radius: 8; background: alpha(accent, 0.07)
      onTap: { setupSection = section; setupConfiguring = false }
      Row {
        fill: width; gap: 8; align: center
        Column {
          weight: 1; gap: 2
          Text { text: label; fontSize: 13; color: foreground; maxLines: 1 }
          Text { text: detail; fontSize: 11; color: muted; maxLines: 2 }
        }
        Text { text: "›"; width: 18; fontSize: 18; align: center; color: accent }
      }
    }
  }
  component SettingsPage() {
    Row {
      fill: width; gap: 8; align: center
      Box {
        width: 44; height: toolbarHeight; align: center
        onTap: { setupOpen = setupSection != "menu"; setupSection = "menu"; setupConfiguring = false }
        ToolbarIcon { kind: "back" }
      }
      Text { text: setupSection == "menu" ? "Settings" : setupSection == "appearance" ? "Appearance" : setupSection == "chat" ? "Chat model" : setupSection == "pair" ? "Pair computer" : setupSection == "filters" ? "Systems" : "Connections"; fontSize: 16; weight: 1; maxLines: 1 }
      Button { height: 28; background: alpha(accent, 0.12); text: "Done"; onTap: { setupOpen = false; setupConfiguring = false; refresh() } }
    }
    if feed.code == 127 {
      HelperInstall { }
    } else if setupSection == "menu" {
      SettingsEntry { label: "Appearance"; detail: "Motion, personality and robot colors"; section: "appearance" }
      SettingsEntry { label: "Connections"; detail: "Factories, local sessions and credentials"; section: "systems" }
      SettingsEntry { label: "Pair computer"; detail: "Approve a read-only connection on your computer"; section: "pair" }
      SettingsEntry { label: "Chat model"; detail: "Endpoint, model and sharing consent"; section: "chat" }
      SettingsEntry { label: "Systems"; detail: "Choose which systems appear"; section: "filters" }
    } else if setupSection == "appearance" {
      RobotAppearance { }
      Text { text: "Controls"; fontSize: 12; color: muted }
      Flow {
        gap: 6
        Button { height: 28; background: alpha(accent, 0.12); text: leftHanded ? "Left handed" : "Right handed"; color: accent; onTap: leftHanded = !leftHanded }
        Button { height: 28; background: alpha(accent, 0.12); text: previewRobots ? "Hide preview robots" : "Preview robots"; onTap: previewRobots = !previewRobots }
      }
      RobotColors { }
    } else if setupSection == "filters" {
      Text { text: "Show"; fontSize: 12; color: muted }
      Box {
        fill: width; height: 48; padding: 10; radius: 6; background: alpha(accent, effectiveSystem == "" ? 0.16 : 0.04)
        onTap: { system = ""; pageSize = 5; agentLimit = 5 }
        Text { text: effectiveSystem == "" ? "✓ All systems" : "All systems"; fontSize: 12; color: effectiveSystem == "" ? accent : foreground; maxLines: 1 }
      }
      for p in selectableProviders key p.id {
        Box {
          fill: width; height: 48; padding: 10; radius: 6; background: alpha(accent, effectiveSystem == p.id ? 0.16 : 0.04)
          onTap: { system = p.id; pageSize = 5; agentLimit = 5 }
          Text { text: effectiveSystem == p.id ? "✓ ${p.name}" : p.name; fontSize: 12; color: effectiveSystem == p.id ? accent : foreground; maxLines: 1 }
        }
      }
      if len(selectableProviders) == 0 { Text { text: "Add a connection to see systems here."; fontSize: 12; color: muted; maxLines: 2 } }
    } else {
      Text { text: setupSection == "chat" ? "Questions and task context go only to the endpoint you configure and consent to." : setupSection == "pair" ? "On your computer, start sharing and create an invitation with miilo local pair invite. Compare both codes before approving. This device can only observe selected work." : "Connect a private factory or choose local sessions. Credentials stay in your configured credential store."; fontSize: 12; color: muted; maxLines: 4 }
      if setupConfiguring {
        if setupSection == "chat" { Terminal { fontSize: 12; command: "export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl chat-setup"; fill: width; height: max(240, screenHeight * 0.65) } }
        else if setupSection == "pair" { Terminal { fontSize: 12; command: "export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl pair"; fill: width; height: max(240, screenHeight * 0.65) } }
        else { Terminal { fontSize: 12; command: "export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl setup"; fill: width; height: max(240, screenHeight * 0.65) } }
        Text { text: setupSection == "chat" ? "Follow the prompts, then tap Done." : "Type q then Enter to finish, or tap Done."; fontSize: 11; color: muted; maxLines: 2 }
      } else {
        if setupSection == "systems" {
          for p in selectableProviders key p.id {
            Row {
              fill: width; height: 28; align: center; gap: 8
              Text { text: p.name; fontSize: 12; weight: 1; maxLines: 1 }
              Text { text: p.health ?? "Observed"; fontSize: 11; color: muted; width: 84; align: end; maxLines: 1 }
            }
          }
        }
        Button { height: 28; background: alpha(accent, 0.12); text: setupSection == "chat" ? "Configure chat model" : setupSection == "pair" ? "Pair computer" : "Manage connections"; color: accent; onTap: setupConfiguring = true }
        if setupSection == "chat" { Text { text: "Voice is not available in this launcher yet."; fontSize: 11; color: muted; maxLines: 2 } }
      }
    }
  }
