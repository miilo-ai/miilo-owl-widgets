"""Authenticated localhost journeys through real HTTP (no external services)."""
import copy
import json
import os
from pathlib import Path
import sys
import tempfile
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'helper'))
from miilo_owl import config, snapshot, operations, transport
from test_helper import fixture


class HttpJourney(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.payloads = []
        cls.visits = []
        cls.current = fixture()
        cls.conflict = False
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, *args):
                pass
            def send_json(self, status, data):
                self.send_response(status)
                self.send_header('Content-Type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps(data).encode())
            def do_GET(self):
                cls.visits.append(self.path)
                if self.headers.get('Authorization') != 'Bearer private-token':
                    return self.send_json(401, {'detail': 'secret should never be echoed'})
                if self.path.endswith('/redirect'):
                    self.send_response(302)
                    self.send_header('Location', '/leaked-token')
                    self.end_headers()
                    return
                if self.path.endswith('/v1/snapshot'):
                    return self.send_json(200, cls.current)
                self.send_json(404, {})
            def do_POST(self):
                if self.headers.get('Authorization') != 'Bearer private-token':
                    return self.send_json(403, {})
                body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                cls.payloads.append((self.path, body))
                if self.path.endswith('/v1/explanations'):
                    return self.send_json(200, {'revision': body['expectedRevision'],
                        'spokenSummary': 'Recovery passed. Review before promotion.', 'changes': [], 'evidence': []})
                if self.path.endswith('/v1/actions'):
                    if cls.conflict:
                        return self.send_json(409, {'detail': 'revision changed'})
                    return self.send_json(200, {'status': 'accepted', 'message': 'Queued for promotion'})
                self.send_json(404, {})
        cls.server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        cls.thread = Thread(target=cls.server.serve_forever, daemon=True)
        cls.thread.start()

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.server.server_close()
        cls.thread.join()

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.env = patch.dict(os.environ, {'MIILO_OWL_CONFIG_DIR': self.tmp.name})
        self.env.start()
        self.__class__.current = fixture()
        self.__class__.conflict = False
        self.__class__.payloads = []
        base = f'http://127.0.0.1:{self.server.server_port}'
        self.home = {'id': 'home', 'url': base+'/home', 'token': 'private-token',
                     'providers': ['factory'], 'allowActions': True}
        self.office = dict(self.home, id='office', url=base+'/office')
        config.save({'version': 1, 'endpoints': [self.home, self.office]})

    def tearDown(self):
        self.env.stop()
        self.tmp.cleanup()

    def test_register_observe_explain_confirm_receipt_and_remove(self):
        observed = snapshot.snapshot(force=True)
        self.assertEqual(2, len(observed['snapshot']['providers']))
        first, second = observed['snapshot']['workItems']
        explained = operations.explain(second['_key'], second['_revisionKey'])
        self.assertIn('Recovery passed', explained['message'])
        self.assertTrue(self.payloads[-1][0].startswith('/office/'))
        result = operations.action(first['_key'], first['actions'][0]['_key'], first['_revisionKey'])
        self.assertEqual('accepted', result['status'])
        path, invocation = self.payloads[-1]
        self.assertEqual('/home/v1/actions', path)
        self.assertEqual('r1', invocation['expectedRevision'])
        self.assertEqual('manual', invocation['confirmation']['method'])
        config.save({'version': 1, 'endpoints': [self.home]})
        self.assertEqual(1, len(snapshot.snapshot()['snapshot']['workItems']))
        with self.assertRaises(ValueError):
            operations.action(second['_key'], second['actions'][0]['_key'], second['_revisionKey'])

    def test_changed_revision_blocks_action_before_post(self):
        item = snapshot.snapshot()['snapshot']['workItems'][0]
        self.__class__.current['workItems'][0]['revision'] = 'r2'
        with self.assertRaisesRegex(ValueError, 'changed'):
            operations.action(item['_key'], item['actions'][0]['_key'], item['_revisionKey'])
        self.assertEqual([], self.payloads)

    def test_race_conflict_is_returned_without_automatic_mutation_retry(self):
        item = snapshot.snapshot()['snapshot']['workItems'][0]
        self.__class__.conflict = True
        with self.assertRaisesRegex(ValueError, '409'):
            operations.action(item['_key'], item['actions'][0]['_key'], item['_revisionKey'])
        self.assertEqual(1, len(self.payloads))

    def test_revoked_token_and_redirect_do_not_leak_credentials(self):
        with self.assertRaises(ValueError) as error:
            transport.request(dict(self.home, token='revoked'), '/v1/snapshot')
        self.assertNotIn('secret', str(error.exception))
        self.__class__.visits = []
        with self.assertRaisesRegex(ValueError, '302'):
            transport.request(self.home, '/redirect')
        self.assertFalse(any('leaked-token' in path for path in self.visits))

if __name__ == '__main__':
    unittest.main()
