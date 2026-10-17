import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import urllib.error

spec=importlib.util.spec_from_file_location('sdk_access',Path(__file__).with_name('sdk_access.py'))
sdk=importlib.util.module_from_spec(spec)
spec.loader.exec_module(sdk)
class AccessTests(unittest.TestCase):
    def test_exact_public_sdk_pin_required(self):
        self.assertEqual(sdk.pinned_reference({'git':'https://github.com/miilo-ai/miilo-sdk','rev':'a'*40}),'a'*40)
        with self.assertRaises(ValueError):sdk.pinned_reference({'git':'https://example.invalid/sdk','rev':'a'*40})
        with self.assertRaises(ValueError):sdk.pinned_reference({'git':'https://github.com/miilo-ai/miilo-sdk','rev':'main'})
    def test_missing_private_access_cannot_qualify(self):
        with tempfile.TemporaryDirectory() as directory:
            output=Path(directory)/'out'
            with patch.dict(sdk.os.environ,{'SDK_READ_TOKEN':'','GITHUB_OUTPUT':str(output)},clear=True),patch.object(sdk.urllib.request,'urlopen',side_effect=urllib.error.HTTPError('https://example.invalid',404,'private',None,None)):
                sdk.main()
            self.assertEqual(output.read_text(),'ready=false\n')
if __name__=='__main__':unittest.main()
