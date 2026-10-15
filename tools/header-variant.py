#!/usr/bin/env python3
"""Compatibility export: current search toolbar works on stock OWL6 launchers."""
import argparse
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
parser.add_argument('--preview-robots', action='store_true')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
source = (root/'70-miilo-fleet.owl').read_text()
if args.preview_robots:
    source = source.replace('state previewRobots = false', 'state previewRobots = true')
args.output.write_text(source)
