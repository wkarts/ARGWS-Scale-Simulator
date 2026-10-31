<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from "vue";
import { RouterLink, RouterView, useRoute, useRouter } from "vue-router";
import { getAppMeta } from "../services/crud";
import { useSessionStore } from "../stores/session";
import logoMark from "../assets/branding/logo-mark.png";
import { logAppError, logAppInfo } from "../services/logger";
import AppSwitch from "../components/AppSwitch.vue";
import { projectConfig, sidebarConfig } from "../config/projectConfig";
import { appBranding, storageKey } from "../config/appBranding";
import { visibleMenuItems, type MenuSection } from "../config/navigation";

const session = useSessionStore();
const router = useRouter();
const route = useRoute();

const meta = reactive({ version: projectConfig.app.version, build_hash: "dev" });
const now = ref(new Date());
const isDarkMode = ref(false);
const sidebarOpen = ref(sidebarConfig.defaultState === "expanded");
const brandLogoFailed = ref(false);
let timer: number | undefined;

const MENU_STATE_KEY = storageKey("menu-state");
const THEME_PREFERENCE_KEY = storageKey("theme");

function readMenuState() {
  if (!sidebarConfig.persistUserMenuState || typeof window === "undefined") return null;
  try { return JSON.parse(window.localStorage.getItem(MENU_STATE_KEY) || "null"); } catch { return null; }
}
function readThemePreference(): "light" | "dark" {
  if (typeof window === "undefined") return "light";
  return window.localStorage.getItem(THEME_PREFERENCE_KEY) === "dark" ? "dark" : "light";
}
function applyTheme(theme: "light" | "dark") {
  if (typeof document !== "undefined") document.documentElement.setAttribute("data-theme", theme);
}

const savedMenuState = readMenuState();
const groupState = reactive<Record<MenuSection, boolean>>({
  dashboard: savedMenuState?.dashboard ?? true,
  cadastro: savedMenuState?.cadastro ?? false,
  simulador: savedMenuState?.simulador ?? true,
  sistema: savedMenuState?.sistema ?? false,
  ferramentas: savedMenuState?.ferramentas ?? false,
  documentacao: savedMenuState?.documentacao ?? false,
});

const sections: { key: MenuSection; title: string; subtitle: string }[] = [
  { key: "dashboard", title: "Visão geral", subtitle: "visão geral" },
  { key: "cadastro", title: "Administração", subtitle: "contas e empresas" },
  { key: "simulador", title: "Balanças", subtitle: "TCP e RS232" },
  { key: "sistema", title: "Ferramentas", subtitle: "configuração e diagnóstico" },
  { key: "ferramentas", title: "Ferramentas", subtitle: "runtime e integrações" },
  { key: "documentacao", title: "Documentação", subtitle: "apoio ao usuário" },
];

const menuBySection = computed(() => {
  const items = visibleMenuItems().filter((item) => !item.permission || session.can(item.permission));
  return sections.map((section) => ({ ...section, items: items.filter((item) => item.section === section.key) })).filter((section) => section.items.length);
});

const pageTitle = computed(() => visibleMenuItems().find((item) => item.route === route.path)?.title || appBranding.appName);
const dateLabel = computed(() => new Intl.DateTimeFormat("pt-BR", { dateStyle: "full" }).format(now.value));
const timeLabel = computed(() => new Intl.DateTimeFormat("pt-BR", { timeStyle: "medium" }).format(now.value));
const activeCompanyLabel = computed(() => session.activeCompanyName || "Empresa geral");

function toggleGroup(key: MenuSection) {
  if (sidebarConfig.menuBehavior === "single-open" && sidebarConfig.collapsePreviousOnNewSelection) {
    for (const section of sections) groupState[section.key] = section.key === key ? !groupState[key] : false;
    return;
  }
  groupState[key] = !groupState[key];
}

async function logout() {
  await session.logout();
  router.push("/login");
}
function closeSidebarOnMobile() { if (window.innerWidth <= 1100) sidebarOpen.value = false; }
function handleBrandLogoError() {
  brandLogoFailed.value = true;
  logAppError("assets", "Falha ao carregar logo-mark.png; fallback visual aplicado.", { asset: "src/assets/branding/logo-mark.png" });
}

function ensureActiveGroup() {
  const current = visibleMenuItems().find((item) => item.route === route.path);
  if (current && sidebarConfig.keepActiveParentExpanded) groupState[current.section] = true;
}

watch(() => route.fullPath, () => { ensureActiveGroup(); closeSidebarOnMobile(); });
watch(groupState, (value) => {
  if (sidebarConfig.persistUserMenuState) localStorage.setItem(MENU_STATE_KEY, JSON.stringify(value));
}, { deep: true });
watch(isDarkMode, (enabled) => {
  const theme = enabled ? "dark" : "light";
  applyTheme(theme);
  localStorage.setItem(THEME_PREFERENCE_KEY, theme);
});

onMounted(async () => {
  const savedTheme = readThemePreference();
  isDarkMode.value = savedTheme === "dark";
  applyTheme(savedTheme);
  try {
    const payload = await getAppMeta();
    meta.version = String(payload.version || meta.version);
    meta.build_hash = String(payload.build_hash || meta.build_hash);
  } catch (error) {
    logAppError("layout", "Falha ao carregar metadados da aplicação.", { error: error instanceof Error ? error.message : String(error) });
  }
  timer = window.setInterval(() => { now.value = new Date(); }, 1000);
  ensureActiveGroup();
  closeSidebarOnMobile();
  logAppInfo("layout", "Shell principal montado.");
});

onBeforeUnmount(() => { if (timer) window.clearInterval(timer); });
</script>

<template>
  <div class="shell-root" :class="{ 'sidebar-collapsed': !sidebarOpen }">
    <div v-if="sidebarOpen" class="shell-overlay" @click="sidebarOpen = false" />

    <aside class="sidebar" :class="{ open: sidebarOpen }">
      <div class="brand-box">
        <img v-if="!brandLogoFailed" :src="logoMark" :alt="appBranding.appName" class="brand-mark" @error="handleBrandLogoError" />
        <div v-else class="brand-mark fallback-logo">{{ appBranding.shortName.slice(0, 2).toUpperCase() }}</div>
        <div class="brand-copy">
          <h1>{{ appBranding.appName }}</h1>
          <div class="muted-light">{{ appBranding.subtitle }}</div>
        </div>
      </div>

      <div class="sidebar-scroll">
        <div v-for="section in menuBySection" :key="section.key" class="menu-group">
          <button class="menu-group-button" @click="toggleGroup(section.key)">
            <span>{{ section.title }}</span>
            <small>{{ section.subtitle }}</small>
          </button>
          <div v-show="groupState[section.key]" class="menu-links submenu-links" :class="`submenu-${sidebarConfig.submenuVisualMode}`">
            <RouterLink v-for="item in section.items" :key="item.route" :to="item.route">{{ item.title }}</RouterLink>
          </div>
        </div>
      </div>

      <div class="sidebar-footer-fixed">
        <div class="clock-box">
          <div class="clock-time">{{ timeLabel }}</div>
          <div class="clock-date">{{ dateLabel }}</div>
        </div>
        <div class="build-box">
          <div><strong>Versão</strong> {{ meta.version }}</div>
          <div v-if="meta.build_hash && meta.build_hash !== 'dev'"><strong>Build</strong> {{ meta.build_hash }}</div>
        </div>
        <div class="user-box enhanced">
          <img v-if="session.user?.photo_url" :src="session.user.photo_url" class="user-avatar-img" alt="Foto do usuário" />
          <div v-else class="user-avatar">{{ (session.user?.nome || session.user?.login || 'U').slice(0, 1).toUpperCase() }}</div>
          <div>
            <strong>{{ session.user?.nome || "Sem usuário" }}</strong>
            <small>{{ activeCompanyLabel }}</small>
          </div>
        </div>
        <div class="actions compact-actions">
          <RouterLink class="secondary link-button" to="/sistema">Config.</RouterLink>
          <button class="secondary" @click="logout">Sair</button>
        </div>
      </div>
    </aside>

    <div class="shell-main">
      <header class="topbar">
        <div class="topbar-left">
          <button v-if="sidebarConfig.allowCollapse" class="icon-button" aria-label="Abrir ou recolher navegação" @click="sidebarOpen = !sidebarOpen">☰</button>
          <div>
            <div class="page-title">{{ pageTitle }}</div>
            <div class="page-subtitle">Laboratório de pesagem</div>
          </div>
        </div>
        <div class="topbar-right">
          <AppSwitch v-model="isDarkMode" label="Modo escuro" />
          <label v-if="(session.user?.company_ids?.length || 0) > 1" class="company-selector">
            <span>Empresa ativa</span>
            <select :value="session.activeCompanyId ?? ''" @change="session.setActiveCompany(Number(($event.target as HTMLSelectElement).value) || null)">
              <option v-if="session.isMaster" value="">Todas / geral</option>
              <option v-for="company in session.user?.company_ids || []" :key="company" :value="company">
                {{ session.user?.company_names?.[session.user.company_ids.indexOf(company)] || `Empresa ${company}` }}
              </option>
            </select>
          </label>
        </div>
      </header>

      <main class="content-area">
        <RouterView />
      </main>
    </div>
  </div>
</template>
