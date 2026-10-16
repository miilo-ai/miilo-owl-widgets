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
  let effectiveSystem = any(allProviders, p => p.id == system) ? system : ""
  let provs = allProviders.filter(p => effectiveSystem == "" || p.id == effectiveSystem)
  let works = (snap.workItems ?? []).filter(w => effectiveSystem == "" || w.providerId == effectiveSystem)
  let actors = (snap.actors ?? []).filter(a => effectiveSystem == "" || a.providerId == effectiveSystem)
  let attention = (snap.attention ?? []).filter(a => effectiveSystem == "" || a.providerId == effectiveSystem)
  let attempts = snap.attempts ?? []
  let evidence = snap.evidence ?? []
  let artifacts = snap.artifacts ?? []
  let busy = actors.filter(a => contains(["working", "running", "reviewing"], a.status))
  let live = works.filter(w => contains(["active", "waiting", "failed", "queued"], w.lifecycle))
  let needle = lower(trim(query))
  let searched = works.filter(w => contains(lower("${w.title} ${w.summary ?? ""} ${w.phase} ${get(find(allProviders, p => p.id == w.providerId), "name", "")}"), needle))
  let focused = find(works, w => w._key == selectedWork)
  let actor = find(actors, a => a._key == selectedActor)
  let receipt = reply == "" || reply == "pending" ? null : parseJson(reply)
  let issueCount = len(data.errors ?? [])
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
      Box { width: 44; height: 44; align: center; onTap: setupOpen = !setupOpen
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
      if len(allProviders) > 1 {
        Box { width: 44; height: 44; align: center; onTap: systemsOpen = !systemsOpen
          Text { text: "≡"; width: 44; fontSize: 24; align: center; color: accent }
        }
      }
      if !requestPending {
        Box { width: 44; height: 44; align: center; onTap: { requestPending = true; replyWork = ""; replyRevision = ""; reply = run("miilo-owl refresh"); reply = "pending" }
          Text { text: "↻"; width: 44; fontSize: 24; align: center; color: accent }
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
        if focused != null { Text { text: "←"; width: 44; height: 44; fontSize: 24; align: center; color: accent; onTap: { selectedWork = ""; proposed = ""; evidenceOpen = false; reply = requestPending ? reply : "" } } }
        else if actor != null { Text { text: "←"; width: 44; height: 44; fontSize: 24; align: center; color: accent; onTap: { selectedActor = ""; renameOpen = false } } }
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
        Button { text: "All systems"; color: effectiveSystem == "" ? accent : muted; onTap: { system = ""; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } }
        for p in allProviders key p.id {
          Button { text: p.name; color: effectiveSystem == p.id ? accent : muted; onTap: { system = p.id; systemsOpen = false; pageSize = 5; agentLimit = 5; selectedWork = ""; selectedActor = ""; proposed = ""; reply = requestPending ? reply : "" } }
        }
      }
    }
  }

  component SetupTerminal() {
    if setupOpen {
      Button { text: leftHanded ? "Controls: left hand" : "Controls: right hand"; onTap: leftHanded = !leftHanded }
      Button { text: previewRobots ? "Hide preview robots" : "Show preview robots"; onTap: previewRobots = !previewRobots }
      Terminal { command: "miilo-owl setup"; height: 340 }
      Text { text: "Add or change systems here. Type q then Enter when done, or tap Done."; style: small; color: muted; maxLines: 3 }
      Button { text: "Done"; onTap: { setupOpen = false; refresh() } }
    }
  }

  component Arc(phase, moving) {
    Canvas {
      fill: width
      height: 25
      let index = indexOf(phases, phase)
      let cx = 12 + clamp(index, 0, 4) / 4 * max(width - 24, 1)
      let xs = range(81).map(i => i * width / 80)
      let points = xs.map(x => [x, 17 - (abs(x - cx) < 12 ? 9 * (1 + cos(abs(x - cx) / 12 * 3.14159)) / 2 : 0)])
      if index >= 0 {
        path(points, stroke: alpha(accent, 0.7), width: 1.5)
        circle(cx, 17, moving ? 3.4 + 0.4 * sin(time * 2) : 3.4, fill: accent)
      } else {
        line(0, 17, width, 17, stroke: alpha(muted, 0.25), width: 1.5)
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

  component WorkDetail() {
    if focused != null {
      Spacer { height: 1; fill: width; background: alpha(muted, 0.2) }
      Text { text: focused.title; style: body; maxLines: 4 }
      Text { text: "${get(find(allProviders, p => p.id == focused.providerId), "name", "System")} · ${focused.phase}"; style: small; color: muted; maxLines: 1 }
      Arc { phase: focused.phase; moving: focused.lifecycle == "active" }
      if focused.summary != null && focused.summary != "" { Text { text: focused.summary; style: small; color: muted; maxLines: 8 } }
      if !requestPending {
        Flow {
          gap: 6
          Button { text: "Explain"; onTap: { requestPending = true; replyWork = focused._key; replyRevision = focused._revisionKey; reply = run("miilo-owl explain ${sh(focused._key)} ${sh(focused._revisionKey)}"); reply = "pending"; proposed = "" } }
          for a in (focused.actions ?? []).filter(a => a._supported) key a._key {
            Button { text: a.label; color: accent; onTap: { proposed = a._key; proposalRevision = focused._revisionKey; proposalWork = focused._key; reply = requestPending ? reply : "" } }
          }
          if focused.nativeUrl != "" { Button { text: "Provider ↗"; onTap: url(focused.nativeUrl) } }
        }
        if !(focused._actionsEnabled ?? false) {
          Text { text: "Actions disabled for this connection"; style: small; color: muted }
          Button { text: "Action settings"; onTap: setupOpen = true }
        }
        if len(focused.actions ?? []) == 0 { Text { text: "No actions currently offered by this provider"; style: small; color: muted } }
        for a in (focused.actions ?? []).filter(a => !a._supported) key a._key {
          Text { text: "${a.label} · ${a._unavailableReason ?? "Use provider"}"; style: small; color: muted; maxLines: 3 }
          if a.nativeUrl != "" { Button { text: "${a.label} ↗"; onTap: url(a.nativeUrl) } }
        }
      }
      if proposed != "" && focused._key == proposalWork {
        let action = find(focused.actions ?? [], a => a._key == proposed)
        if action != null && focused._revisionKey == proposalRevision {
          Text { text: "${action.label}: ${focused.title}?"; style: small; maxLines: 2 }
          Flow {
            gap: 6
            Button { text: "Confirm"; color: accent; onTap: { requestPending = true; replyWork = proposalWork; replyRevision = proposalRevision; reply = run("miilo-owl action ${sh(proposalWork)} ${sh(proposed)} ${sh(proposalRevision)}"); reply = "pending"; proposed = "" } }
            Button { text: "Cancel"; onTap: proposed = "" }
          }
        } else {
          Text { text: "Task changed. Review again."; style: small; color: muted }
        }
      }
      Column {
        gap: 6
        Text { text: "Evidence"; style: small; color: muted }
        let taskEvidence = evidence.filter(e => contains(focused._evidenceKeys ?? [], e._key))
        if len(taskEvidence) == 0 { Text { text: "No evidence supplied"; style: small; color: muted } }
        for e in taskEvidence.take(pageSize) key e._key {
          Text { text: "${e.outcome} · ${e.summary}"; style: small; maxLines: 8 }
          if (e._missingArtifactCount ?? 0) > 0 { Text { text: "${e._missingArtifactCount} referenced artifacts unavailable"; style: small; color: muted } }
        }
        if len(taskEvidence) > pageSize { Button { text: "More evidence"; onTap: pageSize = pageSize + 5 } }
        TaskArtifacts { }
      }
    }
    if reply == "pending" { Text { text: "Working…"; style: small; color: muted } }
    if receipt != null && (replyWork == "" || (focused != null && focused._key == replyWork && focused._revisionKey == replyRevision)) {
      Text { text: receipt.message ?? "Refreshed"; style: small; maxLines: 8; color: receipt.status == "failed" || receipt.status == "conflict" ? red : foreground }
      for c in receipt.changes ?? [] key c.id {
        Text { text: c.name; style: small; maxLines: 4 }
        if c.preview != null && c.preview != "" { Text { text: c.preview; style: small; maxLines: 400 } }
      }
    }
  }
