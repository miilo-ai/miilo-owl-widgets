  // Presentation preferences only: never alter observations or action authority.
  state robotMotion = "full" persist
  state robotPace = "normal" persist
  state robotPersonality = "expressive" persist
  let robotSpeed = robotPace == "slow" ? 0.7 : robotPace == "lively" ? 1.3 : 1
  component RobotAppearance() {
    Text { text: "Robot personality"; style: small; color: muted }
    Flow {
      gap: 6
      for option in ["calm", "expressive"] {
        Button { text: option == "calm" ? "Calm" : "Expressive"; color: robotPersonality == option ? accent : muted; onTap: robotPersonality = option }
      }
    }
    Text { text: "Motion"; style: small; color: muted }
    Flow {
      gap: 6
      for option in ["full", "reduced", "off"] {
        Button { text: option == "full" ? "Full" : option == "reduced" ? "Reduced" : "Off"; color: robotMotion == option ? accent : muted; onTap: robotMotion = option }
      }
    }
    if robotMotion != "off" {
      Flow {
        gap: 6
        for option in ["slow", "normal", "lively"] {
          Button { text: option == "slow" ? "Slow" : option == "lively" ? "Lively" : "Normal"; color: robotPace == option ? accent : muted; onTap: robotPace = option }
        }
      }
    }
  }
