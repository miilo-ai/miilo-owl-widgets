#!/usr/bin/env python3
"""Render evaluated OWL trees as a layout reference, never a native screenshot."""
import argparse
import json
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument('trees', type=Path)
p.add_argument('output', type=Path)
args = p.parse_args()
items = []
for theme in ['dark', 'light']:
    for name in ['70-miilo-fleet', '71-miilo-attention', '72-miilo-board']:
        data = json.loads((args.trees/(name+'-'+theme+'.json')).read_text())
        items.append({'name': name, 'theme': theme, 'tile': data['tile'], 'screen': data['screen'], 'width': data['width'], 'height': data['height']})
html = (Path(__file__).parent/'reference.html').read_text()
args.output.write_text(html.replace('/*DATA*/', json.dumps(items)))
