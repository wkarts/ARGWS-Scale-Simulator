use std::{
    net::{SocketAddr, TcpListener as StdTcpListener},
    sync::{Mutex, OnceLock},
    thread::JoinHandle,
};

use tokio::{net::TcpListener, sync::oneshot};

use crate::{
    app_state::SharedState,
    db::{open_connection, write_app_log, AppLogInput},
    internal_api::{config::InternalApiConfig, routes, state::InternalApiState},
};

struct ApiRuntimeHandle {
    host: String,
    port: u16,
    started_at: String,
    shutdown: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
    docs_url: String,
}

static API_RUNTIME: OnceLock<Mutex<Option<ApiRuntimeHandle>>> = OnceLock::new();

fn runtime_slot() -> &'static Mutex<Option<ApiRuntimeHandle>> {
    API_RUNTIME.get_or_init(|| Mutex::new(None))
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiRuntimeStatus {
    pub running: bool,
    pub host: String,
    pub port: u16,
    pub url: Option<String>,
    pub docs_url: Option<String>,
    pub started_at: Option<String>,
}

pub fn run_blocking(state: &SharedState, host: &str, port: u16) -> Result<(), String> {
    let config = InternalApiConfig::default().with_host_port(host.to_string(), port);
    run_blocking_with_config(state, config)
}

pub fn run_blocking_with_config(
    state: &SharedState,
    config: InternalApiConfig,
) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|err| format!("Falha ao criar runtime Tokio: {err}"))?;
    rt.block_on(run_with_config(state, config))
}

pub async fn run(state: &SharedState, host: &str, port: u16) -> Result<(), String> {
    let config = InternalApiConfig::default().with_host_port(host.to_string(), port);
    run_with_config(state, config).await
}

pub async fn run_with_config(state: &SharedState, config: InternalApiConfig) -> Result<(), String> {
    config.validate_bind()?;

    let api_state = build_api_state(state, &config)?;
    let addr = socket_addr(&config)?;
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|err| format!("Falha ao iniciar API interna em {addr}: {err}"))?;
    println!("API interna ativa em http://{addr}");
    axum::serve(listener, routes::router(api_state))
        .await
        .map_err(|err| format!("Falha no servidor da API interna: {err}"))
}

pub fn start_background(
    state: SharedState,
    config: InternalApiConfig,
) -> Result<ApiRuntimeStatus, String> {
    config.validate_bind()?;

    {
        let guard = runtime_slot()
            .lock()
            .map_err(|_| "Falha ao bloquear runtime da API interna.".to_string())?;
        if let Some(handle) = guard.as_ref() {
            return Ok(ApiRuntimeStatus {
                running: true,
                host: handle.host.clone(),
                port: handle.port,
                url: Some(format!("http://{}:{}", handle.host, handle.port)),
                docs_url: Some(handle.docs_url.clone()),
                started_at: Some(handle.started_at.clone()),
            });
        }
    }

    log_runtime_event(
        &state,
        "info",
        "API iniciando",
        serde_json::json!({ "host": config.host.clone(), "port": config.port }),
    );
    let mut config = config;
    let host = config.host.clone();
    let mut port = config.port;
    let addr = socket_addr(&config)?;
    let std_listener = match StdTcpListener::bind(addr) {
        Ok(listener) => listener,
        Err(err) => {
            let fallback =
                crate::runtime_config::find_available_port(&host, port.saturating_add(1));
            if fallback == 0 {
                let message = format!(
                    "Falha ao iniciar API interna em {addr}: {err}; nenhum fallback disponível."
                );
                log_runtime_event(
                    &state,
                    "error",
                    "Falha ao iniciar API",
                    serde_json::json!({ "error": message }),
                );
                return Err(message);
            }
            config.port = fallback;
            config.base_url = format!("http://{host}:{fallback}");
            port = fallback;
            let fallback_addr = socket_addr(&config)?;
            log_runtime_event(
                &state,
                "warning",
                "Porta indisponível",
                serde_json::json!({ "configured": addr.to_string(), "fallback": fallback_addr.to_string(), "error": err.to_string() }),
            );
            eprintln!("Porta da API interna {addr} indisponível ({err}); usando fallback seguro {fallback_addr}.");
            StdTcpListener::bind(fallback_addr).map_err(|fallback_err| {
                format!("Falha ao iniciar API interna no fallback {fallback_addr}: {fallback_err}")
            })?
        }
    };
    std_listener
        .set_nonblocking(true)
        .map_err(|err| format!("Falha ao configurar listener da API interna: {err}"))?;
    let started_at = chrono::Utc::now().to_rfc3339();
    let docs_url = config.docs_url();
    let (tx, rx) = oneshot::channel::<()>();
    let thread_started_at = started_at.clone();
    let thread_state = state.clone();

    let thread = std::thread::Builder::new()
        .name("template-app-internal-api".to_string())
        .spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(err) => {
                    eprintln!("Falha ao criar runtime da API interna: {err}");
                    return;
                }
            };
            let result = rt.block_on(async move {
                let addr = socket_addr(&config)?;
                let listener = TcpListener::from_std(std_listener).map_err(|err| {
                    format!("Falha ao anexar listener da API interna em {addr}: {err}")
                })?;
                let api_state = build_api_state(&thread_state, &config)?;
                println!("API interna desktop ativa em http://{addr} desde {thread_started_at}");
                axum::serve(listener, routes::router(api_state))
                    .with_graceful_shutdown(async move {
                        let _ = rx.await;
                    })
                    .await
                    .map_err(|err| format!("Falha no servidor da API interna: {err}"))
            });
            if let Err(err) = result {
                eprintln!("{err}");
            }
        })
        .map_err(|err| format!("Falha ao iniciar thread da API interna: {err}"))?;

    let mut guard = runtime_slot()
        .lock()
        .map_err(|_| "Falha ao bloquear runtime da API interna.".to_string())?;
    *guard = Some(ApiRuntimeHandle {
        host: host.clone(),
        port,
        docs_url: docs_url.clone(),
        started_at: started_at.clone(),
        shutdown: Some(tx),
        thread: Some(thread),
    });

    log_runtime_event(
        &state,
        "info",
        "API iniciada com sucesso",
        serde_json::json!({ "url": format!("http://{host}:{port}"), "docs_url": docs_url }),
    );

    Ok(ApiRuntimeStatus {
        running: true,
        host: host.clone(),
        port,
        url: Some(format!("http://{host}:{port}")),
        docs_url: Some(docs_url),
        started_at: Some(started_at),
    })
}

pub fn stop_background() -> Result<ApiRuntimeStatus, String> {
    let mut handle = {
        let mut guard = runtime_slot()
            .lock()
            .map_err(|_| "Falha ao bloquear runtime da API interna.".to_string())?;
        guard.take()
    };

    if let Some(handle) = handle.as_mut() {
        let stopped_host = handle.host.clone();
        let stopped_port = handle.port;
        if let Some(shutdown) = handle.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(thread) = handle.thread.take() {
            let _ = thread.join();
        }
        eprintln!("API interna parada em {stopped_host}:{stopped_port}");
    }

    status()
}

pub fn status() -> Result<ApiRuntimeStatus, String> {
    let guard = runtime_slot()
        .lock()
        .map_err(|_| "Falha ao bloquear runtime da API interna.".to_string())?;
    if let Some(handle) = guard.as_ref() {
        Ok(ApiRuntimeStatus {
            running: true,
            host: handle.host.clone(),
            port: handle.port,
            url: Some(format!("http://{}:{}", handle.host, handle.port)),
            docs_url: Some(handle.docs_url.clone()),
            started_at: Some(handle.started_at.clone()),
        })
    } else {
        Ok(ApiRuntimeStatus {
            running: false,
            host: std::env::var("TEMPLATE_APP_API_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("TEMPLATE_APP_API_PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(61001),
            url: None,
            docs_url: None,
            started_at: None,
        })
    }
}

fn socket_addr(config: &InternalApiConfig) -> Result<SocketAddr, String> {
    format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|err| format!("Endereço da API interna inválido: {err}"))
}

fn log_runtime_event(state: &SharedState, level: &str, message: &str, details: serde_json::Value) {
    if let (Ok(db_path), Ok(data_dir)) = (state.db_path(), state.data_dir()) {
        if let Ok(conn) = open_connection(&db_path) {
            let _ = write_app_log(
                &conn,
                &data_dir,
                AppLogInput {
                    level,
                    category: "internal-api",
                    message,
                    source: Some("internal_api::server"),
                    route: None,
                    details: Some(&details),
                },
            );
        }
    }
}

fn build_api_state(
    state: &SharedState,
    config: &InternalApiConfig,
) -> Result<InternalApiState, String> {
    Ok(InternalApiState {
        db_path: state.db_path()?,
        data_dir: state.data_dir()?,
        started_at: chrono::Utc::now().to_rfc3339(),
        host: config.host.clone(),
        port: config.port,
        config: config.clone(),
    })
}
