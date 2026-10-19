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

  component WorkRows(list, showSystem = false) {
    for w in list.take(pageSize) key w._key {
      Column {
        gap: 2
        fill: width
        onTap: { selectedWork = w._key; reply = requestPending ? reply : ""; evidenceOpen = false; proposed = "" }
        Row {
          gap: 6
          height: 44
          align: center
          fill: width
          OrbitStateIcon { phase: w.phase; attention: w.attention; offline: false; failed: w.lifecycle == "failed"; ink: w.lifecycle == "failed" ? red : w.attention != "none" ? accent : foreground }
          Text { text: get(w.extensions ?? {}, "dev.miilo.session", null) != null && trim(w.summary ?? "") != "" ? w.summary : w.title; fontSize: 12; weight: 1; maxLines: 1 }
          Text { text: "›"; width: 10; style: small; color: muted }
        }
        if showSystem { Text { text: get(find(allProviders, p => p.id == w.providerId), "name", "System"); fontSize: 11; color: muted; maxLines: 1 } }
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
