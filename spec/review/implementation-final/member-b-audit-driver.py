from pathlib import Path
import zipfile,json,hashlib,struct,io,tarfile,tomllib,xml.etree.ElementTree as ET,re
base=Path(__file__).parent;folder=base/'member-b-archives';rows=[]
for p in folder.glob('*.zip'):
 with zipfile.ZipFile(p) as z:
  if 'driver-proof/execution.json' not in z.namelist():continue
  phases=json.loads(z.read('driver-proof/execution.json'));names={x['name']:x for x in phases};inventory=json.loads(z.read('driver-proof/packages.json'))
  for phase in phases:
   log=z.read('driver-proof/'+phase['log']);assert hashlib.sha256(log).hexdigest()==phase['sha256']
   assert phase['exitCode']==(1 if phase['expectedFailure'] else 0),(p,phase)
   if phase['expectedFailure']:
    text=log.decode(errors='replace');assert re.search(r'\berror IL2026\b',text) and re.search(r'\berror IL3050\b',text)
  binaries=[]
  for mode in ['generated','packed']:
   info=json.loads(z.read('driver-proof/'+mode+'-native.json'));binary=z.read('driver-proof/'+mode+'-native/DriverProof'+('.exe' if info['rid']=='win-x64' else ''));assert hashlib.sha256(binary).hexdigest()==info['sha256'];assert info['executed']
   if info['rid']=='win-x64':
    off=struct.unpack_from('<I',binary,0x3c)[0];assert binary[off:off+4]==b'PE\0\0' and struct.unpack_from('<H',binary,off+4)[0]==0x8664 and struct.unpack_from('<II',binary,off+24+112+14*8)==(0,0)
   else:
    assert binary[:5]==b'\x7fELF\x02' and struct.unpack_from('<H',binary,18)[0]==62
    assert not any(s in z.read('driver-proof/'+mode+'-native-dependencies.log').decode().lower() for s in ['coreclr','hostfxr','dotnet'])
   logs={}
   for phase in ['rust-default','rust-async','native-execute']:
    text=z.read('driver-proof/'+names[mode+'-'+phase]['log']).decode(errors='replace');assert '1424 independent cases' in text
    if phase=='native-execute':assert 'x 4 frontends' in text
    logs[phase]=[s for s in text.splitlines() if s.startswith('PASS')]
   binaries.append({'mode':mode,**info,'actualNativeFormatAndHashVerified':True,'conformance':logs})
  packages=[]
  for pkg in inventory['driverPackages']:
   raw=z.read('driver-proof/packages/'+pkg['file']);assert hashlib.sha256(raw).hexdigest()==pkg['sha256']
   if pkg['file'].endswith('.crate'):
    with tarfile.open(fileobj=io.BytesIO(raw)) as t:
     ns=t.getnames();manifest=tomllib.loads(t.extractfile(next(n for n in ns if n.endswith('/Cargo.toml'))).read().decode());deps=manifest['dependencies'];assert set(deps)=={'instrument-api','instrument-scpi'};assert all(not('path' in d or 'git' in d) for d in deps.values());assert not any('/validation/' in n or '/spec/' in n for n in ns)
     for n in ns:
      local=Path('/workspace/instrument-components-keysight')/Path(n).relative_to(Path(n).parts[0])
      if '/src/' in n or n.endswith('/generated/profiles.rs'):assert t.extractfile(n).read().replace(b'\r\n',b'\n')==local.read_bytes().replace(b'\r\n',b'\n'),n
   else:
    with zipfile.ZipFile(io.BytesIO(raw)) as n:
     ns=n.namelist();assert {Path(f).name for f in ns if f.endswith('.dll')}=={'InstrumentComponents.Drivers.Keysight.dll'};doc=ET.fromstring(n.read(next(f for f in ns if f.endswith('.nuspec'))));deps={d.attrib['id']:d.attrib['version'] for d in doc.findall('.//{*}dependency')};assert deps=={'InstrumentComponents.Scpi':'[0.2.0-alpha.1]'}
   packages.append({**pkg,'dependencies':deps,'members':ns})
  sdk=next(r for r in json.loads((base/'member-b-sdk-artifact-audit.json').read_text()) if r['platform']['rid']==binaries[0]['rid'] and r['platform']['framework']==binaries[0]['framework'])
  expected={x['name']:x['sha256'] for x in sdk['packages']}
  for name,h in inventory['sdkPackages'].items():
   if name in expected:assert h==expected[name],name
  rows.append({'artifactId':int(p.stem),'zipSha256':hashlib.sha256(p.read_bytes()).hexdigest(),'phaseCount':len(phases),'phaseHashesAndExitsVerified':True,'sdkPackageJoinVerified':True,'binaries':binaries,'packages':packages})
(base/'member-b-driver-artifact-audit.json').write_text(json.dumps(rows,indent=2)+'\n')
print('PASS own direct actual driver ZIP audits',[(r['artifactId'],r['phaseCount'],r['binaries'][0]['rid'],r['binaries'][0]['framework']) for r in rows])
