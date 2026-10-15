"""User-owned endpoint registration, shared by all launcher widgets."""
import json
import os
import re
import tempfile
import stat
from pathlib import Path
from urllib.parse import urlsplit


def directory():
    return Path(os.environ.get('MIILO_OWL_CONFIG_DIR', Path.home()/'.config/miilo-owl'))


def safe_url(value):
    p = urlsplit(value)
    if p.scheme not in ('http', 'https') or not p.hostname or p.username or p.password or p.query or p.fragment:
        raise ValueError('Use an http(s) endpoint without credentials, query or fragment')
    _ = p.port
    return value.rstrip('/')


def load():
    path = directory()/'config.json'
    if not path.exists():
        return {'version': 1, 'endpoints': []}
    if path.is_symlink() or not stat.S_ISREG(path.stat().st_mode) or path.stat().st_mode & 0o077:
        raise ValueError('Configuration must be a private regular file (chmod 600)')
    data = json.loads(path.read_text())
    validate(data)
    return data


def validate(data):
    if data.get('version') != 1 or not isinstance(data.get('endpoints'), list) or len(data['endpoints']) > 8:
        raise ValueError('Unsupported configuration; at most eight endpoints are supported')
    seen = set()
    for item in data['endpoints']:
        name = item.get('id', '')
        if not re.fullmatch(r'[a-z0-9][a-z0-9-]{0,39}', name) or name in seen:
            raise ValueError('Endpoint names must be unique lowercase words/dashes')
        seen.add(name)
        safe_url(item['url'])
        if not isinstance(item.get('providers'), list) or not all(isinstance(x, str) for x in item['providers']):
            raise ValueError('Choose explicit provider IDs for each endpoint')
        if not isinstance(item.get('token', ''), str) or any(c in item.get('token', '') for c in '\r\n'):
            raise ValueError('Invalid credential')

def private_write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    fd, tmp = tempfile.mkstemp(dir=path.parent, prefix='.miilo-')
    try:
        with os.fdopen(fd, 'w') as f:
            json.dump(data, f)
        os.replace(tmp, path)
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)


def save(data):
    validate(data)
    private_write(directory()/'config.json', data)
    invalidate()


def invalidate():
    (directory()/'snapshot-cache.json').unlink(missing_ok=True)
