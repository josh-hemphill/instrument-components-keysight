"""Compose independently extracted model oracles; never imports backend code."""
import argparse
import json
from pathlib import Path

root = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser()
parser.add_argument('--check', action='store_true')
args = parser.parse_args()
document = json.loads((root / 'cases.json').read_text())
models = ['hp3497a', 'hp8920a', 'hp8920b', 'hp8921a', '34401a', '33120a', '53132a', '34980a', '34937a', '34922a', '34950a', 'hp3585b']
existing = [c for c in document['cases'] if c['id'].split('|')[0] not in {'keysight.' + m for m in models}]
for model in models:
    shard = json.loads((root / 'spec/acceptance' / (('3585b' if model == 'hp3585b' else model) + '.catalog-cases.json')).read_text())
    existing.extend(shard['cases'])
assert len(existing) == len({c['id'] for c in existing})
document['cases'] = existing
if args.check:
    assert document == json.loads((root / 'cases.json').read_text()), 'Acceptance shard drift'
else:
    (root / 'cases.json').write_text(json.dumps(document, indent=2) + '\n')
print(f'PASS {len(existing)} independent acceptance cases')
