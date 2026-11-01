import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const read = path => fs.readFileSync(new URL('../' + path, import.meta.url), 'utf8');
const json = path => JSON.parse(read(path));
test('public product identity and versions agree', () => {
  const version = read('VERSION').trim();
  assert.equal(json('package.json').version, version);
  assert.equal(json('package-lock.json').version, version);
  assert.equal(json('src-tauri/tauri.conf.json').version, version);
  assert.equal(json('src/assets/branding/brand.json').version, version);
  assert.equal(json('src-tauri/tauri.conf.json').productName, 'Scale Simulator');
  assert.equal(json('public/manifest.webmanifest').name, 'Scale Simulator');
  assert.match(read('src/config/projectConfig.ts'), new RegExp(`version: "${version.replaceAll('.', '\\.') }"`));
});
test('upgrade preserves application identity and data namespaces', () => {
  assert.equal(json('src-tauri/tauri.conf.json').identifier, 'br.com.wkarts.scale-simulator-tauri');
  assert.match(read('src/config/projectConfig.ts'), /localDataDir: "scale_simulator_tauri"/);
  assert.match(read('src/config/projectConfig.ts'), /storagePrefix: "scale-simulator"/);
});
test('dashboard uses simulator state without administrative placeholder metrics', () => {
  const dashboard = read('src/pages/DashboardPage.vue');
  assert.match(dashboard, /await listScaleSimulators\(\)/);
  assert.match(dashboard, /clearInterval\(timer\)/);
  assert.match(dashboard, /!desktop \|\| !permitted.value/);
  assert.doesNotMatch(dashboard, /stats\.empresas|stats\.sync_pendente|ativo no template|DashboardChartCard/);
});
test('legacy administration routes remain available but extras stay out of primary navigation', () => {
  const navigation = read('src/config/navigation.ts');
  for (const route of ['licenciamento', 'integracoes', 'sincronizacao', 'api-interna']) {
    assert.ok(read('src/router/index.ts').includes(`path: "${route}"`));
    assert.ok(!navigation.includes(`route: "/${route}"`));
  }
});
test('all nine driver identities and Toledo framing selector are preserved', () => {
  const service = read('src/services/scaleSimulator.ts');
  for (const driver of ['wt1000n','wt27r','wt3000ir','toledo8540','toledo_prix3','a12e','lider_ld2052','filizola','generic_ascii']) assert.ok(service.includes(`"${driver}"`));
  assert.match(read('src/pages/ScaleSimulatorPage.vue'), /toledo8540_stop_bits/);
});

test('serial settings are explicit and oscillation controls remain available', () => {
  const page = read('src/pages/ScaleSimulatorPage.vue');
  for (const field of ['serial_data_bits', 'serial_parity', 'serial_stop_bits']) assert.ok(page.includes(`form.${field}`));
  assert.match(page, /form\.value\.serial_data_bits = toledo \? 7 : 8/);
  assert.match(page, /form\.value\.serial_parity = toledo \? "even" : "none"/);
  assert.match(page, /v-model.number="form.variation_kg"/);
  assert.match(page, /runAction\(selectedSimulator.stable \? 'unstable' : 'stable'\)/);
});
test('frame overrides are opt-in and reset when changing model', () => {
  const service = read('src/services/scaleSimulator.ts');
  const page = read('src/pages/ScaleSimulatorPage.vue');
  assert.match(service, /frame_override_enabled: false/);
  assert.match(page, /form\.value\.frame_override_enabled = false/);
  assert.match(page, /form\.value\.frame_prefix_hex = null/);
  assert.match(page, /form\.value\.frame_suffix_hex = null/);
  assert.match(page, /form\.value\.frame_checksum_mode = "protocol"/);
  assert.match(page, /v-if="form.frame_override_enabled"/);
  assert.match(page, /quadros incompatíveis ou intencionalmente inválidos/);
});

// These fields are installer registry keys, not cosmetic branding.
test('Windows upgrade retains the exact publisher and product keys of stable 0.2.1',()=>{
 const config=json('src-tauri/tauri.conf.json');
 assert.equal(config.bundle.publisher,'WK Arts');
 assert.equal(config.productName,'Scale Simulator');
 assert.equal(config.identifier,'br.com.wkarts.scale-simulator-tauri');
 assert.equal(config.bundle.windows.nsis.installMode ?? 'currentUser','currentUser');
 assert.match(read('src-tauri/Cargo.toml'),/^name = "scale_simulator_tauri"$/m);
 assert.equal(config.bundle.shortDescription,'Simulador de balanças TCP e RS232');
});
