import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
const workflow=fs.readFileSync(new URL('../../../.github/workflows/release.yml',import.meta.url),'utf8');
const code=workflow.match(/<<'NODE'\n([\s\S]*?)\n          NODE/)[1].replace(/^          /gm,'');
test('actual Windows packaging workflow emits numeric MSI override without changing product SemVer',()=>{
 const tmp=fs.mkdtempSync(path.join(os.tmpdir(),'scale-msi-'));
 try {for(const [version,expected] of [['0.3.0','0.3.0.0'],['0.3.0-beta.1','0.3.0.1']]){
  const result=spawnSync(process.execPath,['--input-type=module','-e',code],{env:{...process.env,RUNNER_TEMP:tmp,RELEASE_VERSION:version},encoding:'utf8'});
  assert.equal(result.status,0,result.stderr);assert.deepEqual(JSON.parse(fs.readFileSync(path.join(tmp,'scale-msi-config.json'))),{bundle:{windows:{wix:{version:expected}}}});
 }
 for(const version of ['256.0.0','0.256.0','0.0.65536','0.3.0-beta.65536','bogus'])assert.notEqual(spawnSync(process.execPath,['--input-type=module','-e',code],{env:{...process.env,RUNNER_TEMP:tmp,RELEASE_VERSION:version}}).status,0);
 }finally{fs.rmSync(tmp,{recursive:true,force:true});}
});
test('resume is explicit and builds exact tag with checksummed asset reuse',()=>{assert.match(workflow,/resume_tag:/);assert.match(workflow,/RELEASE_RESUME_TAG: \$\{\{ inputs.resume_tag \}\}/);assert.match(workflow,/ref: \$\{\{ needs.prepare.outputs.tag \}\}/);assert.match(workflow,/Digest mismatch/);assert.match(workflow,/--prerelease --latest=false/);});
