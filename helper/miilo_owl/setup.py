"""Interactive onboarding in the launcher's Linux terminal."""
from getpass import getpass
from .config import load, safe_url, save, validate
from .transport import request


def setup():
    config = load()
    print('Miilo · choose the systems your widgets observe')
    while True:
        for endpoint in config['endpoints']:
            print(f"  {endpoint['id']}: {len(endpoint['providers'])} selected providers")
        operation = input('[a] add/update endpoint, [r] remove, [q] finish: ').strip().lower()
        if operation == 'q':
            return
        name = input('Endpoint name (e.g. home, work): ').strip()
        if operation == 'r':
            config['endpoints'] = [e for e in config['endpoints'] if e['id'] != name]
            save(config)
            continue
        if operation != 'a':
            print('Choose a, r or q.')
            continue
        endpoint = {'id': name, 'url': safe_url(input('Miilo API URL: ').strip()),
                    'token': getpass('Bearer token (hidden; blank if not required): '), 'providers': []}
        raw = request(endpoint, '/v1/snapshot')
        providers = raw.get('providers', [])
        if raw.get('protocolVersion') != '0.1':
            raise ValueError('Unsupported Miilo protocol')
        if not providers:
            print('No providers registered. Add factories/providers in Miilo first, then retry.')
            continue
        for i, provider in enumerate(providers, 1):
            print(f"  {i}. {provider.get('name', provider['id'])}")
        choice = input('Observe which? Enter numbers separated by commas, or all: ').strip()
        if choice == 'all':
            endpoint['providers'] = [p['id'] for p in providers]
        else:
            positions = [int(x.strip()) for x in choice.split(',') if x.strip()]
            if not positions or any(i < 1 or i > len(providers) for i in positions):
                raise ValueError('Choose at least one valid provider')
            endpoint['providers'] = list(dict.fromkeys(providers[i-1]['id'] for i in positions))
        endpoint['allowActions'] = input('Allow bounded provider actions on this endpoint? [y/N]: ').strip().lower() == 'y'
        next_config = {'version': 1, 'endpoints': [e for e in config['endpoints'] if e['id'] != name]+[endpoint]}
        # Validate all names and limits before replacing the user's configuration.
        validate(next_config)
        save(next_config)
        config = next_config
        print('Saved. Add another endpoint or finish. New providers require explicit selection.')
