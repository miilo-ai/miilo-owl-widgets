#!/usr/bin/env python3
"""Compile and evaluate OWL across sizes, themes, data and expanded selections."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'helper'))
from miilo_owl.snapshot import project


def run(args):
    return subprocess.check_output(args, text=True)


def nodes(tree):
    for node in tree:
        yield node
        yield from nodes(node.get('children', []))


def fixtures(folder):
    raw = json.loads((ROOT/'tests/populated.json').read_text())['snapshot']
    raw['protocolVersion'] = '0.1'
    for item in raw['attention']:
        item['subject']['kind'] = 'task'
    raw['attempts'] = [{'id': 'attempt-1', 'providerId': 'factory', 'taskId': '0',
                        'actorIds': ['0'], 'lifecycle': 'active', 'phase': 'verifying'}]
    raw['evidence'] = [{'id': 'proof-1', 'providerId': 'factory', 'attemptId': 'attempt-1',
                        'outcome': 'passed', 'summary': 'No repeated events'}]
    for w in raw['workItems']:
        w.update(revision='r1', summary='Checks passed; awaiting review.', kind='task', actions=[{
            'id': 'approve', 'label': 'Approve', 'risk': 'bounded', 'confirmation': 'explicit',
            'available': True}])
    def add(raw, alias='home'):
        return project({'id': alias, 'providers': ['factory', 'codex'], 'allowActions': True}, raw)
    selected = add(raw)
    cases = {'populated': {'configured': True, 'snapshot': selected, 'errors': []},
             'disconnected': {'configured': False, 'snapshot': {k: [] for k in selected}, 'errors': []},
             'empty': {'configured': True, 'snapshot': {k: [] for k in selected}, 'errors': []}}
    other = add(raw, 'office')
    cases['multiple'] = {'configured': True, 'snapshot': {k: selected[k]+other[k] for k in selected}, 'errors': []}
    cases['partial'] = {**cases['populated'], 'errors': [{'id': 'office', 'message': 'Unavailable'}]}
    crowded = json.loads(json.dumps(raw))
    for name in ['workItems', 'attention']:
        crowded[name] = [dict(item, id=f'{i}-{item["id"]}') for i in range(12) for item in crowded[name]]
    for actor in crowded['actors']:
        actor['name'] = 'Long independent verification agent name'
    cases['crowded'] = {'configured': True, 'snapshot': add(crowded), 'errors': []}
    paths = {}
    for name, data in cases.items():
        path = Path(folder)/(name+'.json')
        path.write_text(json.dumps(data))
        paths[name] = path
    return paths


def main():
    owl = os.environ['OWL_CHECKER_CLASSPATH']
    compiler = os.environ['KOTLIN_COMPILER_CLASSPATH']
    files = sorted(ROOT.glob('*.owl'))
    print(run(['java', '-cp', owl, 'dev.omalauncher.widgets.cli.MainKt', 'check', *map(str, files)]), end='')
    with tempfile.TemporaryDirectory(prefix='miilo-widgets-') as tmp:
        run(['java', '-cp', compiler, 'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler',
             '-no-stdlib', '-no-reflect', '-classpath', owl, '-d', tmp, str(ROOT/'tools/Inspect.kt')])
        paths = fixtures(tmp)
        total = 0
        for theme in ['dark', 'light']:
            for width, height in [(168, 112), (180, 180), (240, 180)]:
                for case, fixture in paths.items():
                    for file in files:
                        result = json.loads(run(['java', '-cp', tmp+os.pathsep+owl, 'InspectKt', str(file), str(fixture), str(width), str(height), theme, 'default']))
                        assert result['tile'] and result['screen']
                        texts = [n['props'].get('text', '') for n in nodes(result['tile']) if n.get('kind') in ('Text', 'Button')]
                        if case == 'disconnected':
                            assert 'Tap to add systems' in texts
                        if case == 'empty':
                            assert any(t in texts for t in ['Nothing needs you', 'No work in flight'])
                        if os.getenv('PREVIEW_DIR') and case == 'populated' and ((file.name.startswith('70') and width == 180) or (not file.name.startswith('70') and width == 168)):
                            dest = Path(os.environ['PREVIEW_DIR']); dest.mkdir(parents=True, exist_ok=True)
                            result.update(width=width, height=height)
                            (dest/(file.stem+'-'+theme+'.json')).write_text(json.dumps(result))
                        total += 1
        for file in files:
            for scenario in ['detail', 'confirm', 'filter', 'actor', 'interaction']:
                result = json.loads(run(['java', '-cp', tmp+os.pathsep+owl, 'InspectKt', str(file), str(paths['multiple']), '180', '180', 'dark', scenario]))
                texts = [n['props'].get('text', '') for n in nodes(result['screen']) if n.get('kind') in ('Text', 'Button')]
                if scenario == 'confirm':
                    assert 'Confirm' in texts and 'Cancel' in texts
                if scenario == 'detail':
                    assert any('No repeated events' in t for t in texts)
                if scenario == 'filter':
                    # Setup buttons may name both systems; task descriptions must not leak office work.
                    assert not any(t.startswith('office /') and ' · ' in t for t in texts)
                total += 1
        print(f'PASS: {total} OWL tile/screen evaluations, light/dark themes, multi-system filters and confirmations')

if __name__ == '__main__':
    main()
