import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const root=new URL('../../../',import.meta.url);
test('every npm command in release prepare exists and Tauri compatibility gate runs before publication',()=>{
 const pkg=JSON.parse(fs.readFileSync(new URL('package.json',root)));
 const workflow=fs.readFileSync(new URL('.github/workflows/release.yml',root),'utf8');
 const prepare=workflow.split('\n  desktop:')[0];
 for(const match of prepare.matchAll(/npm run ([\w:-]+)/g))assert.equal(typeof pkg.scripts[match[1]],'string',`Missing release command ${match[1]}`);
 assert.equal(pkg.scripts['ci:tauri'],'node ./scripts/ci/check-tauri-versions.mjs');
 execFileSync(process.execPath,['scripts/ci/check-tauri-versions.mjs'],{cwd:root,stdio:'pipe'});
 const ci=fs.readFileSync(new URL('.github/workflows/ci.yml',root),'utf8');
 assert.match(ci,/npm run ci:version && npm run ci:tauri/);
});
