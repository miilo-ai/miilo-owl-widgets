  // Presentation only: labels and provider action policy remain authoritative.
  component OrbitStateIcon(phase, attention, offline, failed, ink = accent) {
    Canvas {
      width: 14
      height: 14
      // Optical alignment with the title glyphs, rather than the line box.
      offset: [0, 1]
      if offline {
        circle(7, 7, 4.5, stroke: ink, width: 1)
        line(3, 11, 11, 3, stroke: ink, width: 1)
      } else if failed {
        line(3.5, 3.5, 10.5, 10.5, stroke: ink, width: 1.3)
        line(3.5, 10.5, 10.5, 3.5, stroke: ink, width: 1.3)
      } else if attention != "none" {
        // Canvas text has different ascent from vector strokes.
        text("?!", 7, 10, size: 12, bold: true, align: center, color: ink)
      } else if phase == "implementing" {
        path([[4.5, 3], [1.5, 7], [4.5, 11]], stroke: ink, width: 1.1, cap: round)
        path([[9.5, 3], [12.5, 7], [9.5, 11]], stroke: ink, width: 1.1, cap: round)
        line(8, 3.5, 6, 10.5, stroke: ink, width: 1)
      } else if phase == "verifying" {
        path([[6, 2], [6, 7], [2.5, 11], [3.5, 12], [10.5, 12], [11.5, 11], [8, 7], [8, 2]], stroke: ink, width: 1, cap: round)
        line(5, 2, 9, 2, stroke: ink, width: 1)
        line(5, 8, 9, 8, stroke: ink, width: 1)
      } else if contains(["reviewing", "review"], phase) {
        circle(6, 6, 3.5, stroke: ink, width: 1)
        line(8.5, 8.5, 12, 12, stroke: ink, width: 1.2)
      } else {
        circle(7, 7, 4.5, stroke: ink, width: 1)
        path([[7, 4], [7, 7], [9, 8]], stroke: ink, width: 1, cap: round)
      }
    }
  }
