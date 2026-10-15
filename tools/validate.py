#!/usr/bin/env python3
"""Compile and evaluate OWL across sizes, themes, data and expanded selections."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
PROJECTOR = ROOT/'helper/target/debug/examples/project-fixture'


def run(args):
    return subprocess.check_output(args, text=True)


def nodes(tree):
    for node in tree:
        yield node
        yield from nodes(node.get('children', []))


def assert_row_widths(tree):
    # Native Text fills available width unless sized. An unsized trailing label
    # consumes a Row and leaves its weighted title at zero width in Compose.
    for row in nodes(tree):
        children = row.get('children', [])
        if row.get('kind') == 'Row' and any(c.get('props', {}).get('weight', 0) for c in children):
            for child in children:
                props = child.get('props', {})
                if child.get('kind') == 'Text' and not props.get('weight'):
                    assert props.get('width', 0) > 0, 'Trailing Row text needs an explicit width'


def fixtures(folder):
    raw = json.loads((ROOT/'tests/populated.json').read_text())['snapshot']
    raw['protocolVersion'] = '0.1'
    for item in raw['attention']:
        item['subject']['kind'] = 'task'
    raw['attempts'] = [{'id': 'attempt-1', 'providerId': 'factory', 'taskId': '0',
                        'actorIds': ['0'], 'lifecycle': 'active', 'phase': 'verifying'}]
    raw['evidence'] = [{'id': 'proof-1', 'providerId': 'factory', 'attemptId': 'attempt-1',
                        'outcome': 'passed', 'summary': 'No repeated events', 'artifactIds': ['file-1']}]
    raw['artifacts'] = [{'id': 'file-1', 'providerId': 'factory', 'attemptId': 'attempt-1',
        'kind': 'file', 'name': 'src/worker.rs', 'extensions': {'example': {'fileDiff': '@@ -1 +1 @@\n-old\n+new'}}},
        {'id': 'file-2', 'providerId': 'factory', 'attemptId': 'attempt-1', 'kind': 'file', 'name': 'README.md'}]
    for w in raw['workItems']:
        w.update(revision='r1', summary='Checks passed; awaiting review.', kind='task', actions=[{
            'id': 'approve', 'label': 'Approve', 'risk': 'privileged', 'confirmation': 'explicit',
            'inputSchema': {'type': 'object', 'additionalProperties': False}, 'available': True},
            {'id': 'reject', 'label': 'Reject', 'risk': 'bounded', 'confirmation': 'explicit', 'available': True},
            {'id': 'retry', 'label': 'Retry', 'risk': 'bounded', 'confirmation': 'explicit', 'available': True}])
    def add(raw, alias='home'):
        payload = {'endpoint': {'id': alias, 'url': 'http://127.0.0.1',
            'providers': ['factory', 'codex'], 'allowActions': True}, 'snapshot': raw}
        return json.loads(subprocess.check_output([str(PROJECTOR)], input=json.dumps(payload), text=True))
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
    subprocess.run(['cargo', 'build', '--locked', '--manifest-path', str(ROOT/'helper/Cargo.toml'),
                    '--example', 'project-fixture', '-j', '2'], check=True)
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
            for width, height in [(120, 64), (168, 112), (180, 180), (240, 180), (360, 180), (180, 300)]:
                for case, fixture in paths.items():
                    for file in files:
                        result = json.loads(run(['java', '-cp', tmp+os.pathsep+owl, 'InspectKt', str(file), str(fixture), str(width), str(height), theme, 'default']))
                        assert result['tile'] and result['screen']
                        assert_row_widths(result['tile'] + result['screen'])
                        texts = [n['props'].get('text', '') for n in nodes(result['tile']) if n.get('kind') in ('Text', 'Button')]
                        if file.name.startswith('70') and case == 'populated':
                            fixture_data = json.loads(fixture.read_text())
                            assert not any(w['title'] in texts for w in fixture_data['snapshot']['workItems'])
                            assert not any(n.get('props', {}).get('height') == 25 for n in nodes(result['tile']) if n.get('kind') == 'Canvas')
                        if file.name.startswith('70'):
                            for node in nodes(result['tile']):
                                props = node.get('props', {})
                                if node.get('kind') == 'Canvas':
                                    assert props.get('width', 0) <= width - 16
                                    assert props.get('height', 0) <= height - 16
                                if node.get('kind') == 'Text' and props.get('maxLines') == 2:
                                    assert props.get('height') in (24, 28)
                        if case == 'disconnected':
                            assert ('Tap to add a system' if file.name.startswith('70') else 'Tap to add systems') in texts
                            if file.name.startswith('70'):
                                assert any(n.get('kind') == 'Canvas' for n in nodes(result['tile']))
                        if case == 'empty':
                            assert any(t in texts for t in ['Nothing needs you', 'No work in flight'])
                        if os.getenv('PREVIEW_DIR') and case == 'populated' and ((file.name.startswith('70') and width == 180) or (not file.name.startswith('70') and width == 168)):
                            dest = Path(os.environ['PREVIEW_DIR']); dest.mkdir(parents=True, exist_ok=True)
                            result.update(width=width, height=height)
                            (dest/(file.stem+'-'+theme+'.json')).write_text(json.dumps(result))
                        total += 1
        for file in files:
            for scenario in ['detail', 'confirm', 'filter', 'actor', 'interaction', 'group', 'search', 'no-match', 'agent-tab', 'needs', 'left-hand', 'preview', 'colors', 'search-edit', 'search-done', 'search-clear']:
                result = json.loads(run(['java', '-cp', tmp+os.pathsep+owl, 'InspectKt', str(file), str(paths['multiple']), '180', '180', 'dark', scenario]))
                assert_row_widths(result['tile'] + result['screen'])
                texts = [n['props'].get('text', '') for n in nodes(result['screen']) if n.get('kind') in ('Text', 'Button')]
                if scenario == 'group' and file.name.startswith('70'):
                    assert not any(t.startswith('Agents ') for t in texts)
                if scenario == 'agent-tab' and file.name.startswith('70'):
                    assert 'All work' in texts
                    fixture_data = json.loads(paths['multiple'].read_text())
                    actor = fixture_data['snapshot']['actors'][0]
                    for work in fixture_data['snapshot']['workItems']:
                        if work['providerId'] == actor['providerId'] and actor['_key'] not in work.get('_actorKeys', []):
                            assert work['title'] not in texts
                if scenario == 'preview' and file.name.startswith('70'):
                    tile_texts = [n['props'].get('text', '') for n in nodes(result['tile']) if n.get('kind') == 'Text']
                    assert 'Preview sleep' in tile_texts
                    assert not any('Preview ' in t for t in texts)
                if scenario == 'search-edit':
                    assert any(n.get('kind') == 'Input' and n.get('props', {}).get('focus') for n in nodes(result['screen']))
                    assert '⚙' not in texts and '↻' not in texts
                    assert '✓' in texts
                if scenario in ['search-done', 'search-clear']:
                    assert not any(n.get('kind') == 'Input' for n in nodes(result['screen']))
                    assert '⚙' in texts and '↻' in texts
                if scenario == 'confirm':
                    assert 'Confirm' in texts and 'Cancel' in texts
                if scenario == 'detail':
                    assert any('No repeated events' in t for t in texts)
                    action_rows = [n for n in nodes(result['screen']) if n.get('kind') == 'Flow' and
                        {'Explain', 'Approve', 'Reject', 'Retry'} <= {c.get('props', {}).get('text') for c in n.get('children', [])}]
                    assert len(action_rows) == 1, 'Explain and actions must share one row'
                    assert 'src/worker.rs' in texts
                    assert any('@@ -1 +1 @@' in t for t in texts)
                    assert 'Diff not supplied' in texts
                    assert not any(n.get('kind') == 'Button' and n.get('props', {}).get('text') == 'Evidence' for n in nodes(result['screen']))
                    assert '←' in texts
                    assert not any(n.get('kind') == 'Input' for n in nodes(result['screen']))
                if scenario == 'no-match':
                    assert not any('No repeated events' in t for t in texts)
                if scenario == 'search' and file.name.startswith(('70', '72')):
                    fixture_data = json.loads(paths['multiple'].read_text())
                    assert fixture_data['snapshot']['workItems'][0]['title'] in texts
                if scenario == 'default':
                    assert not any('No repeated events' in t for t in texts)
                if scenario == 'filter':
                    # Setup buttons may name both systems; task descriptions must not leak office work.
                    assert not any(t.startswith('office /') and ' · ' in t for t in texts)
                total += 1
        print(f'PASS: {total} OWL tile/screen evaluations, light/dark themes, multi-system filters and confirmations')

if __name__ == '__main__':
    main()
