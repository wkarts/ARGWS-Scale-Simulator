import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {planVersion} from './version-plan.mjs';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const tmp=fs.mkdtempSync(path.join(os.tmpdir(),'scale-release-source-'));
const work=path.join(tmp,'work'),remote=path.join(tmp,'remote.git'),bin=path.join(tmp,'bin');
const original=fs.readFileSync(path.join(root,'package.json'),'utf8');
try {
 fs.mkdirSync(work);fs.mkdirSync(bin);
 const files=execFileSync('git',['-c',`safe.directory=${root}`,'ls-files','-z'],{cwd:root,encoding:'utf8'}).split('\0').filter(Boolean);
 for(const file of files){const src=path.join(root,file),dest=path.join(work,file);if(fs.lstatSync(src).isSymbolicLink())throw new Error('Snapshot symlink rejected');fs.mkdirSync(path.dirname(dest),{recursive:true});fs.copyFileSync(src,dest);}
 const git=(...args)=>execFileSync('git',args,{cwd:work,encoding:'utf8',stdio:['ignore','pipe','pipe']}).trim();
 git('init','-q');git('config','user.name','Release fixture');git('config','user.email','release-fixture@example.invalid');git('add','.');git('commit','--no-gpg-sign','-qm','Synthetic source');
 const source=git('rev-parse','HEAD');git('tag','v0.0.0');git('init','--bare','-q',remote);git('remote','add','origin',remote);
 const expected=planVersion('develop',JSON.parse(original).version,['v0.0.0']);
 const calls=path.join(tmp,'gh-calls.jsonl'),marker=path.join(tmp,'draft-created');
 const fake=path.join(bin,'gh');
 fs.writeFileSync(fake,`#!/usr/bin/env node\nconst fs=require('fs');const args=process.argv.slice(2);fs.appendFileSync(process.env.FAKE_GH_CALLS,JSON.stringify(args)+'\\n');if(args[0]==='release'&&args[1]==='view'){if(fs.existsSync(process.env.FAKE_GH_MARKER)){console.log(JSON.stringify({isDraft:true}));}else{console.error('HTTP 404: release not found');process.exit(1);}}else if(args[0]==='release'&&args[1]==='create'){fs.writeFileSync(process.env.FAKE_GH_MARKER,'synthetic draft');}else{throw new Error('Unexpected remote operation in local release fixture');}\n`);fs.chmodSync(fake,0o755);
 const event=path.join(tmp,'event.json'),output=path.join(tmp,'outputs');
 fs.writeFileSync(event,JSON.stringify({workflow_run:{event:'push',conclusion:'success',head_repository:{full_name:'wkarts/ARGWS-Scale-Simulator'},head_branch:'develop',head_sha:source}}));
 const env={...process.env,PATH:bin+path.delimiter+process.env.PATH,GITHUB_EVENT_NAME:'workflow_run',GITHUB_REF_NAME:'main',GITHUB_REPOSITORY:'wkarts/ARGWS-Scale-Simulator',GITHUB_EVENT_PATH:event,GITHUB_OUTPUT:output,FAKE_GH_CALLS:calls,FAKE_GH_MARKER:marker};delete env.RELEASE_SOURCE_BRANCH;
 const execute=()=>execFileSync(process.execPath,[path.join(root,'scripts/release/prepare-ci.mjs')],{cwd:work,env,stdio:'inherit'});
 execute();assert(git('tag','-l').split('\n').includes(expected.tag));assert(fs.readFileSync(output,'utf8').includes('prerelease=true'));
 const commit=git('rev-parse',`${expected.tag}^{commit}`);git('checkout','--detach',source);execute();
 assert.equal(git('rev-parse',`${expected.tag}^{commit}`),commit);assert.equal(git('tag','-l').split('\n').length,2);
 const created=fs.readFileSync(calls,'utf8').trim().split('\n').map(JSON.parse).filter(a=>a[0]==='release'&&a[1]==='create');assert.equal(created.length,1);assert(created[0].includes('--prerelease'));assert(created[0].includes('--draft'));
 assert.equal(fs.readFileSync(path.join(root,'package.json'),'utf8'),original);
 console.log('Actual source->beta->retry coordinator passed with real local Git/Cargo and a fake GitHub publisher; no remote publication.');
} finally {fs.rmSync(tmp,{recursive:true,force:true});}
