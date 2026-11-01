<script setup lang="ts">
import { computed, onMounted, onUnmounted, onActivated, onDeactivated, ref } from "vue";
import SavedScaleModels from "../components/SavedScaleModels.vue";
import { frameHex } from "../utils/framePresentation";
import {
  defaultScaleConfig,
  executeScaleSimulatorAction,
  listScaleSimulators,
  loadScaleSimulatorLogs,
  loadScaleSimulatorPresets,
  loadScaleSimulatorSerialPorts,
  previewScaleSimulatorFrame,
  setScaleSimulatorWeight,
  startScaleSimulator,
  stopAllScaleSimulators,
  stopScaleSimulator,
  type ScaleSimulatorConfig,
  type ScaleSimulatorLogEntry,
  type ScaleSimulatorPreset,
  type ScaleSimulatorSerialPortInfo,
  type ScaleSimulatorStatus,
} from "../services/scaleSimulator";

let preserveSavedSerialPort=false;
function applySavedModel(config:ScaleSimulatorConfig){form.value={...defaultScaleConfig(),...config,id:null};preserveSavedSerialPort=true;void refreshPreview();}
const loading = ref(false);
const starting = ref(false);
const error = ref("");
const preview = ref("");
const previewValid = ref(false);
let previewRequest = 0;
const presets = ref<ScaleSimulatorPreset[]>([]);
const serialPorts = ref<ScaleSimulatorSerialPortInfo[]>([]);
const simulators = ref<ScaleSimulatorStatus[]>([]);
const selectedSimulatorId = ref<string | null>(null);
const logs = ref<ScaleSimulatorLogEntry[]>([]);
const weightInput = ref(1234);
const form = ref<ScaleSimulatorConfig>(defaultScaleConfig());
let timer: number | undefined;

const selectedSimulator = computed(() => simulators.value.find((item) => item.id === selectedSimulatorId.value) ?? simulators.value[0] ?? null);

const activeCount = computed(() => simulators.value.length);
const totalClients = computed(() => simulators.value.reduce((sum, item) => sum + item.clients, 0));
const isSerialMode = computed(() => form.value.mode === "serial_continuous" || form.value.mode === "serial_command");
const selectedSerialPortInfo = computed(() =>
  serialPorts.value.find((port) => port.port_name.toLowerCase() === form.value.serial_port.trim().toLowerCase()) ?? null,
);
const serialPortsSummary = computed(() => {
  if (serialPorts.value.length === 0) return "Nenhuma porta COM detectada";
  return serialPorts.value.map((port) => port.port_name).join(", ");
});

function formatKg(value: number): string {
  return new Intl.NumberFormat("pt-BR", { minimumFractionDigits: 0, maximumFractionDigits: 3 }).format(value);
}

function driverLabel(driver: string): string {
  const labels: Record<string, string> = {
    wt1000n: "WT1000N / ADP",
    wt27r: "WT27R",
    wt3000ir: "WT3000IR / ADP",
    toledo8540: "Toledo 8540 / ADP",
    toledo_prix3: "Toledo/Prix 3",
    a12e: "A12e / ADP",
    lider_ld2052: "Líder LD2052 / ADP",
    filizola: "Filizola",
    generic_ascii: "Genérico ASCII",
  };
  return labels[driver] ?? driver;
}

function modeLabel(mode: string): string {
  const labels: Record<string, string> = {
    tcp_server: "TCP contínuo",
    tcp_server_command: "TCP sob comando",
    serial_continuous: "RS232 COM contínuo",
    serial_command: "RS232 COM sob comando",
  };
  return labels[mode] ?? mode;
}

function endpointLabel(simulator: ScaleSimulatorStatus): string {
  if (simulator.mode === "serial_continuous" || simulator.mode === "serial_command") {
    const bits = simulator.serial_data_bits ?? (simulator.driver === "toledo8540" ? 7 : 8);
    const parity = { none: "N", even: "E", odd: "O" }[simulator.serial_parity ?? (simulator.driver === "toledo8540" ? "even" : "none")] ?? "?";
    return `${simulator.serial_port} · ${simulator.baud_rate} bps · ${bits}${parity}${simulator.serial_stop_bits ?? 1}`;
  }
  return `tcp://${simulator.host}:${simulator.port}`;
}

function portTypeLabel(portType: string): string {
  const labels: Record<string, string> = {
    usb: "USB/Serial",
    pci: "Serial PCI",
    bluetooth: "Bluetooth",
    unknown: "Serial",
  };
  return labels[portType] ?? portType;
}

async function refreshSerialPorts() {
  try {
    serialPorts.value = await loadScaleSimulatorSerialPorts();
    if (!preserveSavedSerialPort && isSerialMode.value && serialPorts.value.length > 0 && !selectedSerialPortInfo.value) {
      form.value.serial_port = serialPorts.value[0].port_name;
      await refreshPreview();
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao identificar portas COM do computador.";
    serialPorts.value = [];
  }
}

async function loadAll() {
  loading.value = true;
  error.value = "";
  try {
    if (presets.value.length === 0) {
      presets.value = await loadScaleSimulatorPresets();
    }
    await refreshSerialPorts();
    simulators.value = await listScaleSimulators();
    if (!selectedSimulatorId.value && simulators.value.length > 0) {
      selectedSimulatorId.value = simulators.value[0].id;
    }
    if (selectedSimulatorId.value && !simulators.value.some((item) => item.id === selectedSimulatorId.value)) {
      selectedSimulatorId.value = simulators.value[0]?.id ?? null;
    }
    await refreshLogs();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao carregar simuladores.";
  } finally {
    loading.value = false;
  }
}

async function refreshPreview() {
  const request = ++previewRequest;
  previewValid.value = false;
  try {
    const frame = await previewScaleSimulatorFrame(form.value);
    if (request !== previewRequest) return;
    preview.value = frame;
    previewValid.value = true;
  } catch (err) {
    if (request !== previewRequest) return;
    preview.value = err instanceof Error ? err.message : String(err);
    previewValid.value = false;
  }
}

function resetSerialDefaults() {
  const toledo = form.value.driver === "toledo8540";
  form.value.serial_data_bits = toledo ? 7 : 8;
  form.value.serial_parity = toledo ? "even" : "none";
  form.value.serial_stop_bits = 1;
  form.value.toledo8540_stop_bits = 1;
  form.value.frame_override_enabled = false;
  form.value.frame_prefix_hex = null;
  form.value.frame_suffix_hex = null;
  form.value.frame_checksum_mode = "protocol";
}

function changeDriver() {
  resetSerialDefaults();
  void refreshPreview();
}

function changeStopBits(event: Event) {
  const value = Number((event.target as HTMLSelectElement).value) as 1 | 2;
  form.value.serial_stop_bits = value;
  form.value.toledo8540_stop_bits = value;
}

function applyPreset(preset: ScaleSimulatorPreset) {
  preserveSavedSerialPort=false;
  form.value.driver = preset.driver;
  resetSerialDefaults();
  form.value.mode = preset.serial_mode
    ? (preset.command_mode ? "serial_command" : "serial_continuous")
    : (preset.command_mode ? "tcp_server_command" : "tcp_server");
  form.value.port = preset.default_port;
  const defaultPortExists = serialPorts.value.some((port) => port.port_name.toLowerCase() === preset.default_serial_port.toLowerCase());
  form.value.serial_port = defaultPortExists ? preset.default_serial_port : (serialPorts.value[0]?.port_name ?? preset.default_serial_port);
  form.value.baud_rate = preset.default_baud_rate;
  form.value.name = preset.name.replace(" - contínuo TCP", "").replace(" - sob comando TCP", "");
  void refreshPreview();
}

async function start() {
  starting.value = true;
  error.value = "";
  try {
    if (isSerialMode.value) {
      await refreshSerialPorts();
      if (!selectedSerialPortInfo.value) {
        throw new Error(`A porta ${form.value.serial_port || "COM"} não foi identificada neste computador. Portas detectadas: ${serialPortsSummary.value}.`);
      }
    }
    const result = await startScaleSimulator({ ...form.value, id: null });
    selectedSimulatorId.value = result.id;
    await loadAll();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao iniciar simulador.";
  } finally {
    starting.value = false;
  }
}

async function stop(id: string) {
  await stopScaleSimulator(id);
  if (selectedSimulatorId.value === id) selectedSimulatorId.value = null;
  await loadAll();
}

async function stopAll() {
  await stopAllScaleSimulators();
  selectedSimulatorId.value = null;
  logs.value = [];
  await loadAll();
}

async function updateWeight() {
  if (!selectedSimulator.value) return;
  await setScaleSimulatorWeight(selectedSimulator.value.id, Number(weightInput.value));
  await loadAll();
}

async function runAction(action: string, value?: number | null) {
  if (!selectedSimulator.value) return;
  await executeScaleSimulatorAction(selectedSimulator.value.id, action, value ?? null);
  await loadAll();
}

async function refreshLogs() {
  if (!selectedSimulatorId.value) {
    logs.value = [];
    return;
  }
  try {
    logs.value = await loadScaleSimulatorLogs(selectedSimulatorId.value);
  } catch {
    logs.value = [];
  }
}

function resumePolling() {
  if (timer !== undefined) return;
  void loadAll();
  timer = window.setInterval(() => void loadAll(), 1500);
}
function suspendPolling() {
  if (timer !== undefined) window.clearInterval(timer);
  timer = undefined;
}
onMounted(() => { resumePolling(); void refreshPreview(); });
onActivated(resumePolling);
onDeactivated(suspendPolling);
onUnmounted(suspendPolling);
</script>

<template>
  <div class="page-content-scroll scale-simulator-page">
    <div class="hero-card dashboard-hero">
      <div>
        <p class="eyebrow">Laboratório de pesagem • TCP / RS232</p>
        <h2>Simulação de balanças</h2>
        <p>
          Configure o protocolo e a conexão TCP ou RS232, inicie uma instância e acompanhe o peso e os quadros transmitidos. Em testes seriais no mesmo computador, use um par de portas COM virtuais.
        </p>
      </div>
      <div class="scale-hero-actions">
        <button class="secondary" :disabled="loading" @click="refreshSerialPorts">Atualizar COM</button>
        <button class="secondary" :disabled="loading" @click="loadAll">Atualizar tudo</button>
        <button class="danger" :disabled="activeCount === 0" @click="stopAll">Parar todos</button>
      </div>
    </div>

    <div v-if="error" class="alert error">{{ error }}</div>

    <section class="scale-metrics-grid">
      <div class="scale-metric-card">
        <div class="metric-icon">⚖</div>
        <div>
          <strong>{{ activeCount }}</strong>
          <span>Simuladores ativos</span>
        </div>
      </div>
      <div class="scale-metric-card">
        <div class="metric-icon">🔌</div>
        <div>
          <strong>{{ totalClients }}</strong>
          <span>Conexões/portas abertas</span>
        </div>
      </div>
      <div class="scale-metric-card">
        <div class="metric-icon">📡</div>
        <div>
          <strong>{{ serialPorts.length }}</strong>
          <span>Portas COM detectadas</span>
        </div>
      </div>
      <div class="scale-metric-card">
        <div class="metric-icon">🧩</div>
        <div>
          <strong>{{ presets.length }}</strong>
          <span>Presets de protocolo</span>
        </div>
      </div>
    </section>

    <section class="scale-grid-main">
      <div class="section-card scale-form-card">
        <div class="section-card-header">
          <div>
            <p class="eyebrow">Nova instância</p>
            <h3>Configurar simulador</h3>
          </div>
          <button class="primary" :disabled="starting" @click="start">{{ starting ? "Iniciando..." : "Iniciar" }}</button>
        </div>

        <SavedScaleModels :config="form" @load="applySavedModel" />
        <details class="protocol-catalog" open>
          <summary>Protocolos prontos <span>{{ presets.length }} opções</span></summary>
          <div class="preset-list">
          <button v-for="preset in presets" :key="preset.key" class="preset-button" type="button" @click="applyPreset(preset)">
            <strong>{{ preset.name }}</strong>
            <span>{{ preset.description }}</span>
            <small v-if="preset.adp_equipment">Equipamento ADP: {{ preset.adp_equipment }}</small>
          </button>
          </div>
        </details>

        <div class="form-grid compact">
          <label>
            Nome
            <input v-model="form.name" type="text" @input="refreshPreview" />
          </label>
          <label>
            Driver
            <select v-model="form.driver" @change="changeDriver">
              <option value="wt1000n">WT1000N / ADP</option>
              <option value="wt27r">WT27R</option>
              <option value="wt3000ir">WT3000IR / ADP</option>
              <option value="toledo8540">Toledo 8540 / ADP</option>
              <option value="toledo_prix3">Toledo/Prix 3</option>
              <option value="a12e">A12e / ADP</option>
              <option value="lider_ld2052">Líder LD2052 / ADP</option>
              <option value="filizola">Filizola</option>
              <option value="generic_ascii">Genérico ASCII</option>
            </select>
          </label>
          <label>
            Modo
            <select v-model="form.mode" @change="refreshPreview">
              <option value="serial_continuous">RS232 COM contínuo</option>
              <option value="serial_command">RS232 COM sob comando</option>
              <option value="tcp_server">TCP contínuo</option>
              <option value="tcp_server_command">TCP sob comando</option>
            </select>
          </label>
          <label v-if="!isSerialMode">
            Host
            <input v-model="form.host" type="text" />
          </label>
          <label v-if="!isSerialMode">
            Porta TCP
            <input v-model.number="form.port" type="number" min="1" max="65535" />
          </label>
          <label v-if="isSerialMode">
            Porta serial RS232 detectada
            <select v-if="serialPorts.length > 0" v-model="form.serial_port" @change="refreshPreview">
              <option v-if="form.serial_port && !serialPorts.some(port => port.port_name === form.serial_port)" :value="form.serial_port">{{form.serial_port}} (não disponível neste computador)</option>
              <option v-for="port in serialPorts" :key="port.port_name" :value="port.port_name">
                {{ port.display_name }}{{ port.in_use_by_simulator ? " · em uso" : "" }}
              </option>
            </select>
            <input v-else v-model="form.serial_port" type="text" placeholder="Nenhuma porta COM detectada" @input="refreshPreview" />
          </label>
          <label v-if="isSerialMode">
            Baud rate
            <select v-model.number="form.baud_rate">
              <option :value="1200">1200</option>
              <option :value="2400">2400</option>
              <option :value="4800">4800</option>
              <option :value="9600">9600</option>
              <option :value="19200">19200</option>
              <option :value="38400">38400</option>
              <option :value="57600">57600</option>
              <option :value="115200">115200</option>
            </select>
          </label>
          <label v-if="isSerialMode">
            Bits de dados
            <select v-model.number="form.serial_data_bits">
              <option :value="5">5 bits</option>
              <option :value="6">6 bits</option>
              <option :value="7">7 bits</option>
              <option :value="8">8 bits</option>
            </select>
          </label>
          <label v-if="isSerialMode">
            Paridade
            <select v-model="form.serial_parity">
              <option value="none">Nenhuma (N)</option>
              <option value="even">Par (E)</option>
              <option value="odd">Ímpar (O)</option>
            </select>
          </label>
          <label v-if="isSerialMode">
            Bits de parada
            <select :value="form.serial_stop_bits ?? form.toledo8540_stop_bits ?? 1" @change="changeStopBits">
              <option :value="1">1 stop bit</option>
              <option :value="2">2 stop bits</option>
            </select>
          </label>
          <label>
            Capacidade kg
            <input v-model.number="form.capacity_kg" type="number" min="1" @input="refreshPreview" />
          </label>
          <label>
            Divisão kg
            <input v-model.number="form.division_kg" type="number" min="0.001" step="0.001" @input="refreshPreview" />
          </label>
          <label>
            Peso inicial kg
            <input v-model.number="form.initial_weight_kg" type="number" step="0.001" @input="refreshPreview" />
          </label>
          <label>
            Oscilação kg
            <input v-model.number="form.variation_kg" type="number" step="0.001" @input="refreshPreview" />
          </label>
          <label>
            Intervalo ms
            <input v-model.number="form.interval_ms" type="number" min="50" step="50" />
          </label>
          <label>
            Unidade
            <input v-model="form.unit" type="text" maxlength="4" @input="refreshPreview" />
          </label>
          <label class="checkbox-line">
            <input v-model="form.stable" type="checkbox" @change="refreshPreview" />
            Peso estável
          </label>
        </div>

        <details class="card frame-overrides">
          <summary>Enquadramento avançado · testes de protocolo</summary>
          <p class="muted-text">Por padrão, cada modelo usa seu quadro original. Ative apenas para testar enquadramentos diferentes ou respostas a erros no receptor.</p>
          <label class="checkbox-line"><input v-model="form.frame_override_enabled" type="checkbox" @change="refreshPreview" />Personalizar quadro transmitido</label>
          <div v-if="form.frame_override_enabled">
            <div class="alert warning" role="status">Esta personalização pode gerar quadros incompatíveis ou intencionalmente inválidos. Confira a prévia antes de iniciar. Trocar driver ou preset desativa a personalização.</div>
            <div class="scale-form-grid">
              <label>
                <span><input type="checkbox" :checked="form.frame_prefix_hex != null" @change="form.frame_prefix_hex = ($event.target as HTMLInputElement).checked ? '' : null; refreshPreview()" /> Substituir início do quadro</span>
                <input v-if="form.frame_prefix_hex != null" v-model="form.frame_prefix_hex" type="text" placeholder="02 = STX · vazio = remover" maxlength="23" spellcheck="false" @input="refreshPreview" />
                <small v-else>Marcador original do protocolo</small>
              </label>
              <label>
                <span><input type="checkbox" :checked="form.frame_suffix_hex != null" @change="form.frame_suffix_hex = ($event.target as HTMLInputElement).checked ? '' : null; refreshPreview()" /> Substituir terminador</span>
                <input v-if="form.frame_suffix_hex != null" v-model="form.frame_suffix_hex" type="text" placeholder="0D 0A = CR LF · vazio = remover" maxlength="23" spellcheck="false" @input="refreshPreview" />
                <small v-else>Terminador original do protocolo</small>
              </label>
              <label v-if="form.driver === 'toledo8540'">Checksum Toledo 8540
                <select v-model="form.frame_checksum_mode" @change="refreshPreview"><option value="protocol">Correto (recalculado)</option><option value="corrupt">Corrompido (teste de erro)</option><option value="omit">Sem checksum (teste de erro)</option></select>
              </label>
            </div>
            <p class="muted-text">Até 8 bytes HEX entre 00 e 7F, separados por espaço. STX = 02; ETX = 03; CR = 0D; LF = 0A; CR LF = 0D 0A. Campo marcado e vazio remove o marcador; desmarcado preserva o original.</p>
          </div>
        </details>

        <p v-if="isSerialMode" class="muted-text">Configure a mesma velocidade, paridade e bits no receptor. Ao trocar o driver ou aplicar um preset, o padrão serial é restaurado: Toledo 8540 em 7E1; demais modelos em 8N1. Alterações se aplicam à próxima instância iniciada.</p>
        <div v-if="isSerialMode" class="serial-port-diagnostic">
          <div>
            <strong>Portas RS232/COM identificadas:</strong>
            <span>{{ serialPortsSummary }}</span>
          </div>
          <div v-if="selectedSerialPortInfo">
            <strong>Selecionada:</strong>
            <span>
              {{ selectedSerialPortInfo.display_name }} · {{ portTypeLabel(selectedSerialPortInfo.port_type) }}
              <template v-if="selectedSerialPortInfo.in_use_by_simulator"> · em uso por este simulador</template>
            </span>
          </div>
          <div v-else class="serial-warning">
            Nenhuma porta COM existente foi selecionada. Para simular para o All Driver Platform, crie um par virtual e não use a mesma porta nos dois sistemas. Exemplo: simulador em COM10 e ADP lendo COM11.
          </div>
          <button class="secondary" type="button" @click="refreshSerialPorts">Reidentificar portas COM</button>
        </div>

        <div class="frame-preview">
          <span>Preview do frame</span>
          <code>{{ preview.replace(/\r/g, "\\r").replace(/\n/g, "\\n") }}</code>
          <div v-if="previewValid"><strong>HEX (bytes transmitidos)</strong><code class="frame-hex">{{ frameHex(preview) }}</code></div>
        </div>
      </div>

      <div class="section-card">
        <div class="section-card-header">
          <div>
            <p class="eyebrow">Execução</p>
            <h3>Simuladores ativos</h3>
          </div>
        </div>

        <div v-if="simulators.length === 0" class="empty-state-inline">
          Nenhuma balança simulada ativa. Configure uma porta COM ou TCP e clique em iniciar.
        </div>

        <div v-else class="scale-active-list">
          <button
            v-for="simulator in simulators"
            :key="simulator.id"
            class="scale-active-item"
            :class="{ selected: selectedSimulatorId === simulator.id }"
            type="button"
            @click="selectedSimulatorId = simulator.id; weightInput = simulator.gross_kg; refreshLogs()"
          >
            <div>
              <strong>{{ simulator.name }}</strong>
              <small v-if="simulator.frame_override_enabled" class="badge">Quadro personalizado</small>
              <span>{{ driverLabel(simulator.driver) }} · {{ modeLabel(simulator.mode) }} · {{ endpointLabel(simulator) }}</span>
            </div>
            <div class="scale-active-weight">
              {{ formatKg(simulator.weight_kg) }} kg
              <small>{{ simulator.clients }} cliente(s)</small>
            </div>
          </button>
        </div>

        <div v-if="selectedSimulator" class="scale-control-panel">
          <div class="scale-display">
            <span>{{ selectedSimulator.stable ? "ESTÁVEL" : "OSCILANDO" }}</span>
            <strong>{{ formatKg(selectedSimulator.weight_kg) }}</strong>
            <em>kg</em>
          </div>

          <div class="scale-control-grid">
            <label>
              Novo peso bruto
              <input v-model.number="weightInput" type="number" step="0.001" />
            </label>
            <button @click="updateWeight">Aplicar peso</button>
            <button class="secondary" @click="runAction('tare')">Tarar</button>
            <button class="secondary" @click="runAction('clear_tare')">Limpar tara</button>
            <button class="secondary" @click="runAction('zero')">Zerar</button>
            <button class="secondary" @click="runAction('gross')">Bruto</button>
            <button class="secondary" @click="runAction('net')">Líquido</button>
            <button class="secondary" @click="runAction(selectedSimulator.stable ? 'unstable' : 'stable')">
              {{ selectedSimulator.stable ? "Oscilar" : "Estabilizar" }}
            </button>
          </div>

          <div class="scale-details-grid">
            <span><b>Bruto:</b> {{ formatKg(selectedSimulator.gross_kg) }} kg</span>
            <span><b>Tara:</b> {{ formatKg(selectedSimulator.tare_kg) }} kg</span>
            <span><b>Líquido:</b> {{ formatKg(selectedSimulator.net_kg) }} kg</span>
            <span><b>Frame:</b> <code>{{ selectedSimulator.last_frame.replace(/\r/g, "\\r").replace(/\n/g, "\\n") }}</code></span>
            <span><b>HEX (último quadro):</b> <code class="frame-hex">{{ frameHex(selectedSimulator.last_frame) }}</code></span>
          </div>

          <button class="danger full-width" @click="stop(selectedSimulator.id)">Parar simulador selecionado</button>
        </div>
      </div>
    </section>

    <section class="section-card">
      <div class="section-card-header">
        <div>
          <p class="eyebrow">Diagnóstico</p>
          <h3>Logs da instância selecionada</h3>
        </div>
        <button class="secondary" :disabled="!selectedSimulator" @click="refreshLogs">Atualizar logs</button>
      </div>
      <div v-if="logs.length === 0" class="empty-state-inline">Sem logs para exibir.</div>
      <div v-else class="scale-log-list">
        <div v-for="(log, index) in logs" :key="`${log.at}-${index}`" class="scale-log-row" :class="log.level">
          <span>{{ new Date(log.at).toLocaleString("pt-BR") }}</span>
          <strong>{{ log.level }}</strong>
          <p>{{ log.message }}</p>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>

.scale-simulator-page .scale-hero-actions {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  flex-wrap: wrap;
}

.scale-grid-main {
  display: grid;
  grid-template-columns: minmax(360px, 1fr) minmax(360px, 1fr);
  gap: 1rem;
  align-items: start;
  margin-top: 1rem;
}

.scale-form-card {
  min-width: 0;
}

.preset-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 0.65rem;
  margin-bottom: 1rem;
}

.preset-button {
  text-align: left;
  border: 1px solid var(--border-color, #d8dee8);
  background: #fff;
  border-radius: 12px;
  padding: 0.75rem;
  color: inherit;
}

.preset-button strong,
.preset-button span {
  display: block;
}

.preset-button span {
  color: #667085;
  font-size: 0.8rem;
  margin-top: 0.25rem;
  line-height: 1.25rem;
}

.form-grid.compact {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 0.8rem;
}

.form-grid.compact label {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  font-size: 0.82rem;
  color: #475467;
}

.form-grid.compact input,
.form-grid.compact select {
  width: 100%;
  border: 1px solid #d0d5dd;
  border-radius: 10px;
  padding: 0.62rem 0.7rem;
  font: inherit;
  background: #fff;
  color: #101828;
}

.checkbox-line {
  justify-content: center;
  flex-direction: row !important;
  align-items: center;
}

.checkbox-line input {
  width: auto !important;
}


.serial-port-diagnostic {
  display: grid;
  gap: 0.55rem;
  margin-top: 1rem;
  border: 1px solid #dbe7ff;
  border-radius: 12px;
  padding: 0.8rem;
  background: #f7fbff;
  color: #344054;
  font-size: 0.86rem;
}

.serial-port-diagnostic div {
  display: flex;
  gap: 0.4rem;
  align-items: baseline;
  flex-wrap: wrap;
}

.serial-port-diagnostic strong {
  color: #1d2939;
}

.serial-port-diagnostic button {
  justify-self: start;
}

.serial-warning {
  color: #9a3412;
}

.frame-preview {
  margin-top: 1rem;
  border: 1px dashed #c6d0df;
  border-radius: 12px;
  padding: 0.8rem;
  background: #f8fafc;
}

.frame-preview span {
  display: block;
  color: #667085;
  font-size: 0.78rem;
  margin-bottom: 0.35rem;
}

.frame-preview code,
.scale-details-grid code {
  white-space: pre-wrap;
  word-break: break-all;
}

.scale-active-list {
  display: grid;
  gap: 0.65rem;
}

.scale-active-item {
  width: 100%;
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: center;
  border: 1px solid #d0d5dd;
  background: #fff;
  border-radius: 14px;
  padding: 0.8rem;
  text-align: left;
  color: inherit;
}

.scale-active-item.selected {
  border-color: #2563eb;
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.10);
}

.scale-active-item strong,
.scale-active-item span,
.scale-active-weight small {
  display: block;
}

.scale-active-item span,
.scale-active-weight small {
  color: #667085;
  font-size: 0.8rem;
  margin-top: 0.25rem;
}

.scale-active-weight {
  text-align: right;
  font-weight: 800;
  color: #101828;
  min-width: 120px;
}

.scale-control-panel {
  margin-top: 1rem;
  border-top: 1px solid #eef2f6;
  padding-top: 1rem;
}

.scale-display {
  background: #101828;
  color: #ecfdf3;
  border-radius: 18px;
  padding: 1rem;
  text-align: right;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);
}

.scale-display span,
.scale-display em {
  display: block;
  color: #98a2b3;
  font-style: normal;
  font-size: 0.75rem;
}

.scale-display strong {
  display: block;
  font-size: clamp(2.4rem, 6vw, 4.4rem);
  letter-spacing: 0.08em;
  line-height: 1;
}

.scale-control-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
  gap: 0.65rem;
  margin-top: 1rem;
}

.scale-control-grid label {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  font-size: 0.82rem;
  color: #475467;
}

.scale-control-grid input {
  border: 1px solid #d0d5dd;
  border-radius: 10px;
  padding: 0.62rem 0.7rem;
}

.scale-details-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
  gap: 0.6rem;
  background: #f8fafc;
  border-radius: 12px;
  padding: 0.8rem;
  margin: 1rem 0;
  color: #475467;
  font-size: 0.85rem;
}

.full-width {
  width: 100%;
}

.empty-state-inline {
  border: 1px dashed #d0d5dd;
  border-radius: 12px;
  padding: 1rem;
  color: #667085;
  background: #f8fafc;
}

.scale-log-list {
  display: grid;
  gap: 0.45rem;
  max-height: 320px;
  overflow: auto;
}

.scale-log-row {
  display: grid;
  grid-template-columns: 170px 70px 1fr;
  gap: 0.7rem;
  align-items: start;
  border: 1px solid #eaecf0;
  border-radius: 10px;
  padding: 0.55rem 0.7rem;
  font-size: 0.82rem;
}

.scale-log-row span {
  color: #667085;
}

.scale-log-row p {
  margin: 0;
}

.scale-log-row.error strong {
  color: #b42318;
}

.scale-log-row.warn strong {
  color: #b54708;
}

.scale-log-row.info strong {
  color: #175cd3;
}

button.danger {
  background: #b42318;
  color: #fff;
  border-color: #b42318;
}

@media (max-width: 980px) {
  .scale-grid-main {
    grid-template-columns: 1fr;
  }

  .scale-log-row {
    grid-template-columns: 1fr;
  }
}

/* Scale Simulator: focused, light-first product shell. */
.scale-overview { display: grid; gap: 20px; }
.scale-overview .dashboard-hero { padding: 30px; gap: 28px; }
.scale-overview .dashboard-hero h2 { font-size: clamp(25px, 2.3vw, 36px); margin: 8px 0 12px; letter-spacing: -.035em; }
.scale-overview .dashboard-hero p { max-width: 650px; line-height: 1.6; }
.scale-overview .dashboard-hero .link-button { display: inline-flex; margin-top: 12px; }
.scale-hero-display { background: #103b4a; color: #baf7e4; min-width: 250px; padding: 20px 24px; border-radius: 16px; box-shadow: inset 0 0 0 5px #1b5060; display: grid; gap: 14px; }
.scale-hero-display > span { font-size: 10px; letter-spacing: .16em; }
.scale-hero-display strong { font: 600 39px ui-monospace, monospace; white-space: nowrap; }
.scale-hero-display small { font-size: 16px; }
.scale-overview-heading { display: flex; align-items: center; justify-content: space-between; gap: 16px; }
.scale-overview-heading h3 { margin: 0 0 6px; }
.scale-overview-heading p { margin: 0; }
.scale-empty { padding: 42px 16px; text-align: center; color: var(--text-muted); }
.scale-empty strong { color: var(--text-color); font-size: 17px; }
.scale-help-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.scale-help-grid a { color: var(--text-color); text-decoration: none; }
.scale-help-grid h3 { margin: 0 0 10px; }
.scale-help-grid p { margin: 0; color: var(--text-muted); line-height: 1.6; }
.scale-overview .section-card { padding: 22px; }
.scale-overview .table-scroll { overflow-x: auto; margin-top: 20px; }
@media (max-width: 900px) { .scale-hero-display { display: none; } .scale-help-grid { grid-template-columns: 1fr; } }

.frame-overrides { margin-top: 16px; }
.frame-overrides summary { cursor: pointer; font-weight: 700; }
.frame-overrides .scale-form-grid { margin-top: 12px; display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: 14px; }
.frame-overrides label { display: grid; gap: 8px; }
.frame-overrides small { color: var(--text-muted); }

.frame-hex { display: block; overflow-wrap: anywhere; white-space: pre-wrap; margin-top: 6px; }

</style>
