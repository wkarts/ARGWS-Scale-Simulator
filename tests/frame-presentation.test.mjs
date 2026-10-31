import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import ts from 'typescript';
const source = fs.readFileSync(new URL('../src/utils/framePresentation.ts', import.meta.url), 'utf8');
const compiled = ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2020, module: ts.ModuleKind.ES2020 } }).outputText;
const { frameHex } = await import(`data:text/javascript;base64,${Buffer.from(compiled).toString('base64')}`);
test('HEX exposes NUL, STX, CR, LF and checksum bytes', () => {
  assert.equal(frameHex('\x02\x00123\r\n\x7f'), '02 00 31 32 33 0D 0A 7F');
});
test('empty frame is explicitly unavailable', () => { assert.equal(frameHex(''), '—'); });
test('HEX matches actual UTF-8 bytes rather than UTF-16 code units', () => {
  assert.equal(frameHex('é'), 'C3 A9');
});
test('preview HEX is gated by successful current response', () => {
  const page = fs.readFileSync(new URL('../src/pages/ScaleSimulatorPage.vue', import.meta.url), 'utf8');
  assert.match(page, /v-if="previewValid"/);
  assert.match(page, /const request = \+\+previewRequest/);
  assert.match(page, /if \(request !== previewRequest\) return/);
  assert.match(page, /catch \(err\) \{[\s\S]*?previewValid.value = false/);
  assert.match(page, /frameHex\(selectedSimulator.last_frame\)/);
});
