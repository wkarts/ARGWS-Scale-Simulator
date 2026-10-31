use std::{
    collections::{BTreeMap, HashSet},
    env,
    fs::{self, OpenOptions},
    io::Write,
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
};

use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{app_state::SharedState, db::open_connection};

pub const ENV_FILE_NAME: &str = ".env";
pub const INTERNAL_API_DEFAULT_PORT: u16 = 61001;
pub const LOCAL_WEB_DEFAULT_PORT: u16 = 61002;
pub const AUXILIARY_DEFAULT_PORT: u16 = 61003;
pub const BRIDGE_CORE_DEFAULT_PORT: u16 = 61004;

struct RuntimePortDefinition {
    key: &'static str,
    label: &'static str,
    env_host_key: &'static str,
    env_port_key: &'static str,
    default_port: u16,
    description: &'static str,
}

const INTERNAL_API_PORT_DEFINITION: RuntimePortDefinition = RuntimePortDefinition {
    key: "internal_api",
    label: "API interna",
    env_host_key: "TEMPLATE_APP_API_HOST",
    env_port_key: "TEMPLATE_APP_API_PORT",
    default_port: INTERNAL_API_DEFAULT_PORT,
    description: "API local Axum usada pelo template em modo desktop/headless.",
};

const LOCAL_WEB_PORT_DEFINITION: RuntimePortDefinition = RuntimePortDefinition {
    key: "local_web",
    label: "Servidor web local",
    env_host_key: "TEMPLATE_APP_WEB_HOST",
    env_port_key: "TEMPLATE_APP_WEB_PORT",
    default_port: LOCAL_WEB_DEFAULT_PORT,
    description: "Servidor web/preview usado em desenvolvimento ou distribuição web local.",
};

const AUXILIARY_PORT_DEFINITION: RuntimePortDefinition = RuntimePortDefinition {
    key: "auxiliary",
    label: "Serviços auxiliares",
    env_host_key: "TEMPLATE_APP_AUX_HOST",
    env_port_key: "TEMPLATE_APP_AUX_PORT",
    default_port: AUXILIARY_DEFAULT_PORT,
    description: "Faixa reservada para jobs, filas, webhooks ou workers locais.",
};

const BRIDGE_CORE_PORT_DEFINITION: RuntimePortDefinition = RuntimePortDefinition {
    key: "bridge_core",
    label: "Bridge/core local",
    env_host_key: "TEMPLATE_APP_BRIDGE_HOST",
    env_port_key: "TEMPLATE_APP_BRIDGE_PORT",
    default_port: BRIDGE_CORE_DEFAULT_PORT,
    description: "Bridge local entre UI, backend embarcado e integrações nativas.",
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeSettings {
    pub internal_api_host: String,
    pub internal_api_port: u16,
    pub internal_api_base_url: String,
    pub internal_api_docs_url: String,
    pub internal_api_auto_start: bool,
    pub internal_api_restart_on_config_change: bool,
    pub internal_api_require_token: bool,
    pub internal_api_token: String,
    pub internal_api_docs_public_local: bool,
    pub internal_api_open_scalar_after_start: bool,
    pub internal_api_timeout_ms: u64,
    pub internal_api_log_mode: String,
    pub internal_api_docs_enabled: bool,
    pub internal_api_docs_path: String,
    pub local_web_host: String,
    pub local_web_port: u16,
    pub auxiliary_host: String,
    pub auxiliary_port: u16,
    pub bridge_core_host: String,
    pub bridge_core_port: u16,
    pub tray_enabled: bool,
    pub minimize_to_tray: bool,
    pub close_to_tray: bool,
    pub start_with_windows: bool,
    pub services_auto_start: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeServicePort {
    pub key: String,
    pub label: String,
    pub env_host_key: String,
    pub env_port_key: String,
    pub default_port: u16,
    pub host: String,
    pub configured_port: u16,
    pub effective_port: u16,
    pub available: bool,
    pub fallback_applied: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeSettingsPayload {
    pub env_path: String,
    pub settings: RuntimeSettings,
    pub ports: Vec<RuntimeServicePort>,
    pub warnings: Vec<String>,
}

impl Default for RuntimeSettings {
    fn default() -> Self {
        Self {
            internal_api_host: env_string("TEMPLATE_APP_API_HOST", "127.0.0.1"),
            internal_api_port: env_u16("TEMPLATE_APP_API_PORT", INTERNAL_API_DEFAULT_PORT),
            internal_api_base_url: env_string(
                "TEMPLATE_APP_API_BASE_URL",
                "http://127.0.0.1:61001",
            ),
            internal_api_docs_url: env_string(
                "TEMPLATE_APP_API_SCALAR_URL",
                "http://127.0.0.1:61001/docs",
            ),
            internal_api_auto_start: env_bool("TEMPLATE_APP_API_AUTO_START", false),
            internal_api_restart_on_config_change: env_bool(
                "TEMPLATE_APP_API_RESTART_ON_CONFIG_CHANGE",
                true,
            ),
            internal_api_require_token: env_bool("TEMPLATE_APP_API_REQUIRE_TOKEN", false),
            internal_api_token: env_string("TEMPLATE_APP_API_TOKEN", ""),
            internal_api_docs_public_local: env_bool("TEMPLATE_APP_API_DOCS_PUBLIC_LOCAL", true),
            internal_api_open_scalar_after_start: env_bool(
                "TEMPLATE_APP_API_OPEN_SCALAR_AFTER_START",
                false,
            ),
            internal_api_timeout_ms: env_u64("TEMPLATE_APP_API_TIMEOUT_MS", 8000),
            internal_api_log_mode: env_string("TEMPLATE_APP_API_LOG_MODE", "normal"),
            internal_api_docs_enabled: env_bool("TEMPLATE_APP_API_SCALAR_ENABLED", true),
            internal_api_docs_path: env_string("TEMPLATE_APP_API_DOCS_PATH", "/docs"),
            local_web_host: env_string("TEMPLATE_APP_WEB_HOST", "127.0.0.1"),
            local_web_port: env_u16("TEMPLATE_APP_WEB_PORT", LOCAL_WEB_DEFAULT_PORT),
            auxiliary_host: env_string("TEMPLATE_APP_AUX_HOST", "127.0.0.1"),
            auxiliary_port: env_u16("TEMPLATE_APP_AUX_PORT", AUXILIARY_DEFAULT_PORT),
            bridge_core_host: env_string("TEMPLATE_APP_BRIDGE_HOST", "127.0.0.1"),
            bridge_core_port: env_u16("TEMPLATE_APP_BRIDGE_PORT", BRIDGE_CORE_DEFAULT_PORT),
            tray_enabled: env_bool("TEMPLATE_APP_TRAY_ENABLED", true),
            minimize_to_tray: env_bool("TEMPLATE_APP_TRAY_MINIMIZE_TO_TRAY", true),
            close_to_tray: env_bool("TEMPLATE_APP_TRAY_CLOSE_TO_TRAY", false),
            start_with_windows: env_bool("TEMPLATE_APP_START_WITH_WINDOWS", false),
            services_auto_start: env_bool("TEMPLATE_APP_SERVICES_AUTO_START", false),
        }
    }
}

pub fn init_env_file(state: Option<&SharedState>) -> Result<PathBuf, String> {
    let path = env_file_path(state)?;
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("Falha ao criar diretório do .env: {err}"))?;
        }
        fs::write(&path, default_env_template())
            .map_err(|err| format!("Falha ao criar .env padrão em {}: {err}", path.display()))?;
    }
    load_env_file(&path)?;
    Ok(path)
}

pub fn load_env_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let raw = fs::read_to_string(path).map_err(|err| format!("Falha ao ler .env: {err}"))?;
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || !trimmed.contains('=') {
            continue;
        }
        let mut parts = trimmed.splitn(2, '=');
        let key = parts.next().unwrap_or_default().trim();
        let value = parts
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if !key.is_empty() && env::var(key).is_err() {
            env::set_var(key, value);
        }
    }
    Ok(())
}

pub fn env_file_path(state: Option<&SharedState>) -> Result<PathBuf, String> {
    if let Ok(path) = env::var("TEMPLATE_APP_ENV_FILE") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }
    if let Some(state) = state {
        if let Ok(data_dir) = state.data_dir() {
            return Ok(data_dir.join(ENV_FILE_NAME));
        }
    }
    let config_dir = dirs::config_local_dir()
        .or_else(|| env::current_dir().ok())
        .ok_or_else(|| "Não foi possível resolver diretório para .env.".to_string())?;
    Ok(config_dir.join("template_app_tauri").join(ENV_FILE_NAME))
}

pub fn load_settings(state: &SharedState) -> Result<RuntimeSettingsPayload, String> {
    let env_path = init_env_file(Some(state))?;
    let mut settings = RuntimeSettings::default();
    let db_path = state.db_path()?;
    let conn = open_connection(&db_path)?;
    if let Some(raw) = conn
        .query_row(
            "SELECT valor FROM app_settings WHERE chave = 'runtime.settings' LIMIT 1",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|err| format!("Falha ao ler configurações runtime: {err}"))?
    {
        if !raw.trim().is_empty() {
            settings = serde_json::from_str(&raw)
                .map_err(|err| format!("Configuração runtime inválida no banco: {err}"))?;
        }
    }
    normalize_runtime_settings(&mut settings);
    let (ports, warnings) = build_ports(&settings);
    Ok(RuntimeSettingsPayload {
        env_path: env_path.to_string_lossy().to_string(),
        settings,
        ports,
        warnings,
    })
}

pub fn save_settings(
    state: &SharedState,
    mut settings: RuntimeSettings,
) -> Result<RuntimeSettingsPayload, String> {
    normalize_runtime_settings(&mut settings);
    validate_port_range(&settings)?;
    let env_path = init_env_file(Some(state))?;
    let (ports, warnings) = build_ports(&settings);
    if has_duplicate_configured_ports(&ports) {
        return Err("Há portas configuradas em conflito. Ajuste antes de salvar.".to_string());
    }
    let now = Utc::now().to_rfc3339();
    let db_path = state.db_path()?;
    let conn = open_connection(&db_path)?;
    let serialized = serde_json::to_string(&settings).map_err(|err| err.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (chave, valor, created_at, updated_at) VALUES ('runtime.settings', ?1, COALESCE((SELECT created_at FROM app_settings WHERE chave='runtime.settings'), ?2), ?2)",
        params![serialized, now],
    )
    .map_err(|err| format!("Falha ao persistir configurações runtime: {err}"))?;
    write_env_file(&env_path, &settings)?;
    normalize_runtime_settings(&mut settings);
    Ok(RuntimeSettingsPayload {
        env_path: env_path.to_string_lossy().to_string(),
        settings,
        ports,
        warnings,
    })
}

pub fn default_env_template() -> String {
    r#"# Template App - configuração local
# A porta padrão do Tauri (devUrl 1420) não é alterada por este arquivo.
TEMPLATE_APP_API_HOST=127.0.0.1
TEMPLATE_APP_API_PORT=61001
TEMPLATE_APP_API_AUTO_START=false
TEMPLATE_APP_API_BASE_URL=http://127.0.0.1:61001
TEMPLATE_APP_API_SCALAR_ENABLED=true
TEMPLATE_APP_API_SCALAR_URL=http://127.0.0.1:61001/docs
TEMPLATE_APP_API_DOCS_PATH=/docs
TEMPLATE_APP_API_DOCS_PUBLIC_LOCAL=true
TEMPLATE_APP_API_RESTART_ON_CONFIG_CHANGE=true
TEMPLATE_APP_API_OPEN_SCALAR_AFTER_START=false
TEMPLATE_APP_API_TIMEOUT_MS=8000
TEMPLATE_APP_API_LOG_MODE=normal
TEMPLATE_APP_WEB_HOST=127.0.0.1
TEMPLATE_APP_WEB_PORT=61002
TEMPLATE_APP_AUX_HOST=127.0.0.1
TEMPLATE_APP_AUX_PORT=61003
TEMPLATE_APP_BRIDGE_HOST=127.0.0.1
TEMPLATE_APP_BRIDGE_PORT=61004
TEMPLATE_APP_TRAY_ENABLED=true
TEMPLATE_APP_TRAY_MINIMIZE_TO_TRAY=true
TEMPLATE_APP_TRAY_CLOSE_TO_TRAY=false
TEMPLATE_APP_START_WITH_WINDOWS=false
TEMPLATE_APP_SERVICES_AUTO_START=false
TEMPLATE_APP_API_ALLOW_PUBLIC_NETWORK=false
TEMPLATE_APP_API_REQUIRE_TOKEN=false
TEMPLATE_APP_API_TOKEN=
TEMPLATE_APP_API_TOKEN_HEADER=X-App-Token
# Informe TEMPLATE_APP_API_TOKEN via secret/env real em produção; não commite tokens.
"#
    .to_string()
}

fn write_env_file(path: &Path, settings: &RuntimeSettings) -> Result<(), String> {
    let content = format!(
        "# Template App - gerado pela tela Sistema e parâmetros\n\
TEMPLATE_APP_API_HOST={}\n\
TEMPLATE_APP_API_PORT={}\n\
TEMPLATE_APP_API_BASE_URL={}\n\
TEMPLATE_APP_API_AUTO_START={}\n\
TEMPLATE_APP_API_RESTART_ON_CONFIG_CHANGE={}\n\
TEMPLATE_APP_API_REQUIRE_TOKEN={}\n\
TEMPLATE_APP_API_TOKEN={}\n\
TEMPLATE_APP_API_DOCS_PUBLIC_LOCAL={}\n\
TEMPLATE_APP_API_OPEN_SCALAR_AFTER_START={}\n\
TEMPLATE_APP_API_TIMEOUT_MS={}\n\
TEMPLATE_APP_API_LOG_MODE={}\n\
TEMPLATE_APP_API_SCALAR_ENABLED={}\n\
TEMPLATE_APP_API_SCALAR_URL={}\n\
TEMPLATE_APP_API_DOCS_PATH={}\n\
TEMPLATE_APP_WEB_HOST={}\n\
TEMPLATE_APP_WEB_PORT={}\n\
TEMPLATE_APP_AUX_HOST={}\n\
TEMPLATE_APP_AUX_PORT={}\n\
TEMPLATE_APP_BRIDGE_HOST={}\n\
TEMPLATE_APP_BRIDGE_PORT={}\n\
TEMPLATE_APP_TRAY_ENABLED={}\n\
TEMPLATE_APP_TRAY_MINIMIZE_TO_TRAY={}\n\
TEMPLATE_APP_TRAY_CLOSE_TO_TRAY={}\n\
TEMPLATE_APP_START_WITH_WINDOWS={}\n\
TEMPLATE_APP_SERVICES_AUTO_START={}\n\
TEMPLATE_APP_API_ALLOW_PUBLIC_NETWORK={}\n\
TEMPLATE_APP_API_TOKEN_HEADER={}\n\
# Informe TEMPLATE_APP_API_TOKEN via secret/env real em produção; não commite tokens.\n",
        settings.internal_api_host,
        settings.internal_api_port,
        settings.internal_api_base_url,
        settings.internal_api_auto_start,
        settings.internal_api_restart_on_config_change,
        settings.internal_api_require_token,
        settings.internal_api_token,
        settings.internal_api_docs_public_local,
        settings.internal_api_open_scalar_after_start,
        settings.internal_api_timeout_ms,
        settings.internal_api_log_mode,
        settings.internal_api_docs_enabled,
        settings.internal_api_docs_url,
        settings.internal_api_docs_path,
        settings.local_web_host,
        settings.local_web_port,
        settings.auxiliary_host,
        settings.auxiliary_port,
        settings.bridge_core_host,
        settings.bridge_core_port,
        settings.tray_enabled,
        settings.minimize_to_tray,
        settings.close_to_tray,
        settings.start_with_windows,
        settings.services_auto_start,
        env_bool("TEMPLATE_APP_API_ALLOW_PUBLIC_NETWORK", false),
        env_string("TEMPLATE_APP_API_TOKEN_HEADER", "X-App-Token")
    );
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Falha ao criar diretório do .env: {err}"))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|err| format!("Falha ao abrir .env para escrita: {err}"))?;
    file.write_all(content.as_bytes())
        .map_err(|err| format!("Falha ao escrever .env: {err}"))
}

pub fn build_ports(settings: &RuntimeSettings) -> (Vec<RuntimeServicePort>, Vec<String>) {
    let mut ports = vec![
        port_item(
            &INTERNAL_API_PORT_DEFINITION,
            &settings.internal_api_host,
            settings.internal_api_port,
        ),
        port_item(
            &LOCAL_WEB_PORT_DEFINITION,
            &settings.local_web_host,
            settings.local_web_port,
        ),
        port_item(
            &AUXILIARY_PORT_DEFINITION,
            &settings.auxiliary_host,
            settings.auxiliary_port,
        ),
        port_item(
            &BRIDGE_CORE_PORT_DEFINITION,
            &settings.bridge_core_host,
            settings.bridge_core_port,
        ),
    ];
    let mut warnings = Vec::new();
    let mut seen = HashSet::new();
    for item in &mut ports {
        if !seen.insert(item.configured_port) {
            warnings.push(format!(
                "Porta {} está configurada em mais de um serviço.",
                item.configured_port
            ));
        }
        if !is_port_available(&item.host, item.configured_port) {
            item.available = false;
            item.effective_port =
                find_available_port(&item.host, item.configured_port.saturating_add(1));
            item.fallback_applied = true;
            warnings.push(format!(
                "{}: porta {} indisponível; fallback sugerido {}.",
                item.label, item.configured_port, item.effective_port
            ));
        }
    }
    (ports, warnings)
}

fn port_item(
    definition: &RuntimePortDefinition,
    host: &str,
    configured_port: u16,
) -> RuntimeServicePort {
    RuntimeServicePort {
        key: definition.key.to_string(),
        label: definition.label.to_string(),
        env_host_key: definition.env_host_key.to_string(),
        env_port_key: definition.env_port_key.to_string(),
        default_port: definition.default_port,
        host: host.to_string(),
        configured_port,
        effective_port: configured_port,
        available: true,
        fallback_applied: false,
        description: definition.description.to_string(),
    }
}

pub fn resolve_internal_api_config(
    state: &SharedState,
) -> Result<crate::internal_api::config::InternalApiConfig, String> {
    let payload = load_settings(state)?;
    let port = payload
        .ports
        .iter()
        .find(|item| item.key == "internal_api")
        .map(|item| item.effective_port)
        .unwrap_or(payload.settings.internal_api_port);
    Ok(
        crate::internal_api::config::InternalApiConfig::from_runtime_settings(&payload.settings)
            .with_host_port(payload.settings.internal_api_host, port),
    )
}

fn validate_port_range(settings: &RuntimeSettings) -> Result<(), String> {
    let values = BTreeMap::from([
        ("API interna", settings.internal_api_port),
        ("Servidor web local", settings.local_web_port),
        ("Serviços auxiliares", settings.auxiliary_port),
        ("Bridge/core local", settings.bridge_core_port),
    ]);
    for (label, port) in values {
        if port == 0 {
            return Err(format!("{label}: porta deve estar entre 1 e 65535."));
        }
    }
    Ok(())
}

fn has_duplicate_configured_ports(ports: &[RuntimeServicePort]) -> bool {
    let mut seen = HashSet::new();
    ports.iter().any(|item| !seen.insert(item.configured_port))
}

pub fn is_port_available(host: &str, port: u16) -> bool {
    let bind_host = if matches!(host, "0.0.0.0" | "::" | "[::]") {
        "127.0.0.1"
    } else {
        host
    };
    format!("{bind_host}:{port}")
        .parse::<SocketAddr>()
        .ok()
        .and_then(|addr| TcpListener::bind(addr).ok())
        .is_some()
}

pub fn find_available_port(host: &str, start: u16) -> u16 {
    let start = start.max(1);
    for port in start..=65535 {
        if is_port_available(host, port) {
            return port;
        }
    }
    0
}

fn env_string(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn normalize_runtime_settings(settings: &mut RuntimeSettings) {
    if settings.internal_api_host.trim().is_empty() {
        settings.internal_api_host = "127.0.0.1".to_string();
    }
    if settings.internal_api_port == 0 {
        settings.internal_api_port = INTERNAL_API_DEFAULT_PORT;
    }
    let base_url = format!(
        "http://{}:{}",
        settings.internal_api_host.trim(),
        settings.internal_api_port
    );
    if settings.internal_api_base_url.trim().is_empty()
        || settings.internal_api_base_url.contains("127.0.0.1:61001")
    {
        settings.internal_api_base_url = base_url.clone();
    }
    if settings.internal_api_docs_path.trim().is_empty() {
        settings.internal_api_docs_path = "/docs".to_string();
    }
    if !settings.internal_api_docs_path.starts_with('/') {
        settings.internal_api_docs_path = format!("/{}", settings.internal_api_docs_path.trim());
    }
    if settings.internal_api_docs_url.trim().is_empty()
        || settings.internal_api_docs_url.contains("127.0.0.1:61001")
    {
        settings.internal_api_docs_url = format!("{}{}", base_url, settings.internal_api_docs_path);
    }
    if settings.internal_api_timeout_ms < 1000 {
        settings.internal_api_timeout_ms = 1000;
    }
    if settings.internal_api_log_mode.trim().is_empty() {
        settings.internal_api_log_mode = "normal".to_string();
    }
}

fn env_u16(key: &str, default: u16) -> u16 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|port| *port > 0)
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn env_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

#[allow(dead_code)]
pub fn payload_to_json(payload: RuntimeSettingsPayload) -> Value {
    json!(payload)
}
