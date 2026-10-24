  component WidgetAbout() {
    Column {
      fill: width; gap: 10
      Text { text: "Widget · ${widgetVersion}"; fontSize: 11; color: muted; maxLines: 1 }
      Text { text: "Personal by default. No Miilo account is required to observe your selected sessions and connected systems."; fontSize: 12; maxLines: 4 }
      Text { text: "You choose what to connect and import. Observation uses your configured endpoints. Preferences and connection settings stay in your launcher’s Linux environment; credentials stay outside widget source."; fontSize: 12; maxLines: 6 }
      Text { text: "Model explanations are optional. Selected work context goes to the endpoint you configure and consent to, whose privacy policy applies. Voice is not available in this launcher."; fontSize: 12; maxLines: 5 }
      Text { text: "Observation does not grant permission to act on your systems. Removing the widget preserves connections and observation data. Revoke remote access at its source when a device is no longer trusted."; fontSize: 12; maxLines: 6 }
      Text { text: "Help fund Miilo’s development."; fontSize: 12; color: muted; maxLines: 2 }
      Row {
        fill: width; gap: 8
        if !leftHanded { Spacer { weight: 1 } }
        Button { height: 28; background: alpha(accent, 0.12); color: accent; text: "miilo.tech ↗"; onTap: url("https://miilo.tech/") }
        Button { height: 28; background: alpha(accent, 0.12); color: accent; text: "Sponsor Miilo ↗"; onTap: url("https://github.com/sponsors/miilo-ai") }
        if leftHanded { Spacer { weight: 1 } }
      }
    }
  }
