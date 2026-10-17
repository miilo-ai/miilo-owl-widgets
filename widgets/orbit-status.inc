  // Presentation only: labels and provider action policy remain authoritative.
  component OrbitStateIcon(phase, attention, offline, failed) {
    Canvas {
      width: 14
      height: 14
      let ink = offline ? muted : accent
      if offline {
        circle(7, 7, 4.5, stroke: ink, width: 1)
        line(3, 11, 11, 3, stroke: ink, width: 1)
      } else if failed {
        line(3.5, 3.5, 10.5, 10.5, stroke: ink, width: 1.3)
        line(3.5, 10.5, 10.5, 3.5, stroke: ink, width: 1.3)
      } else if attention != "none" {
        line(7, 2.5, 7, 8, stroke: ink, width: 1.4)
        circle(7, 11, 0.9, fill: ink)
      } else if phase == "implementing" {
        path([[4.5, 3], [1.5, 7], [4.5, 11]], stroke: ink, width: 1.1, cap: round)
        path([[9.5, 3], [12.5, 7], [9.5, 11]], stroke: ink, width: 1.1, cap: round)
        line(8, 3.5, 6, 10.5, stroke: ink, width: 1)
      } else if contains(["verifying", "reviewing", "review"], phase) {
        circle(6, 6, 3.5, stroke: ink, width: 1)
        line(8.5, 8.5, 12, 12, stroke: ink, width: 1.2)
      } else {
        circle(7, 7, 4.5, stroke: ink, width: 1)
        path([[7, 4], [7, 7], [9, 8]], stroke: ink, width: 1, cap: round)
      }
    }
  }
