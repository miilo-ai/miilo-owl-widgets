  component ArtifactPreview(a) {
    let lines = split(a._preview ?? "", "\n")
    for line in lines.take(80) {
      let add = a.kind == "change_set" && startsWith(line, "+") && !startsWith(line, "+++ ")
      let remove = a.kind == "change_set" && startsWith(line, "-") && !startsWith(line, "--- ")
      let hunk = a.kind == "change_set" && (startsWith(line, "@@") || startsWith(line, "+++ ") || startsWith(line, "--- "))
      Text { text: line == "" ? " " : line; fill: width; font: mono; fontSize: 11; maxLines: 8; padding: 3; color: add ? mix(green, foreground, 0.3) : remove ? mix(red, foreground, 0.2) : hunk ? accent : foreground; background: add ? alpha(green, 0.08) : remove ? alpha(red, 0.08) : transparent }
    }
    if len(lines) > 80 || a._previewTruncated { Text { text: "Preview shortened"; fontSize: 10; color: muted } }
  }
  component TaskArtifacts() {
    let linked = artifacts.filter(a => contains(focused._artifactKeys ?? [], a._key))
    Spacer { height: 6 }
    Text { text: "Files & changes"; font: sans; fontSize: 12; color: accent }
    if len(linked) == 0 { Text { text: "No files or change sets supplied"; style: small; color: muted } }
    for a in linked.take(pageSize) key a._key {
      Column {
        fill: width
        gap: 5
        padding: 10
        border: 1
        borderColor: alpha(accent, 0.22)
        background: alpha(accent, 0.025)
        radius: 8
        Row {
          fill: width
          gap: 8
          Text { text: a.name; weight: 1; font: mono; fontSize: 12; color: foreground; maxLines: 4 }
          Text { text: a.kind == "change_set" ? "Diff" : a.kind; width: 44; fontSize: 10; align: end; color: muted; maxLines: 1 }
        }
        Spacer { height: 1; fill: width; background: alpha(accent, 0.13) }
        if (a._preview ?? "") != "" {
          ArtifactPreview { a: a }
        } else if a.kind == "file" || a.kind == "change_set" {
          Text { text: "Diff not supplied"; style: small; color: muted }
        }
        if a._url != "" { Text { text: "Open artifact ↗"; style: small; color: accent; onTap: url(a._url) } }
      }
    }
    if len(linked) > pageSize { Button { text: "More files & changes"; onTap: pageSize = pageSize + 5 } }
  }
