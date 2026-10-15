  component TaskArtifacts() {
    let linked = artifacts.filter(a => contains(focused._artifactKeys ?? [], a._key))
    Text { text: "Files & changes"; style: small; color: muted }
    if len(linked) == 0 { Text { text: "No files or change sets supplied"; style: small; color: muted } }
    for a in linked.take(pageSize) key a._key {
      Column {
        gap: 4
        Text { text: a.name; style: small; maxLines: 4 }
        Text { text: a.kind == "change_set" ? "Change set" : a.kind; style: small; color: muted }
        if a._preview != "" {
          Text { text: a._preview; style: small; maxLines: 400 }
          if a._previewTruncated { Text { text: "Preview shortened"; style: small; color: muted } }
        } else if a.kind == "file" || a.kind == "change_set" {
          Text { text: "Diff not supplied"; style: small; color: muted }
        }
        if a._url != "" { Text { text: "Open artifact ↗"; style: small; color: accent; onTap: url(a._url) } }
      }
    }
    if len(linked) > pageSize { Button { text: "More files & changes"; onTap: pageSize = pageSize + 5 } }
  }
