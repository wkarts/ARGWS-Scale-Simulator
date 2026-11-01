import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import { execFileSync as run } from 'node:child_process';
const tag = process.env.RELEASE_TAG;
if (!/^v\d+\.\d+\.\d+(?:-beta\.\d+)?$/.test(tag || '')) throw new Error('Invalid release tag');
const gh = (...args) => run('gh', args, { encoding: 'utf8' });
const release = JSON.parse(gh('release','view',tag,'--json','isDraft,assets'));
const hash = file => crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
for (const name of fs.readdirSync('release-assets').sort()) {
  const file = path.join('release-assets', name);
  if (!fs.statSync(file).isFile()) continue;
  if (release.assets.some(asset => asset.name === name)) {
    const directory = fs.mkdtempSync(path.join(os.tmpdir(),'scale-asset-'));
    try {
      gh('release','download',tag,'--pattern',name,'--dir',directory);
      if (hash(file) !== hash(path.join(directory,name))) throw new Error(`Remote asset differs: ${name}; publish a new version`);
      console.log(`Identical existing asset: ${name}`);
    } finally { fs.rmSync(directory,{recursive:true,force:true}); }
  } else {
    if (!release.isDraft) throw new Error('Published releases are immutable');
    gh('release','upload',tag,file); // Never use --clobber.
  }
}

// workflow_run uses the default branch workflow definition. Finish through this
// checked-out source script as well, so an older workflow can safely publish it.
if (release.isDraft && fs.existsSync('release-assets/RELEASE-MANIFEST.json') && fs.existsSync('release-assets/SHA256SUMS.txt')) {
  const state=JSON.parse(gh('release','view',tag,'--json','isPrerelease'));
  if (state.isPrerelease) { process.env.TAG=tag; await import('./finalize-continuous.mjs'); }
}
