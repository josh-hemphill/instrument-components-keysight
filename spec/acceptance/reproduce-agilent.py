"""Reproduce independent manual-authored oracles in isolation, without backend reads."""
import hashlib,json,shutil,subprocess,sys,tempfile
from pathlib import Path
root=Path(__file__).resolve().parent
helpers=['build-first-three-oracles.py','build-33120a-v2-oracle.py','build-module-oracles.py','build-3585b-oracle.py']
with tempfile.TemporaryDirectory(prefix='agilent-oracle-') as temporary:
 out=Path(temporary)
 for helper in helpers:
  shutil.copy2(root/helper,out/helper)
  subprocess.run([sys.executable,str(out/helper)],check=True,cwd=out)
 rows=[]
 for model in ['34401a','33120a','53132a','34980a','34937a','34922a','34950a','3585b']:
  produced=out/((model+'-v2' if model=='33120a' else model)+'.catalog-cases.json')
  committed=root/(model+'.catalog-cases.json')
  assert produced.read_bytes()==committed.read_bytes(),model+' oracle drift'
  rows.extend(json.loads(produced.read_text())['cases'])
 assert len(rows)==504 and len({r['id'] for r in rows})==504
 print('PASS all504 independent Agilent cases reproduce byte-for-byte; active shards untouched')
