<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import BasePage from "../../components/base/BasePage.vue";
import BaseSectionCard from "../../components/base/BaseSectionCard.vue";
import { getInternalApiStatus, startInternalApi } from "../../services/crud";

const loading = ref(false);
const error = ref("");
const message = ref("");
const status = ref<Record<string, any>>({});
const frameKey = ref(0);

const running = computed(() => Boolean(status.value.running));
const docsUrl = computed(() => String(status.value.docs_url || status.value.settings?.internal_api_docs_url || "http://127.0.0.1:61001/docs"));
const baseUrl = computed(() => String(status.value.url || status.value.settings?.internal_api_base_url || "http://127.0.0.1:61001"));
const tokenInfo = computed(() => status.value.token_required ? "Token obrigatório" : "Token desativado para ambiente local controlado");

async function load() {
  loading.value = true;
  error.value = "";
  try {
    status.value = await getInternalApiStatus();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao carregar status da API interna.";
  } finally {
    loading.value = false;
  }
}

async function startApi() {
  loading.value = true;
  message.value = "";
  error.value = "";
  try {
    const settings = status.value.settings || {};
    await startInternalApi(settings.internal_api_host, settings.internal_api_port);
    message.value = "API iniciada. A documentação Scalar será carregada dentro da aplicação.";
    await load();
    reloadDocs();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Falha ao iniciar API interna.";
  } finally {
    loading.value = false;
  }
}

function reloadDocs() {
  frameKey.value += 1;
}

function openExternal() {
  window.open(docsUrl.value, "_blank", "noopener,noreferrer");
}

onMounted(load);
</script>

<template>
  <BasePage title="Documentação Scalar" subtitle="Documentação dedicada da API interna em tema claro, isolada do modo escuro da aplicação.">
    <template #actions>
      <button class="secondary" :disabled="loading" @click="load">Atualizar status</button>
      <button class="secondary" :disabled="loading || !running" @click="reloadDocs">Recarregar documentação</button>
      <button class="primary" :disabled="!docsUrl" @click="openExternal">Abrir no navegador</button>
    </template>

    <div v-if="message" class="alert success">{{ message }}</div>
    <div v-if="error" class="alert error">{{ error }}</div>

    <BaseSectionCard title="Scalar da API interna" :subtitle="`${baseUrl} · ${tokenInfo}`">
      <div class="scalar-toolbar">
        <div><strong>URL atual:</strong> <code>{{ docsUrl }}</code></div>
        <span class="module-chip" :class="running ? 'enabled' : 'disabled'">{{ running ? 'API rodando' : 'API parada' }}</span>
      </div>

      <div v-if="!running" class="scalar-empty light-only-panel">
        <h2>API interna parada</h2>
        <p>A documentação Scalar depende da API local. Inicie a API para abrir a documentação dentro da aplicação.</p>
        <button class="primary" :disabled="loading" @click="startApi">Iniciar API</button>
      </div>

      <div v-else class="scalar-dedicated-shell light-only-panel">
        <iframe :key="frameKey" class="scalar-frame dedicated" :src="docsUrl" title="Documentação Scalar da API interna"></iframe>
      </div>
    </BaseSectionCard>
  </BasePage>
</template>
