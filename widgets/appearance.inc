  // Presentation preferences only: never alter observations or action authority.
  state robotMotion = "full" persist
  state robotPace = "normal" persist
  state robotPersonality = "expressive" persist
  let robotSpeed = robotPace == "slow" ? 0.7 : robotPace == "lively" ? 1.3 : 1
  component RobotAppearance() {
    Text { text: "Personality"; style: small; color: muted }
    Flow {
      gap: 6
      for option in ["calm", "expressive"] {
        Button { height: 28; text: option == "calm" ? "Calm" : "Expressive"; background: alpha(accent, robotPersonality == option ? 0.14 : 0); color: robotPersonality == option ? accent : muted; onTap: robotPersonality = option }
      }
    }
    Text { text: "Motion"; style: small; color: muted }
    Flow {
      gap: 6
      for option in ["full", "reduced", "off"] {
        Button { height: 28; text: option == "full" ? "Full" : option == "reduced" ? "Reduced" : "Off"; background: alpha(accent, robotMotion == option ? 0.14 : 0); color: robotMotion == option ? accent : muted; onTap: robotMotion = option }
      }
    }
    if robotMotion != "off" {
      Text { text: "Pace"; fontSize: 12; color: muted }
      Flow {
        gap: 6
        for option in ["slow", "normal", "lively"] {
          Button { height: 28; text: option == "slow" ? "Slow" : option == "lively" ? "Lively" : "Normal"; background: alpha(accent, robotPace == option ? 0.14 : 0); color: robotPace == option ? accent : muted; onTap: robotPace = option }
        }
      }
    }
  }
