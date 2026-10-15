"""Install into an isolated home-like directory, including paths with spaces."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class InstallTest(unittest.TestCase):
    def test_install_preserves_configuration_and_backs_up_widgets(self):
        with tempfile.TemporaryDirectory(prefix='miilo install ') as tmp:
            folder = Path(tmp)
            local, widgets, config = folder/'local', folder/'widgets', folder/'config'
            widgets.mkdir()
            (widgets/'70-miilo-fleet.owl').write_text('old widget')
            config.mkdir()
            config_file = config/'config.json'
            config_file.write_text('{"version":1,"endpoints":[]}')
            config_file.chmod(0o600)
            env = {**os.environ, 'MIILO_OWL_INSTALL_ROOT': str(local),
                   'MIILO_OWL_WIDGET_DIR': str(widgets), 'MIILO_OWL_CONFIG_DIR': str(config)}
            subprocess.run(['bash', str(ROOT/'install.sh'), '--no-setup'], env=env, check=True, capture_output=True)
            output = subprocess.check_output([str(local/'bin/miilo-owl'), 'snapshot'], env=env, text=True)
            self.assertFalse(json.loads(output)['configured'])
            self.assertEqual(3, len(list(widgets.glob('*.owl'))))
            self.assertEqual('old widget', (widgets/'70-miilo-fleet.owl.before-miilo-1.2').read_text())
            self.assertEqual('{"version":1,"endpoints":[]}', config_file.read_text())
            subprocess.run(['bash', str(ROOT/'install.sh'), '--no-setup'], env=env, check=True, capture_output=True)
            self.assertEqual('old widget', (widgets/'70-miilo-fleet.owl.before-miilo-1.2').read_text())

if __name__ == '__main__':
    unittest.main()
