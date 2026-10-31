import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
const root = path.resolve(import.meta.dirname, '..');
test('Tauri dependency guard accepts the distributed locks', () => {
  const result = spawnSync(process.execPath, ['scripts/ci/check-tauri-versions.mjs'], { cwd: root, encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr);
});
test('Tauri wrapper forwards --locked to Cargo without consuming app arguments', { skip: process.platform === 'win32' }, () => {
  const temp = fs.mkdtempSync(path.join(os.tmpdir(), 'scale-tauri-wrapper-'));
  try {
    fs.mkdirSync(path.join(temp, 'scripts/ci'), { recursive: true });
    fs.mkdirSync(path.join(temp, 'node_modules/@tauri-apps/cli'), { recursive: true });
    fs.copyFileSync(path.join(root, 'scripts/ci/run-tauri.mjs'), path.join(temp, 'scripts/ci/run-tauri.mjs'));
    const cargo = path.join(temp, 'cargo');
    fs.writeFileSync(cargo, '#!/bin/sh\necho cargo 1.0.0\n', { mode: 0o755 });
    fs.writeFileSync(path.join(temp, 'node_modules/@tauri-apps/cli/tauri.js'), 'console.log("ARGS="+JSON.stringify(process.argv.slice(2)))');
    for (const [input, expected] of [
      [['build', '--debug'], ['build', '--debug', '--features', 'desktop', '--', '--locked']],
      [['dev', '--', '--offline', '--', '--mode=cli'], ['dev', '--features', 'desktop', '--', '--locked', '--offline', '--', '--mode=cli']],
      [['info'], ['info']],
    ]) {
      const result = spawnSync(process.execPath, ['scripts/ci/run-tauri.mjs', ...input], { cwd: temp, env: { ...process.env, CARGO: cargo }, encoding: 'utf8' });
      assert.equal(result.status, 0, result.stderr);
      assert.deepEqual(JSON.parse(result.stdout.split('ARGS=')[1]), expected);
    }
  } finally { fs.rmSync(temp, { recursive: true, force: true }); }
});
