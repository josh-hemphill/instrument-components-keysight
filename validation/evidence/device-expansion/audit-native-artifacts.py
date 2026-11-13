"""Independently inspect retained GitHub driver ZIPs, binaries, logs and archives."""
import argparse, hashlib, io, json, re, struct, tarfile, tomllib, zipfile
from pathlib import Path
import xml.etree.ElementTree as ET

parser=argparse.ArgumentParser()
parser.add_argument('directory',type=Path)
parser.add_argument('--cases',type=int,required=True)
parser.add_argument('--family',choices=['keysight','tektronix'],required=True)
parser.add_argument('--driver-repository',type=Path,required=True)
args=parser.parse_args(); root=args.directory
artifacts=json.loads((root/'github-artifacts.json').read_text())
selected=[a for a in artifacts['artifacts'] if a['name'].startswith('driver-proof-')]
assert len(selected)==4
records=[]
sha=lambda b:hashlib.sha256(b).hexdigest()
for artifact in selected:
    archive=root/(artifact['name']+'.zip');data=archive.read_bytes()
    assert artifact['digest']=='sha256:'+sha(data)
    with zipfile.ZipFile(io.BytesIO(data)) as packed:
        names=packed.namelist()
        def read(suffix):
            matches=[n for n in names if n.endswith(suffix)]
            assert len(matches)==1,(suffix,matches)
            return packed.read(matches[0])
        execution=json.loads(read('driver-proof/execution.json'))
        phases={p['name']:p for p in execution}
        assert len(phases)==len(execution)
        for p in execution:
            assert p['exitCode']==(1 if p['expectedFailure'] else 0),p
            log=read('driver-proof/'+p['log'])
            assert sha(log)==p['sha256']
            if p['name']=='negative-native-warning-gate':
                text=log.decode(errors='replace')
                assert re.search(r'\berror IL2026\b',text) and re.search(r'\berror IL3050\b',text)
        binaries=[]
        for mode in ['generated','packed']:
            for phase in ['rust-default','rust-async','native-execute']:
                p=phases[mode+'-'+phase]
                log=read('driver-proof/'+p['log']).decode(errors='replace')
                assert str(args.cases)+' independent cases' in log,(artifact['name'],p['name'],log[-1000:])
            info=json.loads(read('driver-proof/'+mode+'-native.json'))
            binary=read('driver-proof/'+mode+'-native/DriverProof'+('.exe' if info['rid']=='win-x64' else ''))
            assert info['executed'] and sha(binary)==info['sha256']
            if info['rid']=='win-x64':
                assert binary[:2]==b'MZ'; offset=struct.unpack_from('<I',binary,0x3c)[0]
                assert binary[offset:offset+4]==b'PE\0\0' and struct.unpack_from('<H',binary,offset+4)[0]==0x8664
                optional=offset+24;assert struct.unpack_from('<H',binary,optional)[0]==0x20b
                assert struct.unpack_from('<II',binary,optional+112+14*8)==(0,0),'CLR header present'
            else:
                assert binary[:5]==b'\x7fELF\x02' and struct.unpack_from('<H',binary,18)[0]==62
                deps=read('driver-proof/'+phases[mode+'-native-dependencies']['log']).decode().lower()
                assert not any(s in deps for s in ['coreclr','hostfxr','dotnet'])
            publish=read('driver-proof/'+phases[mode+'-native-publish']['log']).decode(errors='replace')
            assert not re.search(r'\bwarning IL\d+\b',publish,re.I)
            binaries.append({'mode':mode,**info,'bytes':len(binary),'nativeFormatVerified':True})
        inventory=json.loads(read('driver-proof/packages.json'))
        sdk_name=artifact['name'].replace('driver-proof-','driver-sdk-')
        with zipfile.ZipFile(root/(sdk_name+'.zip')) as sdk:
            expected={p['package']:p['sha256']for p in json.loads(sdk.read('packages.json'))}
            assert set(expected)=={'instrument-api','instrument-runtime','instrument-scpi','instrument-testkit','instrument-components','InstrumentComponents','InstrumentComponents.Abstractions','InstrumentComponents.Runtime','InstrumentComponents.Scpi','InstrumentComponents.TestKit','instrument-driver-tools','cargo-dependencies','instrument-synthetic-driver','InstrumentComponents.SyntheticDriver'}
            # The producer's test-only synthetic provider is intentionally not
            # exported into the independent device consumer's twelve-entry feed.
            del expected['instrument-synthetic-driver'];del expected['InstrumentComponents.SyntheticDriver']
        feeds=json.loads(read('driver-feeds/feeds.json'))
        assert inventory['mode']==feeds['mode']=='pinned-sdk-artifact-bootstrap'
        assert inventory['sdkPackages']==feeds['packages']==expected,'producer/consumer SDK checksum mismatch'
        archives=[]
        for package in inventory['driverPackages']:
            body=read('driver-proof/packages/'+package['file']);assert sha(body)==package['sha256']
            if package['file'].endswith('.crate'):
                with tarfile.open(fileobj=io.BytesIO(body)) as crate:
                    members=crate.getnames();manifest=next(n for n in members if n.endswith('/Cargo.toml'))
                    cargo=tomllib.loads(crate.extractfile(manifest).read().decode())
                    assert set(cargo['dependencies'])=={'instrument-api','instrument-scpi'}
                    assert all(d['version']=='=0.2.0-alpha.1' and not any(k in d for k in ['path','git']) for d in cargo['dependencies'].values())
                    for file in (['hp5334.rs','hp3497.rs','hp_radio.rs','message_backend.rs','qualified_scpi.rs','hp34980.rs','hp3585.rs'] if args.family=='keysight' else ['message_backend.rs','tds1002.rs']):
                        assert any(n.endswith('/src/'+file) for n in members),file
                        member=next(n for n in members if n.endswith('/src/'+file))
                        expected=args.driver_repository/'src'/file
                        assert crate.extractfile(member).read().replace(b'\r\n',b'\n')==expected.read_bytes().replace(b'\r\n',b'\n'),file+' source mismatch'
                    assert not any('/spec/' in n or '/validation/' in n for n in members)
                    for member in members:
                        relative='/'.join(member.split('/')[1:])
                        if relative.startswith('src/') or relative=='generated/profiles.rs':
                            assert crate.extractfile(member).read().replace(b'\r\n',b'\n')==(args.driver_repository/relative).read_bytes().replace(b'\r\n',b'\n'),relative
            else:
                with zipfile.ZipFile(io.BytesIO(body)) as nuget:
                    members=nuget.namelist();doc=ET.fromstring(nuget.read(next(n for n in members if n.endswith('.nuspec'))))
                    deps={d.attrib['id']:d.attrib['version'] for d in doc.findall('.//{*}dependency')}
                    assert deps=={'InstrumentComponents.Scpi':'[0.2.0-alpha.1]'}
                    assert {Path(n).name for n in members if n.endswith('.dll')}=={'InstrumentComponents.Drivers.'+args.family.capitalize()+'.dll'}
            archives.append({**package,'bytes':len(body),'members':members})
        records.append({'artifact':{k:artifact[k] for k in ['id','name','digest','size_in_bytes']},'verifiedZipSha256':sha(data),'phaseCount':len(execution),'casesPerFrontend':args.cases,'binaries':binaries,'packages':archives,'sdkPackages':inventory['sdkPackages']})
    print('PASS',artifact['name'],'ZIP/native/binary execution/log hashes/archive boundaries')
(root/'independent-audit.json').write_text(json.dumps({'schemaVersion':1,'artifacts':records},indent=2)+'\n')
print('PASS all four native platform/framework cells')
