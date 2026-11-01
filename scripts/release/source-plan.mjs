import { planVersion, compare, parseVersion } from './version-plan.mjs';
export function releaseSource(env, event, checkedOutSha) {
  let branch, sha;
  if (env.GITHUB_EVENT_NAME === 'workflow_run') {
    const run = event.workflow_run;
    if (!run || run.event !== 'push' || run.conclusion !== 'success' || run.head_repository?.full_name !== env.GITHUB_REPOSITORY) throw new Error('Untrusted upstream release source');
    branch = run.head_branch; sha = run.head_sha;
  } else if (env.GITHUB_EVENT_NAME === 'workflow_dispatch') {
    branch = env.GITHUB_REF_NAME; sha = env.GITHUB_SHA;
  } else throw new Error('Release source requires workflow_run or workflow_dispatch');
  if (!['main', 'develop'].includes(branch) || !/^[a-f0-9]{40}$/.test(sha || '') || sha !== checkedOutSha) throw new Error('Release source branch/SHA does not match checkout');
  if (env.RELEASE_SOURCE_BRANCH && env.RELEASE_SOURCE_BRANCH !== branch) throw new Error('Declared release branch disagrees with event');
  return { branch, sha };
}
export function planForSource(branch, current, tags, source, annotations) {
  const next = planVersion(branch, current, tags);
  if (branch !== 'develop') return next;
  const base = parseVersion(next.version).base.join('.');
  const matching = tags.filter(tag => tag.startsWith(`v${base}-beta.`) && annotations[tag]?.type === 'tag' && annotations[tag].parent === source && annotations[tag].metadataOnly === true
    && annotations[tag].message.split('\n').filter(line => line.startsWith('Source: ')).length === 1
    && annotations[tag].message.split('\n').includes(`Source: ${source}`)).sort(compare);
  if (!matching.length) return next;
  const tag = matching.at(-1);
  return { tag, version: tag.slice(1), prerelease: true, existing: true };
}
