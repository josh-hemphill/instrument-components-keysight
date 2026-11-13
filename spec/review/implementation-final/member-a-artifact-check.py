from pathlib import Path
import json,hashlib,zipfile,io,tarfile,struct,re,tomllib,xml.etree.ElementTree as ET
root=Path('/workspace/scratch/instrument-components-hp');r6=root/'council/implementation-r6';sha=lambda b:hashlib.sha256(b).hexdigest()
manifest=r6/'final-evidence-manifest.json';m=json.loads(manifest.read_text());assert sha(manifest.read_bytes())=='4ff9bffc3d4b198ac2924a471628163f5d74401ff7489e01d0cdde88952d9c25'
sm=Path(m['sourceManifest']['path']);assert sha(sm.read_bytes())==m['sourceManifest']['sha256'];s=json.loads(sm.read_text());count=0
for repo,record in s['repositories'].items():
 for name,h in record['files'].items():assert sha((Path('/workspace')/repo/name).read_bytes())==h;count+=1
for row in m['evidenceFiles']+m['mainCurrentChecks']:assert sha(Path(row['path']).read_bytes())==row['sha256']
a=json.loads((root/'evidence/hp-3d384a4/remote-audit.json').read_text());assert a['sourceRun']==43322642590 and a['sourceRevision']==m['sourceRevisions']['instrument-components-keysight'];assert len(a['jobs'])==9 and all(j['conclusion']=='success' for j in a['jobs']);out=[];sdkhashes={};driverjoins={}
for ar in a['artifacts']:
 art=ar['artifact'];p=r6/'member-b-archives'/f"{art['id']}.zip";raw=p.read_bytes();assert sha(raw)==ar['zipSha256']==art['digest'][7:]
 with zipfile.ZipFile(io.BytesIO(raw)) as z:
  names=z.namelist()
  def read(suf):
   matches=[n for n in names if n.endswith(suf)];assert len(matches)==1,(suf,matches);return z.read(matches[0])
  driver=art['name'].startswith('driver-proof-');prefix='driver-proof/' if driver else '';records=json.loads(read(prefix+('execution.json' if driver else 'validation.json')));assert len(records)==ar['phaseCount'];byname={p['name']:p for p in records};assert len(byname)==len(records)
  for row in records:
   log=read(prefix+row['log'].replace('\\','/').split('/')[-1]);assert sha(log)==row['sha256'];assert (row['exitCode']!=0)==row['expectedFailure']
   if row['expectedFailure']:assert all(re.search(r'\berror '+d+r'\b',log.decode(errors='replace')) for d in ['IL2026','IL3050'])
   elif 'native-publish' in row['name']:assert not re.search(r'\bwarning IL\d+',log.decode(errors='replace'),re.I)
   if '-metadata' in row['name']:
    for pack in json.loads(log)['packages']:
     if pack['name'] in ['instrument-api','instrument-runtime','instrument-scpi','instrument-testkit'] or (driver and row['name'].startswith('packed') and pack['name']=='instrument-driver-keysight'):assert pack['source'].startswith('registry+')
  for b in ar['binaries']:
   name=prefix+b['mode']+'-native/DriverProof' if driver else 'native/AotConsumer';name+='.exe' if b['rid']=='win-x64' else '';binary=read(name);assert sha(binary)==b['sha256'] and len(binary)==b['bytes']
   if b['rid']=='win-x64':
    off=struct.unpack_from('<I',binary,0x3c)[0];assert binary[off:off+4]==b'PE\0\0' and struct.unpack_from('<H',binary,off+4)[0]==0x8664 and struct.unpack_from('<II',binary,off+24+112+14*8)==(0,0)
   else:
    assert binary[:5]==b'\x7fELF\x02' and struct.unpack_from('<H',binary,18)[0]==62;deps=read(prefix+byname[b['mode']+'-native-dependencies']['log']) if driver else read('native-dependencies.log');assert not any(x in deps.lower() for x in [b'coreclr',b'hostfxr',b'dotnet'])
   if driver:
    for suffix in ['rust-default','rust-async','native-execute']:
     log=read(prefix+byname[b['mode']+'-'+suffix]['log']).decode(errors='replace');assert '1424 independent cases' in log
     if suffix in ['rust-async','native-execute']:assert 'all six HP models' in log and 'pending-write' in log
   else:assert 'six operations, sync/async request and unit contracts' in read('native-aot-behavior.log').decode()
  inventory=json.loads(read(prefix+'packages.json'));rows=inventory['driverPackages'] if driver else inventory;hashes={}
  for row in rows:
   name=row['file'] if driver else row['archive'].replace('\\','/').split('/')[-1];body=read(prefix+'packages/'+name) if driver else read(name);assert sha(body)==row['sha256'];hashes[row['package']]=sha(body)
   if name.endswith('.crate'):
    with tarfile.open(fileobj=io.BytesIO(body),mode='r:gz') as tar:
     ns=tar.getnames();cargo=tomllib.loads(tar.extractfile(next(n for n in ns if n.endswith('/Cargo.toml'))).read().decode());assert all(not ('path' in d or 'git' in d) for d in cargo.get('dependencies',{}).values())
     if driver:
      assert set(cargo['dependencies'])=={'instrument-api','instrument-scpi'} and not any('/validation/' in n for n in ns)
      for file in ['hp5334.rs','hp3497.rs','hp_radio.rs','message_backend.rs','lib.rs']:
       data=tar.extractfile(next(n for n in ns if n.endswith('/src/'+file))).read();assert data.replace(b'\r\n',b'\n')==(Path('/workspace/instrument-components-keysight/src')/file).read_bytes()
     elif row['package']!='instrument-synthetic-driver':
      for f in tar.getmembers():
       if f.isfile() and f.name.endswith('.rs'):assert not any(q in tar.extractfile(f).read().lower() for q in [b'hp5334',b'hp3497',b'hp8920',b'hp8921'])
   elif name.endswith('.nupkg'):
    with zipfile.ZipFile(io.BytesIO(body)) as pkg:
     assert {Path(n).name for n in pkg.namelist() if n.endswith('.dll')}=={row['package']+'.dll'}
     if driver:
      doc=ET.fromstring(pkg.read(next(n for n in pkg.namelist() if n.endswith('.nuspec'))));assert {d.attrib['id']:d.attrib['version'] for d in doc.findall('.//{*}dependency')}=={'InstrumentComponents.Scpi':'[0.2.0-alpha.1]'}
  if driver:driverjoins[art['name']]=inventory['sdkPackages']
  else:sdkhashes[art['name']]=hashes
  out.append({'artifact':art['name'],'id':art['id'],'zipSha256':sha(raw),'phaseCount':len(records),'nativeBinariesVerified':len(ar['binaries']),'packagesHashed':len(rows),'caseLogsVerified':driver})
core=['instrument-api','instrument-runtime','instrument-scpi','instrument-testkit','instrument-components','InstrumentComponents','InstrumentComponents.Abstractions','InstrumentComponents.Runtime','InstrumentComponents.Scpi','InstrumentComponents.TestKit'];joins=0
for driver,d in driverjoins.items():
 sd=sdkhashes[driver.replace('driver-proof-','driver-sdk-')]
 for name in core:assert d[name]==sd[name];joins+=1
result={'reviewer':'A','sourcesHashed':count,'evidenceManifestSha256':sha(manifest.read_bytes()),'artifacts':out,'totalPhases':sum(x['phaseCount'] for x in out),'totalNativeBinaries':sum(x['nativeBinariesVerified'] for x in out),'sdkCoreChecksumJoins':joins,'windowsArchiveSourceComparison':'CRLF normalized to frozen LF source; contents otherwise identical','allChecksPassed':True}
(r6/'member-a-final-artifact-audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
