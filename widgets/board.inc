  let stages = ["queued", "active", "waiting"]
  Column {
    gap: 5
    Heading { label: "work in motion"; count: "${len(live)}" }
    if !configured {
      Text { text: "Tap to add systems"; style: small; color: muted }
    } else if len(live) == 0 {
      Text { text: issueCount > 0 ? "Check connection" : "No work in flight"; style: small; color: muted }
    } else {
      Canvas {
        fill: width
        height: 32
        let y = 16
        line(8, y, width - 8, y, stroke: alpha(muted, 0.25), width: 1.5)
        for phase, i in stages {
          let items = live.filter(w => w.lifecycle == phase)
          let x = 8 + i / 2 * max(width - 16, 1)
          if len(items) > 0 {
            circle(x, y, 3 + min(len(items), 5) * 0.35, fill: accent)
            if phase == "active" {
              circle(x, y, 6 + sin(time * 2), stroke: alpha(accent, 0.25), width: 1)
            }
          }
        }
      }
      Row {
        fill: width
        gap: 4
        for stage in stages {
          Column {
            weight: 1
            gap: 0
            Text { text: "${len(live.filter(w => w.lifecycle == stage))}"; fontSize: 11; color: foreground; maxLines: 1 }
            Text { text: stage == "active" ? "working" : stage; fontSize: 9; color: muted; maxLines: 1 }
          }
        }
      }
      let failed = live.filter(w => w.lifecycle == "failed")
      if len(failed) > 0 { Text { text: "${len(failed)} needs a fix"; fontSize: 10; color: red; maxLines: 1 } }
      if tileHeight >= 180 {
        let current = find(live, w => w.lifecycle == "active")
        if current != null { Text { text: current.title; fontSize: 11; maxLines: 1 } }
      }
    }
  }
  screen {
    title: "work in motion"
    Column {
      scroll: true
      gap: 10
      if setupOpen { SettingsPage { } } else {
        Setup { }
        if focused == null && actor == null { Search { placeholder: "Search work or systems" } }
        if focused != null {
          WorkDetail { }
        } else {
          if needle != "" {
            Text { text: "${len(searched)} matches"; style: small; color: muted }
            WorkRows { list: searched; showSystem: true }
            if len(searched) == 0 { Text { text: "No matching work"; style: small; color: muted } }
          } else {
            WorkGroups { list: live }
            if len(live) == 0 { Text { text: issueCount > 0 ? "Check connection" : "No work in flight"; style: small; color: muted } }
          }
        }
      }
    }
  }
