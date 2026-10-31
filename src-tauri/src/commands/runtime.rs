use std::{env, process::Command};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::{
    app_state::SharedState,
    internal_api::{config::InternalApiConfig, server},
    runtime_config::{self, RuntimeSettings},
};

#[tauri::command]
pub fn internal_api_status(state: State<'_, SharedState>) -> Result<Value, String> {
    let status = server::status()?;
    let settings_payload = runtime_config::load_settings(state.inner())?;
    let settings = settings_payload.settings;
    let docs_url = status
        .docs_url
        .clone()
        .unwrap_or_else(|| settings.internal_api_docs_url.clone());
    Ok(json!({
        "running": status.running,
        "host": if status.running { status.host.clone() } else { settings.internal_api_host.clone() },
        "port": if status.running { status.port } else { settings.internal_api_port },
        "url": status.url.clone().unwrap_or_else(|| settings.internal_api_base_url.clone()),
        "docs_url": docs_url,
        "started_at": status.started_at.clone(),
        "token_required": settings.internal_api_require_token,
        "token_configured": !settings.internal_api_token.trim().is_empty(),
        "auto_start": settings.internal_api_auto_start,
        "docs_public_local": settings.internal_api_docs_public_local,
        "docs_enabled": settings.internal_api_docs_enabled,
        "docs_path": settings.internal_api_docs_path.clone(),
        "timeout_ms": settings.internal_api_timeout_ms,
        "log_mode": settings.internal_api_log_mode.clone(),
        "settings": settings,
        "ports": settings_payload.ports,
        "warnings": settings_payload.warnings,
        "recent_logs": recent_internal_api_logs(state.inner()).unwrap_or_default(),
    }))
}

#[tauri::command]
pub fn internal_api_start(
    state: State<'_, SharedState>,
    host: Option<String>,
    port: Option<u16>,
) -> Result<Value, String> {
    let config = if host.is_some() || port.is_some() {
        let loaded = runtime_config::load_settings(state.inner())?.settings;
        let mut loaded = loaded;
        if let Some(host) = host {
            loaded.internal_api_host = host;
        }
        if let Some(port) = port {
            loaded.internal_api_port = port;
        }
        InternalApiConfig::from_runtime_settings(&loaded)
    } else {
        runtime_config::resolve_internal_api_config(state.inner())?
    };
    let status = server::start_background(state.inner().clone(), config)?;
    serde_json::to_value(status).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn internal_api_stop() -> Result<Value, String> {
    let status = server::stop_background()?;
    serde_json::to_value(status).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn internal_api_restart(
    state: State<'_, SharedState>,
    host: Option<String>,
    port: Option<u16>,
) -> Result<Value, String> {
    let _ = server::stop_background();
    internal_api_start(state, host, port)
}

#[tauri::command]
pub fn runtime_settings_load(state: State<'_, SharedState>) -> Result<Value, String> {
    serde_json::to_value(runtime_config::load_settings(state.inner())?)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn runtime_settings_save(
    state: State<'_, SharedState>,
    settings: RuntimeSettings,
) -> Result<Value, String> {
    let should_restart =
        settings.internal_api_restart_on_config_change && server::status()?.running;
    let saved = runtime_config::save_settings(state.inner(), settings)?;
    if should_restart {
        let _ = server::stop_background();
        let config = runtime_config::resolve_internal_api_config(state.inner())?;
        let _ = server::start_background(state.inner().clone(), config)?;
    }
    Ok(json!({
        "env_path": saved.env_path,
        "settings": saved.settings,
        "ports": saved.ports,
        "warnings": saved.warnings,
        "restarted": should_restart,
    }))
}

#[tauri::command]
pub fn internal_api_test_port(host: String, port: u16) -> Result<Value, String> {
    let available = runtime_config::is_port_available(&host, port);
    let suggested_port = if available {
        port
    } else {
        runtime_config::find_available_port(&host, port.saturating_add(1))
    };
    Ok(json!({
        "host": host,
        "port": port,
        "available": available,
        "suggested_port": suggested_port,
    }))
}

#[tauri::command]
pub fn internal_api_test(base_url: String, timeout_ms: Option<u64>) -> Result<Value, String> {
    let url = format!("{}/health", base_url.trim_end_matches('/'));
    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(8000).max(1000));
    let client = reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|err| format!("Falha ao preparar teste da API: {err}"))?;
    let response = client
        .get(&url)
        .send()
        .map_err(|err| format!("Falha ao conectar na API interna em {url}: {err}"))?;
    let status = response.status().as_u16();
    let body = response.text().unwrap_or_default();
    Ok(json!({ "url": url, "status": status, "ok": (200..300).contains(&status), "body": body }))
}

#[tauri::command]
pub fn runtime_env_example() -> Result<String, String> {
    Ok(runtime_config::default_env_template())
}

#[tauri::command]
pub fn startup_with_windows_set(enabled: bool) -> Result<Value, String> {
    #[cfg(target_os = "windows")]
    {
        let exe = current_exe_string()?;
        let app_name =
            env::var("TEMPLATE_APP_STARTUP_NAME").unwrap_or_else(|_| "TemplateApp".to_string());
        if enabled {
            run_command(
                "reg.exe",
                &[
                    "add",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    &app_name,
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{exe}\""),
                    "/f",
                ],
                "startup-enable",
            )
        } else {
            run_command(
                "reg.exe",
                &[
                    "delete",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    &app_name,
                    "/f",
                ],
                "startup-disable",
            )
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(json!({
            "action": if enabled { "startup-enable" } else { "startup-disable" },
            "implemented": false,
            "message": "Inicialização automática pelo Windows só é aplicada no Windows. A preferência permanece em runtime.settings."
        }))
    }
}

#[tauri::command]
pub fn app_service_install() -> Result<Value, String> {
    service_action("install")
}
#[tauri::command]
pub fn app_service_uninstall() -> Result<Value, String> {
    service_action("uninstall")
}
#[tauri::command]
pub fn app_service_start() -> Result<Value, String> {
    service_action("start")
}
#[tauri::command]
pub fn app_service_stop() -> Result<Value, String> {
    service_action("stop")
}
#[tauri::command]
pub fn app_service_restart() -> Result<Value, String> {
    let stop = service_action("stop").unwrap_or_else(|err| json!({"stop_error": err}));
    let start = service_action("start")?;
    Ok(json!({"action": "restart", "stop": stop, "start": start}))
}
#[tauri::command]
pub fn app_service_status() -> Result<Value, String> {
    service_action("status")
}

fn service_action(action: &str) -> Result<Value, String> {
    #[cfg(target_os = "windows")]
    {
        windows_service_action(action)
    }
    #[cfg(target_os = "linux")]
    {
        linux_service_action(action)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Ok(json!({
            "action": action,
            "implemented": false,
            "status": "unsupported-platform",
            "message": "Serviço nativo preparado apenas para Windows e Linux/systemd nesta etapa."
        }))
    }
}

#[cfg(target_os = "windows")]
fn windows_service_action(action: &str) -> Result<Value, String> {
    let service_name =
        env::var("TEMPLATE_APP_SERVICE_NAME").unwrap_or_else(|_| "TemplateAppServer".to_string());
    match action {
        "install" => {
            let exe = current_exe_string()?;
            run_command(
                "sc.exe",
                &[
                    "create",
                    &service_name,
                    "binPath=",
                    &format!("\"{exe}\" --mode=headless-api"),
                    "start=",
                    "auto",
                    "DisplayName=",
                    "Template App Server",
                ],
                action,
            )
        }
        "uninstall" => run_command("sc.exe", &["delete", &service_name], action),
        "start" => run_command("sc.exe", &["start", &service_name], action),
        "stop" => run_command("sc.exe", &["stop", &service_name], action),
        "status" => run_command("sc.exe", &["query", &service_name], action),
        other => Err(format!("Ação de serviço Windows inválida: {other}")),
    }
}

#[cfg(target_os = "linux")]
fn linux_service_action(action: &str) -> Result<Value, String> {
    let service_name =
        env::var("TEMPLATE_APP_SERVICE_NAME").unwrap_or_else(|_| "template-app-server".to_string());
    let unit_name = format!("{service_name}.service");
    match action {
        "install" => {
            let exe = current_exe_string()?;
            let unit = format!(
                "[Unit]\nDescription=Template App Server\nAfter=network.target\n\n[Service]\nExecStart={exe} --mode=headless-api\nRestart=always\nEnvironment=APP_ENV=production\n\n[Install]\nWantedBy=multi-user.target\n"
            );
            let unit_path = PathBuf::from("/etc/systemd/system").join(&unit_name);
            std::fs::write(&unit_path, unit).map_err(|err| {
                format!(
                    "Falha ao gravar unit systemd em {}: {err}",
                    unit_path.display()
                )
            })?;
            let reload = run_command("systemctl", &["daemon-reload"], "daemon-reload")?;
            let enable = run_command("systemctl", &["enable", &unit_name], "enable")?;
            Ok(
                json!({"action": action, "implemented": true, "unit": unit_path, "reload": reload, "enable": enable}),
            )
        }
        "uninstall" => {
            let _ = run_command("systemctl", &["disable", &unit_name], "disable");
            let unit_path = PathBuf::from("/etc/systemd/system").join(&unit_name);
            if unit_path.exists() {
                std::fs::remove_file(&unit_path).map_err(|err| {
                    format!("Falha ao remover unit {}: {err}", unit_path.display())
                })?;
            }
            let reload = run_command("systemctl", &["daemon-reload"], "daemon-reload")?;
            Ok(json!({"action": action, "implemented": true, "unit": unit_path, "reload": reload}))
        }
        "start" => run_command("systemctl", &["start", &unit_name], action),
        "stop" => run_command("systemctl", &["stop", &unit_name], action),
        "status" => run_command("systemctl", &["status", &unit_name, "--no-pager"], action),
        other => Err(format!("Ação de serviço Linux inválida: {other}")),
    }
}

fn recent_internal_api_logs(state: &SharedState) -> Result<Vec<Value>, String> {
    let db_path = state.db_path()?;
    let conn = crate::db::open_connection(&db_path)?;
    let mut stmt = conn
        .prepare("SELECT level, message, created_at, details_json FROM app_logs WHERE category='internal-api' ORDER BY id DESC LIMIT 50")
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(json!({
                "level": row.get::<_, String>(0)?,
                "message": row.get::<_, String>(1)?,
                "created_at": row.get::<_, String>(2)?,
                "details_json": row.get::<_, Option<String>>(3)?,
            }))
        })
        .map_err(|err| err.to_string())?;
    let mut logs = Vec::new();
    for row in rows {
        logs.push(row.map_err(|err| err.to_string())?);
    }
    Ok(logs)
}

fn current_exe_string() -> Result<String, String> {
    env::current_exe()
        .map_err(|err| format!("Falha ao resolver executável atual: {err}"))
        .map(|path| path.to_string_lossy().to_string())
}

fn run_command(program: &str, args: &[&str], action: &str) -> Result<Value, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("Falha ao executar {program}: {err}"))?;
    Ok(json!({
        "action": action,
        "implemented": true,
        "program": program,
        "args": args,
        "success": output.status.success(),
        "code": output.status.code(),
        "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
        "stderr": String::from_utf8_lossy(&output.stderr).to_string()
    }))
}

#[tauri::command]
pub fn open_print_preview(
    app: AppHandle,
    html: Option<String>,
    title: Option<String>,
) -> Result<bool, String> {
    let label = format!("print-preview-{}", chrono::Utc::now().timestamp_millis());
    let safe_title = title.unwrap_or_else(|| "Pré-visualização de impressão".to_string());
    let document = build_print_document(html.unwrap_or_default(), &safe_title);
    let encoded = general_purpose::STANDARD.encode(document.as_bytes());
    let url = url::Url::parse(&format!("data:text/html;base64,{encoded}"))
        .map_err(|err| format!("Falha ao montar preview de impressão: {err}"))?;
    let window = WebviewWindowBuilder::new(&app, label, WebviewUrl::External(url))
        .title(safe_title)
        .inner_size(1100.0, 760.0)
        .resizable(true)
        .center()
        .build()
        .map_err(|err| format!("Falha ao abrir janela de impressão: {err}"))?;
    let _ = window.set_focus();
    Ok(true)
}

fn build_print_document(html: String, title: &str) -> String {
    let body = if html.trim().is_empty() {
        "<main class=\"empty\">Nenhum conteúdo informado para impressão.</main>".to_string()
    } else {
        html
    };
    format!(
        r#"<!doctype html>
<html lang="pt-BR">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <style>
    :root {{ color-scheme: light only; }}
    body {{ margin: 0; background: #eef2f7; color: #172033; font-family: Inter, Arial, sans-serif; }}
    .shell {{ max-width: 980px; margin: 24px auto; background: #fff; border-radius: 16px; box-shadow: 0 18px 50px rgba(15,23,42,.14); overflow: hidden; }}
    header {{ padding: 20px 28px; border-bottom: 1px solid #e2e8f0; display: flex; justify-content: space-between; gap: 16px; }}
    h1 {{ margin: 0; font-size: 18px; }}
    .meta {{ color: #64748b; font-size: 12px; }}
    .content {{ padding: 28px; }}
    .content table {{ width: 100%; border-collapse: collapse; }}
    .content th, .content td {{ border-bottom: 1px solid #e5e7eb; padding: 8px 10px; text-align: left; }}
    .actions {{ position: sticky; bottom: 0; display: flex; justify-content: flex-end; padding: 12px 28px; background: #f8fafc; border-top: 1px solid #e2e8f0; }}
    button {{ border: 0; border-radius: 10px; padding: 10px 16px; background: #2563eb; color: #fff; font-weight: 700; cursor: pointer; }}
    .empty {{ padding: 48px; text-align: center; color: #64748b; border: 1px dashed #cbd5e1; border-radius: 14px; }}
    @media print {{ body {{ background: #fff; }} .shell {{ margin: 0; max-width: none; box-shadow: none; border-radius: 0; }} .actions {{ display: none; }} }}
  </style>
</head>
<body>
  <section class="shell">
    <header>
      <div><h1>{title}</h1><div class="meta">Template App · Preview de impressão</div></div>
      <div class="meta">Gerado em <span id="now"></span></div>
    </header>
    <div class="content">{body}</div>
    <div class="actions"><button onclick="window.print()">Imprimir</button></div>
  </section>
  <script>document.getElementById('now').textContent = new Date().toLocaleString('pt-BR');</script>
</body>
</html>"#
    )
}

#[tauri::command]
pub fn tray_status() -> Result<Value, String> {
    let enabled = std::env::var("TEMPLATE_APP_TRAY_ENABLED")
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(true);
    let api_status = server::status()?;
    Ok(json!({
        "enabled": enabled,
        "api": api_status,
        "main_window": if enabled { "managed" } else { "tray-disabled" }
    }))
}

#[tauri::command]
pub fn tray_restore_window(app: AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|err| err.to_string())?;
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
    Ok(true)
}

#[tauri::command]
pub fn tray_exit_app(app: AppHandle) -> Result<bool, String> {
    app.exit(0);
    Ok(true)
}
