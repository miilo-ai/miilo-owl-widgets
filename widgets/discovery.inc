  component Search(placeholder) {
    Row {
      fill: width
      gap: 6
      align: center
      if !searchEditing && leftHanded && configured { ToolbarControls { } }
      if searchEditing {
        Box {
          weight: 1
          Input {
            text: query
            placeholder: placeholder
            focus: true
            onChange: { query = value; pageSize = 5; agentLimit = 5 }
            onSubmit: { query = trim(value); pageSize = 5; searchEditing = false }
          }
        }
        Box { width: 36; height: 44; align: center; onTap: searchEditing = false
          Text { text: "✓"; width: 36; fontSize: 22; align: center; color: accent }
        }
      } else {
        Box { weight: 1; height: 44; align: start; padding: 6; radius: 6; background: alpha(muted, 0.08); onTap: searchEditing = true
          Text { text: query == "" ? placeholder : query; fill: width; fontSize: 13; color: query == "" ? muted : foreground; maxLines: 1 }
        }
        if needle != "" { Box { width: 30; height: 44; align: center; onTap: { query = ""; pageSize = 5 }
          Text { text: "×"; width: 30; fontSize: 22; align: center; color: muted }
        } }
        if !leftHanded && configured { ToolbarControls { } }
      }
    }
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
