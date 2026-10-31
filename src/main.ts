import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { registerServiceWorker } from "./core/pwa/registerServiceWorker";
import router from "./router";
import "./styles.css";
import { useSessionStore } from "./stores/session";
import { logAppError, logAppInfo } from "./services/logger";
import { showSplashError, showSplashWarning } from "./services/splash";

function resolveComponentName(instance: unknown): string | null {
  const raw = instance as { type?: { name?: string }; $options?: { name?: string } } | null;
  return raw?.type?.name ?? raw?.$options?.name ?? null;
}

async function bootstrap() {
  const app = createApp(App);
  const pinia = createPinia();
  app.use(pinia);
  app.use(router);

  app.config.errorHandler = (error, instance, info) => {
    logAppError("vue", "Erro global capturado pelo Vue.", {
      info,
      component: resolveComponentName(instance),
      error: error instanceof Error ? error.message : String(error),
    });
    console.error(error);
    showSplashError(error instanceof Error ? error.message : String(error));
  };

  window.addEventListener("error", (event) => {
    logAppError("window", "Erro global de janela.", {
      message: event.message,
      file: event.filename,
      line: event.lineno,
      column: event.colno,
    });
    showSplashError(event.message || "Erro global da aplicação.");
  });

  window.addEventListener("unhandledrejection", (event) => {
    logAppError("promise", "Promise rejeitada sem tratamento.", {
      reason: event.reason instanceof Error ? event.reason.message : String(event.reason),
    });
    showSplashWarning(event.reason instanceof Error ? event.reason.message : String(event.reason));
  });

  const session = useSessionStore(pinia);
  void session;

  try {
    await Promise.race([
      router.isReady(),
      new Promise((resolve) => window.setTimeout(resolve, 3500)),
    ]);
    app.mount("#app");
    logAppInfo("bootstrap", "Aplicação inicializada com sucesso.");
  } catch (error) {
    logAppError("bootstrap", "Falha ao montar aplicação.", {
      error: error instanceof Error ? error.message : String(error),
    });
    const target = document.querySelector("#app");
    if (target) {
      target.innerHTML = `<div style="padding:24px;font-family:Segoe UI,Arial,sans-serif;color:#1f2937"><h2>Falha ao inicializar a aplicação</h2><p>Consulte a página de logs após reiniciar o sistema.</p></div>`;
    }
  }
}

void bootstrap();

void registerServiceWorker();
