  component Arc(phase, moving) {
    Canvas {
      fill: width
      height: 25
      let index = indexOf(phases, phase)
      let cx = 18 + clamp(index, 0, 4) / 4 * max(width - 36, 1)
      // Sample the hump locally, so a wide panel never turns it into a triangle.
      let hump = range(65).map(i => { t: i / 64 }).map(p => [cx - 18 + p.t * 36, 17 - 7.5 * (1 - cos(p.t * 2 * pi)) / 2])
      let points = [[0, 17]] + hump + [[width, 17]]
      if index >= 0 {
        path(points, stroke: alpha(accent, 0.7), width: 1.5, cap: round)
        circle(cx, 17, moving ? 3.4 + 0.4 * sin(time * 2) : 3.4, fill: accent)
      } else {
        line(0, 17, width, 17, stroke: alpha(muted, 0.25), width: 1.5)
      }
    }
  }
