  state system = ""
  state systemsOpen = false
  state setupOpen = false
  state setupSection = "menu"
  state setupConfiguring = false
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
  source feed = cmd("export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl snapshot") every 15s timeout 25s status
  let data = feed.ok ? parseJson(feed.out) : null
  let configured = data != null && data.configured
  let snap = data == null ? null : data.snapshot
  let allProviders = snap == null ? [] : (snap.providers ?? [])
  let allActors = snap.actors ?? []
  let allWorks = snap.workItems ?? []
  // Transcript-backed sessions are inferred observations, not persistent factories.
  let localSessionIds = allActors.filter(a => !(a._pairedComputer ?? false) && get(get(a.extensions ?? {}, "dev.miilo.session", {}), "runtimeStatusAuthoritative", true) == false).map(a => a.providerId)
  let activeActors = allActors.filter(a => contains(["working", "running", "reviewing", "blocked", "failed", "error"], a.status) && !contains(["offline", "unreachable", "unknown"], a.health ?? ""))
  let activeActorKeys = activeActors.map(a => a._key)
  let activeProviderIds = activeActors.map(a => a.providerId) + allWorks.filter(w => (w.attention ?? "none") != "none" || w.lifecycle == "failed").map(w => w.providerId)
  let recentSessionIds = allActors.filter(a => get(get(a.extensions ?? {}, "dev.miilo.session", {}), "recentlyActive", false)).map(a => a.providerId)
  let selectableProviders = allProviders.filter(p => !contains(localSessionIds, p.id) || contains(activeProviderIds, p.id) || contains(recentSessionIds, p.id))
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
  // Linux is also available inside Android; it is not a desktop detector.
  let toolbarHeight = 32
  let fieldHeight = 32
  let tabHeight = 28
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

  component StatusBadge(label, ink = accent, w = 90) {
    Box {
      width: w; height: 22; align: center; radius: 4; background: alpha(ink, 0.1)
      Text { text: label; fill: width; align: center; fontSize: 10; color: ink; maxLines: 1 }
    }
  }
