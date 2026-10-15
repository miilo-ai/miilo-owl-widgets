#!/usr/bin/env python3
"""OWL has no imports: assemble small shared fragments into installable widgets."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
for index, name, internal, description in [
    (70, 'feed', 'fleet', 'Animated agents and work across selected systems'),
    (71, 'attention', 'attention', 'Compact review queue with evidence and provider actions'),
    (72, 'board', 'board', 'Tasks moving through selected factories')]:
    header = f'''widget "{'miilo' if index == 70 else 'miilo '+internal}" {{
  meta {{ owl: 6; description: "{description}"; version: "1.3"; license: "MIT"; category: "dev" }}
  page: 1
  order: {index-63}
  size: 3x2
  minSize: 3x2
  padding: 8
'''
    parts = ['common', 'discovery']+(['robot', 'colors'] if index != 72 else [])+[name]
    fragments = [(ROOT/'widgets'/f'{p}.inc').read_text() for p in parts]
    fragments[0] = fragments[0].replace('  component WorkDetail()', (ROOT/'widgets/artifacts.inc').read_text()+'\n  component WorkDetail()')
    text = header+'\n'.join(fragments)+'}\n'
    filename = {70: '70-miilo-fleet.owl', 71: '71-miilo-attention.owl', 72: '72-miilo-board.owl'}[index]
    (ROOT/filename).write_text(text)
