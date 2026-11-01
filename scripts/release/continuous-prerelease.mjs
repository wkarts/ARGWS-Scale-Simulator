import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {execFileSync} from 'node:child_process';

export function findRelease(run, repo, tag) {
  const pages=JSON.parse(run('gh',['api',`repos/${repo}/releases?per_page=100`,'--paginate','--slurp'],{encoding:'utf8'}));
  const release=pages.flat().find(item=>item.tag_name===tag);
  if (!release) throw new Error(`Release not found: ${tag}`);
  return release;
}

export function continuousPlan({branch, version, source, previousSource, release, oldRef, isAncestor}) {
  if (branch !== 'develop' || !/^\d+\.\d+\.\d+$/.test(version)) throw new Error('Continuous updates require develop and plain SemVer');
  if (!release?.prerelease || release.immutable || release.tag_name !== `v${version}`) throw new Error('Stable or immutable releases cannot be replaced');
  if (![source,previousSource,oldRef].every(value=>/^[a-f0-9]{40}$/.test(value||'')) || !isAncestor) throw new Error('Continuous update requires verified forward source lineage');
  if (source === previousSource) throw new Error('Same source must resume rather than replace');
  return {tag:`v${version}`, version, prerelease:true, existing:true, replacing:true, backupPrefix:`previous-${oldRef}-`};
}

// Keep the previous bytes in the SAME draft until all new platforms are validated.
// Never create another release/tag, overwrite stable assets, or force-push a branch.
export function replaceContinuous({branch,version,source,previousSource,oldRef,isAncestor}, dependencies={}) {
  const run=dependencies.run || execFileSync;
  const git=(...args)=>run('git',args,{encoding:'utf8'}).trim();
  const gh=(...args)=>run('gh',args,{encoding:'utf8'});
  const api=(endpoint,body)=>JSON.parse(run('gh',['api',endpoint,'--method','PATCH','--input','-'],{encoding:'utf8',input:JSON.stringify(body)}));
  const repo=process.env.GITHUB_REPOSITORY;
  if (!/^[\w.-]+\/[\w.-]+$/.test(repo||'')) throw new Error('Invalid repository');
  const release=findRelease(run,repo,`v${version}`);
  const plan=continuousPlan({branch,version,source,previousSource,release,oldRef,isAncestor});
  if (git('ls-remote','origin',`refs/tags/${plan.tag}`).split(/\s/)[0]!==oldRef) throw new Error('Remote tag changed before preparation');
  const unarchived=release.assets.filter(asset=>!asset.name.startsWith('previous-'));
  for (const asset of unarchived) {
    if (!/^[A-Za-z0-9_. -]+$/.test(asset.name) || !/^sha256:[a-f0-9]{64}$/.test(asset.digest||'')) throw new Error('Asset lacks a safe name or SHA256 digest');
  }
  api(`repos/${repo}/releases/${release.id}`,{draft:true,prerelease:true});
  const directory=fs.mkdtempSync(path.join(os.tmpdir(),'scale-previous-'));
  const manifest=path.join(directory,`${plan.backupPrefix}manifest.json`);
  try {
    fs.writeFileSync(manifest,JSON.stringify({tag:plan.tag,oldRef,previousSource,nextSource:source,assets:release.assets},null,2)+'\n');
    if (!release.assets.some(asset=>asset.name===path.basename(manifest))) gh('release','upload',plan.tag,manifest);
  } finally { fs.rmSync(directory,{recursive:true,force:true}); }
  for (const asset of unarchived) api(`repos/${repo}/releases/assets/${asset.id}`,{name:plan.backupPrefix+asset.name});
  git('config','user.name','github-actions[bot]');
  git('config','user.email','41898282+github-actions[bot]@users.noreply.github.com');
  git('tag','-f','-a',plan.tag,'-m',`Scale Simulator ${version}\nSource: ${source}\nChannel: continuous-develop`);
  git('push',`--force-with-lease=refs/tags/${plan.tag}:${oldRef}`,'origin',`refs/tags/${plan.tag}`);
  return plan;
}
