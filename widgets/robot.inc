  component Bot(agent, s) {
    Canvas {
      width: s
      height: s
      let cx = width / 2
      let cy = height * 0.48
      let r = min(width, height) * 0.27
      let on = contains(["working", "running"], agent.status)
      let resting = contains(["idle", "resting"], agent.status) && agent.health == "healthy"
      let phase = max(indexOf(actors, agent), 0) * 0.8
      let blinking = (time + phase) % 5.8 < 0.16
      let led = agent.health == "degraded" ? red : resting ? alpha(accent, 0.55) : accent
      let bob = on ? sin(time * 1.8 + phase) * 1.2 : 0
      let y = cy + bob
      // A soft halo, rounded shell, inset visor, and two luminous eyes.
      if on {
        circle(cx, y, r * 1.6, fill: alpha(accent, 0.07 + 0.03 * sin(time * 2)))
      }
      line(cx, y - r, cx, y - r * 1.35, stroke: alpha(led, 0.6), width: 1.5)
      circle(cx, y - r * 1.48, r * 0.12, fill: led)
      rect(cx - r * 1.25, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(accent, 0.55))
      rect(cx + r * 1.03, y - r * 0.25, r * 0.22, r * 0.55, radius: 2, fill: alpha(accent, 0.55))
      rect(cx - r, y - r * 0.8, r * 2, r * 1.6, radius: r * 0.55, fill: mix(background, accent, 0.28), stroke: alpha(accent, 0.65), width: 1)
      rect(cx - r * 0.76, y - r * 0.48, r * 1.52, r * 0.86, radius: r * 0.3, fill: alpha(background, 0.9))
      if resting || blinking {
        line(cx - r * 0.5, y, cx - r * 0.2, y, stroke: led, width: 1.5, cap: round)
        line(cx + r * 0.2, y, cx + r * 0.5, y, stroke: led, width: 1.5, cap: round)
      } else {
        circle(cx - r * 0.34, y - r * 0.06, r * 0.14, fill: led)
        circle(cx + r * 0.34, y - r * 0.06, r * 0.14, fill: led)
      }
      if resting {
        // Keep the drifting letters inside the compact robot canvas.
        for i in range(2) {
          let drift = (time * 0.28 + i * 0.5) % 1
          text("z", cx + r * (0.8 + drift * 0.55), y - r * (0.65 + drift * 0.8), size: 7 + drift * 2, color: alpha(accent, 0.45 * (1 - drift)))
        }
      }
      line(cx - r * 0.18, y + r * 0.56, cx + r * 0.18, y + r * 0.56, stroke: alpha(accent, 0.6), width: 1.5)
      rect(cx - r * 0.48, y + r, r * 0.96, r * 0.24, radius: 2, fill: alpha(accent, 0.35))
    }
  }
