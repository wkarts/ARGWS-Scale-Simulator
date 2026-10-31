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
  git('add','VERSION','package.json','package-lock.json','src-tauri/Cargo.toml','src-tauri/Cargo.lock','src-tauri/tauri.conf.json','src/config/projectConfig.ts');
  if (git('diff','--cached','--name-only')) git('commit','-m',`chore(release): ${p.version}`);
  git('tag','-a',p.tag,'-m',`Scale Simulator ${p.version}\nSource: ${source}`);
  git('push','origin',`refs/tags/${p.tag}`);
}
fs.appendFileSync(process.env.GITHUB_OUTPUT,`tag=${p.tag}\nversion=${p.version}\nprerelease=${p.prerelease}\n`);
