import fs from 'node:fs';
import {findRelease} from './continuous-prerelease.mjs';
import crypto from 'node:crypto';
import {execFileSync as run} from 'node:child_process';
const gh=(...args)=>run('gh',args,{encoding:'utf8'});
const tag=process.env.TAG,repo=process.env.GITHUB_REPOSITORY;
if (!/^v\d+\.\d+\.\d+$/.test(tag||'') || !/^[\w.-]+\/[\w.-]+$/.test(repo||'')) throw new Error('Invalid release target');
const release=findRelease(run,repo,tag);
if (!release.draft || !release.prerelease || release.immutable) throw new Error('Only draft pre-releases can be finalized');
const manifest=JSON.parse(fs.readFileSync('release-assets/RELEASE-MANIFEST.json','utf8'));
const head=run('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim();
if (`v${manifest.version}`!==tag || manifest.commit!==head) throw new Error('Manifest belongs to another release source');
for (const item of [...manifest.assets,{name:'RELEASE-MANIFEST.json',sha256:crypto.createHash('sha256').update(fs.readFileSync('release-assets/RELEASE-MANIFEST.json')).digest('hex')},{name:'SHA256SUMS.txt',sha256:crypto.createHash('sha256').update(fs.readFileSync('release-assets/SHA256SUMS.txt')).digest('hex')}]) {
  const remote=release.assets.find(asset=>asset.name===item.name);
  if (!remote || remote.state!=='uploaded' || remote.digest!==`sha256:${item.sha256}`) throw new Error(`Unverified new asset: ${item.name}`);
}
// Only after the complete new set is uploaded and verified can archived bytes go.
for (const asset of release.assets.filter(asset=>/^previous-[a-f0-9]{40}-/.test(asset.name))) gh('api',`repos/${repo}/releases/assets/${asset.id}`,'--method','DELETE');
