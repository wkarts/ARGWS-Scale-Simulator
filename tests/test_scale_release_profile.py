import importlib.util,json,pathlib,tempfile,unittest
ROOT=pathlib.Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('validator',ROOT/'scripts/ci/validate-tauri-project.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
class ReleaseProfile(unittest.TestCase):
 def test_profile_is_exact_not_a_validation_bypass(self):
  with tempfile.TemporaryDirectory() as temp:
   root=pathlib.Path(temp);(root/'.github').mkdir();p=root/'.github/release-profile.json';p.write_text(json.dumps(v.SCALE_RELEASE_PROFILE));self.assertTrue(v.scale_release_profile(root))
   altered=dict(v.SCALE_RELEASE_PROFILE);altered['targets']=['web'];p.write_text(json.dumps(altered))
   with self.assertRaises(ValueError):v.scale_release_profile(root)
 def test_every_target_and_integrity_gate_remains_required(self):
  workflow=(ROOT/'.github/workflows/release.yml').read_text()
  for marker in v.SCALE_RELEASE_MARKERS:self.assertIn(marker,workflow)
  for target in v.SCALE_RELEASE_PROFILE['targets'][:5]:self.assertIn(target,v.SCALE_RELEASE_MARKERS)
  for marker in ['asset.digest','Digest mismatch','--draft=false','needs: [prepare, desktop, web]']:self.assertIn(marker,v.SCALE_RELEASE_MARKERS)
 def test_mobile_workflows_are_retained(self):
  for name in ['mobile-android.yml','mobile-ios.yml']:
   content=(ROOT/'.github/workflows'/name).read_text();self.assertIn('actions/upload-artifact',content)
   if name=='mobile-android.yml':self.assertIn('--features mobile',content)
   else:
    self.assertIn('build-ios-unsigned.sh',content);self.assertIn('--features mobile',(ROOT/'scripts/mobile/build-ios-unsigned.sh').read_text())
if __name__=='__main__':unittest.main()
