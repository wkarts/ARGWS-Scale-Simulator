import fs from 'node:fs';
import { replaceContinuous } from './continuous-prerelease.mjs';
import { execFileSync as run } from 'node:child_process';
import { releaseSource, planForSource } from './source-plan.mjs';
const git = (...args) => run('git', args, { encoding: 'utf8' }).trim();
const source = git('rev-parse','HEAD');
const event = JSON.parse(fs.readFileSync(process.env.GITHUB_EVENT_PATH, 'utf8'));
const { branch } = releaseSource(process.env, event, source);
const tags = git('tag','--list').split('\n').filter(Boolean);
const annotations = {};
for (const tag of tags.filter(tag => /^v\d+\.\d+\.\d+$/.test(tag))) {
  const type = git('cat-file','-t',`refs/tags/${tag}`);
  const parent = type === 'tag' ? (git('rev-list','--parents','-n','1',tag).split(' ')[1] || '') : '';
  const metadata = new Set(['VERSION','package.json','package-lock.json','src-tauri/Cargo.toml','src-tauri/Cargo.lock','src-tauri/tauri.conf.json','src/config/projectConfig.ts','src/assets/branding/brand.json']);
  const commit = git('rev-parse',`${tag}^{commit}`);
  const metadataOnly = (parent === source || commit === source) && git('diff','--name-only',source,tag).split('\n').filter(Boolean).every(file => metadata.has(file));
  annotations[tag] = { type, parent, commit, metadataOnly, message: type === 'tag' ? git('tag','-l',tag,'--format=%(contents)') : '' };
}
const resume = process.env.RELEASE_RESUME_TAG || '';
let p;
if (resume) {
  if (process.env.GITHUB_EVENT_NAME !== 'workflow_dispatch' || branch !== 'develop' || !/^v\d+\.\d+\.\d+$/.test(resume) || !tags.includes(resume)) throw new Error('Resume requires an existing numeric tag and an explicit develop dispatch');
  const version = JSON.parse(git('show',`${resume}:package.json`)).version;
  if (`v${version}` !== resume) throw new Error('Resume tag/version mismatch');
  const release = JSON.parse(run('gh',['release','view',resume,'--json','isDraft,isPrerelease'],{encoding:'utf8'}));
  if (!release.isDraft || !release.isPrerelease) throw new Error('Resume accepts only an existing draft pre-release');
  p = {tag: resume, version, prerelease: true, existing: true};
} else {
  const version = JSON.parse(fs.readFileSync('package.json')).version;
  try { p = planForSource(branch, version, tags, source, annotations); }
  catch (error) {
    if (branch !== 'develop' || !String(error.message).startsWith('Immutable ')) throw error;
    const tag = `v${version}`, annotation = annotations[tag];
    const markers = (annotation?.message || '').split('\n').filter(line=>line.startsWith('Source: '));
    if (annotation?.type !== 'tag' || markers.length !== 1) throw error;
    const previousSource = markers[0].slice(8);
    let isAncestor = false;
    try { git('merge-base','--is-ancestor',previousSource,source); isAncestor = true; } catch {}
    p = replaceContinuous({branch,version,source,previousSource,oldRef:git('rev-parse',`refs/tags/${tag}`),isAncestor});
  }
}
if (branch === 'develop' && !p.prerelease) throw new Error('Develop must remain a GitHub pre-release');
console.log(`Verified source ${branch}@${source}; target ${p.tag}; prerelease=${p.prerelease}`);
if (p.existing) {
  // Same-source retries reuse only the current draft generation.
  console.log(`Using verified ${p.tag}`);
} else {
  run(process.execPath,['scripts/release/prepare-release.mjs',p.version],{stdio:'inherit'});
  run(process.execPath,['scripts/ci/check-version-sync.mjs','--tag',p.tag],{stdio:'inherit'});
  git('config','user.name','github-actions[bot]');
  git('config','user.email','41898282+github-actions[bot]@users.noreply.github.com');
  git('add','VERSION','package.json','package-lock.json','src-tauri/Cargo.toml','src-tauri/Cargo.lock','src-tauri/tauri.conf.json','src/config/projectConfig.ts', 'src/assets/branding/brand.json');
  if (git('diff','--cached','--name-only')) git('commit','-m',`chore(release): ${p.version}`);
  git('tag','-a',p.tag,'-m',`Scale Simulator ${p.version}\nSource: ${source}`);
  git('push','origin',`refs/tags/${p.tag}`);
}
// Stage binaries in a draft release, not in the account's Actions artifact quota.
const gh = (...args) => run('gh', args, { encoding: 'utf8' });
let existingRelease;
try { existingRelease = JSON.parse(gh('release','view',p.tag,'--json','isDraft')); }
catch (error) {
  // Only a confirmed not-found permits creation. Authentication/network errors stop.
  const stderr = String(error.stderr || '');
  if (!/release not found|HTTP 404/i.test(stderr)) throw error;
  gh('release','create',p.tag,'--verify-tag','--draft','--title',`Scale Simulator ${p.tag}`,'--generate-notes',...(p.prerelease ? ['--prerelease'] : []));
}
fs.appendFileSync(process.env.GITHUB_OUTPUT,`tag=${p.tag}\nversion=${p.version}\nprerelease=${p.prerelease}\n`);
