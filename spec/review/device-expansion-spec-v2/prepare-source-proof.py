"""Prepare an isolated source consumer; public package proof remains a separate gate."""
import argparse
import hashlib
import json
import shutil
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('repository', type=Path)
parser.add_argument('output', type=Path)
args = parser.parse_args()
repository, output = args.repository.resolve(), args.output.resolve()
core = Path('/workspace/instrument-components')
assert not output.exists(), 'Use a new empty proof directory'
assert output != repository and repository not in output.parents
ignore = shutil.ignore_patterns('.git', 'target', 'bin', 'obj', '__pycache__')
shutil.copytree(repository, output, ignore=ignore)
settings = json.loads((output / 'driver-package.json').read_text())
package = settings['rustPackage']
manifest = output / 'validation/rust/Cargo.toml'
text = manifest.read_text().replace(package + '="=0.2.0-alpha.1"', package + '={path="../.."}')
text += '\n[patch.crates-io]\n'
for name in ['instrument-api', 'instrument-runtime', 'instrument-scpi', 'instrument-testkit']:
    text += name + '={path=' + json.dumps(str(core / 'crates' / name)) + '}\n'
manifest.write_text(text)
for lock in output.rglob('Cargo.lock'):
    lock.unlink()

vendor_project = output / settings['dotnetProject']
vendor_project.write_text(vendor_project.read_text().replace(
    '<PackageReference Include="InstrumentComponents.Scpi" Version="[0.2.0-alpha.1]"/>',
    '<ProjectReference Include="' + str(core / 'dotnet/src/InstrumentComponents.Scpi/InstrumentComponents.Scpi.csproj') + '"/>'))
proof_project = output / 'validation/dotnet/DriverProof.csproj'
text = proof_project.read_text()
for name, path in {
    settings['nugetPackage']: vendor_project,
    'InstrumentComponents.Runtime': core / 'dotnet/src/InstrumentComponents.Runtime/InstrumentComponents.Runtime.csproj',
    'InstrumentComponents.TestKit': core / 'dotnet/src/InstrumentComponents.TestKit/InstrumentComponents.TestKit.csproj',
}.items():
    old = '<PackageReference Include="' + name + '" Version="[0.2.0-alpha.1]"/>'
    assert old in text, name
    text = text.replace(old, '<ProjectReference Include="' + str(path) + '"/>')
proof_project.write_text(text)
files = {}
for path in output.rglob('*'):
    if path.is_file():
        files[str(path.relative_to(output))] = hashlib.sha256(path.read_bytes()).hexdigest()
(output / 'source-proof-inputs.json').write_text(json.dumps({
    'mode': 'source integration only; not public package availability',
    'repository': str(repository), 'core': str(core), 'preparedFiles': files,
}, indent=2) + '\n')
print('Prepared isolated source consumer:', output)
