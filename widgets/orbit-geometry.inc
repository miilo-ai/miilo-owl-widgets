  // Deterministic atom planes: the robot occupies an always-open nucleus.
  component OrbitField(items, anchor, offline, energized, slot = 0) {
    let dormant = !energized && contains(["idle", "resting", "ready"], anchor.status)
    let moving = robotMotion == "full" && !offline && !dormant
    let clock = moving ? time * robotSpeed : 0
    Box {
      fill: width
      height: 192
      Canvas {
        fill: true
        height: 192
        let cx = width / 2
        let cy = height / 2
        let fit = 1
        let phase = max(slot, 0) * 2.399963229728653
        let sweep = sin(clock * 2 * pi / 30 + phase) * 0.17
        let planes = [0, 1].map(lane => {
          radius: (lane == 0 ? 122 : 98) * fit,
          rise: (58 + sin(clock * 2 * pi / 18 + phase + lane * 0.9) * 6) * fit,
          tilt: (lane == 0 ? -0.44 : 0.44) + sweep
        })
        let points = items.map(w => {
          key: w._key,
          lane: indexOf(items.map(q => q._key), w._key) % 2,
          angle: indexOf(items.map(q => q._key), w._key) * 2 * pi / max(len(items), 1) + phase + (moving && w.lifecycle == "active" && w.attention == "none" ? clock * 0.24 : 0),
          active: w.lifecycle == "active",
          attention: w.attention != "none" || w.lifecycle == "failed"
        }).map(p => {
          key: p.key, active: p.active, attention: p.attention, angle: p.angle,
          radius: planes[p.lane].radius, rise: planes[p.lane].rise, tilt: planes[p.lane].tilt
        }).map(p => {
          key: p.key, active: p.active, attention: p.attention,
          z: sqrt(p.radius * p.radius - p.rise * p.rise) * sin(p.angle),
          scale: 1 + sqrt(p.radius * p.radius - p.rise * p.rise) * sin(p.angle) / p.radius * 0.13,
          px: cx + cos(p.angle) * p.radius * cos(p.tilt) - sin(p.angle) * p.rise * sin(p.tilt),
          py: cy + cos(p.angle) * p.radius * sin(p.tilt) + sin(p.angle) * p.rise * cos(p.tilt)
        })
        // Split on camera depth zero. Equal-width halves share exact endpoints.
        for side in [-1, 1] {
          let depth = side == 1 ? 1 : 0.84
          for plane in planes {
            let rail = range(65).map(i => {x: cos((side == 1 ? i : i + 64) * pi / 64) * plane.radius, y: sin((side == 1 ? i : i + 64) * pi / 64) * plane.rise}).map(p => [cx + p.x * cos(plane.tilt) - p.y * sin(plane.tilt), cy + p.x * sin(plane.tilt) + p.y * cos(plane.tilt)])
            if !offline && !dormant {
              // Portable soft neon: layered strokes, no platform blur dependency.
              path(rail, stroke: alpha(accent, 0.04 * depth), width: 5, cap: round)
              path(rail, stroke: alpha(accent, 0.12 * depth), width: 3, cap: round)
              path(rail, stroke: alpha(accent, 0.18 * depth), width: 1.8, cap: round)
            }
            path(rail, stroke: alpha(offline || dormant ? muted : accent, (offline ? 0.16 : dormant ? 0.5 : 0.55) * depth), width: 1, cap: round)
          }
        }
        // Anchored Bot uses its actual head center, preserving eyes and expressions.
        let antennaY = cy - 112 * fit * 0.27 * 1.48
        let zapTime = (clock + max(slot, 0) * 2.9) % 9
        let candidates = points.filter(p => p.active && !p.attention && p.py < cy + 9 && pow(p.px - cx, 2) + pow(p.py - antennaY, 2) > 576).sortBy(p => pow(p.px - cx, 2) + pow(p.py - antennaY, 2))
        if moving && energized && robotPersonality != "calm" && contains(["working", "running"], anchor.status) && !contains(["failed", "degraded"], anchor.health) && len(candidates) > 0 && zapTime > 2.4 && zapTime < 2.58 {
          let target = candidates[0]
          let dx = target.px - cx
          let dy = target.py - antennaY
          let distance = sqrt(dx * dx + dy * dy)
          let strength = sin((zapTime - 2.4) / 0.18 * pi)
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
          let depth = p.z >= 0 ? 1 : 0.84
          circle(p.px, p.py, 10 * p.scale * fit, fill: alpha(ink, (offline ? 0.03 : 0.07) * depth))
          circle(p.px, p.py, 4.5 * p.scale * fit, fill: alpha(ink, (offline ? 0.35 : 0.9) * depth))
          if p.attention { circle(p.px, p.py, 8 * p.scale * fit, stroke: alpha(ink, depth), width: 1.2) }
        }
      }
      Box {
        fill: true
        align: center
        Bot { agent: anchor; s: 112; slot: slot; anchored: true }
      }
    }
  }
