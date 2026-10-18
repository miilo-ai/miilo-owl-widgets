  component Setup() {
    if focused != null || actor != null || !configured {
      Row {
        fill: width
        gap: 4
        align: center
        if !leftHanded { Spacer { weight: 1 } }
        if focused != null { Box { width: 44; height: toolbarHeight; align: center; onTap: { selectedWork = ""; proposed = ""; evidenceOpen = false; reply = requestPending ? reply : "" } ToolbarIcon { kind: "back" } } }
        else if actor != null { Box { width: 44; height: toolbarHeight; align: center; onTap: { selectedActor = ""; renameOpen = false } ToolbarIcon { kind: "back" } } }
        ToolbarControls { }
        if leftHanded { Spacer { weight: 1 } }
      }
    }
    if !configured {
      Text { text: data == null ? "Install helper, then add systems" : "Add Miilo endpoints and choose providers"; style: small; color: muted; maxLines: 2 }
    }
    if issueCount > 0 {
      Text { text: "${issueCount} endpoint(s) unavailable"; style: small; color: red; maxLines: 1 }
    }
    if systemsOpen && len(allProviders) > 1 {
      Flow {
        gap: 5
        if effectiveSystem != "" { Button { text: "All"; color: effectiveSystem == "" ? accent : muted; onTap: { system = ""; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } } }
        for p in selectableProviders key p.id {
          Button { text: p.name; color: effectiveSystem == p.id ? accent : muted; onTap: { system = p.id; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } }
        }
      }
    }
  }

  component SetupTerminal() {
    if setupOpen {
      Row {
        gap: 6
        Button { text: "Systems"; color: setupSection == "systems" ? accent : muted; onTap: setupSection = "systems" }
        Button { text: "Chat model"; color: setupSection == "chat" ? accent : muted; onTap: setupSection = "chat" }
      }
      if setupSection == "chat" {
        Text { text: "Choose your chat endpoint, model and credential file. Consent applies to the questions and task context you share."; style: small; color: muted; maxLines: 4 }
        Terminal { command: "miilo-owl chat-setup"; fill: width; height: max(1, screenHeight * 0.6) }
        Text { text: "Voice integration is not implemented in this launcher yet. Transcription and speech output will use separate endpoint settings."; style: small; color: muted; maxLines: 4 }
      } else {
      RobotAppearance { }
      Button { text: leftHanded ? "Controls: left hand" : "Controls: right hand"; onTap: leftHanded = !leftHanded }
      Button { text: previewRobots ? "Hide preview robots" : "Show preview robots"; onTap: previewRobots = !previewRobots }
      Terminal { command: "miilo-owl setup"; fill: width; height: max(1, screenHeight * 0.6) }
      Text { text: "Add or change systems here. Type q then Enter when done, or tap Done."; style: small; color: muted; maxLines: 3 }
      }
      Button { text: "Done"; onTap: { setupOpen = false; setupSection = "systems"; refresh() } }
    }
  }

  component WorkRows(list, showSystem = false) {
    for w in list.take(pageSize) key w._key {
      Column {
        gap: 2
        fill: width
        onTap: { selectedWork = w._key; reply = requestPending ? reply : ""; evidenceOpen = false; proposed = "" }
        Row {
          gap: 6
          fill: width
          Text { text: w.title; style: body; weight: 1; maxLines: 1 }
          Text { text: "›"; width: 10; style: small; color: muted }
        }
        Text { text: showSystem ? "${get(find(allProviders, p => p.id == w.providerId), "name", "System")} · ${w.phase}" : "${w.phase} · ${w.lifecycle}"; style: small; color: muted; maxLines: 1 }
      }
    }
    if len(list) > pageSize {
      Row {
        fill: width
        if !leftHanded { Spacer { weight: 1 } }
        Button { text: "+${len(list) - pageSize} more"; onTap: pageSize = pageSize + 5 }
        if leftHanded { Spacer { weight: 1 } }
      }
    }
  }

