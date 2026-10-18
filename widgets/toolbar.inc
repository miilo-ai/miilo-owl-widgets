  component ToolbarIcon(kind) {
    Canvas {
      width: 24
      height: 24
      if kind == "settings" {
        circle(12, 12, 7.2, stroke: foreground, width: 1.5)
        circle(12, 12, 2.8, stroke: foreground, width: 1.5)
        for i in [0, 1, 2, 3, 4, 5, 6, 7] {
          let angle = i * 0.7853981634
          line(12 + cos(angle) * 7.2, 12 + sin(angle) * 7.2, 12 + cos(angle) * 10.5, 12 + sin(angle) * 10.5, stroke: foreground, width: 1.5)
        }
      } else if kind == "refresh" {
        let rail = range(33).map(i => [12 + 9.5 * cos(0.35 + i * 5.38 / 32), 12 + 9.5 * sin(0.35 + i * 5.38 / 32)])
        path(rail, stroke: foreground, width: 1.5, cap: round)
        path([[15.1, 7], [20.1, 7], [20.1, 2]], stroke: foreground, width: 1.5, cap: round)
      } else if kind == "back" {
        path([[20, 12], [4, 12], [11, 5]], stroke: foreground, width: 1.5, cap: round)
        path([[4, 12], [11, 19]], stroke: foreground, width: 1.5, cap: round)
      } else if kind == "menu" {
        for y in [5, 12, 19] { line(3, y, 21, y, stroke: foreground, width: 1.5) }
      }
    }
  }
  component ToolbarControls() {
    Row {
      gap: 4
      align: center
      Box { width: 44; height: toolbarHeight; align: center; onTap: setupOpen = !setupOpen
        ToolbarIcon { kind: "settings" }
      }
      if !requestPending {
        Box { width: 44; height: toolbarHeight; align: center; onTap: { requestPending = true; replyWork = ""; replyRevision = ""; reply = run("miilo-owl refresh"); reply = "pending" }
          ToolbarIcon { kind: "refresh" }
        }
      }
      if len(allProviders) > 1 {
        Box { width: 44; height: toolbarHeight; align: center; onTap: systemsOpen = !systemsOpen
          ToolbarIcon { kind: "menu" }
        }
      }
    }
  }
