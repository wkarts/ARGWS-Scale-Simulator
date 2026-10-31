<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { RouterLink } from "vue-router";
import { isTauriRuntime } from "../core/runtime/RuntimeProvider";
import { useSessionStore } from "../stores/session";
import { listScaleSimulators, type ScaleSimulatorStatus } from "../services/scaleSimulator";
import DashboardMetricCard from "../components/dashboard/DashboardMetricCard.vue";
const session = useSessionStore();
const desktop = isTauriRuntime();
const permitted = computed(() => session.can("config:view"));
const instances = ref<ScaleSimulatorStatus[]>([]);
const loading = ref(false);
const loaded = ref(false);
const error = ref("");
const updated = ref("");
let timer: ReturnType<typeof setInterval> | undefined;
let disposed = false;
const running = computed(() => instances.value.filter(item => item.running));
const tcp = computed(() => running.value.filter(item => item.mode.startsWith("tcp")));
const serial = computed(() => running.value.filter(item => item.mode.startsWith("serial")));
const clients = computed(() => tcp.value.reduce((sum, item) => sum + item.clients, 0));
const available = computed(() => desktop && permitted.value && loaded.value && !error.value);
const metric = (count: number) => available.value ? count : "—";
async function load() {
  if (!desktop || !permitted.value || loading.value) return;
  loading.value = true;
  try {
    const result = await listScaleSimulators();
    if (disposed) return;
    instances.value = result;
    loaded.value = true;
    error.value = "";
    updated.value = new Date().toLocaleTimeString("pt-BR");
  } catch (err) {
    if (!disposed) error.value = err instanceof Error ? err.message : String(err);
  } finally { loading.value = false; }
}
onMounted(() => { void load(); timer = setInterval(() => { if (!document.hidden) void load(); }, 5000); });
onBeforeUnmount(() => { disposed = true; if (timer) clearInterval(timer); });
</script>

<template>
  <div class="page-content-scroll dashboard-page scale-overview">
    <section class="hero-card dashboard-hero">
      <div>
        <p class="eyebrow">LABORATÓRIO DE PESAGEM</p>
        <h2>Integrações sob controle.</h2>
        <p>Simule indicadores, ajuste o peso e acompanhe a comunicação TCP e RS232 em um só lugar.</p>
        <RouterLink v-if="permitted" class="primary link-button" to="/simulador-balancas">Abrir simulador →</RouterLink>
      </div>
      <div class="scale-hero-display" aria-hidden="true"><span>SCALE SIMULATOR</span><strong>0.000 <small>kg</small></strong><span>TCP / RS232 · 9 MODELOS</span></div>
    </section>
    <div v-if="!desktop" class="alert info">Prévia web: a comunicação TCP e RS232 está disponível no aplicativo desktop. Os indicadores abaixo não estão conectados a um simulador.</div>
    <div v-else-if="!permitted" class="alert info">Seu perfil não tem acesso às instâncias. Solicite a permissão de configuração ao administrador.</div>
    <div v-if="error" class="alert error" role="alert">Não foi possível consultar as instâncias. {{ error }} Os dados exibidos podem estar desatualizados.</div>
    <section class="dashboard-metrics-grid">
      <DashboardMetricCard title="Balanças em execução" :value="metric(running.length)" subtitle="Instâncias ativas agora" icon="◉" status="info" />
      <DashboardMetricCard title="Servidores TCP" :value="metric(tcp.length)" subtitle="Contínuo ou por comando" icon="⇄" status="info" />
      <DashboardMetricCard title="Portas RS232" :value="metric(serial.length)" subtitle="Instâncias seriais em execução" icon="↔" status="info" />
      <DashboardMetricCard title="Clientes TCP" :value="metric(clients)" subtitle="Conexões informadas pelo simulador" icon="⌁" status="info" />
    </section>
    <section class="card section-card">
      <div class="scale-overview-heading"><div><h3>Instâncias do simulador</h3><p class="muted-text">{{ updated ? `Última consulta às ${updated} · atualização a cada 5 segundos` : 'Estado de execução consultado diretamente no aplicativo' }}</p></div><button class="secondary" :disabled="loading || !desktop || !permitted" @click="load">{{ loading ? 'Consultando...' : 'Atualizar' }}</button></div>
      <div v-if="instances.length" class="table-scroll"><table><thead><tr><th>Balança</th><th>Conexão</th><th>Peso líquido</th><th>Estado</th></tr></thead><tbody><tr v-for="item in instances" :key="item.id"><td><strong>{{ item.name }}</strong><br><small>{{ item.driver }}</small></td><td>{{ item.mode.startsWith('serial') ? `${item.serial_port} · ${item.baud_rate} baud` : `${item.host}:${item.port}` }}</td><td>{{ item.net_kg.toLocaleString('pt-BR', { minimumFractionDigits: 3 }) }} kg</td><td><span class="badge">{{ item.running ? 'Em execução' : 'Parada' }}</span></td></tr></tbody></table></div>
      <div v-else class="scale-empty"><strong>{{ loading ? 'Consultando balanças...' : available ? 'Nenhuma balança em execução' : 'Estado das balanças indisponível' }}</strong><p>{{ available ? 'Escolha um modelo, configure a conexão e inicie sua primeira instância no simulador.' : 'Use o aplicativo desktop para consultar e controlar suas instâncias.' }}</p></div>
    </section>
    <section class="scale-help-grid">
      <RouterLink to="/documentacao/guia" class="card section-card"><h3>Comece pelo teste certo →</h3><p>Escolha o indicador, a porta e o modo de envio.</p></RouterLink>
      <RouterLink v-if="permitted" to="/logs" class="card section-card"><h3>Investigue a comunicação →</h3><p>Consulte os logs do aplicativo. Os quadros de cada balança ficam no simulador.</p></RouterLink>
    </section>
  </div>
</template>
