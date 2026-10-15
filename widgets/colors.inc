  component RobotColors() {
    Text { text: "Robot colors"; style: small; color: muted }
    Flow {
      gap: 6
      for a in actors.take(agentLimit) key a._key {
        Button { text: replace(a.name, "-", " "); color: colorAgent == a._key ? accent : muted; onTap: colorAgent = a._key }
      }
    }
    if len(actors) > agentLimit { Button { text: "More agents"; onTap: agentLimit = agentLimit + 5 } }
    if colorAgent != "" {
      let automatic = abs(sum(split(colorAgent, "").map(c => indexOf(split("0123456789abcdef", ""), c)))) % len(robotPalette)
      Flow {
        gap: 6
        Button { text: "Automatic"; onTap: robotColors = robotColors.filter(p => p.key != colorAgent) }
        for c, i in robotPalette {
          Button { text: c.name; color: c.value; onTap: robotColors = robotColors.filter(p => p.key != colorAgent) + [{key: colorAgent, color: i}] }
        }
        Button { text: "Shuffle"; onTap: robotColors = robotColors.filter(p => p.key != colorAgent) + [{key: colorAgent, color: (get(find(robotColors, p => p.key == colorAgent), "color", automatic) + 1 + floor(time) % (len(robotPalette) - 1)) % len(robotPalette)}] }
      }
    }
  }
