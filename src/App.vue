<script setup lang="ts">
import { onErrorCaptured, onMounted, reactive, ref } from "vue";
import { logAppError } from "./services/logger";
import AppSplash from "./components/AppSplash.vue";
import AppStartupSplash from "./components/AppStartupSplash.vue";
import { showSplashError } from "./services/splash";
import { runApplicationStartup, type StartupState } from "./services/startup";

const fatalError = ref("");
const startupVisible = ref(true);
const startupState = reactive<StartupState>({
  progress: 0,
  message: "Inicializando aplicação...",
  detail: "Preparando ambiente local",
});

function resolveComponentName(instance: unknown): string | null {
  const raw = instance as { type?: { name?: string }; $options?: { name?: string } } | null;
  return raw?.type?.name ?? raw?.$options?.name ?? null;
}

onMounted(async () => {
  try {
    await runApplicationStartup((state) => {
      startupState.progress = state.progress;
      startupState.message = state.message;
      startupState.detail = state.detail;
    });
    startupVisible.value = false;
  } catch (error) {
    fatalError.value = error instanceof Error ? error.message : "Falha ao inicializar aplicação.";
    showSplashError(fatalError.value);
  }
});

onErrorCaptured((error, instance, info) => {
  fatalError.value = error instanceof Error ? error.message : "Falha inesperada na interface.";
  logAppError("vue", "Erro capturado no componente raiz.", {
    info,
    component: resolveComponentName(instance),
    error: fatalError.value,
  });
  showSplashError(fatalError.value);
  return false;
});
</script>

<template>
  <AppStartupSplash
    v-if="startupVisible && !fatalError"
    :progress="startupState.progress"
    :message="startupState.message"
    :detail="startupState.detail"
  />
  <template v-else>
    <AppSplash />
    <div v-if="fatalError" class="fatal-screen">
      <div class="fatal-card">
        <h2>Erro ao renderizar a aplicação</h2>
        <p>{{ fatalError }}</p>
        <p class="muted-text">Abra a página de logs após reiniciar o sistema para obter mais detalhes.</p>
      </div>
    </div>
    <router-view v-else />
  </template>
</template>
