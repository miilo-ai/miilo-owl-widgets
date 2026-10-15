"""Boundary tests for configuration, isolation, and revision-bound operations."""
import copy
import io
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'helper'))
from miilo_owl import config, snapshot as observation, operations, setup


def fixture():
    return {'protocolVersion': '0.1', 'providers': [{'id': 'factory', 'name': 'Factory'}],
        'workItems': [{'id': 'task-1', 'providerId': 'factory', 'title': 'Review recovery',
            'kind': 'task', 'revision': 'r1', 'lifecycle': 'waiting', 'phase': 'reviewing',
            'attention': 'approval', 'actions': [{'id': 'approve', 'label': 'Approve',
                'available': True, 'risk': 'bounded', 'confirmation': 'explicit', 'inputSchema': None}]}],
        'actors': [], 'attention': [], 'attempts': [], 'evidence': []}


class HelperTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.env = patch.dict(os.environ, {'MIILO_OWL_CONFIG_DIR': self.tmp.name})
        self.env.start()
        self.endpoint = {'id': 'home', 'url': 'http://127.0.0.1:3700', 'token': 'private-token',
                         'providers': ['factory'], 'allowActions': True}
        config.save({'version': 1, 'endpoints': [self.endpoint]})

    def tearDown(self):
        self.env.stop()
        self.tmp.cleanup()

    def item(self):
        return observation.project(self.endpoint, fixture())['workItems'][0]

    def test_two_endpoints_identical_native_ids_stay_isolated(self):
        second = dict(self.endpoint, id='work')
        config.save({'version': 1, 'endpoints': [self.endpoint, second]})
        with patch.object(observation, 'request', return_value=fixture()):
            data = observation.snapshot()
        tasks = data['snapshot']['workItems']
        self.assertEqual(2, len(tasks))
        self.assertNotEqual(tasks[0]['_key'], tasks[1]['_key'])
        self.assertNotEqual(tasks[0]['providerId'], tasks[1]['providerId'])
        self.assertNotIn('private-token', json.dumps(data))

    def test_explicit_selection_excludes_unselected_and_new_providers(self):
        raw = fixture()
        raw['providers'].append({'id': 'new', 'name': 'New factory'})
        raw['workItems'].append(dict(raw['workItems'][0], providerId='new'))
        data = observation.project(self.endpoint, raw)
        self.assertEqual(1, len(data['providers']))
        self.assertEqual(1, len(data['workItems']))
        self.assertEqual([], observation.project(dict(self.endpoint, providers=[]), raw)['workItems'])

    def test_failed_endpoint_does_not_hide_healthy_endpoints(self):
        other = dict(self.endpoint, id='work')
        def read(endpoint, path):
            if endpoint['id'] == 'work':
                raise ValueError('down')
            return fixture()
        with patch.object(observation, 'request', side_effect=read):
            result = observation.collect({'endpoints': [self.endpoint, other]})
        self.assertEqual('partial', result['status'])
        self.assertEqual(1, len(result['snapshot']['workItems']))
        self.assertEqual('work', result['errors'][0]['id'])

    def test_duplicate_ids_rejected(self):
        raw = fixture()
        raw['workItems'] *= 2
        with self.assertRaises(ValueError):
            observation.project(self.endpoint, raw)

    def test_stale_revision_never_posts(self):
        item = self.item()
        updated = fixture()
        updated['workItems'][0]['revision'] = 'r2'
        with patch.object(observation, 'request', return_value=updated), patch.object(operations, 'request') as post:
            with self.assertRaisesRegex(ValueError, 'changed'):
                operations.action(item['_key'], item['actions'][0]['_key'], item['_revisionKey'])
            post.assert_not_called()

    def test_confirmation_payload_scoped_and_stable_across_retries(self):
        item = self.item()
        with patch.object(observation, 'request', return_value=fixture()), patch.object(operations, 'request', return_value={'status': 'accepted'}) as post:
            operations.action(item['_key'], item['actions'][0]['_key'], item['_revisionKey'])
            first = post.call_args.args[2]
            operations.action(item['_key'], item['actions'][0]['_key'], item['_revisionKey'])
            second = post.call_args.args[2]
        self.assertEqual('factory', first['providerId'])
        self.assertEqual({'kind': 'task', 'id': 'task-1'}, first['subject'])
        self.assertEqual('r1', first['expectedRevision'])
        self.assertEqual('manual', first['confirmation']['method'])
        self.assertEqual(first['idempotencyKey'], second['idempotencyKey'])

    def test_strong_auth_destructive_and_forms_not_supported(self):
        for replacement in [{'confirmation': 'strong'}, {'risk': 'destructive'},
                            {'inputSchema': {'required': ['reason']}}, {'available': False}, {'expiresAt': '2000-01-01T00:00:00Z'}]:
            raw = fixture()
            raw['workItems'][0]['actions'][0].update(replacement)
            action = observation.project(self.endpoint, raw)['workItems'][0]['actions'][0]
            self.assertFalse(action['_supported'])
        self.assertFalse(observation.project(dict(self.endpoint, allowActions=False), fixture())['workItems'][0]['actions'][0]['_supported'])

    def test_config_private_and_rejects_unsafe_urls(self):
        self.assertEqual(0, (Path(self.tmp.name)/'config.json').stat().st_mode & 0o077)
        for url in ['file:///etc/passwd', 'http://user:pass@localhost', 'https://host?token=secret']:
            with self.assertRaises(ValueError):
                config.safe_url(url)
        path = Path(self.tmp.name)/'config.json'
        path.chmod(0o644)
        with self.assertRaisesRegex(ValueError, 'private'):
            config.load()

    def test_native_links_and_opaque_targets_are_safe(self):
        raw = fixture()
        raw['workItems'][0]['nativeUrl'] = 'javascript:alert(1)'
        item = observation.project(self.endpoint, raw)['workItems'][0]
        self.assertEqual('', item['nativeUrl'])
        with self.assertRaises(ValueError):
            operations.target('$(touch /tmp/unwanted)', item['_revisionKey'])

    def test_shared_cache_reuses_fetch_and_config_change_invalidates(self):
        with patch.object(observation, 'request', return_value=fixture()) as read:
            observation.snapshot()
            observation.snapshot()
            self.assertEqual(1, read.call_count)
            config.save({'version': 1, 'endpoints': [dict(self.endpoint, providers=[])]})
            self.assertEqual([], observation.snapshot()['snapshot']['workItems'])
            self.assertEqual(2, read.call_count)

    def test_many_factories_and_scoped_graph_links(self):
        raw = fixture()
        raw.update(providers=[], workItems=[], actors=[], attempts=[], evidence=[], attention=[])
        for i in range(20):
            provider = f'factory-{i}'
            raw['providers'].append({'id': provider, 'name': provider})
            raw['actors'].append({'id': 'worker', 'providerId': provider})
            for j in range(50):
                task_id = f'task-{j}'
                raw['workItems'].append(dict(fixture()['workItems'][0], id=task_id, providerId=provider))
                raw['attempts'].append({'id': task_id, 'providerId': provider, 'taskId': task_id, 'actorIds': ['worker']})
                raw['evidence'].append({'id': task_id, 'providerId': provider, 'attemptId': task_id, 'summary': 'Passed'})
                raw['attention'].append({'id': task_id, 'providerId': provider, 'subject': {'kind': 'task', 'id': task_id}})
        selected = dict(self.endpoint, providers=[p['id'] for p in raw['providers']])
        result = observation.project(selected, raw)
        self.assertEqual(1000, len(result['workItems']))
        self.assertEqual(1000, len({w['_key'] for w in result['workItems']}))
        first, other = result['workItems'][0], result['workItems'][50]
        self.assertNotEqual(first['_actorKeys'], other['_actorKeys'])
        self.assertNotEqual(first['_evidenceKeys'], other['_evidenceKeys'])
        self.assertEqual(first['_key'], result['attention'][0]['_workKey'])
        raw['attention'][0]['subject']['kind'] = 'actor'
        self.assertEqual('', observation.project(selected, raw)['attention'][0]['_workKey'])

    def test_onboarding_adds_two_endpoints_and_can_remove_one(self):
        config.save({'version': 1, 'endpoints': []})
        answers = ['a', 'home', 'http://127.0.0.1:3700', 'all', 'n',
                   'a', 'office', 'http://127.0.0.1:3701', '1', 'y', 'q']
        with patch('builtins.input', side_effect=answers), patch.object(setup, 'getpass', return_value='secret'), patch.object(setup, 'request', return_value=fixture()), patch('sys.stdout', new=io.StringIO()):
            setup.setup()
        self.assertEqual(2, len(config.load()['endpoints']))
        with patch('builtins.input', side_effect=['r', 'home', 'q']), patch('sys.stdout', new=io.StringIO()):
            setup.setup()
        self.assertEqual(['office'], [e['id'] for e in config.load()['endpoints']])

if __name__ == '__main__':
    unittest.main()
