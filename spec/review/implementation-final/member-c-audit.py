import json,pathlib,hashlib,zipfile,struct,tarfile,io,tomllib,xml.etree.ElementTree as ET
base=pathlib.Path(__file__).parent
remote=json.load(open('/workspace/scratch/instrument-components-hp/evidence/hp-3d384a4/remote-audit.json'))
sha=lambda b:hashlib.sha256(b).hexdigest()
reports=[];sdkrows={};drivers=[];phasecount=0
for item in remote['artifacts']:
 a=item['artifact'];raw=(base/f"member-c-{a['id']}.zip").read_bytes();assert a['digest']=='sha256:'+sha(raw)
 z=zipfile.ZipFile(io.BytesIO(raw));names=z.namelist();driver=a['name'].startswith('driver-proof');prefix='driver-proof/' if driver else ''
 def read(name):
  matches=[n for n in names if n==name or n.endswith('/'+name)];assert len(matches)==1,(name,matches)
  return z.read(matches[0])
 phases=json.loads(read(prefix+'execution.json' if driver else 'validation.json'));phasecount+=len(phases);byname={p['name']:p for p in phases}
 for p in phases:
  fname=p['log'].replace('\\','/').split('/')[-1];log=read(prefix+fname);assert sha(log)==p['sha256'];assert p['exitCode']==(1 if p['expectedFailure'] else 0)
 binaries=[];execution=[]
 for mode in (['generated','packed'] if driver else ['sdk']):
  info=json.loads(read(prefix+mode+'-native.json' if driver else 'platform.json'));win=info['rid']=='win-x64'
  binary=read(prefix+mode+'-native/DriverProof'+('.exe' if win else '') if driver else 'native/AotConsumer'+('.exe' if win else ''))
  assert sha(binary)==info['sha256']
  if win:
   assert binary[:2]==b'MZ';pe=struct.unpack_from('<I',binary,0x3c)[0];assert binary[pe:pe+4]==b'PE\0\0';assert struct.unpack_from('<H',binary,pe+4)[0]==0x8664
   opt=pe+24;assert struct.unpack_from('<H',binary,opt)[0]==0x20b;assert struct.unpack_from('<II',binary,opt+112+14*8)==(0,0)
  else:
   assert binary[:5]==b'\x7fELF\x02';assert struct.unpack_from('<H',binary,18)[0]==62
   deps=read(prefix+mode+'-native-dependencies.log' if driver else 'native-dependencies.log').decode().lower();assert 'libcoreclr' not in deps and 'libhostfxr' not in deps
  assert info.get('executed',info.get('nativeExecuted')) is True
  if driver:
   for ph in ['rust-default','rust-async','native-execute']:
    log=read(prefix+byname[mode+'-'+ph]['log']).decode();assert '1424 independent cases' in log
    if ph in ['rust-async','native-execute']:
     assert 'all six HP models' in log and 'selected-route' in log
    execution.append({'mode':mode,'phase':ph,'passLines':[l for l in log.splitlines() if l.startswith('PASS ')]})
  else:
   assert 'six operations' in read('native-aot-behavior.log').decode()
  binaries.append({'mode':mode,'rid':info['rid'],'framework':info['framework'],'sha256':sha(binary),'bytes':len(binary),'format':'PE-x64-no-CLR' if win else 'ELF64-x64','executionChecked':True})
 inventory=json.loads(read(prefix+'packages.json' if driver else 'packages.json'));packages=inventory['driverPackages'] if driver else inventory;package_rows={};package_reports=[]
 for p in packages:
  fname=p.get('file',p.get('archive','')).replace('\\','/').split('/')[-1];body=read(prefix+'packages/'+fname if driver else fname);assert sha(body)==p['sha256'];package_rows[p['package']]=p['sha256']
  if fname.endswith('.crate'):
   t=tarfile.open(fileobj=io.BytesIO(body));members=t.getnames();toml=tomllib.loads(t.extractfile(next(n for n in members if n.endswith('/Cargo.toml'))).read().decode());deps=toml.get('dependencies',{})
   if driver:
    assert set(deps)=={'instrument-api','instrument-scpi'}
    for file in ['hp5334.rs','hp3497.rs','hp_radio.rs','message_backend.rs']:
     packed=t.extractfile(next(n for n in members if n.endswith('/src/'+file))).read();assert packed.replace(b'\r\n',b'\n')==(pathlib.Path('/workspace/instrument-components-keysight/src')/file).read_bytes().replace(b'\r\n',b'\n')
    assert not any('/validation/' in n or '/spec/' in n or '/provenance/' in n for n in members)
   elif p['package'] in {'instrument-api','instrument-runtime','instrument-scpi','instrument-testkit','instrument-components'}:
    assert not any('keysight' in n.lower() or '/hp5334' in n.lower() or '/hp3497' in n.lower() or '/hp_radio' in n.lower() for n in members)
    assert not any('keysight' in dep for dep in deps)
   package_reports.append({'package':p['package'],'sha256':sha(body),'dependencies':list(deps),'memberCount':len(members)})
  elif fname.endswith('.nupkg'):
   nz=zipfile.ZipFile(io.BytesIO(body));members=nz.namelist();xml=ET.fromstring(nz.read(next(n for n in members if n.endswith('.nuspec'))));deps=[e.attrib.get('id') for e in xml.iter() if e.tag.endswith('dependency')]
   if not driver:assert not any('Keysight' in str(dep) for dep in deps)
   else:assert deps==['InstrumentComponents.Scpi']
   package_reports.append({'package':p['package'],'sha256':sha(body),'dependencies':deps,'members':members})
 suffix=a['name'].split('-latest-')[-1];platform='windows' if 'windows' in a['name'] else 'ubuntu';key=(platform,suffix)
 if driver:drivers.append((key,inventory))
 else:sdkrows[key]=package_rows
 reports.append({'artifactId':a['id'],'name':a['name'],'zipSha256':sha(raw),'phaseCount':len(phases),'phaseHashesAndExitsVerified':True,'binaries':binaries,'packages':package_reports,'execution':execution})
joins=0
for key,inv in drivers:
 for name,h in sdkrows[key].items():
  if name in {'instrument-api','instrument-runtime','instrument-scpi','instrument-testkit','instrument-components','InstrumentComponents','InstrumentComponents.Abstractions','InstrumentComponents.Runtime','InstrumentComponents.Scpi','InstrumentComponents.TestKit'}:
   assert inv['sdkPackages'][name]==h;joins+=1
assert phasecount==264 and len(reports)==8 and sum(len(r['binaries']) for r in reports)==12 and joins==40
out={'member':'C','sourceRun':43322642590,'sourceRevision':remote['sourceRevision'],'zipCount':len(reports),'phaseCount':phasecount,'binaryCount':12,'corePackageJoins':joins,'artifacts':reports}
(base/'member-c-artifact-audit.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='artifacts'}));print('PASS independent8ZIPs/264phases/12nativebinaries/40corejoins; allHPsources match packedcrate')
