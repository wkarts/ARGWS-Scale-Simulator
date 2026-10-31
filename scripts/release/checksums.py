import hashlib, json, os, pathlib, subprocess
root = pathlib.Path('release-assets')
files = sorted(f for f in root.iterdir() if f.is_file())
if len(files) < 7:
    raise SystemExit('Incomplete release artifacts')
assets = [{'name': f.name, 'size': f.stat().st_size, 'sha256': hashlib.sha256(f.read_bytes()).hexdigest()} for f in files]
manifest = {'product': 'Scale Simulator', 'version': os.environ['RELEASE_VERSION'], 'commit': subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(), 'assets': assets}
(root/'RELEASE-MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
assets.append({'name':'RELEASE-MANIFEST.json','sha256':hashlib.sha256((root/'RELEASE-MANIFEST.json').read_bytes()).hexdigest()})
(root/'SHA256SUMS.txt').write_text(''.join(f"{a['sha256']}  {a['name']}\n" for a in assets))
