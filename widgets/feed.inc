  let tileActors = (previewRobots ? actors.take(1) : actors) + (previewRobots ? [
    {_key: "preview-resting", providerId: "", name: "Preview sleep", status: "resting", health: "healthy", _preview: true},
    {_key: "preview-working", providerId: "", name: "Preview working", status: "working", health: "healthy", _preview: true}
  ] : [])
  component StateMark(warning, working, offline, s = 18) {
    Canvas {
      width: s
      height: s
      let k = s / 18
      let mark = warning ? (orange ?? foreground) : working ? (green ?? foreground) : foreground
      if offline {
        circle(9*k, 9*k, 5*k, stroke: alpha(muted, 0.7), width: 1.3)
        line(4*k, 14*k, 14*k, 4*k, stroke: muted, width: 1.3)

      }
    }
  }
  let availableWidth = max(tileWidth - 16, 32)
  let availableHeight = max(tileHeight - 16, 32)
  let showNames = availableHeight >= 88
  let columns = max(1, min(len(tileActors), floor((availableWidth + 6) / 76)))
  let rows = max(1, min(2, floor((availableHeight - 16) / (showNames ? 96 : 50))))
  let capacity = columns * rows
  let cellWidth = (availableWidth - (columns - 1) * 6) / columns
  let cellHeight = (availableHeight - 16 - (rows - 1) * 6) / rows
  let robotSize = clamp(min(cellWidth - 10, cellHeight - (showNames ? 32 : 0)), 30, 100)
  let usedRows = ceil(min(len(tileActors), capacity) / columns)
  let groupHeight = usedRows * (robotSize + (showNames ? 30 : 0)) + max(usedRows - 1, 0) * 6 + (len(tileActors) > capacity ? 20 : 0) + (issueCount > 0 ? 20 : 0) + (len(actors) == 0 ? 20 : 0)
  Box {
    fill: true
  Column {
    fill: true
    justify: availableHeight >= groupHeight ? "center" : "start"
    gap: 5
    if !configured {
      Column {
        fill: width
        align: center
        gap: 5
        onTap: { setupOpen = true; screen() }
        Bot { agent: {_key: "welcome", name: "Miilo", status: "ready", health: "healthy"}; s: clamp(min(availableWidth - 20, availableHeight - 30), 16, 96) }
        Text { text: "Tap to add a system"; width: availableWidth; height: 24; align: center; fontSize: 10; color: muted; maxLines: 2 }
      }
    } else {
      Grid {
        columns: columns
        gap: 6
        fill: width
        for a, slot in tileActors.take(capacity) key a._key {
          let visibleCount = min(len(tileActors), capacity)
          let neighbors = [
            {x: -1, y: 0, valid: slot % columns > 0},
            {x: 1, y: 0, valid: slot % columns < columns - 1 && slot + 1 < visibleCount},
            {x: 0, y: -1, valid: slot >= columns},
            {x: 0, y: 1, valid: slot + columns < visibleCount}
          ].filter(n => n.valid)
          let neighbor = len(neighbors) > 0 ? neighbors[floor((time + slot * 3.7) / 17) % len(neighbors)] : {x: 0, y: 0}

          let owned = live.filter(w => contains(w._actorKeys ?? [], a._key))
          let errors = any(owned, w => w.lifecycle == "failed") || contains(["failed", "error"], a.status)
          let warning = any(owned, w => w.lifecycle == "failed" || w.attention != "none") || any(attention, n => n.providerId == a.providerId && any(owned, w => w._key == n._workKey))
          let working = any(owned, w => w.lifecycle == "active") || contains(["working", "running"], a.status)
          let offline = contains(["offline", "unreachable", "disconnected"], a.status) || contains(["offline", "unreachable", "unknown"], a.health)
          Column {
            align: center
            gap: 2
            onTap: { expandedSystem = a.providerId; workflowActor = (a._preview ?? false) ? "" : a._key; selectedActor = ""; selectedWork = ""; needsOnly = false; query = ""; pageSize = 5; screen() }
            Box {
              width: robotSize + 10
              height: robotSize
              align: center
              Bot { agent: a; s: robotSize; lookX: neighbor.x; lookY: neighbor.y; slot: slot; alert: offline ? "" : errors ? "!!" : warning ? "?!" : "" }
              Box {
                width: max(1, robotSize - 6)
                height: max(1, robotSize * 0.92 - 6)
                align: bottomEnd
                if offline { StateMark { warning: warning; working: working; offline: offline; s: clamp(robotSize * 0.24, 12, 21) } }
              }
            }
            if showNames { Text { text: replace(a.name, "-", " "); width: cellWidth; height: 28; align: center; fontSize: 10; maxLines: 2 } }
          }
        }
      }
      if len(tileActors) > capacity { Text { text: "+${len(tileActors) - capacity}"; fontSize: 10; color: muted; maxLines: 1 } }
      if len(actors) == 0 { Text { text: len(live) == 0 ? "No work in flight" : "No agents reported"; style: small; color: muted } }
      if issueCount > 0 { Text { text: "${issueCount} unavailable"; fontSize: 10; color: muted } }
    }
  }
    Box {
      fill: true
      align: bottomEnd
      Text { text: "miilo.tech"; width: 64; height: 12; fontSize: 9; color: alpha(muted, 0.65); align: end; maxLines: 1 }
    }
  }
  screen {
    title: "miilo"
    Column {
      scroll: true
      gap: 10
      Setup { }
      if focused == null && actor == null { Search { placeholder: "Search systems, agents or work" } }
      if setupOpen { RobotColors { } }
      SetupTerminal { }
      if focused != null {
        WorkDetail { }
      } else if actor != null {
        Bot { agent: actor; s: 48 }
        Text { text: actor.name; style: body; maxLines: 1 }
        Flow {
          Button { text: renameOpen ? "Cancel name" : "Name"; onTap: renameOpen = !renameOpen }
        }
        if renameOpen && !requestPending {
          Input { text: actor.name; placeholder: "Display name"; onSubmit: { requestPending = true; replyWork = ""; reply = run("miilo-owl name-agent ${sh(actor._key)} ${sh(trim(value))}"); reply = "pending"; renameOpen = false } }
        }
        let owned = works.filter(w => w.providerId == actor.providerId && contains(w._actorKeys ?? [], actor._key))
        WorkRows { list: owned }
        if len(owned) == 0 { Text { text: "No assigned work reported"; style: small; color: muted } }
        WorkDetail { }
      } else {
        let actionable = works.filter(w => w.attention != "none" || any(attention, a => a._workKey == w._key))
        let visible = (needle == "" ? live : searched).filter(w => !needsOnly || any(actionable, a => a._key == w._key)).sortBy(w => any(actionable, a => a._key == w._key) ? 0 : w.lifecycle == "active" ? 1 : 2)
        WorkViewToolbar { count: len(actionable) }
        if observatoryView == "orbit" && expandedSystem != "" && any(provs, p => p.id == expandedSystem) {
          QuietSystemTasks { provider: find(provs, p => p.id == expandedSystem); list: visible }
        } else if observatoryView == "orbit" {
          OrbitView { list: visible }
        } else if needle != "" {
          let matches = actors.filter(a => contains(lower("${a.name} ${a.id}"), needle))
          if !needsOnly && len(matches) > 0 { AgentRows { list: matches } }
          WorkRows { list: visible; showSystem: true }
          if len(visible) == 0 && (needsOnly || len(matches) == 0) { Text { text: "No matches"; style: small; color: muted } }
        } else if expandedSystem == "" || !any(provs, p => p.id == expandedSystem) {
          QuietFleet { list: visible }
        } else {
          QuietSystemTasks { provider: find(provs, p => p.id == expandedSystem); list: visible }
        }
      }
    }
  }
