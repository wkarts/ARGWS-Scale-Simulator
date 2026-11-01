import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {releaseSource,planForSource} from '../source-plan.mjs';
import {validateCargoMetadata} from '../cargo-metadata.mjs';
const sha='a'.repeat(40),repo='wkarts/ARGWS-Scale-Simulator';
const event={workflow_run:{event:'push',conclusion:'success',head_repository:{full_name:repo},head_branch:'develop',head_sha:sha}};
const env={GITHUB_EVENT_NAME:'workflow_run',GITHUB_REF_NAME:'main',GITHUB_REPOSITORY:repo};
test('default main workflow publishes the verified develop source only as beta',()=>{const source=releaseSource(env,event,sha);assert.equal(source.branch,'develop');const p=planForSource(source.branch,'0.3.0',['v0.2.1'],sha,{});assert.equal(p.version,'0.3.0');assert.equal(p.prerelease,true);});
test('wrong checkout, repository and conflicting declared branch are rejected',()=>{assert.throws(()=>releaseSource(env,event,'b'.repeat(40)));assert.throws(()=>releaseSource({...env,GITHUB_REPOSITORY:'other/repo'},event,sha));assert.throws(()=>releaseSource({...env,RELEASE_SOURCE_BRANCH:'main'},event,sha));});
test('manual dispatch uses its actual branch and SHA',()=>{assert.equal(releaseSource({GITHUB_EVENT_NAME:'workflow_dispatch',GITHUB_REF_NAME:'develop',GITHUB_SHA:sha},{},sha).branch,'develop');});
test('retry reuses the immutable beta tag from the exact source',()=>{const tag='v0.3.0';const p=planForSource('develop','0.3.0',['v0.2.1',tag],sha,{[tag]:{type:'tag',parent:sha,metadataOnly:true,message:`Scale Simulator\nSource: ${sha}`}});assert.equal(p.tag,tag);assert.equal(p.existing,true);});
test('different source or lightweight tag cannot claim previous beta assets',()=>{const tag='v0.3.0';for(const a of [{type:'tag',message:'Source: '+'b'.repeat(40)},{type:'commit',message:'Source: '+sha}])assert.throws(()=>planForSource('develop','0.3.0',['v0.2.1',tag],sha,{[tag]:a}),/Immutable/);});
test('large real child stdout does not overflow metadata capture',()=>{validateCargoMetadata(process.cwd(),(command,args,options)=>{assert(args.includes('--locked'));return spawnSync(process.execPath,['-e','process.stdout.write("x".repeat(3*1024*1024))'],options);});});
test('real metadata errors remain fatal with diagnostic cause',()=>{assert.throws(()=>validateCargoMetadata(process.cwd(),()=>({status:null,signal:'SIGTERM',error:{code:'ENOBUFS',message:'buffer'},stderr:'warnings'})),/ENOBUFS/);assert.throws(()=>validateCargoMetadata(process.cwd(),()=>({status:1,stderr:'lock mismatch'})),/lock mismatch/);});

test('source marker alone cannot reuse an unrelated or modified code tag',()=>{const tag='v0.3.0';for(const extra of [{parent:'b'.repeat(40),metadataOnly:true},{parent:sha,metadataOnly:false}]){assert.throws(()=>planForSource('develop','0.3.0',['v0.2.1',tag],sha,{[tag]:{type:'tag',message:`Source: ${sha}`,...extra}}),/Immutable/);}});
