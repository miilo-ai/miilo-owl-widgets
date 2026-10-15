"""Fresh reads and revision-bound operations; OWL passes opaque hex keys only."""
import re
from datetime import datetime, timezone
from .config import invalidate, load
from .snapshot import key, collect
from .transport import request


def target(item_key, revision):
    if not all(re.fullmatch(r'[0-9a-f]{32}', x or '') for x in (item_key, revision)):
        raise ValueError('Invalid target')
    config = load()
    data = collect(config)
    item = next((w for w in data['snapshot']['workItems'] if w['_key'] == item_key), None)
    if item is None:
        raise ValueError('Task unavailable; refresh before acting')
    if item['_revisionKey'] != revision:
        raise ValueError('Task changed; review the latest revision')
    endpoint = next(e for e in config['endpoints'] if e['id'] == item['_source'])
    return endpoint, item


def explain(item_key, revision):
    endpoint, item = target(item_key, revision)
    bundle = request(endpoint, '/v1/explanations', {
        'providerId': item['_provider'], 'workItemId': item['id'],
        'expectedRevision': item['revision'], 'question': None,
        'preferences': {'detail': 'concise', 'lead': 'cause',
                        'includeTechnicalDetails': False, 'speakSensitiveValues': False}})
    if bundle.get('revision') != item['revision']:
        raise ValueError('Explanation revision changed; refresh')
    return {'status': 'completed', 'message': bundle.get('spokenSummary') or bundle.get('cause') or 'No explanation provided',
            'changes': bundle.get('changes', []), 'evidence': bundle.get('evidence', [])}


def action(item_key, action_key, revision):
    endpoint, item = target(item_key, revision)
    candidate = next((a for a in item.get('actions', []) if a['_key'] == action_key), None)
    if not candidate or not candidate['_supported']:
        raise ValueError('Action unavailable here; use the provider interface')
    stable = key(endpoint['id'], item['_provider'], item['id'], candidate['id'], item['revision'])
    payload = {'protocolVersion': '0.1', 'requestId': stable, 'providerId': item['_provider'],
        'actionId': candidate['id'], 'subject': {'kind': item.get('kind', 'task'), 'id': item['id']},
        'expectedRevision': item['revision'], 'confirmation': {
            'method': 'manual', 'confirmedAt': datetime.now(timezone.utc).isoformat()},
        'input': {}, 'idempotencyKey': stable}
    # Do not automatically retry a mutation or turn an accepted receipt into completion.
    try:
        receipt = request(endpoint, '/v1/actions', payload)
    finally:
        invalidate()
    status = receipt.get('status')
    if status not in ('accepted', 'completed', 'rejected', 'failed', 'conflict'):
        raise ValueError('Invalid action receipt; inspect provider before retrying')
    return {'status': status, 'message': receipt.get('message') or f'Provider: {status}'}
