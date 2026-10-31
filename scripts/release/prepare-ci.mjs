import fs from 'node:fs';
import { execFileSync as run } from 'node:child_process';
import { planVersion } from './version-plan.mjs';
const git = (...args) => run('git', args, { encoding: 'utf8' }).trim();
const branch = process.env.GITHUB_REF_NAME;
const p = planVersion(branch, JSON.parse(fs.readFileSync('package.json')).version, git('tag','--list').split('\n'));
const source = git('rev-parse','HEAD');
if (p.existing) {
  // Never move or overwrite a published tag; a dispatch can retry its exact build.
  console.log(`Reusing immutable ${p.tag}`);
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
