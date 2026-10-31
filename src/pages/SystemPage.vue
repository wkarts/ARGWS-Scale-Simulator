<script setup lang="ts">
import { RouterLink } from "vue-router";
import { onMounted, reactive, ref } from "vue";
import {
  getSystemInfo,
  loadRuntimeSettings,
  saveRuntimeSettings,
  setStartupWithWindows,
  setSystemDataDir,
  type RuntimeSettings,
} from "../services/crud";
import { appFeatures, databaseConfig, internalApiConfig, projectConfig, sidebarConfig, trayConfig } from "../config/projectConfig";
import BasePage from "../components/base/BasePage.vue";
import BaseSectionCard from "../components/base/BaseSectionCard.vue";
import AppSwitch from "../components/AppSwitch.vue";

const info = ref<Record<string, unknown>>({});
const ports = ref<Array<Record<string, unknown>>>([]);
const warnings = ref<string[]>([]);
const envPath = ref("");
const form = reactive({ dataDir: "" });
const runtimeForm = reactive<RuntimeSettings>({
  internal_api_host: "127.0.0.1",
  internal_api_port: 61001,
  internal_api_base_url: "http://127.0.0.1:61001",
  internal_api_docs_url: "http://127.0.0.1:61001/docs",
  internal_api_auto_start: false,
  internal_api_restart_on_config_change: true,
  internal_api_require_token: false,
  internal_api_token: "",
  internal_api_docs_public_local: true,
  internal_api_open_scalar_after_start: false,
  internal_api_timeout_ms: 8000,
  internal_api_log_mode: "normal",
  internal_api_docs_enabled: true,
  internal_api_docs_path: "/docs",
  local_web_host: "127.0.0.1",
  local_web_port: 61002,
  auxiliary_host: "127.0.0.1",
  auxiliary_port: 61003,
  bridge_core_host: "127.0.0.1",
  bridge_core_port: 61004,
  tray_enabled: true,
  minimize_to_tray: true,
  close_to_tray: false,
  start_with_windows: false,
  services_auto_start: false,
});
const loading = ref(false);
const message = ref("");
const error = ref("");

function applyRuntime(settings: RuntimeSettings) {
  Object.assign(runtimeForm, settings);
}

async function load() {
  loading.value = true;
  error.value = "";
  try {
    info.value = await getSystemInfo();
    form.dataDir = String(info.value.data_dir || "");
    const runtime = await loadRuntimeSettings();
    applyRuntime(runtime.settings);
    ports.value = runtime.ports || [];
    warnings.value = runtime.warnings || [];
    envPath.value = runtime.env_path || "";
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao carregar informações do sistema.";
  } finally {
    loading.value = false;
  }
}

async function persist() {
  message.value = "";
  error.value = "";
  try {
    info.value = await setSystemDataDir(form.dataDir);
    message.value = "Diretório de parâmetros/dados atualizado. Reinicie a aplicação se necessário.";
    await load();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao atualizar diretório de dados.";
  }
}

async function persistRuntime() {
  loading.value = true;
  message.value = "";
  error.value = "";
  try {
    const saved = await saveRuntimeSettings({ ...runtimeForm });
    applyRuntime(saved.settings);
    ports.value = saved.ports || [];
    warnings.value = saved.warnings || [];
    envPath.value = saved.env_path || envPath.value;
    if (runtimeForm.start_with_windows) await setStartupWithWindows(true);
    message.value = "Configurações runtime salvas no banco local e no .env. Reinicie serviços afetados para aplicar portas em execução.";
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao salvar configurações runtime.";
  } finally {
    loading.value = false;
  }
}

onMounted(load);
</script>

<template>
  <BasePage title="Sistema e parâmetros" subtitle="Configurações operacionais do simulador, diretórios, portas, tray e serviços locais.">
    <template #actions>
      <button class="secondary" :disabled="loading" @click="load">Recarregar</button>
    </template>

    <div v-if="message" class="alert success">{{ message }}</div>
    <div v-if="error" class="alert error">{{ error }}</div>
    <div v-for="warning in warnings" :key="warning" class="alert warning">{{ warning }}</div>

    <div class="grid columns-2 mobile-columns-1">
      <BaseSectionCard title="Diretórios">
        <div class="info-grid compact-info">
          <div class="info-item"><strong>Banco local</strong><code>{{ info.db_path || '-' }}</code></div>
          <div class="info-item"><strong>Diretório de dados</strong><code>{{ info.data_dir || '-' }}</code></div>
          <div class="info-item"><strong>.env ativo</strong><code>{{ envPath || '-' }}</code></div>
          <div class="info-item"><strong>Bootstrap config</strong><code>{{ info.bootstrap_config || '-' }}</code></div>
        </div>
      </BaseSectionCard>

      <BaseSectionCard title="Aplicação">
        <div class="info-grid compact-info">
          <div class="info-item"><strong>Produto</strong><code>{{ info.product_name || projectConfig.app.name }}</code></div>
          <div class="info-item"><strong>Versão</strong><code>{{ info.version || projectConfig.app.version }}</code></div>
          <div class="info-item"><strong>Build</strong><code>{{ info.build_hash || '-' }}</code></div>
          <div class="info-item"><strong>Modo</strong><code>{{ projectConfig.app.mode }}</code></div>
        </div>
      </BaseSectionCard>
    </div>

    <BaseSectionCard title="Personalização do local dos parâmetros" subtitle="Controla o banco local e diretórios de exportação. A alteração copia o banco atual para o novo local.">
      <div class="grid columns-3 mobile-columns-1">
        <div class="field" style="grid-column: span 2;">
          <label>Diretório de dados/parâmetros</label>
          <input v-model="form.dataDir" type="text" placeholder="C:\\ScaleSimulator\\dados" />
        </div>
        <div class="actions align-end">
          <button class="primary" :disabled="loading" @click="persist">Salvar diretório</button>
        </div>
      </div>
    </BaseSectionCard>

    <BaseSectionCard title="Portas e serviços locais" subtitle="Portas de infraestrutura do aplicativo. Configure a comunicação das balanças na tela Simulador.">
      <div class="grid columns-4 mobile-columns-1">
        <label class="field"><span>Host API interna</span><input v-model="runtimeForm.internal_api_host" type="text" /></label>
        <label class="field"><span>Porta API interna</span><input v-model.number="runtimeForm.internal_api_port" type="number" min="1" max="65535" /></label>
        <label class="field"><span>URL base API</span><input v-model="runtimeForm.internal_api_base_url" type="text" /></label>
        <label class="field"><span>URL Scalar</span><input v-model="runtimeForm.internal_api_docs_url" type="text" /></label>
        <label class="field"><span>Host web local</span><input v-model="runtimeForm.local_web_host" type="text" /></label>
        <label class="field"><span>Porta web local</span><input v-model.number="runtimeForm.local_web_port" type="number" min="1" max="65535" /></label>
        <label class="field"><span>Host auxiliares</span><input v-model="runtimeForm.auxiliary_host" type="text" /></label>
        <label class="field"><span>Porta auxiliares</span><input v-model.number="runtimeForm.auxiliary_port" type="number" min="1" max="65535" /></label>
        <label class="field"><span>Host bridge/core</span><input v-model="runtimeForm.bridge_core_host" type="text" /></label>
        <label class="field"><span>Porta bridge/core</span><input v-model.number="runtimeForm.bridge_core_port" type="number" min="1" max="65535" /></label>
      </div>

      <div class="switch-grid">
        <AppSwitch v-model="runtimeForm.internal_api_auto_start" label="Iniciar API interna com a aplicação" />
        <AppSwitch v-model="runtimeForm.internal_api_require_token" label="Exigir token na API interna" />
        <AppSwitch v-model="runtimeForm.internal_api_docs_public_local" label="Scalar público em localhost" />
        <AppSwitch v-model="runtimeForm.internal_api_docs_enabled" label="Documentação Scalar habilitada" />
        <AppSwitch v-model="runtimeForm.services_auto_start" label="Iniciar serviços internos automaticamente" />
        <AppSwitch v-model="runtimeForm.tray_enabled" label="Habilitar tray icon" />
        <AppSwitch v-model="runtimeForm.minimize_to_tray" label="Minimizar para bandeja" />
        <AppSwitch v-model="runtimeForm.close_to_tray" label="Fechar para bandeja" />
        <AppSwitch v-model="runtimeForm.start_with_windows" label="Iniciar com o Windows" />
      </div>

      <div class="actions align-end">
        <button class="primary" :disabled="loading" @click="persistRuntime">Salvar runtime/.env</button>
      </div>

      <div class="info-grid compact-info">
        <div v-for="item in ports" :key="String(item.key)" class="info-item">
          <strong>{{ item.label }}</strong>
          <code>{{ item.host }}:{{ item.effective_port }} · padrão {{ item.default_port }} · {{ item.available ? 'livre' : 'fallback' }}</code>
        </div>
      </div>
    </BaseSectionCard>

    <div class="grid columns-2 mobile-columns-1">
      <BaseSectionCard title="Administração avançada" subtitle="Recursos administrativos preservados para instalações existentes.">
        <details><summary>Mostrar recursos adicionais</summary>
          <div class="actions top-gap-12">
            <RouterLink class="secondary link-button" to="/api-interna">API local</RouterLink>
            <RouterLink class="secondary link-button" to="/licenciamento">Licenciamento</RouterLink>
            <RouterLink class="secondary link-button" to="/integracoes">Integrações</RouterLink>
            <RouterLink class="secondary link-button" to="/sincronizacao">Sincronização</RouterLink>
          </div>
        </details>
      </BaseSectionCard>
      <BaseSectionCard title="Execução e serviços">
        <div class="info-grid compact-info">
          <div class="info-item"><strong>Banco</strong><code>{{ databaseConfig.driver }}</code></div>
          <div class="info-item"><strong>API interna</strong><code>{{ internalApiConfig.enabled ? `${runtimeForm.internal_api_host}:${runtimeForm.internal_api_port}` : 'desativada' }}</code></div>
          <div class="info-item"><strong>Tray</strong><code>{{ runtimeForm.tray_enabled ? 'ativo' : 'oculto' }}</code></div>
          <div class="info-item"><strong>Sidebar</strong><code>{{ sidebarConfig.menuBehavior }}</code></div>
        </div>
      </BaseSectionCard>
    </div>
  </BasePage>
</template>
