import { planVersion } from './version-plan.mjs';
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
  const plan = planVersion(branch, current, tags);
  if (!plan.existing) return plan;
  const a = annotations[plan.tag];
  const exact = a?.type === 'tag' && a.metadataOnly === true && (a.commit === source || a.parent === source)
    && a.message.split('\n').filter(line => line.startsWith('Source: ')).length === 1
    && a.message.split('\n').includes(`Source: ${source}`);
  if (!exact) throw new Error(`Immutable ${plan.tag} belongs to another source; explicitly resume the existing release instead of moving its tag`);
  return plan;
}
