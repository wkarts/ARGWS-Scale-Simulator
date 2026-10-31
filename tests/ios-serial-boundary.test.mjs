import test from 'node:test';import assert from 'node:assert/strict';import fs from 'node:fs';
test('iOS excludes desktop serial backend without changing desktop feature availability',()=>{
 const cargo=fs.readFileSync('src-tauri/Cargo.toml','utf8');assert.match(cargo,/\[target\.'cfg\(not\(target_os = "ios"\)\)'\.dependencies\]\s*serialport = "4\.6"/);
 const source=fs.readFileSync('src-tauri/src/commands/scale_simulator.rs','utf8');
 for(const name of ['list_serial_ports_with_usage','serial_builder','preflight_serial_port','run_serial_loop'])assert.ok(source.includes('#[cfg(not(target_os = "ios"))]\nfn '+name+'('));
 assert.match(source,/#\[cfg\(target_os = "ios"\)\]\nfn preflight_serial_port[\s\S]*RS232 local não está disponível/);
 assert.equal(JSON.parse(fs.readFileSync('src-tauri/tauri.ios.conf.json')).bundle.iOS.minimumSystemVersion,'15.0');
});
