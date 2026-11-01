import {test} from 'node:test';
import assert from 'node:assert/strict';
import {planVersion,compare,parseVersion} from '../version-plan.mjs';
test('main stable preserves current version',()=>assert.deepEqual(planVersion('main','0.2.1',[]),{version:'0.2.1',tag:'v0.2.1',prerelease:false,existing:false}));
test('develop uses plain SemVer and GitHub prerelease flag',()=>assert.deepEqual(planVersion('develop','0.3.0',['v0.2.1','v0.3.0-beta.1']),{version:'0.3.0',tag:'v0.3.0',prerelease:true,existing:false}));
test('repeated release never increments version or creates beta tags',()=>{for(const branch of ['main','develop']){const p=planVersion(branch,'0.3.0',['v0.2.1','v0.3.0']);assert.equal(p.tag,'v0.3.0');assert.equal(p.existing,true);}});
test('no implicit version bump on initial develop release',()=>assert.equal(planVersion('develop','0.3.0',[]).version,'0.3.0'));
test('reject malformed version, unsupported branch and regression',()=>{assert.throws(()=>parseVersion('01.1.2'));assert.throws(()=>planVersion('feature/x','0.3.0',[]));assert.throws(()=>planVersion('main','0.2.0',['v0.2.1']));assert.throws(()=>planVersion('develop','0.3.0-beta.1',[]));assert.ok(compare('0.3.0','0.2.1')>0);});
