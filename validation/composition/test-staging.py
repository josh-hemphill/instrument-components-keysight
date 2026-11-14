#!/usr/bin/env python3
"""Exercise valid-lock input staging before package execution; synthetic files only."""
import hashlib,json,runpy,sys,tempfile,types,unittest
from pathlib import Path
from unittest.mock import patch
HERE=Path(__file__).resolve().parent
VALIDATOR=HERE/'validate.py'
VERSION=json.loads((HERE.parents[1]/'driver-package.json').read_text())['version']

class StagingTests(unittest.TestCase):
    def test_valid_helper_lock_records_first_package_before_next_audit(self):
        with tempfile.TemporaryDirectory() as directory:
            base=Path(directory);root=base/'helper';root.mkdir();(root/'qualification_logs.py').write_bytes(b'logger\n')
            sources=base/'sources';(sources/'siglent').mkdir(parents=True)
            drivers=base/'drivers';artifact=drivers/'siglent';artifact.mkdir(parents=True)
            registry=base/'registry';registry.mkdir();nuget=base/'nuget';nuget.mkdir()
            crate=artifact/('instrument-driver-siglent-'+VERSION+'.crate');crate.write_bytes(b'synthetic crate')
            nupkg=artifact/('InstrumentComponents.Drivers.Siglent.'+VERSION+'.nupkg');nupkg.write_bytes(b'synthetic next audit')
            rows=[dict(package='instrument-driver-siglent',file=crate.name,sha256=hashlib.sha256(crate.read_bytes()).hexdigest()),dict(package='InstrumentComponents.Drivers.Siglent',file=nupkg.name,sha256=hashlib.sha256(nupkg.read_bytes()).hexdigest())]
            (artifact/'packages.json').write_text(json.dumps(dict(mode='pinned-sdk-artifact-bootstrap',driverPackages=rows)))
            lock=dict(sdkCommit='a'*40,qualificationHelpers=dict(payloadCommit='a'*40,loggerSha256=hashlib.sha256(b'logger\n').hexdigest()),drivers=[dict(vendor='siglent',sourceCommit='b'*40,goldenFiles={})])
            (root/'composition-lock.json').write_text(json.dumps(lock))
            feeds=types.ModuleType('instrument_driver_tools.feeds');feeds.VERSION=VERSION
            feeds.prepare=lambda *args:dict(cargoRegistry=str(registry),nugetFeed=str(nuget))
            def extract(*args):
                target=registry/('instrument-driver-siglent-'+VERSION);target.mkdir()
                (target/'Cargo.toml').write_text('[dependencies]\ninstrument-api={version="='+VERSION+'"}\ninstrument-scpi={version="='+VERSION+'"}\n')
            feeds.extract=extract
            class NextAudit(Exception):pass
            with patch.dict(sys.modules,{'instrument_driver_tools':types.ModuleType('instrument_driver_tools'),'instrument_driver_tools.feeds':feeds}):
                loaded=runpy.run_path(str(VALIDATOR))
            main=loaded['main'];scope=main.__globals__;scope['ROOT']=root;scope['audit_driver_rust']=lambda *args:None
            def next_audit(*args):raise NextAudit()
            scope['audit_driver_nuget']=next_audit
            args=['validate.py','--sdk-artifacts',str(base/'sdk'),'--driver-artifacts',str(drivers),'--sources',str(sources),'--output',str(base/'output'),'--framework','net8.0','--development-inputs']
            with patch.object(sys,'argv',args):
                with self.assertRaises(NextAudit):main()

if __name__=='__main__':unittest.main()
