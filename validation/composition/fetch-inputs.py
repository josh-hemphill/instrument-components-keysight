#!/usr/bin/env python3
"""Fetch locked public source goldens and successful CI artifacts; verify transfers."""
import argparse,hashlib,json,re,subprocess,urllib.request,zipfile
from pathlib import Path,PurePosixPath
from instrument_driver_tools.feeds import extract
ROOT=Path(__file__).resolve().parent
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);p.add_argument('--os',choices=['ubuntu-latest','windows-latest']);p.add_argument('--framework',choices=['net8.0','net10.0']);p.add_argument('--sources-only',action='store_true');a=p.parse_args();out=a.output.resolve()
if out.exists() and any(out.iterdir()):raise ValueError('Empty input output directory required')
if not a.sources_only and (not a.os or not a.framework):raise ValueError('OS/framework required for artifacts')
out.mkdir(parents=True,exist_ok=True);lock=json.loads((ROOT/'composition-lock.json').read_text());records=[]
def api(path):return json.loads(subprocess.check_output(['gh','api',path],text=True))
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
for row in lock['drivers']:
 repo=row['repository'];commit=row['sourceCommit'];vendor=row['vendor']
 if repo!='josh-hemphill/instrument-components-'+vendor or not re.fullmatch('[0-9a-f]{40}',commit):raise ValueError('Unexpected locked repository/commit')
 archive=out/(vendor+'-source.tar.gz');urllib.request.urlretrieve('https://codeload.github.com/'+repo+'/tar.gz/'+commit,archive)
 raw=out/(vendor+'-source');extract(archive,raw);roots=list(raw.iterdir())
 if len(roots)!=1 or not roots[0].is_dir():raise ValueError('Unexpected source archive root')
 source=out/'sources'/vendor;source.parent.mkdir(exist_ok=True);roots[0].rename(source)
 for file,expected in row['goldenFiles'].items():
  parts=PurePosixPath(file).parts
  if not parts or '..' in parts or file.startswith('/') or '\\' in file:raise ValueError('Unexpected source path')
  if sha(source/file)!=expected:raise ValueError('Golden source hash mismatch')
 if 'source_ref: '+lock['sdkCommit'] not in (source/'.github/workflows/ci.yml').read_text():raise ValueError('SDK producer pin mismatch')
 record=dict(vendor=vendor,sourceCommit=commit,sourceArchiveSha256=sha(archive))
 if not a.sources_only:
  run=api('repos/'+repo+'/actions/runs/'+str(row['runId']))
  if run['head_sha']!=commit or run['conclusion']!='success' or run['path']!='.github/workflows/ci.yml':raise ValueError('Pinned vendor CI must have succeeded at exact source commit')
  name='driver-proof-'+a.os+'-'+a.framework;artifacts=api('repos/'+repo+'/actions/runs/'+str(row['runId'])+'/artifacts?per_page=100')['artifacts'];found=[artifact for artifact in artifacts if artifact['name']==name and not artifact['expired']]
  if len(found)!=1:raise ValueError('Missing/duplicate proof artifact')
  artifact=found[0];packed=out/(vendor+'-proof.zip')
  with packed.open('wb') as stream:
   result=subprocess.run(['gh','api','repos/'+repo+'/actions/artifacts/'+str(artifact['id'])+'/zip'],stdout=stream,stderr=subprocess.PIPE)
   if result.returncode:raise ValueError('Hosted artifact download failed; no signed redirect URL is logged')
  if artifact.get('digest')!='sha256:'+sha(packed):raise ValueError('Hosted artifact transfer digest mismatch')
  destination=out/'drivers'/vendor;destination.mkdir(parents=True)
  with zipfile.ZipFile(packed) as z:
   if len(z.infolist())>10000 or sum(i.file_size for i in z.infolist())>1024**3:raise ValueError('Excessive artifact')
   for member in z.infolist():
    path=PurePosixPath(member.filename)
    if path.is_absolute() or '..' in path.parts or '\\' in member.filename or ':' in member.filename or (member.external_attr>>16)&0o170000==0o120000:raise ValueError('Unsafe artifact path')
    target=destination.joinpath(*path.parts)
    if member.is_dir():target.mkdir(parents=True,exist_ok=True)
    else:
     target.parent.mkdir(parents=True,exist_ok=True)
     with target.open('xb') as stream:stream.write(z.read(member))
  manifests=list(destination.rglob('packages.json'))
  if len(manifests)!=1:raise ValueError('Expected one package manifest')
  if manifests[0].parent!=destination:(destination/'packages.json').write_bytes(manifests[0].read_bytes())
  record.update(runId=row['runId'],artifactId=artifact['id'],artifactDigest=artifact['digest'])
 records.append(record);print('Verified '+vendor+' source'+(' and CI artifact' if not a.sources_only else ''),flush=True)
report=dict(lockSha256=sha(ROOT/'composition-lock.json'),drivers=records)
(out/'verified-inputs.json').write_text(json.dumps(report,indent=2)+'\n')
if not a.sources_only:(out/'drivers/verified-inputs.json').write_text(json.dumps(report,indent=2)+'\n')
