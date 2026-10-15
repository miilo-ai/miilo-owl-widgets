"""Aggregate selected providers without confusing IDs across private clouds."""
import copy
import hashlib
import fcntl
import os
from datetime import datetime, timezone
import json
import time
from concurrent.futures import ThreadPoolExecutor
from urllib.parse import urlsplit
from .config import directory, load, private_write
from .transport import request
from .graph import link

COLLECTIONS = ('providers', 'actors', 'workItems', 'attention', 'attempts', 'evidence')


def key(*parts):
    return hashlib.sha256(json.dumps(parts, separators=(',', ':')).encode()).hexdigest()[:32]


def web_url(value):
    if not isinstance(value, str):
        return ''
    try:
        p = urlsplit(value or '')
        return value if p.scheme in ('http', 'https') and p.hostname and not p.username and not p.password else ''
    except (ValueError, TypeError):
        return ''


def project(endpoint, snapshot):
    if snapshot.get('protocolVersion') != '0.1':
        raise ValueError('Unsupported Miilo snapshot protocol')
    for name in COLLECTIONS:
        collection = snapshot.get(name, [])
        if not isinstance(collection, list) or any(not isinstance(item, dict) or not isinstance(item.get('id'), str) or not item['id'] for item in collection):
            raise ValueError('Invalid snapshot records')
    chosen = set(endpoint['providers'])
    available = {p['id'] for p in snapshot.get('providers', [])}
    chosen &= available
    result = {name: [] for name in COLLECTIONS}
    for name in COLLECTIONS:
        seen = set()
        for original in snapshot.get(name, []):
            provider = original['id'] if name == 'providers' else original.get('providerId')
            if provider not in chosen:
                continue
            identity = (provider, original['id'])
            if identity in seen:
                raise ValueError('Duplicate identity in provider snapshot')
            seen.add(identity)
            item = copy.deepcopy(original)
            item['_source'] = endpoint['id']
            item['_provider'] = provider
            item['_key'] = key(endpoint['id'], provider, name, item['id'])
            item['_revisionKey'] = key(item.get('revision', ''))
            item['nativeUrl'] = web_url(item.get('nativeUrl'))
            if name == 'providers':
                item['id'] = key(endpoint['id'], provider)
                item['name'] = f"{endpoint['id']} / {item.get('name', provider)}"
            else:
                item['providerId'] = key(endpoint['id'], provider)
            actions = item.get('actions') or []
            if not isinstance(actions, list) or any(not isinstance(a, dict) or not isinstance(a.get('id'), str) for a in actions):
                raise ValueError('Invalid advertised actions')
            if len({a['id'] for a in actions}) != len(actions):
                raise ValueError('Duplicate action identity')
            item['actions'] = actions
            for action in actions:
                action['_key'] = key(endpoint['id'], provider, item['id'], action['id'])
                action['nativeUrl'] = web_url(action.get('nativeUrl'))
                schema = action.get('inputSchema') or {}
                expiry = action.get('expiresAt')
                try:
                    expired = expiry is not None and datetime.fromisoformat(expiry.replace('Z', '+00:00')) <= datetime.now(timezone.utc)
                except (ValueError, TypeError, AttributeError):
                    expired = True
                action['_supported'] = (endpoint.get('allowActions') is True and action.get('available') is True
                    and action.get('risk') in ('read', 'bounded')
                    and action.get('confirmation') in ('none', 'explicit')
                    and not schema and not expired)  # Nontrivial forms stay in the provider UI.
            result[name].append(item)
    return link(result)


def collect(config):
    endpoints = config['endpoints']
    def one(endpoint):
        try:
            raw = request(endpoint, '/v1/snapshot')
            return project(endpoint, raw), None
        except (ValueError, KeyError, TypeError):
            return None, {'id': endpoint['id'], 'message': 'Unavailable or incompatible; check access'}
    result = {name: [] for name in COLLECTIONS}
    errors = []
    if endpoints:
        with ThreadPoolExecutor(max_workers=min(len(endpoints), 4)) as pool:
            for data, error in pool.map(one, endpoints):
                if error:
                    errors.append(error)
                else:
                    for name in COLLECTIONS:
                        result[name].extend(data[name])
    return {'configured': bool(endpoints), 'snapshot': result, 'errors': errors,
            'observedAt': time.time(), 'status': 'partial' if errors else 'ready'}


def _snapshot(force=False):
    config = load()
    fingerprint = key(config)
    path = directory()/'snapshot-cache.json'
    if not force and path.exists() and not path.is_symlink():
        try:
            cached = json.loads(path.read_text())
            age = time.time()-cached['data']['observedAt']
            if cached['fingerprint'] == fingerprint and 0 <= age < 8:
                return cached['data']
        except (ValueError, KeyError, TypeError):
            pass
    data = collect(config)
    private_write(path, {'fingerprint': fingerprint, 'data': data})
    return data


def snapshot(force=False):
    folder = directory()
    folder.mkdir(parents=True, exist_ok=True, mode=0o700)
    path = folder/'snapshot.lock'
    if path.is_symlink():
        raise ValueError('Invalid cache lock')
    fd = os.open(path, os.O_CREAT | os.O_RDWR, 0o600)
    with os.fdopen(fd, 'w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        return _snapshot(force)
