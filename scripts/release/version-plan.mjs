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
  const valid = tags.filter(t => /^v\d+\.\d+\.\d+(?:-beta\.\d+)?$/.test(t));
  const stable = valid.filter(t => !t.includes('-')).sort(compare).at(-1);
  let base = parseVersion(current).base.join('.');
  if (branch === 'main') {
    if (parseVersion(current).beta !== null) throw new Error('main must contain a stable version');
    if (stable && compare(base, stable) < 0) throw new Error('Stable version cannot regress');
    return { version: base, tag: `v${base}`, prerelease: false, existing: valid.includes(`v${base}`) };
  }
  if (!stable || compare(base, stable) <= 0) {
    const parts = parseVersion(stable || base).base; parts[2]++; base = parts.join('.');
  }
  const numbers = valid.filter(t => t.startsWith(`v${base}-beta.`)).map(t => parseVersion(t).beta);
  const version = `${base}-beta.${Math.max(0, ...numbers) + 1}`;
  return { version, tag: `v${version}`, prerelease: true, existing: false };
}
