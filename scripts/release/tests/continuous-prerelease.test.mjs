import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {continuousPlan,replaceContinuous} from '../continuous-prerelease.mjs';
const base={branch:'develop',version:'0.3.0',source:'a'.repeat(40),previousSource:'b'.repeat(40),oldRef:'c'.repeat(40),isAncestor:true,release:{id:10,tag_name:'v0.3.0',prerelease:true,immutable:false,assets:[{id:20,name:'Scale-Simulator-0.3.0-web.zip',digest:'sha256:'+'d'.repeat(64)}]}};
test('continuous develop retains one numeric version and archives previous generation',()=>{assert.deepEqual(continuousPlan(base),{tag:'v0.3.0',version:'0.3.0',prerelease:true,existing:true,replacing:true,backupPrefix:'previous-'+base.oldRef+'-'});});
test('stable, immutable, unrelated, backward and ambiguous updates fail closed',()=>{for(const change of [{branch:'main'},{version:'0.3.0-beta.1'},{isAncestor:false},{source:base.previousSource},{previousSource:''},{release:{...base.release,prerelease:false}},{release:{...base.release,immutable:true}},{release:{...base.release,tag_name:'v0.2.1'}}])assert.throws(()=>continuousPlan({...base,...change}));});
test('previous bytes are renamed in draft before leased tag update, never deleted during preparation',()=>{
 const calls=[]; const old=process.env.GITHUB_REPOSITORY; process.env.GITHUB_REPOSITORY='wkarts/ARGWS-Scale-Simulator';
 try { const result=replaceContinuous(base,{run:(cmd,args,options)=>{calls.push({cmd,args,input:options.input});if(cmd==='gh'&&args[0]==='api'&&!args.includes('PATCH'))return JSON.stringify([[base.release]]);if(cmd==='gh'&&args.includes('PATCH'))return '{}';if(cmd==='git'&&args[0]==='ls-remote')return base.oldRef+'\trefs/tags/v0.3.0';return '';}});
 assert.equal(result.tag,'v0.3.0');
 const draft=calls.findIndex(c=>c.input&&JSON.parse(c.input).draft===true),backup=calls.findIndex(c=>c.args[0]==='release'&&c.args[1]==='upload'),rename=calls.findIndex(c=>c.input&&JSON.parse(c.input).name),push=calls.findIndex(c=>c.cmd==='git'&&c.args[0]==='push');
 assert(draft>=0&&backup>draft&&rename>backup&&push>rename);
 assert(calls[push].args.includes('--force-with-lease=refs/tags/v0.3.0:'+base.oldRef));
 assert(!calls.some(c=>c.args.includes('DELETE')||c.args.includes('--clobber')));
 } finally { if(old===undefined)delete process.env.GITHUB_REPOSITORY;else process.env.GITHUB_REPOSITORY=old; }
});
test('publication collects only current generation and verifies remote hashes before archive deletion',()=>{
 const workflow=fs.readFileSync('.github/workflows/release.yml','utf8'),finalize=fs.readFileSync('scripts/release/finalize-continuous.mjs','utf8');
 assert.match(workflow,/--pattern "Scale-Simulator-\*"/);
 const upload=fs.readFileSync('scripts/release/upload-draft-assets.mjs','utf8');
 assert.match(upload,/await import\('\.\/finalize-continuous.mjs'\)/);
 assert.match(upload,/state.isPrerelease/);
 assert(finalize.indexOf('remote.digest')<finalize.indexOf("'DELETE'"));
 assert.match(finalize,/manifest.commit!==head/);
 assert.match(finalize,/release.draft/);
});

test('release manifest requires every installer format and excludes mixed generations',async()=>{
 const os=await import('node:os'),path=await import('node:path'),{spawnSync}=await import('node:child_process');
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'scale-matrix-'));
 const output=path.join(root,'release-assets');fs.mkdirSync(output);
 const names=[];
 for(const [target,suffixes] of Object.entries({'x86_64-pc-windows-msvc':['.exe','.msi'],'i686-pc-windows-msvc':['.exe','.msi'],'x86_64-unknown-linux-gnu':['.AppImage','.deb','.rpm'],'aarch64-apple-darwin':['.dmg','.app.tar.gz'],'x86_64-apple-darwin':['.dmg','.app.tar.gz']}))for(const suffix of suffixes)names.push(`Scale-Simulator-0.3.0-${target}${suffix}`);
 names.push('Scale-Simulator-0.3.0-web.zip','Scale-Simulator-0.3.0-sources.zip');
 const script=path.resolve('scripts/release/checksums.py');
 try {
  for(const name of names)fs.writeFileSync(path.join(output,name),'synthetic bundle');
  // A local disposable repository provides the real manifest commit.
  for(const args of [['init','-q'],['-c','user.name=Fixture','-c','user.email=fixture@example.invalid','commit','--allow-empty','-qm','fixture']]){const r=spawnSync('git',args,{cwd:root,encoding:'utf8'});assert.equal(r.status,0,r.stderr);}
  const execute=()=>spawnSync('python',[script],{cwd:root,encoding:'utf8',env:{...process.env,RELEASE_VERSION:'0.3.0'}});
  assert.equal(execute().status,0);
  fs.unlinkSync(path.join(output,names[0]));assert.notEqual(execute().status,0);
  fs.writeFileSync(path.join(output,names[0]),'synthetic bundle');
  const archived=path.join(output,'previous-'+base.oldRef+'-old.exe');fs.writeFileSync(archived,'old');assert.equal(execute().status,0);assert.equal(fs.existsSync(archived),false);
  fs.writeFileSync(path.join(output,'Scale-Simulator-0.2.1-web.zip'),'wrong version');assert.notEqual(execute().status,0);
 } finally {fs.rmSync(root,{recursive:true,force:true});}
});
