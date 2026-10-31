export type RuntimeMode = "desktop" | "headless-api" | "windows-service" | "linux-service" | "cli" | "worker";
export type SidebarDefaultState = "expanded" | "collapsed";
export type MenuBehavior = "single-open" | "multi-open" | "free";
export type SubmenuVisualMode = "default" | "tree" | "compact" | "indented";
export type DatabaseDriver = "sqlite" | "mysql" | "postgres";

export const projectConfig = {
  app: {
    name: "Scale Simulator",
    shortName: "Scale Simulator",
    productName: "Scale Simulator",
    windowTitle: "Scale Simulator",
    subtitle: "Laboratório de pesagem • TCP / RS232",
    description: "Simulação de balanças e indicadores para testar integrações TCP e RS232.",
    version: "0.2.1",
    mode: "desktop" as RuntimeMode,
    identifier: "br.com.wkarts.scale-simulator-tauri",
    developer: "Wallace Kleiton",
    localDataDir: "scale_simulator_tauri",
    storagePrefix: "scale-simulator",
    supportUrl: "",
    documentationUrl: "",
  },
  features: {
    licensing: true,
    about: true,
    userGuide: true,
    logs: true,
    systemSettings: true,
    genericEntities: true,
    technicalSheet: true,
    sync: true,
    internalApi: true,
    scalarDocs: true,
    databaseSettings: true,
    integrations: true,
    tray: true,
    windowsService: true,
    linuxService: true,
    autoStartWithWindows: false,
    headlessMode: true,
    printPreview: true,
  },
  defaultAdmin: {
    enabled: true,
    username: "admin",
    password: "", // No credential is embedded in frontend metadata.
    forcePasswordChangeOnFirstLogin: true,
  },
  database: {
    driver: "sqlite" as DatabaseDriver,
    sqlite: { path: "app.db" },
    mysql: { host: "127.0.0.1", port: 3306, database: "template_app", username: "root", password: "" },
    postgres: { host: "127.0.0.1", port: 5432, database: "template_app", username: "postgres", password: "" },
    firebird: {
      supported: false,
      scope: "out-of-scope",
      note: "Firebird ignorado por compatibilidade nesta etapa.",
    },
  },
  dashboard: {
    enabled: true,
    demoMode: false,
    showSystemCards: true,
    showBusinessCards: true,
    showIntegrationCards: true,
    showCharts: true,
    blocks: {
      systemHealth: true,
      userStats: true,
      companyStats: true,
      licensingStatus: true,
      internalApiStatus: true,
      integrationStatus: true,
      syncStatus: false,
      financialSummary: false,
      customBusinessCards: false,
    },
  },
  api: {
    enabled: true,
    autoStart: false,
    restartOnConfigChange: true,
    host: "127.0.0.1",
    port: 61001,
    baseUrl: "http://127.0.0.1:61001",
    scalarUrl: "http://127.0.0.1:61001/docs",
    docsPath: "/docs",
    timeoutMs: 8000,
    logMode: "normal",
    openScalarAfterStart: false,
    docs: true,
    docsProvider: "scalar" as const,
    security: {
      bindHost: "127.0.0.1",
      allowPublicNetwork: false,
      requireToken: false,
      tokenHeader: "X-App-Token",
      corsEnabled: false,
      rateLimitEnabled: false,
      docsPublic: false,
      docsPublicLocal: true,
    },
  },
  sidebar: {
    defaultState: "expanded" as SidebarDefaultState,
    allowCollapse: true,
    menuBehavior: "single-open" as MenuBehavior,
    keepActiveParentExpanded: true,
    collapsePreviousOnNewSelection: true,
    persistUserMenuState: false,
    showSubmenuTreeLine: true,
    submenuVisualMode: "tree" as SubmenuVisualMode,
  },
  tray: {
    enabled: true,
    minimizeToTray: true,
    closeToTray: false,
    alwaysUseTray: false,
    askBeforeExit: true,
    showServiceControls: true,
    showInternalApiStatus: true,
  },
  startup: {
    enabled: false,
    mode: "disabled" as "disabled" | "user-login" | "machine-startup",
  },
  integrations: {
    enabled: true,
    allowExternalApis: true,
    allowWebhooks: true,
    allowTokens: true,
    allowRequestLogs: true,
    allowRetryQueue: true,
  },
};

export const appFeatures = projectConfig.features;
export const sidebarConfig = projectConfig.sidebar;
export const dashboardConfig = projectConfig.dashboard;
export const internalApiConfig = projectConfig.api;
export const databaseConfig = projectConfig.database;

export const trayConfig = projectConfig.tray;
export const startupConfig = projectConfig.startup;
export const integrationsConfig = projectConfig.integrations;