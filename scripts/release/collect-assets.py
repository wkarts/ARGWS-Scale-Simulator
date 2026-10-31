import os, pathlib, shutil, tarfile
root = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', 'src-tauri/target')) / os.environ['TARGET'] / 'release/bundle'
out = pathlib.Path('release-assets'); out.mkdir(exist_ok=True)
prefix = f"Scale-Simulator-{os.environ['RELEASE_VERSION']}-{os.environ['TARGET']}"
for f in root.rglob('*'):
    if f.is_file() and f.suffix.lower() in {'.exe', '.msi', '.dmg', '.deb', '.rpm', '.appimage'}:
        shutil.copy2(f, out / f'{prefix}-{f.name}')
for app in root.glob('macos/*.app'):
    with tarfile.open(out / f'{prefix}.app.tar.gz', 'w:gz') as archive:
        archive.add(app, arcname=app.name)
if not any(out.iterdir()):
    raise SystemExit(f'No bundles found: {root}')
