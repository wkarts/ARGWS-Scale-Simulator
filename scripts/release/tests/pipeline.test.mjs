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

test('Tauri wrapper owns the Cargo locked argument; workflow must not pass it as a Tauri option',()=>{
 const workflow=read('.github/workflows/release.yml');
 assert.match(workflow,/npm run tauri -- build --target/);
 assert.doesNotMatch(workflow,/npm run tauri -- build --locked/);
 assert.match(workflow,/asset.digest/);
 assert.match(workflow,/Digest mismatch/);
});

test('draft lookup uses a collection which includes drafts, not the published-tag endpoint', async()=>{
 const workflow=read('.github/workflows/release.yml');
 assert.doesNotMatch(workflow,/getReleaseByTag/);
 const script=workflow.split('          script: |\n')[1].split('\n      - ')[0].split('\n').map(line=>line.replace(/^ {12}/,'')).join('\n');
 const AsyncFunction=Object.getPrototypeOf(async function(){}).constructor;
 const crypto=await import('node:crypto');
 const body=Buffer.from('verified existing ZIP');
 const digest='sha256:'+crypto.createHash('sha256').update(body).digest('hex');
 const names=['web','sources'].map(kind=>`Scale-Simulator-0.1.11-${kind}.zip`);
 const assets=names.map(name=>({name,state:'uploaded',digest}));
 const outputs={};const downloads=[];
 const deps={
  'node:fs':{mkdirSync(){},readFileSync(){return body;}},
  'node:crypto':crypto,
  'node:child_process':{execFileSync(command,args){assert.equal(command,'gh');downloads.push(args);}},
 };
 const execute=new AsyncFunction('require','github','context','core','process',script);
 const github={rest:{repos:{listReleases(){}}},paginate:async()=>[{tag_name:'v0.1.11',draft:true,assets}]};
 await execute(name=>deps[name],github,{repo:{owner:'fixture',repo:'fixture'}},{setOutput:(k,v)=>outputs[k]=v},{env:{TAG:'v0.1.11',VERSION:'0.1.11'}});
 assert.equal(outputs.restored,'true');assert.equal(downloads.length,2);
 assets[0].digest='sha256:'+'0'.repeat(64);
 await assert.rejects(()=>execute(name=>deps[name],github,{repo:{}},{setOutput(){}},{env:{TAG:'v0.1.11',VERSION:'0.1.11'}}),/Digest mismatch/);
 for (const [target, suffixes] of [
  ['x86_64-pc-windows-msvc',['.exe','.msi']],
  ['x86_64-unknown-linux-gnu',['.AppImage','.deb','.rpm']],
  ['aarch64-apple-darwin',['.dmg','.app.tar.gz']],
 ]) {
  assets.splice(0,assets.length,...suffixes.map(suffix=>({name:`Scale-Simulator-0.1.11-${target}${suffix}`,state:'uploaded',digest})));
  const prior=downloads.length;
  await execute(name=>deps[name],github,{repo:{}},{setOutput:(k,v)=>outputs[k]=v},{env:{TAG:'v0.1.11',VERSION:'0.1.11',TARGET:target}});
  assert.equal(outputs.restored,'true');assert.equal(downloads.length-prior,suffixes.length);
  assets.pop();
  await execute(name=>deps[name],github,{repo:{}},{setOutput:(k,v)=>outputs[k]=v},{env:{TAG:'v0.1.11',VERSION:'0.1.11',TARGET:target}});
  assert.equal(outputs.restored,'false');
 }

});
