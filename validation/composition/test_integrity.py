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
if __name__=='__main__':unittest.main()
