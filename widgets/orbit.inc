  state observatoryView = "quiet" persist
  state inspectionView = ""
  state orbitPage = 0
  component WorkViewToolbar(count) {
    Row {
      fill: width
      gap: 4
      Box { width: 64; height: tabHeight; align: center; radius: 6; background: !needsOnly ? alpha(accent, 0.09) : transparent; onTap: { needsOnly = false; workflowActor = ""; pageSize = 5 } Text { text: "All work"; fill: width; align: center; fontSize: 11; maxLines: 1; color: !needsOnly ? accent : muted } }
      Box { width: max(98, 84 + len("${count}") * 7); height: tabHeight; align: center; radius: 6; background: needsOnly ? alpha(accent, 0.09) : transparent; onTap: { needsOnly = true; workflowActor = ""; pageSize = 5 } Text { text: "Needs you (${count})"; fill: width; align: center; fontSize: 11; maxLines: 1; color: needsOnly ? accent : muted } }
      Spacer { weight: 1 }
      Row {
        gap: 4
        Box { width: 60; height: tabHeight; align: center; radius: 6; background: observatoryView != "orbit" ? alpha(accent, 0.09) : transparent; onTap: { observatoryView = "quiet"; expandedSystem = ""; workflowActor = ""; query = "" } Text { text: "Compact"; fill: width; align: center; fontSize: 11; maxLines: 1; color: observatoryView != "orbit" ? accent : muted } }
        Box { width: 48; height: tabHeight; align: center; radius: 6; background: observatoryView == "orbit" ? alpha(accent, 0.09) : transparent; onTap: { observatoryView = "orbit"; expandedSystem = ""; workflowActor = ""; inspectionView = ""; orbitPage = 0 } Text { text: "Detail"; fill: width; align: center; fontSize: 11; maxLines: 1; color: observatoryView == "orbit" ? accent : muted } }
      }
    }
  }
  component OrbitCluster(provider, items, slot = 0, fit = 0.85) {
    let team = actors.filter(a => a.providerId == provider.id)
    let offline = contains(["offline", "unreachable", "unknown", "stale"], provider.health ?? "") || contains(["offline", "unreachable", "stale"], provider.status ?? "") || any(team, a => contains(["offline", "unreachable", "unknown"], a.health ?? ""))
    let representative = len(team) > 0 ? team[0] : {_key: provider.id, providerId: provider.id, name: provider.name, status: "ready", health: "unknown"}
    let anchor = offline ? {_key: representative._key, providerId: provider.id, name: representative.name, status: "offline", health: "unreachable"} : representative
    let errors = any(items, w => w.lifecycle == "failed") || any(team, a => contains(["failed", "error"], a.status))
    let warning = any(items, w => w.attention != "none") || any(team, a => a.status == "blocked")
    let mark = offline ? "" : errors ? "!!" : warning ? "?!" : ""
    let current = items.sortBy(w => w._key).take(8)
    Column {
      fill: width
      padding: screenWidth < 600 ? 8 : 10
      gap: 1
      background: alpha(accent, 0.035)
      radius: 12
      onTap: { inspectionView = "orbit"; expandedSystem = provider.id; workflowActor = ""; selectedActor = ""; selectedWork = ""; pageSize = 5; agentsOpen = false }
      Row {
        fill: width
        align: center
        Text { text: provider.name; weight: 1; fontSize: screenWidth < 600 ? 10 : 11; maxLines: screenWidth < 600 ? 2 : 1 }
        Text { text: offline ? "Unavailable" : "${len(items)} ${len(items) == 1 ? "task" : "tasks"}"; width: screenWidth < 600 ? 42 : 60; align: end; fontSize: 10; color: muted; maxLines: 1 }
      }
      OrbitField { items: current; anchor: anchor; offline: offline; energized: any(items, w => w.lifecycle == "active"); slot: slot; alert: mark; fit: screenWidth < 600 ? min(fit, 0.45) : fit; fieldSize: screenWidth < 600 ? 92 : 164 }
      if len(items) == 0 { Text { text: offline ? "Waiting for a fresh observation" : "No tasks in flight"; style: small; color: muted } }
      for w in current key w._key {
        Row {
          fill: width
          height: 28
          gap: 6
          align: center
          onTap: { selectedWork = w._key; reply = requestPending ? reply : ""; proposed = ""; evidenceOpen = false }
          let rowInk = offline ? muted : w.lifecycle == "failed" ? mix(red, foreground, 0.15) : w.attention != "none" ? accent : w.phase == "verifying" ? mix(cyan, foreground, 0.4) : foreground
          OrbitStateIcon { phase: w.phase; attention: w.attention; offline: offline; failed: w.lifecycle == "failed"; ink: rowInk }
          // Match visible title glyphs to the icon center; Qt line leading sits above it.
          Text { text: get(w.extensions ?? {}, "dev.miilo.session", null) != null && trim(w.summary ?? "") != "" ? w.summary : w.title; weight: 1; fontSize: screenWidth < 600 ? 10 : 12; maxLines: 1; color: rowInk; offset: [0, 2] }
        }
      }
      if len(items) > 8 { Button { text: "+${len(items) - 8} ${len(items) == 9 ? "task" : "tasks"} · inspect system"; onTap: { inspectionView = "orbit"; expandedSystem = provider.id; workflowActor = ""; pageSize = 12 } } }
    }
  }
  component OrbitView(list) {
    let systems = provs.filter(p => !needsOnly || any(list, w => w.providerId == p.id))
    let page = min(orbitPage, max(0, ceil(len(systems) / 6) - 1))
    // Content-sized columns, not a desktop/mobile breakpoint.
    let columns = max(1, min(len(systems), min(4, floor((screenWidth - 14) / 150))))
    Grid {
      width: min(max(screenWidth - 24, 100), columns * (screenWidth < 600 ? 240 : 330) + (columns - 1) * 10)
      columns: columns
      gap: 10
      for p, i in systems.filter((p, i) => i >= page * 6 && i < (page + 1) * 6) key p.id {
        OrbitCluster { provider: p; items: list.filter(w => w.providerId == p.id); slot: indexOf(allProviders.map(q => q.id), p.id); fit: min(0.85, ((min(max(screenWidth - 24, 100), columns * (screenWidth < 600 ? 240 : 330) + (columns - 1) * 10) - (columns - 1) * 10) / columns - 20) / 270) }
      }
    }
    if len(systems) == 0 { Text { text: needsOnly ? "Nothing needs you" : "No systems to display"; style: small; color: muted } }
    if len(systems) > 6 {
      Row {
        fill: width
        gap: 8
        Button { text: "Previous"; onTap: orbitPage = max(0, page - 1) }
        Text { text: "${page + 1} / ${ceil(len(systems) / 6)}"; width: 56; align: center; style: small; color: muted }
        Button { text: "Next"; onTap: orbitPage = min(ceil(len(systems) / 6) - 1, page + 1) }
      }
    }
    Text { text: "Motion reflects observed activity. Select work to inspect it."; fontSize: 10; color: muted; maxLines: 2 }
  }
