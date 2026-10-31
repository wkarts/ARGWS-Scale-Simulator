import { test } from 'node:test';
import assert from 'node:assert/strict';
import { planVersion, compare, parseVersion } from '../version-plan.mjs';
test('first release preserves current 0.1.11', () => assert.equal(planVersion('main','0.1.11',[]).tag,'v0.1.11'));
test('stable is idempotent', () => assert.equal(planVersion('main','0.1.11',['v0.1.11']).existing,true));
test('develop always exceeds stable', () => {
  const p=planVersion('develop','0.1.11',['v0.1.11','v0.1.12-beta.2']);
  assert.equal(p.version,'0.1.12-beta.3'); assert.ok(compare(p.version,'0.1.11')>0); assert.equal(p.prerelease,true);
});
test('future minor retained', () => assert.equal(planVersion('develop','0.2.0',['v0.1.11']).version,'0.2.0-beta.1'));
test('numeric ordering', () => assert.ok(compare('0.1.12-beta.10','0.1.12-beta.9')>0));
test('reject invalid version/branch/regression', () => {
 assert.throws(()=>parseVersion('01.1.2')); assert.throws(()=>planVersion('feature/x','0.1.11',[]));
 assert.throws(()=>planVersion('main','0.1.10',['v0.1.11'])); assert.throws(()=>planVersion('main','0.2.0-beta.1',[]));
});

test('initial beta also exceeds current version', () => assert.equal(planVersion('develop','0.1.11',[]).version,'0.1.12-beta.1'));
