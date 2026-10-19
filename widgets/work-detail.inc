  component WorkReply() {
    if reply == "pending" { Text { text: "Working…"; style: small; color: muted } }
    if receipt != null && (replyWork == "" || (focused != null && focused._key == replyWork && focused._revisionKey == replyRevision)) {
      Text { text: receipt.message ?? "Refreshed"; style: small; maxLines: 8; color: receipt.status == "failed" || receipt.status == "conflict" ? red : foreground }
      for c in receipt.changes ?? [] key c.id {
        Text { text: c.name; style: small; maxLines: 4 }
        if c.preview != null && c.preview != "" { Text { text: c.preview; style: small; maxLines: 400 } }
      }
    }
  }
  component WorkDetail() {
    if focused != null {
      Spacer { height: 1; fill: width; background: alpha(muted, 0.2) }
      Text { text: focused.title; font: sans; fontSize: 16; color: foreground; maxLines: 4 }
      Row {
        fill: width
        gap: 8
        align: center
        Text { text: get(find(allProviders, p => p.id == focused.providerId), "name", "System"); weight: 1; fontSize: 11; color: muted; maxLines: 1 }
        StatusBadge { label: focused.phase }
      }
      Arc { phase: focused.phase; moving: focused.lifecycle == "active" }
      if focused.summary != null && focused.summary != "" { Text { text: focused.summary; style: small; color: muted; maxLines: 8 } }
      if (focused.attention ?? "none") != "none" {
        let notes = attention.filter(a => a._workKey == focused._key && (a.message ?? "") != "" && a.message != focused.summary)
        for note in notes key note._key { Text { text: note.message; style: small; color: foreground; maxLines: 6 } }
        if len(notes) == 0 && (focused.summary ?? "") == "" { Text { text: "Provider requests attention but supplied no review notes."; style: small; color: muted; maxLines: 3 } }
      }
      if !requestPending {
        Flow {
          gap: 6
          Button { height: 44; background: alpha(accent, 0.12); text: "Explain"; color: accent; onTap: { requestPending = true; replyWork = focused._key; replyRevision = focused._revisionKey; reply = run("export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl explain ${sh(focused._key)} ${sh(focused._revisionKey)}"); reply = "pending"; proposed = "" } }
          for a in (focused.actions ?? []).filter(a => a._supported) key a._key {
            Button { text: a.label; color: accent; onTap: { proposed = a._key; proposalRevision = focused._revisionKey; proposalWork = focused._key; reply = requestPending ? reply : "" } }
          }
          if focused.nativeUrl != "" { Button { text: "Provider ↗"; onTap: url(focused.nativeUrl) } }
        }
        if !(focused._actionsEnabled ?? false) {
          Text { text: "Actions disabled for this connection"; style: small; color: muted }
          Button { height: 44; background: alpha(accent, 0.12); text: "Action settings"; onTap: { setupOpen = true; setupSection = "systems"; setupConfiguring = false } }
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
            Button { text: "Confirm"; color: accent; onTap: { requestPending = true; replyWork = proposalWork; replyRevision = proposalRevision; reply = run("export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl action ${sh(proposalWork)} ${sh(proposed)} ${sh(proposalRevision)}"); reply = "pending"; proposed = "" } }
            Button { text: "Cancel"; onTap: proposed = "" }
          }
        } else {
          Text { text: "Task changed. Review again."; style: small; color: muted }
        }
      }
      WorkReply { }
      Column {
        gap: 6
        Spacer { height: 6 }
        Text { text: "Evidence"; font: sans; fontSize: 12; color: accent }
        let taskEvidence = evidence.filter(e => contains(focused._evidenceKeys ?? [], e._key))
        if len(taskEvidence) == 0 { Text { text: "No evidence supplied"; style: small; color: muted } }
        for e in taskEvidence.take(pageSize) key e._key {
          Row {
            fill: width
            padding: 10
            gap: 10
            radius: 7
            background: alpha(accent, 0.035)
            border: 1
            borderColor: alpha(accent, 0.14)
            let passed = contains(["passed", "pass", "success"], lower(e.outcome))
            let failed = contains(["failed", "fail", "error"], lower(e.outcome))
            let ink = passed ? mix(green, foreground, 0.25) : failed ? mix(red, foreground, 0.15) : accent
            StatusBadge { label: e.outcome; ink: ink; w: 58 }
            Text { text: e.summary; weight: 1; fontSize: 12; maxLines: 8 }
          }
          if (e._missingArtifactCount ?? 0) > 0 { Text { text: "${e._missingArtifactCount} referenced artifacts unavailable"; style: small; color: muted } }
        }
        if len(taskEvidence) > pageSize { Button { text: "More evidence"; onTap: pageSize = pageSize + 5 } }
        TaskArtifacts { }
      }
    }
    if focused == null { WorkReply { } }
  }
