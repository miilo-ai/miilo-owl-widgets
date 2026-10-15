  component Bot(agent, s, lookX = 0, lookY = 0, slot = -1, alert = "") {
    Canvas {
      width: s
      height: s
      let automatic = abs(sum(split(agent._key ?? agent.id ?? agent.name ?? "", "").map(c => indexOf(split("0123456789abcdef", ""), c)))) % len(robotPalette)
      let chosen = get(find(robotColors, p => p.key == (agent._key ?? "")), "color", automatic)
      let ink = mix(robotPalette[chosen % len(robotPalette)].value, foreground, 0.12)
      let blocked = contains(["blocked", "failed"], agent.status) || any(works, w => contains(w._actorKeys ?? [], agent._key ?? "") && (w.lifecycle == "failed" || w.attention != "none"))
      let wistful = blocked && (time + max(slot, 0) * 3.7) % 23 < 2.5
      let cx = width / 2
      let cy = height * 0.48
      let r = min(width, height) * 0.27
      let on = contains(["working", "running"], agent.status)
      let resting = contains(["idle", "resting"], agent.status) && agent.health == "healthy" && !blocked
      let phase = (slot >= 0 ? slot : max(indexOf(actors, agent), 0)) * 0.8
      let glanceTime = (time + max(slot, 0) * 3.7) % 17
      let glance = !resting && glanceTime < 2 ? pow(sin(glanceTime / 2 * 3.14159), 2) : 0
      let eyeX = lookX * r * 0.2 * glance
      let eyeY = lookY * r * 0.16 * glance + (wistful ? r * 0.14 : 0)
      let blinking = (time + phase) % 5.8 < 0.16
      let led = agent.health == "degraded" ? red : resting ? alpha(ink, 0.55) : ink
      let bob = on ? sin(time * 1.8 + phase) * 1.2 : 0
      let y = cy + bob
      // A soft halo, rounded shell, inset visor, and two luminous eyes.
      if on {
        circle(cx, y, r * 1.6, fill: alpha(ink, 0.07 + 0.03 * sin(time * 2)))
      }
      line(cx, y - r, cx, y - r * 1.35, stroke: alpha(led, 0.6), width: 1.5)
      circle(cx, y - r * 1.48, r * 0.12, fill: led)
      rect(cx - r * 1.25, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(ink, 0.55))
      rect(cx + r * 1.03, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(ink, 0.55))
      rect(cx - r, y - r * 0.8, r * 2, r * 1.6, radius: r * 0.55, fill: mix(background, ink, 0.28), stroke: alpha(ink, 0.65), width: 1)
      rect(cx - r * 0.76, y - r * 0.48, r * 1.52, r * 0.86, radius: r * 0.3, fill: alpha(background, 0.9))
      if wistful {
        line(cx - r * 0.48 + eyeX, y + r * 0.08, cx - r * 0.2 + eyeX, y + r * 0.17, stroke: led, width: 1.8, cap: round)
        line(cx + r * 0.2 + eyeX, y + r * 0.17, cx + r * 0.48 + eyeX, y + r * 0.08, stroke: led, width: 1.8, cap: round)
      } else if resting || blinking {
        line(cx - r * 0.5, y, cx - r * 0.2, y, stroke: led, width: 1.5, cap: round)
        line(cx + r * 0.2, y, cx + r * 0.5, y, stroke: led, width: 1.5, cap: round)
      } else {
        circle(cx - r * 0.34 + eyeX, y - r * 0.06 + eyeY, r * 0.14, fill: led)
        circle(cx + r * 0.34 + eyeX, y - r * 0.06 + eyeY, r * 0.14, fill: led)
      }
      if alert != "" {
        let markSize = min(s * 0.34, max(19.2, s * 0.288))
        let markX = cx + r * 0.88 - 8
        let markY = y - r * 0.7 - 7
        if alert == "!!" {
          text("!", markX, markY, size: markSize, bold: true, color: mix(red, foreground, 0.15))
          text("!", markX + markSize * 0.3, markY, size: markSize, bold: true, color: mix(red, foreground, 0.15))
        } else {
          text(alert, markX, markY, size: markSize, bold: true, color: mix(orange, foreground, 0.2))
        }
      }
      if resting {
        // Keep the drifting letters inside the compact robot canvas.
        for i in range(2) {
          let drift = (time * 0.28 + i * 0.5) % 1
          text("z", cx + r * (0.8 + drift * 0.55), y - r * (0.65 + drift * 0.8), size: max(9, s * 0.12) + drift * 2, bold: true, color: alpha(mix(ink, foreground, 0.8), 0.9 * (1 - drift * drift)))
        }
      }
      if wistful { path([[cx - r * 0.18, y + r * 0.62], [cx, y + r * 0.52], [cx + r * 0.18, y + r * 0.62]], stroke: alpha(ink, 0.6), width: 1.5) }
      else { line(cx - r * 0.18, y + r * 0.56, cx + r * 0.18, y + r * 0.56, stroke: alpha(ink, 0.6), width: 1.5) }
      rect(cx - r * 0.48, y + r, r * 0.96, r * 0.24, radius: 2, fill: alpha(ink, 0.35))
    }
  }
