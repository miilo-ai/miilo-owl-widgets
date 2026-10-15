"""Bounded authenticated HTTP; errors never echo endpoint URLs or credentials."""
import json
from urllib.error import HTTPError, URLError
from urllib.request import Request, build_opener, HTTPRedirectHandler
from .config import safe_url


class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None  # Never forward a bearer token to a redirected host.


def request(endpoint, path, body=None):
    headers = {'Accept': 'application/json'}
    if endpoint.get('token'):
        headers['Authorization'] = 'Bearer '+endpoint['token']
    payload = None if body is None else json.dumps(body).encode()
    if payload is not None:
        headers['Content-Type'] = 'application/json'
    req = Request(safe_url(endpoint['url'])+path, data=payload, headers=headers)
    try:
        with build_opener(NoRedirect).open(req, timeout=8) as response:
            raw = response.read(2_000_001)
        if len(raw) > 2_000_000:
            raise ValueError('Provider response exceeds the widget limit')
        data = json.loads(raw)
        if not isinstance(data, dict):
            raise ValueError('Expected a JSON object')
        return data
    except HTTPError as exc:
        code = exc.code
        exc.close()
        raise ValueError(f'HTTP {code}; refresh or check access') from None
    except (URLError, TimeoutError, OSError):
        raise ValueError('Endpoint unavailable; check network access') from None
    except json.JSONDecodeError:
        raise ValueError('Endpoint returned invalid JSON') from None
