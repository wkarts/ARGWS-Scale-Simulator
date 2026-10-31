import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const rootDir = dirname(fileURLToPath(import.meta.url));
const envFile = resolve(rootDir, '.env');

function loadDotEnv(path) {
  if (!existsSync(path)) return;
  const content = readFileSync(path, 'utf8');
  for (const rawLine of content.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#') || !line.includes('=')) continue;
    const index = line.indexOf('=');
    const key = line.slice(0, index).trim();
    let value = line.slice(index + 1).trim();
    value = value.replace(/^['"]|['"]$/g, '');
    if (key && process.env[key] === undefined) {
      process.env[key] = value;
    }
  }
}

function envBool(name, fallback = false) {
  const value = process.env[name];
  if (value === undefined || value === null || value === '') return fallback;
  return ['1', 'true', 'yes', 'sim', 'on'].includes(String(value).trim().toLowerCase());
}

function setDefaultEnv(name, value) {
  if (process.env[name] === undefined || process.env[name] === '') {
    process.env[name] = String(value);
  }
}

function forceEnv(name, value) {
  process.env[name] = String(value);
}

loadDotEnv(envFile);

const publishAllPorts = envBool('SCALE_SIMULATOR_PUBLISH_ALL_PORTS', false);
const startAllPorts = envBool('SCALE_SIMULATOR_START_ALL_PORTS', false);
const respectCloudPanelPort = envBool('SCALE_SIMULATOR_RESPECT_CLOUDPANEL_PORT', true);
const publicBindHost = process.env.SCALE_SIMULATOR_PUBLIC_BIND_HOST || '0.0.0.0';

const binPath = resolve(rootDir, process.env.SCALE_SIMULATOR_BINARY || './bin/scale_simulator_tauri');
const dataDir = resolve(rootDir, process.env.SCALE_SIMULATOR_DATA_DIR || './data');
const logsDir = resolve(rootDir, process.env.SCALE_SIMULATOR_LOGS_DIR || './logs');
const distDir = resolve(rootDir, process.env.SCALE_SIMULATOR_WEB_DIST_DIR || './dist');

mkdirSync(dataDir, { recursive: true });
mkdirSync(logsDir, { recursive: true });

if (!existsSync(binPath)) {
  console.error(`[cloudpanel] Binário não encontrado: ${binPath}`);
  console.error('[cloudpanel] Gere o release Linux com scripts/linux/build-cloudpanel-release.sh antes do deploy.');
  process.exit(1);
}

if (!existsSync(resolve(distDir, 'index.html'))) {
  console.error(`[cloudpanel] Frontend dist não encontrado em: ${distDir}`);
  console.error('[cloudpanel] Execute npm run build:web antes de empacotar ou copie a pasta dist para o release.');
  process.exit(1);
}

if (publishAllPorts) {
  forceEnv('SCALE_SIMULATOR_API_HOST', process.env.SCALE_SIMULATOR_API_HOST || publicBindHost);
  forceEnv('SCALE_SIMULATOR_WEB_HOST', process.env.SCALE_SIMULATOR_WEB_HOST || publicBindHost);
  forceEnv('SCALE_SIMULATOR_WEBHOOK_HOST', process.env.SCALE_SIMULATOR_WEBHOOK_HOST || publicBindHost);
  forceEnv('SCALE_SIMULATOR_WEBSOCKET_HOST', process.env.SCALE_SIMULATOR_WEBSOCKET_HOST || publicBindHost);

  setDefaultEnv('SCALE_SIMULATOR_API_ALLOW_PUBLIC_NETWORK', 'true');
  setDefaultEnv('SCALE_SIMULATOR_API_CORS', 'true');
  setDefaultEnv('SCALE_SIMULATOR_WEB_BIND_LAN', 'true');
  setDefaultEnv('SCALE_SIMULATOR_WEBHOOK_ALLOW_LAN', 'true');
  setDefaultEnv('SCALE_SIMULATOR_WEBHOOK_ALLOW_EXTERNAL', 'true');
  setDefaultEnv('SCALE_SIMULATOR_WEBSOCKET_ALLOW_LAN', 'true');
  setDefaultEnv('SCALE_SIMULATOR_WEBSOCKET_ALLOW_EXTERNAL', 'true');
}

if (startAllPorts) {
  forceEnv('SCALE_SIMULATOR_WEBHOOK_ENABLED', 'true');
  forceEnv('SCALE_SIMULATOR_WEBHOOK_AUTO_START', 'true');
  forceEnv('SCALE_SIMULATOR_WEBSOCKET_ENABLED', 'true');
  forceEnv('SCALE_SIMULATOR_WEBSOCKET_AUTO_START', 'true');
  forceEnv('SCALE_SIMULATOR_SERVICES_AUTO_START', 'true');
}

const apiHost = process.env.SCALE_SIMULATOR_API_HOST || '127.0.0.1';
const apiPort = process.env.SCALE_SIMULATOR_API_PORT || '61001';

// CloudPanel normalmente injeta PORT para a aplicação Node.js.
// Quando SCALE_SIMULATOR_RESPECT_CLOUDPANEL_PORT=true, PORT vira a porta do WebPort.
// Para publicar todas as portas diretamente, use SCALE_SIMULATOR_RESPECT_CLOUDPANEL_PORT=false.
const webPort = respectCloudPanelPort && process.env.PORT
  ? process.env.PORT
  : (process.env.SCALE_SIMULATOR_WEB_PORT || '61002');
const webHost = process.env.SCALE_SIMULATOR_WEB_HOST || '127.0.0.1';
const webhookHost = process.env.SCALE_SIMULATOR_WEBHOOK_HOST || '127.0.0.1';
const webhookPort = process.env.SCALE_SIMULATOR_WEBHOOK_PORT || '61003';
const websocketHost = process.env.SCALE_SIMULATOR_WEBSOCKET_HOST || '127.0.0.1';
const websocketPort = process.env.SCALE_SIMULATOR_WEBSOCKET_PORT || '61004';

process.env.SCALE_SIMULATOR_ENV_FILE = process.env.SCALE_SIMULATOR_ENV_FILE || envFile;
process.env.SCALE_SIMULATOR_WEB_DIST_DIR = distDir;
process.env.SCALE_SIMULATOR_API_HOST = apiHost;
process.env.SCALE_SIMULATOR_API_PORT = apiPort;
process.env.SCALE_SIMULATOR_API_BASE_URL = process.env.SCALE_SIMULATOR_API_BASE_URL || `http://127.0.0.1:${apiPort}`;
process.env.SCALE_SIMULATOR_WEB_HOST = webHost;
process.env.SCALE_SIMULATOR_WEB_PORT = String(webPort);
process.env.SCALE_SIMULATOR_WEB_ENABLED = process.env.SCALE_SIMULATOR_WEB_ENABLED || 'true';
process.env.SCALE_SIMULATOR_WEB_AUTO_START = process.env.SCALE_SIMULATOR_WEB_AUTO_START || 'true';
process.env.SCALE_SIMULATOR_SERVICES_AUTO_START = process.env.SCALE_SIMULATOR_SERVICES_AUTO_START || 'true';
process.env.SCALE_SIMULATOR_WEBHOOK_HOST = webhookHost;
process.env.SCALE_SIMULATOR_WEBHOOK_PORT = webhookPort;
process.env.SCALE_SIMULATOR_WEBSOCKET_HOST = websocketHost;
process.env.SCALE_SIMULATOR_WEBSOCKET_PORT = websocketPort;

const args = [
  '--mode=headless-api',
  '--host', apiHost,
  '--port', String(apiPort),
  '--data-dir', dataDir,
  '--start-web-proxy',
  '--start-services'
];

if (process.env.SCALE_SIMULATOR_WEBHOOK_ENABLED === 'true') {
  args.push('--start-webhook');
}

if (process.env.SCALE_SIMULATOR_WEBSOCKET_ENABLED === 'true') {
  args.push('--start-websocket');
}

console.log(`[cloudpanel] Iniciando ${binPath}`);
console.log(`[cloudpanel] WebPort:   http://${webHost}:${webPort}`);
console.log(`[cloudpanel] API:       http://${apiHost}:${apiPort}`);
console.log(`[cloudpanel] Webhook:   http://${webhookHost}:${webhookPort}${process.env.SCALE_SIMULATOR_WEBHOOK_BASE_PATH || '/webhooks'} (${process.env.SCALE_SIMULATOR_WEBHOOK_ENABLED === 'true' ? 'enabled' : 'disabled'})`);
console.log(`[cloudpanel] WebSocket: ws://${websocketHost}:${websocketPort}${process.env.SCALE_SIMULATOR_WEBSOCKET_PATH || '/ws'} (${process.env.SCALE_SIMULATOR_WEBSOCKET_ENABLED === 'true' ? 'enabled' : 'disabled'})`);
console.log(`[cloudpanel] Data dir:  ${dataDir}`);
console.log(`[cloudpanel] Publish all ports: ${publishAllPorts ? 'true' : 'false'}`);

const child = spawn(binPath, args, {
  cwd: rootDir,
  env: process.env,
  stdio: 'inherit'
});

function shutdown(signal) {
  if (!child.killed) {
    child.kill(signal);
  }
}

process.on('SIGINT', () => shutdown('SIGINT'));
process.on('SIGTERM', () => shutdown('SIGTERM'));

child.on('exit', (code, signal) => {
  if (signal) {
    console.error(`[cloudpanel] Processo encerrado por sinal: ${signal}`);
    process.exit(128);
  }
  process.exit(code ?? 0);
});
