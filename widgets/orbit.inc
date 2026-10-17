  state observatoryView = "quiet" persist
  state orbitPage = 0
  component ViewChoice() {
    Row {
      fill: width
      gap: 6
      Button { text: "Quiet"; color: observatoryView != "orbit" ? accent : muted; onTap: observatoryView = "quiet" }
      Button { text: "Orbit"; color: observatoryView == "orbit" ? accent : muted; onTap: { observatoryView = "orbit"; orbitPage = 0 } }
      Spacer { weight: 1 }
    }
  }
  component OrbitCluster(provider, items, slot = 0) {
    let team = actors.filter(a => a.providerId == provider.id)
    let offline = contains(["offline", "unreachable", "unknown", "stale"], provider.health ?? "") || contains(["offline", "unreachable", "stale"], provider.status ?? "") || any(team, a => contains(["offline", "unreachable", "unknown"], a.health ?? ""))
    let representative = len(team) > 0 ? team[0] : {_key: provider.id, providerId: provider.id, name: provider.name, status: "ready", health: "unknown"}
    let anchor = offline ? {_key: representative._key, providerId: provider.id, name: representative.name, status: "offline", health: "unreachable"} : representative
    let current = items.sortBy(w => w._key).take(8)
    Column {
      fill: width
      padding: 12
      gap: 3
      background: alpha(accent, 0.035)
      radius: 12
      Row {
        fill: width
        align: center
        Text { text: provider.name; weight: 1; fontSize: 14; maxLines: 1 }
        Text { text: offline ? "Unavailable" : "${len(items)} work"; width: 92; align: end; fontSize: 10; color: muted; maxLines: 1 }
      }
      OrbitField { items: current; anchor: anchor; offline: offline; energized: any(items, w => w.lifecycle == "active"); slot: slot }
      if len(items) == 0 { Text { text: offline ? "Waiting for a fresh observation" : "No work in flight"; style: small; color: muted } }
      for w in current key w._key {
        Row {
          fill: width
          height: 44
          gap: 6
          align: center
          onTap: { selectedWork = w._key; reply = requestPending ? reply : ""; proposed = ""; evidenceOpen = false }
          Text { text: w.title; weight: 1; fontSize: 11; maxLines: 1 }
          Text { text: offline ? "Stale" : w.attention != "none" ? "Needs you" : w.phase; width: 78; align: end; fontSize: 10; color: offline ? muted : accent; maxLines: 1 }
        }
      }
      if len(items) > 8 { Button { text: "+${len(items) - 8} work · inspect system"; onTap: { observatoryView = "quiet"; expandedSystem = provider.id; workflowActor = ""; pageSize = 12 } } }
    }
  }
  component OrbitView(list) {
    let systems = provs.filter(p => !needsOnly || any(list, w => w.providerId == p.id))
    let page = min(orbitPage, max(0, ceil(len(systems) / 6) - 1))
    Grid {
      fill: width
      columns: screenWidth >= 760 ? 2 : 1
      gap: 12
      for p, i in systems.filter((p, i) => i >= page * 6 && i < (page + 1) * 6) key p.id {
        OrbitCluster { provider: p; items: list.filter(w => w.providerId == p.id); slot: i }
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
