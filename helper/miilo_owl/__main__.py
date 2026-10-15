import argparse
import json
from .config import invalidate
from .operations import action, explain
from .setup import setup
from .snapshot import snapshot


def main():
    parser = argparse.ArgumentParser(prog='miilo-owl')
    sub = parser.add_subparsers(dest='command', required=True)
    for name in ('setup', 'snapshot', 'refresh'):
        sub.add_parser(name)
    for name in ('explain', 'action'):
        p = sub.add_parser(name)
        p.add_argument('item')
        if name == 'action':
            p.add_argument('action')
        p.add_argument('revision')
    args = parser.parse_args()
    try:
        if args.command == 'setup':
            setup()
            return
        if args.command == 'refresh':
            invalidate()
            result = snapshot(force=True)
        elif args.command == 'snapshot':
            result = snapshot()
        elif args.command == 'explain':
            result = explain(args.item, args.revision)
        else:
            result = action(args.item, args.action, args.revision)
    except (ValueError, KeyError, TypeError, OSError):
        # Never include a raw transport error, credential, URL or remote payload.
        result = {'status': 'failed', 'message': 'Request failed. Refresh and check setup/access.'}
    print(json.dumps(result))

if __name__ == '__main__':
    main()
