import hashlib, json, os, pathlib, subprocess
root = pathlib.Path('release-assets')
files = sorted(f for f in root.iterdir() if f.is_file() and f.name not in {'SHA256SUMS.txt', 'RELEASE-MANIFEST.json'})
required = ['x86_64-pc-windows-msvc', 'i686-pc-windows-msvc', 'x86_64-unknown-linux-gnu', 'aarch64-apple-darwin', 'x86_64-apple-darwin', '-web.zip', '-sources.zip']
for target in required:
    if not any(target in f.name for f in files):
        raise SystemExit(f'Incomplete release: missing {target}')
assets = [{'name': f.name, 'size': f.stat().st_size, 'sha256': hashlib.sha256(f.read_bytes()).hexdigest()} for f in files]
manifest = {'product': 'Scale Simulator', 'version': os.environ['RELEASE_VERSION'], 'commit': subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(), 'assets': assets}
(root/'RELEASE-MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
assets.append({'name':'RELEASE-MANIFEST.json','sha256':hashlib.sha256((root/'RELEASE-MANIFEST.json').read_bytes()).hexdigest()})
(root/'SHA256SUMS.txt').write_text(''.join(f"{a['sha256']}  {a['name']}\n" for a in assets))
