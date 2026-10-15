#!/usr/bin/env python3
"""Generate an OWL8 widget for the optional patched Omalauncher header API."""
import argparse
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
parser.add_argument('--preview-robots', action='store_true')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
source = (root/'70-miilo-fleet.owl').read_text().replace('owl: 6;', 'owl: 8;')
if args.preview_robots:
    source = source.replace('state previewRobots = false', 'state previewRobots = true')
start = source.index('    Row {\n      fill: width\n      gap: 4\n      align: center\n      if !leftHanded')
end = source.index('    if !configured {', start)
controls = source[start:end].replace('      fill: width\n', '')
for condition in ['!leftHanded', 'leftHanded']:
    controls = controls.replace(f'      if {condition} {{ Spacer {{ weight: 1 }} }}\n', '')
source = source[:start]+source[end:]
source = source.replace('    title: "miilo"\n', '    title: "miilo"\n    header {\n'+controls+'    }\n')
args.output.write_text(source)
