  component AgentRows(list) {
    for a in list.take(agentLimit) key a._key {
      Row {
        gap: 8
        fill: width
        align: center
        onTap: { selectedActor = a._key; selectedWork = ""; renameOpen = false; proposed = "" }
        Bot { agent: a; s: 38 }
        Column {
          weight: 1
          gap: 1
          Text { text: a.name; style: body; maxLines: 1 }
          Text { text: a.status; style: small; color: muted; maxLines: 1 }
        }
        Text { text: "›"; width: 10; style: small; color: muted }
      }
    }
    if len(list) > agentLimit { Button { text: "+${len(list) - agentLimit} agents"; onTap: agentLimit = agentLimit + 5 } }
  }
  component QuietPortrait(provider, items, s = 80) {
    let team = actors.filter(a => a.providerId == provider.id)
    let representative = len(team) > 0 ? team[0] : {_key: provider.id, name: provider.name, health: provider.health ?? "unknown"}
    let offline = contains(["offline", "unreachable", "unknown", "stale"], provider.health ?? "") || contains(["offline", "unreachable", "stale"], provider.status ?? "") || any(team, a => contains(["offline", "unreachable", "unknown"], a.health ?? ""))
    let errors = any(items, w => w.lifecycle == "failed") || any(team, a => contains(["failed", "error"], a.status))
    let warning = any(items, w => w.attention != "none") || any(team, a => a.status == "blocked")
    let working = any(items, w => w.lifecycle == "active") || any(team, a => contains(["working", "running"], a.status))
    let portrait = {_key: representative._key, providerId: provider.id, name: provider.name, health: offline ? "unreachable" : representative.health, status: offline ? "offline" : errors ? "failed" : warning ? "blocked" : working ? "working" : "resting"}
    CompactOrbit { agent: portrait; items: items; s: s; offline: offline; slot: indexOf(allProviders.map(p => p.id), provider.id); alert: offline ? "" : errors ? "!!" : warning ? "?!" : "" }
  }
  component QuietFleet(list) {
    let systems = provs.filter(p => !needsOnly || any(list, w => w.providerId == p.id))
    let columns = max(1, min(4, min(len(systems), floor(max(screenWidth - 18, 1) / 88))))
    Grid {
      fill: width
      columns: columns
      gap: 6
      for p in systems.take(providerLimit) key p.id {
        Column {
          align: center
          gap: 2
          onTap: { inspectionView = "quiet"; expandedSystem = p.id; workflowActor = ""; selectedActor = ""; pageSize = 5; agentsOpen = false }
          QuietPortrait { provider: p; items: live.filter(w => w.providerId == p.id); s: min(128, max(48, (screenWidth - 24 - (columns - 1) * 6) / columns)) }
          Text { text: replace(p.name, "-", " "); width: max(48, (screenWidth - 24 - (columns - 1) * 8) / columns); height: 28; align: center; fontSize: 10; maxLines: 2 }
        }
      }
    }
    if len(systems) == 0 { Text { text: needsOnly ? "Nothing needs you" : "No systems to display"; style: small; color: muted } }
    if len(systems) > providerLimit { Button { text: "More systems"; onTap: providerLimit = providerLimit + 6 } }
  }
  component QuietSystemTasks(provider, list) {
    let team = actors.filter(a => a.providerId == provider.id)
    let tasks = list.filter(w => w.providerId == provider.id && (workflowActor == "" || contains(w._actorKeys ?? [], workflowActor)))
    Button { text: "Back"; onTap: { observatoryView = inspectionView == "" ? observatoryView : inspectionView; inspectionView = ""; expandedSystem = ""; workflowActor = ""; agentsOpen = false } }
    Text { text: provider.name; style: body; maxLines: 2 }
    Flow {
      fill: width
      gap: 8
      for a in team.take(4) key a._key {
        Column {
          width: 76
          align: center
          gap: 2
          onTap: { workflowActor = workflowActor == a._key ? "" : a._key; pageSize = 5 }
          Bot { agent: a; s: 42 }
          Text { text: replace(a.name, "-", " "); width: 76; align: center; fontSize: 10; color: workflowActor == a._key ? accent : muted; maxLines: 2 }
          if workflowActor == a._key { Spacer { width: 30; height: 2; background: accent } }
        }
      }
    }
    WorkRows { list: tasks }
    if len(tasks) == 0 { Text { text: workflowActor != "" ? "No assigned work reported" : "No work in flight"; style: small; color: muted } }
    if len(team) > 4 { Button { text: agentsOpen ? "Hide agents" : "More agents"; onTap: agentsOpen = !agentsOpen } }
    if agentsOpen { AgentRows { list: team } }
  }
