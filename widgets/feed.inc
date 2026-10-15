  Column {
    gap: 6
    Heading { label: "miilo feed"; count: "${len(busy)} working" }
    if !configured {
      Text { text: "Tap to add systems"; style: small; color: muted }
    } else {
      Row {
        gap: 5
        fill: width
        for a in actors.take(tileWidth < 190 ? 3 : 4) key a._key {
          Column {
            weight: 1
            gap: 1
            Bot { agent: a; s: tileWidth < 190 ? 40 : 48 }
            Text { text: a.name; fontSize: 10; maxLines: 1 }
          }
        }
      }
      let preview = live.sortBy(w => w.lifecycle == "failed" ? 0 : w.attention != "none" ? 1 : 2).take(tileHeight >= 230 ? 2 : 1)
      for w in preview key w._key {
        Column {
          gap: 1
          fill: width
          Text { text: w.title; fontSize: 11; maxLines: 1 }
          Text { text: "${get(find(allProviders, p => p.id == w.providerId), "name", "System")} · ${w.lifecycle == "failed" ? "Needs a fix" : w.attention != "none" ? "Action" : w.phase}"; fontSize: 10; color: w.lifecycle == "failed" ? red : muted; maxLines: 1 }
          Arc { phase: w.phase; moving: w.lifecycle == "active" }
        }
      }
      if len(live) == 0 { Text { text: issueCount > 0 ? "Check connection" : "No work in flight"; style: small; color: muted } }
      if issueCount > 0 { Text { text: "${issueCount} unavailable"; fontSize: 10; color: red } }
    }
  }
  screen {
    title: "miilo feed"
    Column {
      scroll: true
      gap: 10
      Setup { }
      SetupTerminal { }
      for a in actors.take(actorLimit) key a._key {
        Row {
          gap: 8
          fill: width
          align: center
          onTap: { selectedActor = a._key; selectedWork = ""; reply = requestPending ? reply : ""; proposed = "" }
          Bot { agent: a; s: 38 }
          Column {
            weight: 1
            gap: 1
            Text { text: a.name; style: body; maxLines: 1 }
            Text { text: "${get(find(allProviders, p => p.id == a.providerId), "name", "System")} · ${a.status}"; style: small; color: muted; maxLines: 1 }
          }
          Text { text: "›"; style: small; color: muted }
        }
      }
      if len(actors) > actorLimit { Button { text: "More agents"; onTap: actorLimit = actorLimit + 8 } }
      if actor != null {
        Text { text: actor.name; style: label; color: accent }
        let owned = works.filter(w => w.providerId == actor.providerId && contains(w._actorKeys ?? [], actor._key))
        WorkRows { list: owned }
        if len(owned) == 0 { Text { text: "No assigned work reported"; style: small; color: muted } }
      } else {
        WorkRows { list: live }
      }
      WorkDetail { }
    }
  }
