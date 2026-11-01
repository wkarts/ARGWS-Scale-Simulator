import hashlib, json, os, pathlib, re, subprocess
root = pathlib.Path('release-assets')
archived = [f for f in root.iterdir() if f.is_file() and re.match(r'^previous-[a-f0-9]{40}-', f.name)]
files = sorted(f for f in root.iterdir() if f.is_file() and f not in archived and f.name not in {'SHA256SUMS.txt', 'RELEASE-MANIFEST.json'})
version = os.environ['RELEASE_VERSION']
required = {
    'x86_64-pc-windows-msvc': ['.exe', '.msi'],
    'i686-pc-windows-msvc': ['.exe', '.msi'],
    'x86_64-unknown-linux-gnu': ['.AppImage', '.deb', '.rpm'],
    'aarch64-apple-darwin': ['.dmg', '.app.tar.gz'],
    'x86_64-apple-darwin': ['.dmg', '.app.tar.gz'],
}
if len(files) != 13 or any(not f.name.startswith(f'Scale-Simulator-{version}-') for f in files):
    raise SystemExit('Incomplete or mixed-generation release: expected exactly 13 current-version bundles')
for target, suffixes in required.items():
    for suffix in suffixes:
        if sum(f.name.startswith(f'Scale-Simulator-{version}-{target}') and f.name.endswith(suffix) for f in files) != 1:
            raise SystemExit(f'Incomplete release: missing or duplicate {target} {suffix}')
for suffix in ['-web.zip', '-sources.zip']:
    if not (root/f'Scale-Simulator-{version}{suffix}').is_file():
        raise SystemExit(f'Incomplete release: missing {suffix}')
assets = [{'name': f.name, 'size': f.stat().st_size, 'sha256': hashlib.sha256(f.read_bytes()).hexdigest()} for f in files]
manifest = {'product': 'Scale Simulator', 'version': os.environ['RELEASE_VERSION'], 'commit': subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(), 'assets': assets}
(root/'RELEASE-MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
assets.append({'name':'RELEASE-MANIFEST.json','sha256':hashlib.sha256((root/'RELEASE-MANIFEST.json').read_bytes()).hexdigest()})
(root/'SHA256SUMS.txt').write_text(''.join(f"{a['sha256']}  {a['name']}\n" for a in assets))

# Older main workflow definitions download all draft files; keep only current bundles
# in this disposable runner directory. Remote backups remain until final verification.
for previous in archived:
    previous.unlink()
