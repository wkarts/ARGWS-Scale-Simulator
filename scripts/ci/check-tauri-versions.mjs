import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const read = (name) => fs.readFileSync(path.join(root, name), 'utf8');
const pkg = JSON.parse(read('package.json'));
const npmLock = JSON.parse(read('package-lock.json'));
const cargoLock = read('src-tauri/Cargo.lock');
const expected = '2.12.4';
assert.equal(pkg.dependencies['@tauri-apps/api'], expected);
assert.equal(npmLock.packages[''].dependencies['@tauri-apps/api'], expected);
assert.equal(npmLock.packages['node_modules/@tauri-apps/api'].version, expected);
for (const manifest of ['src-tauri/Cargo.toml', 'src-tauri/generic-license-tauri/Cargo.toml']) {
  assert.match(read(manifest), /tauri = \{ version = "=2\.12\.4"/);
}
assert.match(cargoLock, /\[\[package\]\]\nname = "tauri"\nversion = "2\.12\.4"/);
assert.equal(pkg.devDependencies['@tauri-apps/cli'], npmLock.packages['node_modules/@tauri-apps/cli'].version);
assert.match(read('src-tauri/Cargo.toml'), /tauri-build = \{ version = "=2\.7\.3"/);
assert.match(cargoLock, /\[\[package\]\]\nname = "tauri-build"\nversion = "2\.7\.3"/);
const installed = path.join(root, 'node_modules/@tauri-apps/api/package.json');
if (fs.existsSync(installed)) assert.equal(JSON.parse(fs.readFileSync(installed, 'utf8')).version, expected);
console.log(`Tauri Rust/API alinhados em ${expected}; CLI e tauri-build fixados com versões próprias.`);
