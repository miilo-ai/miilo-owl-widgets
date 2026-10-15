"""Exercise the compiled Rust executable over authenticated real localhost HTTP."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread
import unittest

BINARY = os.environ.get('MIILO_OWL_BINARY', str(Path(__file__).resolve().parents[1]/'helper/target/debug/miilo-owl'))


def fixture():
    return {'protocolVersion': '0.1', 'providers': [{'id': 'factory', 'name': 'Factory'}],
        'workItems': [{'id': 'task-1', 'providerId': 'factory', 'title': 'Review recovery',
            'kind': 'task', 'revision': 'r1', 'lifecycle': 'waiting', 'phase': 'reviewing',
            'attention': 'approval', 'actions': [{'id': 'approve', 'label': 'Approve',
                'available': True, 'risk': 'bounded', 'confirmation': 'explicit', 'inputSchema': None}]}],
        'actors': [], 'attention': [], 'attempts': [], 'evidence': []}


def key(*parts):
    return hashlib.sha256(json.dumps(parts, separators=(',', ':')).encode()).hexdigest()[:32]


class HttpJourney(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
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
                if cls.mode == 'redirect':
                    self.send_response(302)
                    self.send_header('Location', '/leaked-token')
                    self.end_headers()
                    return
                if cls.mode == 'large':
                    return self.send_json(200, {'excess': 'x'*2_000_000})
                if cls.mode == 'invalid':
                    self.send_response(200)
                    self.end_headers()
                    self.wfile.write(b'not-json private-token')
                    return
                return self.send_json(200, cls.current)

            def do_POST(self):
                if self.headers.get('Authorization') != 'Bearer private-token':
                    return self.send_json(403, {})
                body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                cls.payloads.append((self.path, body))
                if self.path.endswith('/v1/explanations'):
                    return self.send_json(200, {'revision': cls.explanation_revision or body['expectedRevision'],
                        'spokenSummary': 'Recovery passed. Review before promotion.', 'changes': [], 'evidence': []})
                if cls.conflict:
                    return self.send_json(409, {'detail': 'revision changed private-token'})
                return self.send_json(200, {'status': 'accepted', 'message': 'Queued for promotion'})
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
        self.env = {**os.environ, 'MIILO_OWL_CONFIG_DIR': self.tmp.name}
        cls = self.__class__
        cls.current, cls.conflict, cls.mode = fixture(), False, ''
        cls.payloads, cls.visits, cls.explanation_revision = [], [], None
        base = f'http://127.0.0.1:{self.server.server_port}'
        self.home = {'id': 'home', 'url': base+'/home', 'token': 'private-token',
                     'providers': ['factory'], 'allowActions': True}
        self.office = dict(self.home, id='office', url=base+'/office')
        self.save([self.home, self.office])

    def tearDown(self):
        self.tmp.cleanup()

    def save(self, endpoints):
        path = Path(self.tmp.name)/'config.json'
        path.write_text(json.dumps({'version': 1, 'endpoints': endpoints}))
        path.chmod(0o600)

    def run_cli(self, *args, ok=True):
        result = subprocess.run([BINARY, *args], env=self.env, capture_output=True, text=True, timeout=25)
        self.assertEqual(result.returncode, 0 if ok else 1, result.stderr+result.stdout)
        self.assertNotIn('private-token', result.stdout+result.stderr)
        return json.loads(result.stdout)

    def item(self):
        return self.run_cli('refresh')['snapshot']['workItems'][0]

    def test_observe_explain_confirm_receipt_and_remove(self):
        observed = self.run_cli('snapshot')
        first, second = observed['snapshot']['workItems']
        self.assertNotEqual(first['_key'], second['_key'])
        explained = self.run_cli('explain', second['_key'], second['_revisionKey'])
        self.assertIn('Recovery passed', explained['message'])
        self.assertTrue(self.payloads[-1][0].startswith('/office/'))
        result = self.run_cli('action', first['_key'], first['actions'][0]['_key'], first['_revisionKey'])
        self.assertEqual('accepted', result['status'])
        path, invocation = self.payloads[-1]
        self.assertEqual('/home/v1/actions', path)
        self.assertEqual('r1', invocation['expectedRevision'])
        self.assertEqual('manual', invocation['confirmation']['method'])
        self.save([self.home])
        self.assertEqual(1, len(self.run_cli('snapshot')['snapshot']['workItems']))
        self.run_cli('action', second['_key'], second['actions'][0]['_key'], second['_revisionKey'], ok=False)

    def test_changed_revision_and_explanation_rejected(self):
        item = self.item()
        self.__class__.explanation_revision = 'r2'
        self.run_cli('explain', item['_key'], item['_revisionKey'], ok=False)
        self.__class__.payloads = []
        self.__class__.current['workItems'][0]['revision'] = 'r2'
        self.run_cli('action', item['_key'], item['actions'][0]['_key'], item['_revisionKey'], ok=False)
        self.assertEqual([], self.payloads)

    def test_race_conflict_not_retried_and_invalidates_cache(self):
        item = self.item()
        self.__class__.conflict = True
        self.run_cli('action', item['_key'], item['actions'][0]['_key'], item['_revisionKey'], ok=False)
        self.assertEqual(1, len(self.payloads))
        self.assertFalse((Path(self.tmp.name)/'snapshot-cache.json').exists())

    def test_revoked_token_redirect_and_invalid_responses(self):
        self.save([dict(self.home, token='revoked')])
        self.assertEqual('partial', self.run_cli('refresh')['status'])
        self.save([self.home])
        for mode in ['redirect', 'large', 'invalid']:
            self.__class__.mode = mode
            self.assertEqual('partial', self.run_cli('refresh')['status'])
        self.assertFalse(any('leaked-token' in p for p in self.visits))

    def test_keys_match_previous_python_helper_including_unicode(self):
        for revision in ['r1', 'révision-😀', 'quote"\\\n', 'delete\x7f']:
            self.__class__.current['workItems'][0]['revision'] = revision
            item = self.item()
            self.assertEqual(key(revision), item['_revisionKey'])
            self.assertEqual(key('home', 'factory', 'workItems', 'task-1'), item['_key'])
            self.assertEqual(key('home', 'factory', 'task-1', 'approve'), item['actions'][0]['_key'])

    def test_concurrent_widget_processes_share_one_fetch(self):
        self.save([self.home])
        processes = [subprocess.Popen([BINARY, 'snapshot'], env=self.env, stdout=subprocess.PIPE, text=True) for _ in range(3)]
        for process in processes:
            self.assertEqual('ready', json.loads(process.communicate(timeout=20)[0])['status'])
            self.assertEqual(0, process.returncode)
        self.assertEqual(1, len(self.visits))


if __name__ == '__main__':
    unittest.main()
