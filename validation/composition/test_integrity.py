"""Regression for actual Windows archive provenance failure; no network/IO devices."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
spec=importlib.util.spec_from_file_location('composition_validate',Path(__file__).parent/'validate.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
class Integrity(unittest.TestCase):
 def test_windows_checkout_endings_only(self):
  with tempfile.TemporaryDirectory() as root:
   linux=Path(root)/'linux.rs';windows=Path(root)/'windows.rs';changed=Path(root)/'changed.rs'
   linux.write_bytes(b'fn main() {\n query("READ?");\n}\n');windows.write_bytes(linux.read_bytes().replace(b'\n',b'\r\n'));changed.write_bytes(windows.read_bytes().replace(b'READ?',b'RESET'))
   self.assertEqual(module.source_sha(linux),module.source_sha(windows));self.assertNotEqual(module.sha(linux),module.sha(windows));self.assertNotEqual(module.source_sha(linux),module.source_sha(changed))
 def test_public_mode_removes_artifact_sources_and_uses_empty_cargo_home(self):
  import xml.etree.ElementTree as ET
  with tempfile.TemporaryDirectory() as directory:
   root=Path(directory);dotnet=root/'dotnet';dotnet.mkdir();(dotnet/'NuGet.Config').write_text('<configuration><packageSources><add key="bootstrap" value="/artifact-feed"/></packageSources></configuration>')
   env={'CARGO_HOME':'/artifact-cargo-home'};module.configure_public_registry(env,root,dotnet)
   self.assertEqual(list(Path(env['CARGO_HOME']).iterdir()),[])
   sources=ET.parse(dotnet/'NuGet.Config').findall('.//packageSources/add');self.assertEqual([(s.get('key'),s.get('value')) for s in sources],[('nuget','https://api.nuget.org/v3/index.json')])
if __name__=='__main__':unittest.main()
