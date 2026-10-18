  component Bot(agent, s, lookX = 0, lookY = 0, slot = -1, alert = "", anchored = false) {
    Canvas {
      width: s
      height: s
      let automatic = abs(sum(split(agent._key ?? agent.id ?? agent.name ?? "", "").map(c => indexOf(split("0123456789abcdef", ""), c)))) % len(robotPalette)
      let chosen = get(find(robotColors, p => p.key == (agent._key ?? "")), "color", automatic)
      let ink = mix(robotPalette[chosen % len(robotPalette)].value, foreground, 0.12)
      let motion = robotMotion != "off"
      let expressive = robotPersonality != "calm" && robotMotion == "full"
      let clock = motion ? time * robotSpeed : 0
      let unavailable = contains(["offline", "unreachable", "disconnected"], agent.status) || contains(["offline", "unreachable", "unknown"], agent.health)
      let owned = works.filter(w => contains(w._actorKeys ?? [], agent._key ?? ""))
      let failed = alert == "!!" || agent.status == "failed" || agent.status == "error" || any(owned, w => w.lifecycle == "failed")
      let focused = !unavailable && !failed && (agent.status == "reviewing" || any(owned, w => w.lifecycle == "active" && contains(["verifying", "reviewing"], w.phase)))
      let blocked = alert != "" || contains(["blocked", "failed"], agent.status) || any(works, w => contains(w._actorKeys ?? [], agent._key ?? "") && (w.lifecycle == "failed" || w.attention != "none"))
      let expressionAlert = unavailable ? "" : failed ? "!!" : blocked ? "?!" : ""
      let wistful = expressive && !unavailable && blocked && (clock + max(slot, 0) * 3.7) % 23 < 2.5
      let cx = width / 2
      let cy = height * (anchored ? 0.5 : 0.48)
      let r = min(width, height) * 0.27
      let on = !unavailable && !failed && contains(["working", "running"], agent.status)
      let resting = !unavailable && contains(["idle", "resting"], agent.status) && agent.health == "healthy" && !blocked
      let phase = (slot >= 0 ? slot : max(indexOf(actors, agent), 0)) * 0.8
      let glanceTime = (clock + max(slot, 0) * 3.7) % 17
      let glance = expressive && !unavailable && !resting && glanceTime < 2 ? pow(sin(glanceTime / 2 * 3.14159), 2) : 0
      let eyeX = lookX * r * 0.2 * glance
      let eyeY = lookY * r * 0.16 * glance + (wistful ? r * 0.14 : 0)
      let blinking = motion && !unavailable && (clock + phase) % 5.8 < 0.16
      let wink = expressive && !blocked && !focused && !unavailable && !resting && (clock + phase) % 29 > 27.8 && (clock + phase) % 29 < 28.05
      let led = unavailable ? muted : failed || agent.health == "degraded" ? red : resting ? alpha(ink, 0.55) : ink
      let bob = on && expressive && !anchored ? sin(clock * 1.8 + phase) * 1.2 : 0
      let y = cy + bob
      // A soft halo, rounded shell, inset visor, and two luminous eyes.
      if on && expressive {
        circle(cx, y, r * 1.6, fill: alpha(ink, 0.07 + 0.03 * sin(clock * 2)))
      }
      line(cx, y - r, cx, y - r * 1.35, stroke: alpha(led, 0.6), width: 1.5)
      circle(cx, y - r * 1.48, r * 0.12, fill: led)
      rect(cx - r * 1.25, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(ink, 0.55))
      rect(cx + r * 1.03, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(ink, 0.55))
      rect(cx - r, y - r * 0.8, r * 2, r * 1.6, radius: r * 0.55, fill: mix(background, ink, 0.28), stroke: alpha(ink, 0.65), width: 1)
      rect(cx - r * 0.76, y - r * 0.48, r * 1.52, r * 0.86, radius: r * 0.3, fill: alpha(background, 0.9))
      if unavailable {
        for side in [-1, 1] {
          let ex = cx + side * r * 0.34
          line(ex - r * 0.1, y - r * 0.17, ex + r * 0.1, y + r * 0.03, stroke: led, width: 1.5, cap: round)
          line(ex + r * 0.1, y - r * 0.17, ex - r * 0.1, y + r * 0.03, stroke: led, width: 1.5, cap: round)
        }
      } else if failed {
        circle(cx - r * 0.34, y - r * 0.06, r * 0.21, fill: ink)
        circle(cx + r * 0.34, y - r * 0.06, r * 0.14, fill: ink)
      } else if wistful {
        line(cx - r * 0.48 + eyeX, y + r * 0.08, cx - r * 0.2 + eyeX, y + r * 0.17, stroke: led, width: 1.8, cap: round)
        line(cx + r * 0.2 + eyeX, y + r * 0.17, cx + r * 0.48 + eyeX, y + r * 0.08, stroke: led, width: 1.8, cap: round)
      } else if resting || blinking {
        line(cx - r * 0.5, y, cx - r * 0.2, y, stroke: led, width: 1.5, cap: round)
        line(cx + r * 0.2, y, cx + r * 0.5, y, stroke: led, width: 1.5, cap: round)
      } else if focused {
        rect(cx - r * 0.53 + eyeX, y - r * 0.1, r * 0.32, r * 0.12, radius: 1, fill: led)
        rect(cx + r * 0.21 + eyeX, y - r * 0.1, r * 0.32, r * 0.12, radius: 1, fill: led)
      } else if wink {
        line(cx - r * 0.5, y, cx - r * 0.2, y, stroke: led, width: 1.5, cap: round)
        circle(cx + r * 0.34, y - r * 0.06, r * 0.14, fill: led)
      } else {
        circle(cx - r * 0.34 + eyeX, y - r * 0.06 + eyeY, r * 0.14, fill: led)
        circle(cx + r * 0.34 + eyeX, y - r * 0.06 + eyeY, r * 0.14, fill: led)
      }
      if expressionAlert != "" {
        let markSize = min(s * 0.34, max(19.2, s * 0.288))
        let markX = cx + r * 0.88 - 8
        let markY = y - r * 0.7 - 7
        if expressionAlert == "!!" {
          text("!", markX, markY, size: markSize, bold: true, color: mix(red, foreground, 0.15))
          text("!", markX + markSize * 0.3, markY, size: markSize, bold: true, color: mix(red, foreground, 0.15))
        } else {
          text(expressionAlert, markX, markY, size: markSize, bold: true, color: mix(orange, foreground, 0.2))
        }
      }
      if resting {
        // Keep the drifting letters inside the compact robot canvas.
        for i in range(2) {
          let drift = (motion && robotMotion == "full" ? clock * 0.28 + i * 0.5 : 0.2 + i * 0.4) % 1
          text("z", cx + r * (0.8 + drift * 0.55), y - r * (0.65 + drift * 0.8), size: max(9, s * 0.12) + drift * 2, bold: true, color: alpha(mix(ink, foreground, 0.8), 0.9 * (1 - drift * drift)))
        }
      }
      if failed || wistful { path([[cx - r * 0.18, y + r * 0.62], [cx, y + r * 0.52], [cx + r * 0.18, y + r * 0.62]], stroke: alpha(ink, 0.6), width: 1.5) }
      else { line(cx - r * 0.18, y + r * 0.56, cx + r * 0.18, y + r * 0.56, stroke: alpha(ink, 0.6), width: 1.5) }
      rect(cx - r * 0.48, y + r, r * 0.96, r * 0.24, radius: 2, fill: alpha(ink, 0.35))
    }
  }
