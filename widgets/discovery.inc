  component Search(placeholder) {
    Input {
      text: query
      placeholder: placeholder
      onChange: { query = value; pageSize = 5; agentLimit = 5 }
      onSubmit: { query = trim(value); pageSize = 5 }
    }
    if needle != "" { Button { text: "Clear"; onTap: { query = ""; pageSize = 5 } } }
  }

  component BackToList() {
    Button { text: "←"; onTap: { selectedWork = ""; proposed = ""; evidenceOpen = false; reply = requestPending ? reply : "" } }
  }

  component SystemRow(provider, count) {
    Row {
      fill: width
      gap: 6
      onTap: { expandedSystem = expandedSystem == provider.id ? "" : provider.id; pageSize = 5; agentLimit = 5; workflowActor = "" }
      Text { text: provider.name; style: body; weight: 1; maxLines: 1 }
      Text { text: count; width: max(20, len(count) * 6); fontSize: 10; color: muted; align: end; maxLines: 1 }
      Text { text: expandedSystem == provider.id ? "⌄" : "›"; width: 10; style: small; color: muted }
    }
  }

  component WorkGroups(list) {
    let systems = provs.filter(p => any(list, w => w.providerId == p.id))
    for p in systems.take(providerLimit) key p.id {
      let tasks = list.filter(w => w.providerId == p.id)
      SystemRow { provider: p; count: "${len(tasks)}" }
      if expandedSystem == p.id {
        Text { text: "${len(tasks.filter(w => w.lifecycle == "active"))} working · ${len(tasks.filter(w => w.lifecycle == "waiting"))} waiting · ${len(tasks.filter(w => w.lifecycle == "failed"))} failed"; style: small; color: muted; maxLines: 1 }
        WorkRows { list: tasks }
      }
    }
    if len(systems) > providerLimit { Button { text: "More systems"; onTap: providerLimit = providerLimit + 5 } }
  }
