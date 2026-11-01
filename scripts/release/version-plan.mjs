// Versions are derived from committed product version and immutable remote tags.
export function parseVersion(value) {
  const m = /^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-beta\.(0|[1-9]\d*))?$/.exec(value);
  if (!m) throw new Error(`Unsupported SemVer: ${value}`);
  return { base: m.slice(1, 4).map(Number), beta: m[4] === undefined ? null : Number(m[4]) };
}
export function compare(a, b) {
  a = parseVersion(a); b = parseVersion(b);
  for (let i = 0; i < 3; i++) if (a.base[i] !== b.base[i]) return a.base[i] - b.base[i];
  if (a.beta === b.beta) return 0;
  if (a.beta === null) return 1;
  if (b.beta === null) return -1;
  return a.beta - b.beta;
}
export function planVersion(branch, current, tags) {
  if (!['main', 'develop'].includes(branch)) throw new Error('Releases only from main/develop');
  if (parseVersion(current).beta !== null) throw new Error('Product version must use plain SemVer');
  const valid = tags.filter(t => /^v\d+\.\d+\.\d+$/.test(t));
  const latest = valid.sort(compare).at(-1);
  if (latest && compare(current, latest) < 0) throw new Error('Version cannot regress');
  return {version: current, tag: `v${current}`, prerelease: branch === 'develop', existing: valid.includes(`v${current}`)};
}
