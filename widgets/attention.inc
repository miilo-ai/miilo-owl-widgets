  let ordered = attention.sortBy(a => a.level == "intervention" ? 0 : a.level == "approval" ? 1 : 2)
  Column {
    gap: 5
    Heading { label: "needs you"; count: "${len(attention)}" }
    if !configured {
      Text { text: "Tap to add systems"; style: small; color: muted }
    } else if len(ordered) == 0 {
      Text { text: issueCount > 0 ? "Check connection" : "Nothing needs you"; style: small; color: muted }
    } else {
      let a = ordered[0]
      Row {
        gap: 8
        align: center
        Bot { agent: {status: "reviewing", health: "healthy"}; s: 34 }
        Text { text: a.level == "intervention" ? "Needs a fix" : a.level == "approval" ? "Review" : "Decision"; style: small; color: a.level == "intervention" ? red : accent; weight: 1; maxLines: 1 }
      }
      Text { text: a._title ?? a.message; fontSize: 11; maxLines: 1 }
      Text { text: get(find(allProviders, p => p.id == a.providerId), "name", "System"); fontSize: 10; color: muted; maxLines: 1 }
    }
  }
  screen {
    title: "needs you"
    Column {
      scroll: true
      gap: 10
      if setupOpen { SettingsPage { } } else {
        Setup { }
        if focused == null && actor == null { Search { placeholder: "Search attention" } }
        if focused != null {
          WorkDetail { }
        } else {
          let matches = ordered.filter(a => contains(lower("${a._title ?? a.message} ${a.reason ?? ""} ${get(find(allProviders, p => p.id == a.providerId), "name", "")}"), needle))
        for a in matches.take(pageSize) key a._key {
            Column {
            gap: 2
            fill: width
            onTap: { selectedWork = a._workKey ?? ""; evidenceOpen = false; proposed = ""; reply = requestPending ? reply : "" }
            Row {
              fill: width
              Text { text: a._title ?? a.message; style: body; weight: 1; maxLines: 2 }
              Text { text: "›"; width: 10; style: small; color: muted }
            }
            Text { text: "${get(find(allProviders, p => p.id == a.providerId), "name", "System")} · ${a.level}"; style: small; color: a.level == "intervention" ? red : muted; maxLines: 1 }
            if a._workKey == "" { Text { text: a.reason; style: small; color: muted; maxLines: 2 } }
          }
        }
        if len(matches) == 0 { Text { text: issueCount > 0 ? "Check connection" : "Nothing needs you"; style: small; color: muted } }
        if len(matches) > pageSize { Button { text: "+${len(matches) - pageSize} more"; onTap: pageSize = pageSize + 5 } }
        }
      }
    }
  }
