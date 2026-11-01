import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {parseVersion} from './version-plan.mjs';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const original=fs.readFileSync(path.join(root,'package.json'),'utf8');
const version=parseVersion(JSON.parse(original).version).base.join('.')+'-beta.999';
const tmp=fs.mkdtempSync(path.join(os.tmpdir(),'scale-beta-metadata-'));
try {
  const files=execFileSync('git',['-c',`safe.directory=${root}`,'ls-files','-z'],{cwd:root,encoding:'utf8'}).split('\0').filter(Boolean);
  for(const file of files){
    const source=path.join(root,file),target=path.join(tmp,file);
    if(fs.lstatSync(source).isSymbolicLink())throw new Error(`Unsupported release snapshot symlink: ${file}`);
    fs.mkdirSync(path.dirname(target),{recursive:true});fs.copyFileSync(source,target);
  }
  // Execute the actual preparation implementation, not a mocked Cargo adapter.
  for(const args of [[path.join(root,'scripts/release/prepare-release.mjs'),version],[path.join(root,'scripts/ci/check-version-sync.mjs'),'--tag',`v${version}`]])
    execFileSync(process.execPath,args,{cwd:tmp,stdio:'inherit'});
  assert.equal(fs.readFileSync(path.join(root,'package.json'),'utf8'),original);
  console.log(`Actual beta preparation passed in isolated snapshot (${version}); no tag, commit or release created.`);
} finally { fs.rmSync(tmp,{recursive:true,force:true}); }
