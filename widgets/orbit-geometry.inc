  component OrbitField(items, anchor, offline, energized, slot = 0) {
    let moving = robotMotion == "full" && !offline
    let clock = moving ? time * robotSpeed : 0
    Box {
      fill: width
      height: 192
      Canvas {
        fill: true
        height: 192
        let cx = width / 2
        let cy = height / 2
        let rx = max(30, min(width * 0.38, 142))
        let ry = 48
        let tilt = -0.12
        // A 20-second, +/-45 degree rock; freeze with the existing motion policy.
        let crossTilt = sin(clock * 2 * pi / 20 + pi / 4 + slot * 0.3) * pi / 4
        let points = items.map(w => {
          key: w._key,
          lane: indexOf(items.map(q => q._key), w._key) % 2,
          angle: indexOf(items.map(q => q._key), w._key) * 2 * pi / max(len(items), 1) + (moving && w.lifecycle == "active" ? clock * 0.24 : 0),
          active: w.lifecycle == "active",
          attention: w.attention != "none" || w.lifecycle == "failed"
        }).map(p => {
          key: p.key, angle: p.angle, active: p.active, attention: p.attention,
          radius: p.lane == 0 ? rx : rx * 0.72,
          rise: p.lane == 0 ? ry : 30,
          tilt: p.lane == 0 ? tilt : crossTilt
        }).map(p => {
          key: p.key, active: p.active, attention: p.attention,
          z: sin(p.angle),
          px: cx + cos(p.angle) * p.radius * cos(p.tilt) - sin(p.angle) * p.rise * sin(p.tilt),
          py: cy + cos(p.angle) * p.radius * sin(p.tilt) + sin(p.angle) * p.rise * cos(p.tilt)
        })
        let ring = range(129).map(i => {x: cos((i % 128) * 2 * pi / 128) * rx, y: sin((i % 128) * 2 * pi / 128) * ry}).map(p => [cx + p.x * cos(tilt) - p.y * sin(tilt), cy + p.x * sin(tilt) + p.y * cos(tilt)])
        path(ring, stroke: alpha(accent, offline ? 0.08 : 0.24), width: 1, cap: round)
        let inner = range(129).map(i => {x: cos((i % 128) * 2 * pi / 128) * rx * 0.72, y: sin((i % 128) * 2 * pi / 128) * 30}).map(p => [cx + p.x * cos(crossTilt) - p.y * sin(crossTilt), cy + p.x * sin(crossTilt) + p.y * cos(crossTilt)])
        path(inner, stroke: alpha(accent, offline ? 0.06 : 0.22), width: 1, cap: round)
        if moving && energized && slot < 3 {
          let cycle = (clock + slot * 1.7) % 5
          if cycle < 0.7 {
            let fade = min(cycle / 0.12, min((0.7 - cycle) / 0.22, 1))
            let a = floor((clock + slot * 1.7) / 5) * 2.399 + cycle * 0.8
            // The light follows the same ellipse. Never perturb the travel rail.
            let filament = range(33).map(k => {x: cos(a + k / 32 * 0.8) * rx, y: sin(a + k / 32 * 0.8) * ry}).map(p => [cx + p.x * cos(tilt) - p.y * sin(tilt), cy + p.x * sin(tilt) + p.y * cos(tilt)])
            path(filament, stroke: alpha(accent, fade * 0.06), width: 4, cap: round)
            path(filament, stroke: alpha(accent, fade * 0.2), width: 2, cap: round)
            path(filament, stroke: alpha(mix(accent, foreground, 0.2), fade * 0.85), width: 0.9, cap: round)
          }
        }
        // Decorative electrical activity, never a provider message or action receipt.
        // Match the shared 93dp Bot antenna, including its expressive bob.
        let antennaY = cy + 93 * (0.48 - 0.5 - 0.27 * 1.48) + (robotPersonality != "calm" ? sin(clock * 1.8 + max(slot, 0) * 0.8) * 1.2 : 0)
        let zapTime = (clock + max(slot, 0) * 2.9) % 9
        let candidates = points.filter(p => p.active && !p.attention && pow(p.px - cx, 2) + pow(p.py - antennaY, 2) > 576).sortBy(p => pow(p.px - cx, 2) + pow(p.py - antennaY, 2))
        if moving && energized && robotPersonality != "calm" && contains(["working", "running"], anchor.status) && !contains(["failed", "degraded"], anchor.health) && len(candidates) > 0 && zapTime > 2.4 && zapTime < 2.58 {
          let target = candidates[0]
          let dx = target.px - cx
          let dy = target.py - antennaY
          let distance = sqrt(dx * dx + dy * dy)
          let strength = sin((zapTime - 2.4) / 0.18 * pi)
          // A cool secondary tone remains visible when the host omits ANSI cyan.
          let electric = mix(accent, mix(cyan, hsl(185, 0.8, 0.65), 0.75), 0.8)
          let bolt = range(7).map(i => {t: i / 6, bend: i == 0 || i == 6 ? 0 : (i % 2 == 0 ? -1 : 1) * (3 + sin(floor(clock / 9) + i) * 1.5)}).map(p => [cx + dx * p.t - dy / distance * p.bend, antennaY + dy * p.t + dx / distance * p.bend])
          path(bolt, stroke: alpha(electric, strength * 0.08), width: 5, cap: round)
          path(bolt, stroke: alpha(electric, strength * 0.28), width: 2.5, cap: round)
          path(bolt, stroke: alpha(mix(electric, foreground, 0.2), strength * 0.95), width: 1, cap: round)
          circle(cx, antennaY, 4, fill: alpha(electric, strength * 0.35))
          circle(target.px, target.py, 7, stroke: alpha(electric, strength * 0.65), width: 1)
        }
        for p in points.sortBy(p => p.z) {
          let ink = offline ? muted : p.attention ? accent : mix(accent, foreground, 0.15)
          let scale = 1 + p.z * 0.15
          circle(p.px, p.py, 10 * scale, fill: alpha(ink, offline ? 0.03 : 0.07))
          circle(p.px, p.py, 4.5 * scale, fill: alpha(ink, offline ? 0.35 : 0.9))
          if p.attention { circle(p.px, p.py, 8 * scale, stroke: ink, width: 1.2) }
        }

      }
      Box {
        fill: true
        align: center
        Bot { agent: anchor; s: 93; slot: slot }
      }
    }
  }
