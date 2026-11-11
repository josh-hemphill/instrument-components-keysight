#!/usr/bin/env python3
"""Run a static all-five app from checksummed package artifacts and locked goldens.

Input fetching is separate. No device source/project reference enters either
consumer. Local development artifacts are explicitly distinguished from pinned
hosted CI artifacts and from public-registry validation.
"""
import argparse,hashlib,json,os,platform,re,shutil,struct,subprocess,tomllib,zipfile
from pathlib import Path
import xml.etree.ElementTree as ET
from instrument_driver_tools.feeds import prepare,extract,VERSION
ROOT=Path(__file__).resolve().parent
VENDORS=['keysight','keithley','rigol','siglent','tektronix']
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def source_sha(path):
 # Git's Windows checkout may use CRLF. Compare canonical source text while
 # retaining exact byte hashes for every transferred archive and checksum file.
 return hashlib.sha256(path.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
def configure_public_registry(env,out,dotnet):
 publicCargo=out/'public-cargo-home';publicCargo.mkdir();env['CARGO_HOME']=str(publicCargo)
 (dotnet/'NuGet.Config').write_text('<configuration><packageSources><clear/><add key="nuget" value="https://api.nuget.org/v3/index.json"/></packageSources></configuration>')
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--sdk-artifacts',type=Path,required=True);p.add_argument('--driver-artifacts',type=Path,required=True);p.add_argument('--sources',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--framework',choices=['net8.0','net10.0'],required=True);p.add_argument('--development-inputs',action='store_true');p.add_argument('--public-registry',action='store_true',help='Resolve all consumer implementations from public registries with fresh caches; artifacts remain golden/provenance references only');a=p.parse_args()
 out=a.output.resolve()
 if out==ROOT or ROOT in out.parents or out in ROOT.parents:raise ValueError('Separate output directory required')
 if out.exists() and any(out.iterdir()):raise ValueError('Empty output directory required')
 if platform.system() not in ['Linux','Windows'] or platform.machine().lower() not in ['x86_64','amd64']:raise ValueError('Native x64 Linux/Windows required')
 out.mkdir(parents=True,exist_ok=True);feed=prepare(a.sdk_artifacts,out/'feeds');lock=json.loads((ROOT/'composition-lock.json').read_text());inputs=[]
 if not a.development_inputs:
  provenance=json.loads((a.driver_artifacts/'verified-inputs.json').read_text())
  if provenance['lockSha256']!=sha(ROOT/'composition-lock.json'):raise ValueError('Input lock mismatch')
 registry=Path(feed['cargoRegistry']);nuget=Path(feed['nugetFeed'])
 for entry in lock['drivers']:
  vendor=entry['vendor'];source=a.sources/vendor;artifact=a.driver_artifacts/vendor
  for file,expected in entry['goldenFiles'].items():
   if sha(source/file)!=expected:raise ValueError('Golden source hash mismatch: '+vendor+'/'+file)
  record=json.loads((artifact/'packages.json').read_text());expected={'instrument-driver-'+vendor:'crate','InstrumentComponents.Drivers.'+vendor.title():'nupkg'}
  if record['mode']!='pinned-sdk-artifact-bootstrap' or {r['package'] for r in record['driverPackages']}!=set(expected):raise ValueError('Unexpected driver package proof')
  for row in record['driverPackages']:
   found=list(artifact.rglob(row['file']))
   if len(found)!=1 or sha(found[0])!=row['sha256']:raise ValueError('Package hash mismatch')
   archive=found[0]
   if archive.name!=row['package']+'-'+VERSION+'.crate' and archive.name!=row['package']+'.'+VERSION+'.nupkg':raise ValueError('Unexpected version')
   if archive.suffix=='.crate':
    extract(archive,registry);directory=registry/(row['package']+'-'+VERSION);cargo=tomllib.loads((directory/'Cargo.toml').read_text());deps=cargo.get('dependencies',{})
    if set(deps)!={'instrument-api','instrument-scpi'} or any(d.get('version')!='='+VERSION or 'path' in d or 'git' in d for d in deps.values()):raise ValueError('Driver Rust dependency boundary')
    for file in ['src/lib.rs','generated/profiles.rs']:
     if source_sha(directory/file)!=entry['goldenFiles'][file]:raise ValueError('Packaged Rust implementation differs from locked source')
    if any('provenance' in part.parts or 'cases' in part.name.lower() for part in directory.rglob('*')):raise ValueError('Development assets bundled in driver')
    files={str(f.relative_to(directory)).replace('\\','/'):sha(f) for f in directory.rglob('*') if f.is_file()}
    (directory/'.cargo-checksum.json').write_text(json.dumps(dict(files=files,package=row['sha256'])))
   else:
    with zipfile.ZipFile(archive) as z:
     manifest=ET.fromstring(z.read(next(n for n in z.namelist() if n.endswith('.nuspec'))));deps={d.attrib['id']:d.attrib['version'] for d in manifest.findall('.//{*}dependency')}
     if deps!={'InstrumentComponents.Scpi':'['+VERSION+']'} or {Path(n).name for n in z.namelist() if n.endswith('.dll')}!={row['package']+'.dll'}:raise ValueError('Driver NuGet dependency boundary')
    shutil.copy2(archive,nuget)
   inputs.append(dict(vendor=vendor,sourceCommit=entry['sourceCommit'],package=row['package'],sha256=row['sha256']))
 rust=out/'app/rust';dotnet=out/'app/dotnet';(rust/'src').mkdir(parents=True);(rust/'generated').mkdir();dotnet.mkdir(parents=True)
 shutil.copy2(ROOT/'rust/src/main.rs',rust/'src/main.rs');shutil.copy2(ROOT/'dotnet/Program.cs',dotnet/'Program.cs')
 for entry in lock['drivers']:
  vendor=entry['vendor'];name=vendor.title();source=a.sources/vendor
  text=(source/'validation/rust/src/main.rs').read_text().replace('include!("../../../generated/cases.rs");','include!("../generated/'+vendor+'-cases.rs");').replace('fn main() -> Result<()>','pub fn run() -> Result<()>')
  (rust/'src'/ (vendor+'.rs')).write_text(text);shutil.copy2(source/'generated/cases.rs',rust/'generated'/(vendor+'-cases.rs'))
  text=(source/'validation/dotnet/Program.cs').read_text().replace('await Proof.Run();','namespace CompositionProof.'+name+';').replace('GeneratedDrivers.','Generated'+name+'.')
  (dotnet/(name+'Proof.cs')).write_text(text)
  (dotnet/(name+'Cases.cs')).write_text((source/'generated/GeneratedCases.cs').read_text().replace('namespace GeneratedDrivers','namespace Generated'+name))
 names=['instrument-components','instrument-api','instrument-runtime','instrument-scpi','instrument-testkit']+['instrument-driver-'+v for v in VENDORS]
 manifest='[package]\nname="all-device-composition"\nversion="0.0.0"\nedition="2021"\npublish=false\n[workspace]\n[dependencies]\n'+''.join(n+'="='+VERSION+'"\n' for n in names)+'tokio={version="1",features=["rt","macros"],optional=true}\n[features]\ndefault=[]\nasync=['+','.join(json.dumps(n+'/async') for n in names)+',"dep:tokio"]\n';(rust/'Cargo.toml').write_text(manifest)
 template=(a.sources/'keysight/validation/dotnet/DriverProof.csproj').read_text();template=template[:template.index('<ItemGroup>')]+'<ItemGroup>'+''.join('<PackageReference Include="'+n+'" Version="['+VERSION+']"/>' for n in ['InstrumentComponents','InstrumentComponents.Runtime','InstrumentComponents.TestKit']+['InstrumentComponents.Drivers.'+v.title() for v in VENDORS])+'</ItemGroup></Project>';(dotnet/'Composition.csproj').write_text(template.replace('net8.0',a.framework))
 shutil.copy2(feed['nugetConfig'],dotnet/'NuGet.Config')
 env=dict(os.environ,CARGO_HOME=feed['CARGO_HOME'],CARGO_TARGET_DIR=str(out/'cargo-target'),NUGET_PACKAGES=str(out/'nuget-cache'),DOTNET_CLI_HOME=str(out/'dotnet-home'),DOTNET_NOLOGO='1',DOTNET_CLI_TELEMETRY_OPTOUT='1');records=[];publicPackages=[]
 if a.public_registry:
  configure_public_registry(env,out,dotnet)
 cargoOffline=[] if a.public_registry else ['--offline']
 def run(name,cmd,cwd=out,negative=False):
  print(name,flush=True);log=out/(name+'.log')
  with log.open('w') as stream:r=subprocess.run(list(map(str,cmd)),cwd=cwd,env=env,stdout=stream,stderr=subprocess.STDOUT)
  text=log.read_text(errors='replace');records.append(dict(name=name,exitCode=r.returncode,expectedFailure=negative,log=log.name,sha256=sha(log)));(out/'execution.json').write_text(json.dumps(records,indent=2)+'\n')
  if (r.returncode==0 or any(not re.search(r'\berror '+diagnostic+r'\b',text) for diagnostic in ['IL2026','IL3050'])) if negative else (r.returncode!=0 or re.search(r'\bwarning IL\d+\b',text,re.I)):raise ValueError(name+' failed:\n'+text[-10000:])
  return text
 for feature in [[],['--features','async']]:
  tag='async' if feature else 'default';run('rust-'+tag,['cargo','run']+cargoOffline+['--release']+feature,rust);run('rust-'+tag+'-clippy',['cargo','clippy']+cargoOffline+['--release']+feature+['--','-D','warnings'],rust)
  metadata=json.loads(run('rust-'+tag+'-metadata',['cargo','metadata']+cargoOffline+['--format-version','1']+feature,rust))
  if any(p['name'] in names and not (p.get('source') or '').startswith('registry+') for p in metadata['packages']):raise ValueError('Source implementation leaked into app')
 if a.public_registry:
  lockData=tomllib.loads((rust/'Cargo.lock').read_text())
  for package in lockData['package']:
   if package['name'] in names:
    if package.get('source')!='registry+https://github.com/rust-lang/crates.io-index':raise ValueError('Expected public crates.io implementation')
    publicPackages.append(dict(ecosystem='Cargo',name=package['name'],version=package['version'],sha256=package['checksum']))
 rid='win-x64' if os.name=='nt' else 'linux-x64';native=out/'native';publish=['dotnet','publish','Composition.csproj','-c','Release','-r',rid,'--self-contained','true','-o',native,'--configfile',dotnet/'NuGet.Config'];run('native-publish',publish,dotnet)
 assets=json.loads((dotnet/'obj/project.assets.json').read_text())
 if any(v['type']!='package' for v in assets['libraries'].values()):raise ValueError('ProjectReference in app')
 if a.public_registry:
  for key in assets['libraries']:
   name,version=key.split('/')
   if name.startswith('InstrumentComponents'):
    archive=Path(env['NUGET_PACKAGES'])/name.lower()/version.lower()/(name.lower()+'.'+version.lower()+'.nupkg')
    publicPackages.append(dict(ecosystem='NuGet',name=name,version=version,sha256=sha(archive)))
 binary=native/('Composition.exe' if os.name=='nt' else 'Composition');data=binary.read_bytes()
 if os.name=='nt':
  offset=struct.unpack_from('<I',data,0x3c)[0];optional=offset+24
  if data[offset:offset+4]!=b'PE\0\0' or struct.unpack_from('<H',data,offset+4)[0]!=0x8664 or struct.unpack_from('<H',data,optional)[0]!=0x20b or struct.unpack_from('<II',data,optional+112+14*8)!=(0,0):raise ValueError('Expected native PE without CLR header')
 elif data[:5]!=b'\x7fELF\x02' or struct.unpack_from('<H',data,18)[0]!=62:raise ValueError('Expected native x64 ELF')
 else:
  dependencies=run('native-dependencies',['ldd',binary])
  if any(n in dependencies.lower() for n in ['coreclr','hostfxr','dotnet']):raise ValueError('Shared .NET runtime')
 run('native-execute',[binary]);run('native-lifecycle-repeat',[binary])
 if os.name!='nt':
  command=['bwrap','--die-with-parent','--unshare-net','--ro-bind','/usr','/usr','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--proc','/proc','--dev','/dev']
  for path in ['/usr/share/dotnet','/usr/lib/dotnet','/usr/local/share/dotnet','/usr/local/dotnet']:
   if Path(path).is_dir():command+=['--tmpfs',path]
  command+=['--ro-bind',str(native),'/app','--clearenv','--setenv','PATH','/usr/bin:/bin','--','/app/Composition']
  if os.environ.get('CI')=='true':command=['sudo','--']+command
  run('native-runtime-isolation',command)
 with (dotnet/'Program.cs').open('a') as stream:stream.write('\ninternal static class WarningProbe{[System.Diagnostics.CodeAnalysis.RequiresUnreferencedCode("intentional gate")][System.Diagnostics.CodeAnalysis.RequiresDynamicCode("intentional AOT gate")]internal static void Trigger(){} internal static void Run()=>Trigger();}\n')
 path=dotnet/'Program.cs';path.write_text(path.read_text().replace('await Composition.Run();','WarningProbe.Run();await Composition.Run();'));run('negative-native-warning',publish,dotnet,negative=True)
 (out/'composition.json').write_text(json.dumps(dict(mode='public-registry' if a.public_registry else 'local-bootstrap-development' if a.development_inputs else 'pinned-ci-artifact-bootstrap',lockSha256=sha(ROOT/'composition-lock.json'),rid=rid,framework=a.framework,nativeExecuted=True,sha256=sha(binary),referenceArtifactInputs=inputs,consumedPublicPackages=publicPackages,sdkArtifactHashes=feed['packages']),indent=2)+'\n')
 print('PASS static all-five app; '+('public-registry package execution; physical evidence remains unverified' if a.public_registry else 'bootstrap evidence only, not public registry or hardware evidence'),flush=True)
if __name__=='__main__':main()
