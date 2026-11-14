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
from qualification_logs import execute_logged
ROOT=Path(__file__).resolve().parent
VENDORS=['keysight','keithley','rigol','siglent','tektronix']
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def source_sha(path):
 # Git's Windows checkout may use CRLF. Compare canonical source text while
 # retaining exact byte hashes for every transferred archive and checksum file.
 return hashlib.sha256(path.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
def configure_public_registry(env,out,dotnet):
 publicCargo=out/'public-cargo-home';publicCargo.mkdir();env['CARGO_HOME']=str(publicCargo)
 (dotnet/'NuGet.Config').write_text('<configuration><packageSources><clear/><add key="nuget" value="https://api.nuget.org/v3/index.json"/></packageSources></configuration>', encoding='utf-8')
def audit_driver_rust(directory,goldens):
 cargo=tomllib.loads((directory/'Cargo.toml').read_text(encoding='utf-8'));package=cargo['package'];deps=cargo.get('dependencies',{})
 if package['version']!=VERSION or package['name'] not in {'instrument-driver-'+v for v in VENDORS}:raise ValueError('Driver Rust identity boundary')
 if set(deps)!={'instrument-api','instrument-scpi'} or any(d.get('version')!='='+VERSION or 'path' in d or 'git' in d for d in deps.values()):raise ValueError('Driver Rust dependency boundary')
 expected={k for k in goldens if k.startswith('src/') or k=='generated/profiles.rs'}
 members={str(p.relative_to(directory)).replace('\\','/') for p in directory.rglob('*') if p.is_file()}
 metadata={'Cargo.toml','Cargo.toml.orig','Cargo.lock','.cargo_vcs_info.json','README.md','LICENSE-MIT','LICENSE-APACHE'}
 if members-expected-metadata:raise ValueError('Non-production member bundled in driver: '+str(sorted(members-expected-metadata)))
 if {n for n in members if n.startswith('src/') or n=='generated/profiles.rs'}!=expected:raise ValueError('Unmatched shipped Rust source member')
 for file in sorted(expected):
  if source_sha(directory/file)!=goldens[file]:raise ValueError('Rust implementation differs from locked source: '+file)
def audit_driver_nuget(archive,name):
 with zipfile.ZipFile(archive) as z:
  names=z.namelist()
  if len(names)!=len(set(names)):raise ValueError('Duplicate NuGet member')
  manifest=ET.fromstring(z.read(next(n for n in names if n.endswith('.nuspec'))));deps={d.attrib['id']:d.attrib['version'] for d in manifest.findall('.//{*}dependency')}
  if manifest.findtext('.//{*}id')!=name or manifest.findtext('.//{*}version')!=VERSION:raise ValueError('Installed NuGet identity mismatch')
  if deps!={'InstrumentComponents.Scpi':'['+VERSION+']'} or {Path(n).name for n in names if n.endswith('.dll')}!={name+'.dll'}:raise ValueError('Driver NuGet dependency boundary')
  if any('provenance' in n.lower() or 'cases' in Path(n).name.lower() for n in names):raise ValueError('Development assets bundled in NuGet')
  result=hashlib.sha256()
  for n in sorted(names):
   if n=='.signature.p7s' or n.endswith('/'):continue
   key=n.encode();data=z.read(n);result.update(len(key).to_bytes(8,'big'));result.update(key);result.update(len(data).to_bytes(8,'big'));result.update(data)
  return result.hexdigest()
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--sdk-artifacts',type=Path,required=True);p.add_argument('--driver-artifacts',type=Path,required=True);p.add_argument('--sources',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--framework',choices=['net8.0','net10.0'],required=True);p.add_argument('--development-inputs',action='store_true');p.add_argument('--public-registry',action='store_true',help='Resolve all consumer implementations from public registries with fresh caches; artifacts remain golden/provenance references only');a=p.parse_args()
 out=a.output.resolve()
 if out==ROOT or ROOT in out.parents or out in ROOT.parents:raise ValueError('Separate output directory required')
 if out.exists() and any(out.iterdir()):raise ValueError('Empty output directory required')
 if platform.system() not in ['Linux','Windows'] or platform.machine().lower() not in ['x86_64','amd64']:raise ValueError('Native x64 Linux/Windows required')
 out.mkdir(parents=True,exist_ok=True);feed=prepare(a.sdk_artifacts,out/'feeds');lock=json.loads((ROOT/'composition-lock.json').read_text(encoding='utf-8'))
 helpers=lock['qualificationHelpers']
 if helpers['payloadCommit']!=lock['sdkCommit'] or source_sha(ROOT/'qualification_logs.py')!=helpers['loggerSha256']:raise ValueError('Qualification helper/payload/logger lock mismatch')
 inputs=[]
 if not a.development_inputs:
  provenance=json.loads((a.driver_artifacts/'verified-inputs.json').read_text(encoding='utf-8'))
  if provenance['lockSha256']!=sha(ROOT/'composition-lock.json'):raise ValueError('Input lock mismatch')
 registry=Path(feed['cargoRegistry']);nuget=Path(feed['nugetFeed'])
 for entry in lock['drivers']:
  vendor=entry['vendor'];source=a.sources/vendor;artifact=a.driver_artifacts/vendor
  for file,expected in entry['goldenFiles'].items():
   if source_sha(source/file)!=expected:raise ValueError('Golden source hash mismatch: '+vendor+'/'+file)
  record=json.loads((artifact/'packages.json').read_text(encoding='utf-8'));expected={'instrument-driver-'+vendor:'crate','InstrumentComponents.Drivers.'+vendor.title():'nupkg'}
  if record['mode']!='pinned-sdk-artifact-bootstrap' or {r['package'] for r in record['driverPackages']}!=set(expected):raise ValueError('Unexpected driver package proof')
  for row in record['driverPackages']:
   found=list(artifact.rglob(row['file']))
   if len(found)!=1 or sha(found[0])!=row['sha256']:raise ValueError('Package hash mismatch')
   archive=found[0]
   if archive.name!=row['package']+'-'+VERSION+'.crate' and archive.name!=row['package']+'.'+VERSION+'.nupkg':raise ValueError('Unexpected version')
   if archive.suffix=='.crate':
    extract(archive,registry);directory=registry/(row['package']+'-'+VERSION);cargo=tomllib.loads((directory/'Cargo.toml').read_text(encoding='utf-8'));deps=cargo.get('dependencies',{})
    if set(deps)!={'instrument-api','instrument-scpi'} or any(d.get('version')!='='+VERSION or 'path' in d or 'git' in d for d in deps.values()):raise ValueError('Driver Rust dependency boundary')
    audit_driver_rust(directory,entry['goldenFiles'])
    files={str(f.relative_to(directory)).replace('\\','/'):sha(f) for f in directory.rglob('*') if f.is_file()}
    (directory/'.cargo-checksum.json').write_text(json.dumps(dict(files=files,package=row['sha256'])), encoding='utf-8')
   else:
    audit_driver_nuget(archive,row['package'])
    shutil.copy2(archive,nuget)
   inputs.append(dict(vendor=vendor,sourceCommit=entry['sourceCommit'],package=row['package'],sha256=row['sha256']))
 rust=out/'app/rust';dotnet=out/'app/dotnet';(rust/'src').mkdir(parents=True);(rust/'generated').mkdir();dotnet.mkdir(parents=True)
 shutil.copy2(ROOT/'rust/src/main.rs',rust/'src/main.rs')
 for file in (ROOT/'dotnet').glob('*.cs'):shutil.copy2(file,dotnet/file.name)
 for entry in lock['drivers']:
  vendor=entry['vendor'];name=vendor.title();source=a.sources/vendor
  text=(source/'validation/rust/src/main.rs').read_text(encoding='utf-8').replace('include!("../../../generated/cases.rs");','include!("../generated/'+vendor+'-cases.rs");').replace('fn main() -> Result<()>','pub fn package_proof_main() -> Result<()>').replace('crate::', 'crate::'+vendor+'::')
  if vendor=='keysight':
   text+='\npub fn composed_additions(io: &Script, native: bool) -> Result<Vec<Arc<dyn DriverFactory>>> { let mut seen=BTreeSet::new(); let mut all=Vec::new(); for case in generated_cases::cases() { let id=case.id.split("|").next().unwrap(); if ["keysight.34401a","keysight.33120a","keysight.53132a","keysight.hp3585b","keysight.34980a","keysight.hp5334a","keysight.hp5334b","keysight.hp3497a","keysight.hp8920a","keysight.hp8920b","keysight.hp8921a"].contains(&id) && seen.insert(id.to_owned()) {all.extend(case_factories(&case,io,native)?);} } assert_eq!(seen.len(),11); Ok(all) }\n'
  elif vendor=='tektronix':
   text+='\npub fn composed_additions(io: &Script, native: bool) -> Result<Vec<Arc<dyn DriverFactory>>> { Ok(vec![tds_fixture::factory(io,native)?]) }\n'
  (rust/'src'/ (vendor+'.rs')).write_text(text, encoding='utf-8');shutil.copy2(source/'generated/cases.rs',rust/'generated'/(vendor+'-cases.rs'))
  helpers=rust/'src'/vendor;helpers.mkdir()
  for file in sorted((source/'validation/rust/src').rglob('*.rs')):
   if file.name=='main.rs':continue
   target=helpers/file.relative_to(source/'validation/rust/src');target.parent.mkdir(parents=True,exist_ok=True)
   target.write_text(file.read_text(encoding='utf-8').replace('crate::','crate::'+vendor+'::'), encoding='utf-8')
  text=(source/'validation/dotnet/Program.cs').read_text(encoding='utf-8').replace('await Proof.Run();','namespace CompositionProof.'+name+';').replace('GeneratedDrivers.','Generated'+name+'.')
  (dotnet/(name+'Proof.cs')).write_text(text, encoding='utf-8')
  for file in sorted((source/'validation/dotnet').rglob('*.cs')):
   if file==source/'validation/dotnet/Program.cs' or {'obj','bin'} & set(file.parts):continue
   target=dotnet/name/file.relative_to(source/'validation/dotnet');target.parent.mkdir(parents=True,exist_ok=True)
   target.write_text('namespace CompositionProof.'+name+';\n'+file.read_text(encoding='utf-8').replace('GeneratedDrivers.','Generated'+name+'.'), encoding='utf-8')
  (dotnet/(name+'Cases.cs')).write_text((source/'generated/GeneratedCases.cs').read_text(encoding='utf-8').replace('namespace GeneratedDrivers','namespace Generated'+name), encoding='utf-8')
 names=['instrument-components','instrument-api','instrument-runtime','instrument-scpi','instrument-testkit']+['instrument-driver-'+v for v in VENDORS]
 manifest='[package]\nname="all-device-composition"\nversion="0.0.0"\nedition="2021"\npublish=false\n[workspace]\n[dependencies]\n'+''.join(n+'="='+VERSION+'"\n' for n in names)+'tokio={version="1",features=["rt","macros","time","sync"],optional=true}\n[features]\ndefault=[]\nasync=['+','.join(json.dumps(n+'/async') for n in names)+',"dep:tokio"]\n';manifest+='[profile.release.package.all-device-composition]\nopt-level=0\n';(rust/'Cargo.toml').write_text(manifest, encoding='utf-8')
 template=(a.sources/'keysight/validation/dotnet/DriverProof.csproj').read_text(encoding='utf-8');template=template[:template.index('<ItemGroup>')]+'<ItemGroup>'+''.join('<PackageReference Include="'+n+'" Version="['+VERSION+']"/>' for n in ['InstrumentComponents','InstrumentComponents.Runtime','InstrumentComponents.TestKit']+['InstrumentComponents.Drivers.'+v.title() for v in VENDORS])+'</ItemGroup></Project>';(dotnet/'Composition.csproj').write_text(template.replace('net8.0',a.framework), encoding='utf-8')
 shutil.copy2(feed['nugetConfig'],dotnet/'NuGet.Config')
 env=dict(os.environ,CARGO_HOME=feed['CARGO_HOME'],CARGO_TARGET_DIR=str(out/'cargo-target'),NUGET_PACKAGES=str(out/'nuget-cache'),NUGET_HTTP_CACHE_PATH=str(out/'nuget-http-cache'),DOTNET_CLI_HOME=str(out/'dotnet-home'),DOTNET_NOLOGO='1',DOTNET_CLI_TELEMETRY_OPTOUT='1');records=[];publicPackages=[]
 if a.public_registry:
  configure_public_registry(env,out,dotnet)
 cargoOffline=[] if a.public_registry else ['--offline']
 def run(name,cmd,cwd=out,negative=False,structured_output=False):
  print(name,flush=True);phase=execute_logged(name,cmd,cwd=cwd,env=env,output=out,structured_output=structured_output);phase['expectedFailure']=negative
  text=(out/phase['log']).read_text(errors='replace',encoding='utf-8');diagnostic=(out/phase['stderrLog']).read_text(errors='replace',encoding='utf-8') if 'stderrLog' in phase else '';combined=text+diagnostic
  records.append(phase);(out/'execution.json').write_text(json.dumps(records,indent=2)+'\n',encoding='utf-8')
  if (phase['exitCode']==0 or any(not re.search(r'\berror '+code+r'\b',combined) for code in ['IL2026','IL3050'])) if negative else (phase['exitCode']!=0 or re.search(r'\bwarning IL\d+\b',combined,re.I)):raise ValueError(name+' failed:\n'+combined[-10000:])
  return text

 for feature in [[],['--features','async']]:
  tag='async' if feature else 'default';run('rust-'+tag,['cargo','run']+cargoOffline+['--release']+feature,rust);run('rust-'+tag+'-clippy',['cargo','clippy']+cargoOffline+['--release']+feature+['--','-D','warnings'],rust)
  metadata=json.loads(run('rust-'+tag+'-metadata',['cargo','metadata']+cargoOffline+['--format-version','1']+feature,rust,structured_output=True))
  if any(p['name'] in names and not (p.get('source') or '').startswith('registry+') for p in metadata['packages']):raise ValueError('Source implementation leaked into app')
 if a.public_registry:
  lockData=tomllib.loads((rust/'Cargo.lock').read_text(encoding='utf-8'))
  for package in lockData['package']:
   if package['name'] in names:
    if package.get('source')!='registry+https://github.com/rust-lang/crates.io-index':raise ValueError('Expected public crates.io implementation')
    archives=list(Path(env['CARGO_HOME']).glob('registry/cache/*/'+package['name']+'-'+package['version']+'.crate'))
    if len(archives)!=1 or sha(archives[0])!=package['checksum']:raise ValueError('Actual installed crate identity mismatch')
    if package['name'].startswith('instrument-driver-'):
     vendor=package['name'].removeprefix('instrument-driver-');entry=next(e for e in lock['drivers'] if e['vendor']==vendor)
     inspected=out/'public-inspected'/vendor;extract(archives[0],inspected);directory=inspected/(package['name']+'-'+package['version'])
     audit_driver_rust(directory,entry['goldenFiles'])
    publicPackages.append(dict(ecosystem='Cargo',name=package['name'],version=package['version'],archiveSha256=sha(archives[0]),registryChecksum=package['checksum']))
 rid='win-x64' if os.name=='nt' else 'linux-x64';native=out/'native';publish=['dotnet','publish','Composition.csproj','-c','Release','-r',rid,'--self-contained','true','-o',native,'--configfile',dotnet/'NuGet.Config'];run('native-publish',publish,dotnet)
 assets=json.loads((dotnet/'obj/project.assets.json').read_text(encoding='utf-8'))
 if any(v['type']!='package' for v in assets['libraries'].values()):raise ValueError('ProjectReference in app')
 if a.public_registry:
  for key in assets['libraries']:
   name,version=key.split('/')
   if name.startswith('InstrumentComponents'):
    archive=Path(env['NUGET_PACKAGES'])/name.lower()/version.lower()/(name.lower()+'.'+version.lower()+'.nupkg')
    if version!=VERSION:raise ValueError('Unexpected installed NuGet version')
    payload=audit_driver_nuget(archive,name) if name.startswith('InstrumentComponents.Drivers.') else None
    publicPackages.append(dict(ecosystem='NuGet',name=name,version=version,archiveSha256=sha(archive),canonicalPayloadSha256=payload))
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
 executedBinarySha256=sha(binary)
 consumerFiles={str(p.relative_to(out/'app')).replace('\\','/'):sha(p) for p in (out/'app').rglob('*') if p.is_file() and p.suffix in ['.rs','.cs','.csproj','.toml'] and not {'obj','bin'} & set(p.parts)}
 with (dotnet/'Program.cs').open('a', encoding='utf-8') as stream:stream.write('\ninternal static class WarningProbe{[System.Diagnostics.CodeAnalysis.RequiresUnreferencedCode("intentional gate")][System.Diagnostics.CodeAnalysis.RequiresDynamicCode("intentional AOT gate")]internal static void Trigger(){} internal static void Run()=>Trigger();}\n')
 path=dotnet/'Program.cs';path.write_text(path.read_text(encoding='utf-8').replace('await Composition.Run();','WarningProbe.Run();await Composition.Run();'), encoding='utf-8');negativePublish=list(publish);negativePublish[negativePublish.index('-o')+1]=out/'warning-native';run('negative-native-warning',negativePublish,dotnet,negative=True)
 if sha(binary)!=executedBinarySha256:raise ValueError('Negative probe changed executed binary')
 (out/'composition.json').write_text(json.dumps(dict(helperSourceCommit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(), loggerSha256=sha(ROOT/'qualification_logs.py'), mode='public-registry' if a.public_registry else 'local-bootstrap-development' if a.development_inputs else 'pinned-ci-artifact-bootstrap',lockSha256=sha(ROOT/'composition-lock.json'),rid=rid,framework=a.framework,nativeExecuted=True,sha256=executedBinarySha256,referenceArtifactInputs=inputs,consumedPublicPackages=publicPackages,sdkArtifactHashes=feed['packages'],consumerFiles=consumerFiles,negativeProbeProgramSha256=sha(dotnet/'Program.cs'),hardwareValidated=False),indent=2)+'\n', encoding='utf-8')
 print('PASS static all-five app; '+('public-registry package execution; physical evidence remains unverified' if a.public_registry else 'bootstrap evidence only, not public registry or hardware evidence'),flush=True)
if __name__=='__main__':main()
