import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const read = path => fs.readFileSync(new URL('../../../'+path, import.meta.url),'utf8');
test('publication is gated on every platform and stays draft until completion',()=>{
 const workflow=read('.github/workflows/release.yml');
 assert.match(workflow,/needs: \[prepare, desktop, web\]/);
 assert.match(workflow,/workflow_run\.conclusion == 'success'/);
 assert.match(read('scripts/release/prepare-ci.mjs'),/'--draft'/);
 assert.doesNotMatch(workflow,/actions\/(upload|download)-artifact/);
 assert.match(workflow,/--draft=false/);
});
test('immutable assets require exact content match and never clobber',()=>{
 const upload=read('scripts/release/upload-draft-assets.mjs');
 assert.match(upload,/hash\(file\) !== hash\(path.join\(directory,name\)\)/);
 assert.match(upload,/if \(!release.isDraft\) throw/);
 assert.doesNotMatch(upload,/gh\([^\n]*'--clobber'/);
});
test('checksums require all targets, not just any seven files',()=>{
 const script=read('scripts/release/checksums.py');
 for(const target of ['x86_64-pc-windows-msvc','i686-pc-windows-msvc','x86_64-unknown-linux-gnu','aarch64-apple-darwin','x86_64-apple-darwin','-web.zip','-sources.zip'])assert.ok(script.includes(target));
});
