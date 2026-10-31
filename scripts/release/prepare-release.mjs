import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const version = process.argv[2];
if (!version || !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-beta\.(0|[1-9]\d*))?$/.test(version)) {
  console.error('Uso: node scripts/release/prepare-release.mjs <version>');
  process.exit(1);
}

const root = process.cwd();

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}
`);
}

function updatePackageJson() {
  const filePath = path.join(root, 'package.json');
  const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  data.version = version;
  writeJson(filePath, data);
}

function updatePackageLock() {
  const filePath = path.join(root, 'package-lock.json');
  if (!fs.existsSync(filePath)) {
    return;
  }
  const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  data.version = version;
  if (data.packages && data.packages['']) {
    data.packages[''].version = version;
  }
  writeJson(filePath, data);
}

function updateTauriConf() {
  const filePath = path.join(root, 'src-tauri', 'tauri.conf.json');
  const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  data.version = version;
  writeJson(filePath, data);
}

function updateProjectConfig() {
  const filePath = path.join(root, 'src', 'config', 'projectConfig.ts');
  if (!fs.existsSync(filePath)) {
    return;
  }
  const content = fs.readFileSync(filePath, 'utf8');
  const updated = content.replace(/version:\s*["'][^"']+["']/, `version: "${version}"`);
  fs.writeFileSync(filePath, updated);
}

function updateCargoToml() {
  const filePath = path.join(root, 'src-tauri', 'Cargo.toml');
  const content = fs.readFileSync(filePath, 'utf8');
  const updated = content.replace(/(\[package\][\s\S]*?^version\s*=\s*")([^"]+)(")/m, `$1${version}$3`);
  fs.writeFileSync(filePath, updated);
}

fs.writeFileSync(path.join(root, 'VERSION'), `${version}
`);
updatePackageJson();
updatePackageLock();
updateTauriConf();
updateCargoToml();
updateProjectConfig();
console.log(`Versão atualizada para ${version}`);

// Keep only this project's package entry in Cargo.lock synchronized.
const lockPath = path.join(root, 'src-tauri', 'Cargo.lock');
const cargoName = fs.readFileSync(path.join(root, 'src-tauri', 'Cargo.toml'), 'utf8').match(/\[package\][\s\S]*?^name\s*=\s*"([^"]+)"/m)[1];
const lock = fs.readFileSync(lockPath, 'utf8');
const blocks = lock.split('[[package]]');
let found = false;
for (let i = 1; i < blocks.length; i++) {
  if (blocks[i].match(/^\s*name = "([^"]+)"/)[1] === cargoName) {
    blocks[i] = blocks[i].replace(/^version = "[^"]+"/m, `version = "${version}"`);
    found = true;
  }
}
if (!found) throw new Error('Project missing from Cargo.lock');
fs.writeFileSync(lockPath, blocks.join('[[package]]'));

const brandFile = path.join(root, 'src/assets/branding/brand.json');
if (fs.existsSync(brandFile)) { const brand=JSON.parse(fs.readFileSync(brandFile,'utf8')); brand.version=version; fs.writeFileSync(brandFile,JSON.stringify(brand,null,2)+'\n'); }

const cargoCheck=spawnSync(process.env.CARGO||'cargo',['metadata','--manifest-path','src-tauri/Cargo.toml','--locked','--format-version','1'],{cwd:root,encoding:'utf8'});
if(cargoCheck.status!==0)throw new Error(`Cargo.lock inconsistente: ${cargoCheck.stderr||cargoCheck.error||cargoCheck.status}`);
