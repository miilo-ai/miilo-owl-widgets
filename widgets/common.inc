  state system = ""
  state systemsOpen = false
  state setupOpen = false
  state pageSize = 5
  state agentLimit = 5
  state needsOnly = false
  state agentsOpen = false
  state leftHanded = false
  state previewRobots = false
  state robotColors = [] persist
  state colorAgent = ""
  state workflowActor = ""
  state query = ""
  state searchEditing = false
  state expandedSystem = ""
  state providerLimit = 5
  state showInactiveSessions = false persist
  state renameOpen = false
  state selectedWork = ""
  state selectedActor = ""
  state evidenceOpen = false
  state reply = ""
  state requestPending = false
  state replyWork = ""
  state replyRevision = ""
  state proposed = ""
  state proposalRevision = ""
  state proposalWork = ""
  source feed = cmd("miilo-owl snapshot") every 15s timeout 25s status
  let data = feed.ok ? parseJson(feed.out) : null
  let configured = data != null && data.configured
  let snap = data == null ? null : data.snapshot
  let allProviders = snap == null ? [] : (snap.providers ?? [])
  let allActors = snap.actors ?? []
  let allWorks = snap.workItems ?? []
  // Transcript-backed sessions are inferred observations, not persistent factories.
  let localSessionIds = allActors.filter(a => get(get(a.extensions ?? {}, "dev.miilo.session", {}), "runtimeStatusAuthoritative", true) == false).map(a => a.providerId)
  let activeActors = allActors.filter(a => contains(["working", "running", "reviewing", "blocked", "failed", "error"], a.status) && !contains(["offline", "unreachable", "unknown"], a.health ?? ""))
  let activeActorKeys = activeActors.map(a => a._key)
  let activeProviderIds = activeActors.map(a => a.providerId) + allWorks.filter(w => (w.attention ?? "none") != "none" || w.lifecycle == "failed").map(w => w.providerId)
  let inactiveSessionCount = len(allProviders.filter(p => contains(localSessionIds, p.id) && !contains(activeProviderIds, p.id)))
  let selectableProviders = allProviders.filter(p => showInactiveSessions || !contains(localSessionIds, p.id) || contains(activeProviderIds, p.id))
  let effectiveSystem = any(selectableProviders, p => p.id == system) ? system : ""
  let provs = selectableProviders.filter(p => effectiveSystem == "" || p.id == effectiveSystem).sortBy(p => contains(activeProviderIds, p.id) ? 0 : 1)
  let visibleProviderIds = provs.map(p => p.id)
  let works = allWorks.filter(w => contains(visibleProviderIds, w.providerId))
  let actors = allActors.filter(a => contains(visibleProviderIds, a.providerId)).sortBy(a => contains(activeActorKeys, a._key) ? 0 : 1)
  let attention = (snap.attention ?? []).filter(a => effectiveSystem == "" || a.providerId == effectiveSystem)
  let attempts = snap.attempts ?? []
  let evidence = snap.evidence ?? []
  let artifacts = snap.artifacts ?? []
  let busy = actors.filter(a => contains(["working", "running", "reviewing"], a.status))
  let live = works.filter(w => contains(["active", "waiting", "failed", "queued"], w.lifecycle))
  let needle = lower(trim(query))
  let searched = works.filter(w => contains(lower("${w.title} ${w.summary ?? ""} ${w.phase} ${get(find(allProviders, p => p.id == w.providerId), "name", "")}"), needle))
  let focused = find(allWorks, w => w._key == selectedWork)
  let actor = find(allActors, a => a._key == selectedActor)
  let receipt = reply == "" || reply == "pending" ? null : parseJson(reply)
  let issueCount = len(data.errors ?? [])
  let toolbarHeight = linux ? 34 : 44
  let fieldHeight = linux ? 32 : 44
  let tabHeight = linux ? 28 : 44
  let phases = ["planning", "implementing", "verifying", "reviewing", "promoting"]
  on reply { requestPending = value == "pending"; refresh() }
  onTap: screen()

  let robotPalette = [
    {name: "Blue", value: blue ?? accent}, {name: "Green", value: green ?? accent},
    {name: "Purple", value: purple ?? magenta ?? accent}, {name: "Orange", value: orange ?? accent},
    {name: "Cyan", value: cyan ?? accent}, {name: "Red", value: red ?? accent}
  ]

  component Heading(label, count) {
    Row {
      fill: width
      align: center
      Text { text: label; fontSize: 11; color: accent; weight: 1; maxLines: 1 }
      Text { text: count; width: max(16, len(count) * 6); align: end; fontSize: 10; color: muted; maxLines: 1 }
    }
  }

  component ToolbarControls() {
    Row {
      gap: 4
      align: center
      Box { width: 44; height: toolbarHeight; align: center; onTap: setupOpen = !setupOpen
        Canvas {
          width: 24
          height: 24
          circle(12, 12, 7.2, stroke: foreground, width: 1.5)
          circle(12, 12, 2.8, stroke: foreground, width: 1.5)
          for i in [0, 1, 2, 3, 4, 5, 6, 7] {
            let angle = i * 0.7853981634
            line(12 + cos(angle) * 7.2, 12 + sin(angle) * 7.2, 12 + cos(angle) * 10.5, 12 + sin(angle) * 10.5, stroke: foreground, width: 1.5)
          }
        }
      }
      if !requestPending {
        Box { width: 44; height: toolbarHeight; align: center; onTap: { requestPending = true; replyWork = ""; replyRevision = ""; reply = run("miilo-owl refresh"); reply = "pending" }
          Text { text: "↻"; width: 44; fontSize: 24; align: center; color: accent }
        }
      }
      if len(allProviders) > 1 {
        Box { width: 44; height: toolbarHeight; align: center; onTap: systemsOpen = !systemsOpen
          Text { text: "≡"; width: 44; fontSize: 24; align: center; color: accent }
        }
      }
    }
  }

  component Setup() {
    if focused != null || actor != null || !configured {
      Row {
        fill: width
        gap: 4
        align: center
        if !leftHanded { Spacer { weight: 1 } }
        if focused != null { Text { text: "←"; width: 44; height: toolbarHeight; fontSize: 24; align: center; color: accent; onTap: { selectedWork = ""; proposed = ""; evidenceOpen = false; reply = requestPending ? reply : "" } } }
        else if actor != null { Text { text: "←"; width: 44; height: toolbarHeight; fontSize: 24; align: center; color: accent; onTap: { selectedActor = ""; renameOpen = false } } }
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
    if focused == null && actor == null && inactiveSessionCount > 0 {
      Button { text: showInactiveSessions ? "Hide inactive sessions" : "Show inactive sessions (${inactiveSessionCount})"; onTap: { showInactiveSessions = !showInactiveSessions; pageSize = 5 } }
    }
    if systemsOpen && len(allProviders) > 1 {
      Flow {
        gap: 5
        Button { text: "All systems"; color: effectiveSystem == "" ? accent : muted; onTap: { system = ""; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } }
        for p in selectableProviders key p.id {
          Button { text: p.name; color: effectiveSystem == p.id ? accent : muted; onTap: { system = p.id; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } }
        }
      }
    }
  }

  component SetupTerminal() {
    if setupOpen {
      RobotAppearance { }
      Button { text: leftHanded ? "Controls: left hand" : "Controls: right hand"; onTap: leftHanded = !leftHanded }
      Button { text: previewRobots ? "Hide preview robots" : "Show preview robots"; onTap: previewRobots = !previewRobots }
      Terminal { command: "miilo-owl setup"; fill: width; height: max(1, screenHeight * 0.6) }
      Text { text: "Add or change systems here. Type q then Enter when done, or tap Done."; style: small; color: muted; maxLines: 3 }
      Button { text: "Done"; onTap: { setupOpen = false; refresh() } }
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
