#[path = "toledo8540_encoder.rs"]
mod toledo8540_encoder;
#[path = "frame_test_overrides.rs"]
mod frame_test_overrides;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read, Write};
use std::net::TcpListener as StdTcpListener;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

static SIMULATORS: LazyLock<Mutex<HashMap<String, SimulatorRuntime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ScaleSimulatorDriver {
    #[default]
    Wt1000n,
    Wt27r,
    Wt3000ir,
    Toledo8540,
    ToledoPrix3,
    A12e,
    LiderLd2052,
    Filizola,
    GenericAscii,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScaleSimulatorMode {
    #[default]
    TcpServer,
    TcpServerCommand,
    SerialContinuous,
    SerialCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorConfig {
    pub id: Option<String>,
    pub name: String,
    pub driver: ScaleSimulatorDriver,
    pub mode: ScaleSimulatorMode,
    pub host: String,
    pub port: u16,
    pub serial_port: String,
    pub baud_rate: u32,
    #[serde(default)]
    pub toledo8540_stop_bits: Option<u8>,
    #[serde(default)]
    pub serial_data_bits: Option<u8>,
    #[serde(default)]
    pub serial_parity: Option<String>,
    #[serde(default)]
    pub serial_stop_bits: Option<u8>,
    #[serde(default)]
    pub frame_override_enabled: bool,
    #[serde(default)]
    pub frame_prefix_hex: Option<String>,
    #[serde(default)]
    pub frame_suffix_hex: Option<String>,
    #[serde(default)]
    pub frame_checksum_mode: Option<String>,
    pub capacity_kg: f64,
    pub division_kg: f64,
    pub initial_weight_kg: f64,
    pub variation_kg: f64,
    pub interval_ms: u64,
    pub stable: bool,
    pub zero_tracking: bool,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorPreset {
    pub key: String,
    pub name: String,
    pub description: String,
    pub driver: ScaleSimulatorDriver,
    pub default_port: u16,
    pub default_serial_port: String,
    pub default_baud_rate: u32,
    pub command_mode: bool,
    pub serial_mode: bool,
    pub sample_frame: String,
    pub adp_equipment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorStatus {
    pub id: String,
    pub name: String,
    pub driver: ScaleSimulatorDriver,
    pub mode: ScaleSimulatorMode,
    pub host: String,
    pub port: u16,
    pub serial_port: String,
    pub baud_rate: u32,
    pub serial_data_bits: u8,
    pub serial_parity: String,
    pub serial_stop_bits: u8,
    pub frame_override_enabled: bool,
    pub running: bool,
    pub clients: usize,
    pub weight_kg: f64,
    pub gross_kg: f64,
    pub tare_kg: f64,
    pub net_kg: f64,
    pub stable: bool,
    pub last_frame: String,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorLogEntry {
    pub at: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorSerialPortInfo {
    pub port_name: String,
    pub display_name: String,
    pub port_type: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub in_use_by_simulator: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSimulatorActionResult {
    pub id: String,
    pub message: String,
    pub status: Option<ScaleSimulatorStatus>,
}

#[derive(Debug)]
struct ScaleState {
    gross_kg: f64,
    tare_kg: f64,
    stable: bool,
    gross_mode: bool,
    last_frame: String,
}

struct SimulatorRuntime {
    config: ScaleSimulatorConfig,
    started_at: DateTime<Utc>,
    state: Arc<Mutex<ScaleState>>,
    clients: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    logs: Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>,
    task: JoinHandle<()>,
}

#[tauri::command]
pub fn scale_simulator_serial_ports() -> Result<Vec<ScaleSimulatorSerialPortInfo>, String> {
    list_serial_ports_with_usage()
}

#[tauri::command]
pub fn scale_simulator_presets() -> Vec<ScaleSimulatorPreset> {
    vec![
        ScaleSimulatorPreset {
            key: "adp_wt1000n_serial_stream".to_string(),
            name: "ADP WT1000N - RS232 stream".to_string(),
            description: "Compatível com parser WT1000N do All Driver Platform: status, bruto, tara e líquido separados por vírgula.".to_string(),
            driver: ScaleSimulatorDriver::Wt1000n,
            default_port: 9101,
            default_serial_port: "COM1".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "0,1234,0,1234\r\n".to_string(),
            adp_equipment: "WT1000N".to_string(),
        },
        ScaleSimulatorPreset {
            key: "adp_wt3000ir_serial_stream".to_string(),
            name: "ADP WT3000IR - RS232 stream".to_string(),
            description: "Compatível com parser WT3000IR do All Driver Platform.".to_string(),
            driver: ScaleSimulatorDriver::Wt3000ir,
            default_port: 9102,
            default_serial_port: "COM2".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "ST,GS,+001234kg 000000kg\r\n".to_string(),
            adp_equipment: "WT3000IR".to_string(),
        },
        ScaleSimulatorPreset {
            key: "adp_toledo8540_serial_stream".to_string(),
            name: "ADP Toledo 8540 - RS232 stream".to_string(),
            description: "18 bytes: STX + SWA/SWB/SWC + peso/tara + CR + checksum; serial 7E1 ou 7E2.".to_string(),
            driver: ScaleSimulatorDriver::Toledo8540,
            default_port: 9103,
            default_serial_port: "COM3".to_string(),
            default_baud_rate: 4800,
            command_mode: false,
            serial_mode: true,
            sample_frame: toledo8540_encoder::encode(1234.0, 0.0, true, true, 80000.0, 1.0, "kg"),
            adp_equipment: "TOLEDO8540".to_string(),
        },
        ScaleSimulatorPreset {
            key: "adp_a12e_serial_stream".to_string(),
            name: "ADP A12e - RS232 stream".to_string(),
            description: "Compatível com parser A12e do All Driver Platform.".to_string(),
            driver: ScaleSimulatorDriver::A12e,
            default_port: 9104,
            default_serial_port: "COM4".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "wn+001234kg\r\n".to_string(),
            adp_equipment: "A12e".to_string(),
        },
        ScaleSimulatorPreset {
            key: "adp_lider_ld2052_serial_stream".to_string(),
            name: "ADP Líder LD2052 - RS232 stream".to_string(),
            description: "Compatível com parser LIDERLD2052 do ADP: SOH + peso 7 ASCII + espaço + E/I + STX final.".to_string(),
            driver: ScaleSimulatorDriver::LiderLd2052,
            default_port: 9105,
            default_serial_port: "COM5".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "\u{0001}0001234 E\u{0002}".to_string(),
            adp_equipment: "LIDERLD2052".to_string(),
        },
        ScaleSimulatorPreset {
            key: "adp_wt1000n_serial_command".to_string(),
            name: "ADP WT1000N - RS232 sob comando".to_string(),
            description: "Mesmo frame WT1000N, respondendo a P, PESO, READ ou ENQ.".to_string(),
            driver: ScaleSimulatorDriver::Wt1000n,
            default_port: 9111,
            default_serial_port: "COM6".to_string(),
            default_baud_rate: 9600,
            command_mode: true,
            serial_mode: true,
            sample_frame: "0,1234,0,1234\r\n".to_string(),
            adp_equipment: "WT1000N".to_string(),
        },
        ScaleSimulatorPreset {
            key: "wt27r_serial_comando".to_string(),
            name: "WT27R - RS232 COM sob comando".to_string(),
            description: "Simula balança WT27R respondendo comandos de leitura pela porta serial.".to_string(),
            driver: ScaleSimulatorDriver::Wt27r,
            default_port: 9112,
            default_serial_port: "COM7".to_string(),
            default_baud_rate: 9600,
            command_mode: true,
            serial_mode: true,
            sample_frame: "=+001234kg\r\n".to_string(),
            adp_equipment: "WT27R".to_string(),
        },
        ScaleSimulatorPreset {
            key: "wt1000n_tcp_continuo".to_string(),
            name: "WT1000N - TCP contínuo".to_string(),
            description: "Alternativa TCP para homologação sem par virtual serial instalado.".to_string(),
            driver: ScaleSimulatorDriver::Wt1000n,
            default_port: 9101,
            default_serial_port: "COM1".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: false,
            sample_frame: "0,1234,0,1234\r\n".to_string(),
            adp_equipment: "WT1000N".to_string(),
        },
        ScaleSimulatorPreset {
            key: "toledo_prix3_serial".to_string(),
            name: "Toledo/Prix 3 - RS232 COM".to_string(),
            description: "Frame STX/ETX genérico para integrações Toledo/Prix fora do parser ADP.".to_string(),
            driver: ScaleSimulatorDriver::ToledoPrix3,
            default_port: 9113,
            default_serial_port: "COM8".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "\u{0002}+001234kg\u{0003}".to_string(),
            adp_equipment: "TOLEDO_PRIX3".to_string(),
        },
        ScaleSimulatorPreset {
            key: "filizola_serial".to_string(),
            name: "Filizola - RS232 COM".to_string(),
            description: "Frame ASCII simples em porta COM para homologações rápidas.".to_string(),
            driver: ScaleSimulatorDriver::Filizola,
            default_port: 9114,
            default_serial_port: "COM9".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "P+001234kg\r\n".to_string(),
            adp_equipment: "FILIZOLA".to_string(),
        },
        ScaleSimulatorPreset {
            key: "generic_ascii_serial".to_string(),
            name: "Genérico ASCII - RS232 COM".to_string(),
            description: "Frame genérico em porta COM1-COMXXX.".to_string(),
            driver: ScaleSimulatorDriver::GenericAscii,
            default_port: 9199,
            default_serial_port: "COM10".to_string(),
            default_baud_rate: 9600,
            command_mode: false,
            serial_mode: true,
            sample_frame: "+001234 kg\r\n".to_string(),
            adp_equipment: "GENERIC_ASCII".to_string(),
        },
    ]
}

#[tauri::command]
pub fn scale_simulator_preview_frame(config: ScaleSimulatorConfig) -> Result<String, String> {
    validate_config(&config)?;
    let state = ScaleState {
        gross_kg: normalize_weight(config.initial_weight_kg, config.division_kg),
        tare_kg: 0.0,
        stable: config.stable,
        gross_mode: true,
        last_frame: String::new(),
    };
    transmission_frame(&config, &state)
}

#[tauri::command]
pub async fn scale_simulator_start(config: ScaleSimulatorConfig) -> Result<ScaleSimulatorActionResult, String> {
    validate_config(&config)?;
    prune_finished_simulators()?;
    let id = config
        .id
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut config = config;
    config.id = Some(id.clone());

    {
        let simulators = SIMULATORS
            .lock()
            .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
        if simulators.contains_key(&id) {
            return Err("Já existe um simulador ativo com esse identificador.".to_string());
        }
        if config.mode.is_tcp()
            && simulators
                .values()
                .any(|runtime| runtime.config.mode.is_tcp() && runtime.config.port == config.port)
        {
            return Err(format!(
                "A porta TCP {} já está em uso por outro simulador nesta aplicação.",
                config.port
            ));
        }
        if config.mode.is_serial()
            && simulators.values().any(|runtime| {
                runtime.config.mode.is_serial()
                    && runtime
                        .config
                        .serial_port
                        .eq_ignore_ascii_case(config.serial_port.trim())
            })
        {
            return Err(format!(
                "A porta serial {} já está em uso por outro simulador nesta aplicação.",
                config.serial_port
            ));
        }
    }

    if config.mode.is_serial() {
        preflight_serial_port(&config)?;
    }

    let state = Arc::new(Mutex::new(ScaleState {
        gross_kg: normalize_weight(config.initial_weight_kg, config.division_kg),
        tare_kg: 0.0,
        stable: config.stable,
        gross_mode: true,
        last_frame: String::new(),
    }));
    let clients = Arc::new(AtomicUsize::new(0));
    let shutdown = Arc::new(AtomicBool::new(false));
    let logs = Arc::new(Mutex::new(VecDeque::with_capacity(200)));

    let task_config = config.clone();
    let task_state = state.clone();
    let task_clients = clients.clone();
    let task_shutdown = shutdown.clone();
    let task_logs = logs.clone();

    let task = if config.mode.is_serial() {
        push_log(
            &logs,
            "info",
            format!(
                "Iniciando simulador {} em {} a {} bps.",
                config.name, config.serial_port, config.baud_rate
            ),
        );
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                run_serial_loop(
                    task_config,
                    task_state,
                    task_clients,
                    task_shutdown,
                    task_logs,
                );
            })
            .await;
        })
    } else {
        let bind_addr = format!("{}:{}", config.host, config.port);
        push_log(
            &logs,
            "info",
            format!("Iniciando simulador {} na porta TCP {}.", config.name, config.port),
        );
        let listener = StdTcpListener::bind(&bind_addr)
            .map_err(|err| format!("Falha ao abrir porta TCP {bind_addr}: {err}"))?;
        listener
            .set_nonblocking(true)
            .map_err(|err| format!("Falha ao configurar porta TCP {bind_addr} como non-blocking: {err}"))?;
        let listener = TcpListener::from_std(listener)
            .map_err(|err| format!("Falha ao converter listener TCP {bind_addr}: {err}"))?;
        tokio::spawn(async move {
            run_tcp_server(
                listener,
                task_config,
                task_state,
                task_clients,
                task_shutdown,
                task_logs,
            )
            .await;
        })
    };

    let runtime = SimulatorRuntime {
        config: config.clone(),
        started_at: Utc::now(),
        state,
        clients,
        shutdown,
        logs,
        task,
    };

    let status = status_from_runtime(&id, &runtime)?;
    let mut simulators = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
    simulators.insert(id.clone(), runtime);

    Ok(ScaleSimulatorActionResult {
        id,
        message: "Simulador iniciado com sucesso.".to_string(),
        status: Some(status),
    })
}

#[tauri::command]
pub fn scale_simulator_stop(id: String) -> Result<ScaleSimulatorActionResult, String> {
    let runtime = {
        let mut simulators = SIMULATORS
            .lock()
            .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
        simulators.remove(&id)
    };

    let Some(runtime) = runtime else {
        return Err("Simulador não encontrado.".to_string());
    };

    runtime.shutdown.store(true, Ordering::SeqCst);
    runtime.task.abort();

    Ok(ScaleSimulatorActionResult {
        id,
        message: "Simulador parado com sucesso.".to_string(),
        status: None,
    })
}

#[tauri::command]
pub fn scale_simulator_stop_all() -> Result<ScaleSimulatorActionResult, String> {
    let runtimes = {
        let mut simulators = SIMULATORS
            .lock()
            .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
        simulators.drain().collect::<Vec<_>>()
    };

    let count = runtimes.len();
    for (_, runtime) in runtimes {
        runtime.shutdown.store(true, Ordering::SeqCst);
        runtime.task.abort();
    }

    Ok(ScaleSimulatorActionResult {
        id: "all".to_string(),
        message: format!("{} simulador(es) parado(s).", count),
        status: None,
    })
}

#[tauri::command]
pub fn scale_simulator_list() -> Result<Vec<ScaleSimulatorStatus>, String> {
    let simulators = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
    simulators
        .iter()
        .map(|(id, runtime)| status_from_runtime(id, runtime))
        .collect()
}

#[tauri::command]
pub fn scale_simulator_logs(id: String) -> Result<Vec<ScaleSimulatorLogEntry>, String> {
    let simulators = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
    let runtime = simulators
        .get(&id)
        .ok_or_else(|| "Simulador não encontrado.".to_string())?;
    let logs = runtime
        .logs
        .lock()
        .map_err(|_| "Falha ao ler logs do simulador.".to_string())?;
    Ok(logs.iter().cloned().collect())
}

#[tauri::command]
pub fn scale_simulator_set_weight(id: String, weight_kg: f64) -> Result<ScaleSimulatorActionResult, String> {
    with_runtime(&id, |runtime| {
        {
            let mut state = runtime
                .state
                .lock()
                .map_err(|_| "Falha ao alterar peso do simulador.".to_string())?;
            state.gross_kg = normalize_weight(weight_kg, runtime.config.division_kg);
            state.stable = runtime.config.stable;
            push_log(
                &runtime.logs,
                "info",
                format!("Peso bruto alterado para {:.3} kg.", state.gross_kg),
            );
        }
        Ok(ScaleSimulatorActionResult {
            id: id.clone(),
            message: "Peso alterado com sucesso.".to_string(),
            status: Some(status_from_runtime(&id, runtime)?),
        })
    })
}

#[tauri::command]
pub fn scale_simulator_action(id: String, action: String, value: Option<f64>) -> Result<ScaleSimulatorActionResult, String> {
    with_runtime(&id, |runtime| {
        let action_normalized = action.trim().to_lowercase();
        {
            let mut state = runtime
                .state
                .lock()
                .map_err(|_| "Falha ao executar ação do simulador.".to_string())?;
            match action_normalized.as_str() {
                "zero" | "zerar" => {
                    state.gross_kg = 0.0;
                    state.tare_kg = 0.0;
                    push_log(&runtime.logs, "info", "Comando ZERO aplicado.".to_string());
                }
                "tare" | "tarar" => {
                    state.tare_kg = value.unwrap_or(state.gross_kg).max(0.0);
                    state.gross_mode = false;
                    push_log(&runtime.logs, "info", format!("Tara aplicada: {:.3} kg.", state.tare_kg));
                }
                "clear_tare" | "limpar_tara" => {
                    state.tare_kg = 0.0;
                    state.gross_mode = true;
                    push_log(&runtime.logs, "info", "Tara removida.".to_string());
                }
                "gross" | "bruto" => {
                    state.gross_mode = true;
                    push_log(&runtime.logs, "info", "Modo bruto ativado.".to_string());
                }
                "net" | "liquido" => {
                    state.gross_mode = false;
                    push_log(&runtime.logs, "info", "Modo líquido ativado.".to_string());
                }
                "stable" | "estavel" => {
                    state.stable = true;
                    push_log(&runtime.logs, "info", "Status estável ativado.".to_string());
                }
                "unstable" | "oscilando" => {
                    state.stable = false;
                    push_log(&runtime.logs, "info", "Status oscilando ativado.".to_string());
                }
                _ => return Err("Ação inválida para o simulador.".to_string()),
            }
        }
        Ok(ScaleSimulatorActionResult {
            id: id.clone(),
            message: "Ação executada com sucesso.".to_string(),
            status: Some(status_from_runtime(&id, runtime)?),
        })
    })
}

impl ScaleSimulatorMode {
    fn is_tcp(&self) -> bool {
        matches!(self, Self::TcpServer | Self::TcpServerCommand)
    }

    fn is_serial(&self) -> bool {
        matches!(self, Self::SerialContinuous | Self::SerialCommand)
    }

    fn is_command(&self) -> bool {
        matches!(self, Self::TcpServerCommand | Self::SerialCommand)
    }
}

fn list_serial_ports_with_usage() -> Result<Vec<ScaleSimulatorSerialPortInfo>, String> {
    let ports = serialport::available_ports()
        .map_err(|err| format!("Falha ao identificar portas seriais RS232/COM do computador: {err}"))?;

    let active_serial_ports = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao verificar portas seriais em uso pelo simulador.".to_string())?
        .values()
        .filter(|runtime| runtime.config.mode.is_serial())
        .map(|runtime| runtime.config.serial_port.trim().to_ascii_uppercase())
        .collect::<Vec<_>>();

    let mut items = ports
        .into_iter()
        .map(|port| {
            let port_name = port.port_name;
            let port_key = port_name.trim().to_ascii_uppercase();
            let mut description = None;
            let mut manufacturer = None;
            let mut serial_number = None;
            let mut vid = None;
            let mut pid = None;
            let port_type = match port.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    description = info.product;
                    manufacturer = info.manufacturer;
                    serial_number = info.serial_number;
                    vid = Some(info.vid);
                    pid = Some(info.pid);
                    "usb".to_string()
                }
                serialport::SerialPortType::PciPort => "pci".to_string(),
                serialport::SerialPortType::BluetoothPort => "bluetooth".to_string(),
                serialport::SerialPortType::Unknown => "unknown".to_string(),
            };
            let display_name = build_serial_port_display_name(
                &port_name,
                description.as_deref(),
                manufacturer.as_deref(),
                &port_type,
            );

            ScaleSimulatorSerialPortInfo {
                port_name,
                display_name,
                port_type,
                description,
                manufacturer,
                serial_number,
                vid,
                pid,
                in_use_by_simulator: active_serial_ports.iter().any(|active| active == &port_key),
            }
        })
        .collect::<Vec<_>>();

    items.sort_by(|left, right| compare_serial_port_names(&left.port_name, &right.port_name));
    Ok(items)
}

fn build_serial_port_display_name(
    port_name: &str,
    description: Option<&str>,
    manufacturer: Option<&str>,
    port_type: &str,
) -> String {
    let mut parts = vec![port_name.to_string()];
    if let Some(value) = description.filter(|value| !value.trim().is_empty()) {
        parts.push(value.trim().to_string());
    }
    if let Some(value) = manufacturer.filter(|value| !value.trim().is_empty()) {
        parts.push(value.trim().to_string());
    }
    if parts.len() == 1 {
        parts.push(port_type.to_uppercase());
    }
    parts.join(" · ")
}

fn compare_serial_port_names(left: &str, right: &str) -> std::cmp::Ordering {
    serial_port_sort_key(left).cmp(&serial_port_sort_key(right))
}

fn serial_port_sort_key(port_name: &str) -> (String, u32, String) {
    let trimmed = port_name.trim().to_ascii_uppercase();
    let prefix = trimmed
        .chars()
        .take_while(|ch| !ch.is_ascii_digit())
        .collect::<String>();
    let number_text = trimmed
        .chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    let number = number_text.parse::<u32>().unwrap_or(u32::MAX);
    (prefix, number, trimmed)
}


fn prune_finished_simulators() -> Result<(), String> {
    let mut simulators = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao limpar simuladores finalizados.".to_string())?;
    simulators.retain(|_, runtime| !runtime.shutdown.load(Ordering::SeqCst) && !runtime.task.is_finished());
    Ok(())
}

fn serial_settings(config: &ScaleSimulatorConfig) -> (u8, &str, u8) {
    let toledo = matches!(config.driver, ScaleSimulatorDriver::Toledo8540);
    (
        config
            .serial_data_bits
            .unwrap_or(if toledo { 7 } else { 8 }),
        config
            .serial_parity
            .as_deref()
            .unwrap_or(if toledo { "even" } else { "none" }),
        config.serial_stop_bits.unwrap_or_else(|| {
            if toledo {
                config.toledo8540_stop_bits.unwrap_or(1)
            } else {
                1
            }
        }),
    )
}

fn serial_builder(config: &ScaleSimulatorConfig) -> serialport::SerialPortBuilder {
    let (bits, parity, stop) = serial_settings(config);
    serialport::new(config.serial_port.trim(), config.baud_rate)
        .timeout(Duration::from_millis(250))
        .data_bits(match bits {
            5 => serialport::DataBits::Five,
            6 => serialport::DataBits::Six,
            7 => serialport::DataBits::Seven,
            _ => serialport::DataBits::Eight,
        })
        .parity(match parity {
            "even" => serialport::Parity::Even,
            "odd" => serialport::Parity::Odd,
            _ => serialport::Parity::None,
        })
        .stop_bits(if stop == 2 {
            serialport::StopBits::Two
        } else {
            serialport::StopBits::One
        })
}

fn preflight_serial_port(config: &ScaleSimulatorConfig) -> Result<(), String> {
    let serial_name = config.serial_port.trim();
    let builder = serial_builder(config);
    match builder.open() {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Falha ao abrir a porta serial {serial_name}: {err}. Se a intenção é simular para o All Driver Platform no mesmo computador, crie um par virtual COM, por exemplo: simulador em COM10 e ADP lendo COM11. Não use a mesma COM nos dois programas."
        )),
    }
}

fn with_runtime<T>(id: &str, callback: impl FnOnce(&SimulatorRuntime) -> Result<T, String>) -> Result<T, String> {
    let simulators = SIMULATORS
        .lock()
        .map_err(|_| "Falha ao bloquear lista de simuladores.".to_string())?;
    let runtime = simulators
        .get(id)
        .ok_or_else(|| "Simulador não encontrado.".to_string())?;
    callback(runtime)
}

fn validate_config(config: &ScaleSimulatorConfig) -> Result<(), String> {
    if config.frame_override_enabled {
        frame_test_overrides::validate(
            frame_kind(&config.driver),
            config.frame_prefix_hex.as_deref(),
            config.frame_suffix_hex.as_deref(),
            config.frame_checksum_mode.as_deref().unwrap_or("protocol"),
        )?;
    }

    if !matches!(config.serial_data_bits, None | Some(5..=8))
        || !matches!(config.serial_stop_bits, None | Some(1) | Some(2))
        || !matches!(
            config.serial_parity.as_deref(),
            None | Some("none" | "even" | "odd")
        )
    {
        return Err("Configuração serial inválida: use 5–8 bits, paridade none/even/odd e 1 ou 2 stop bits.".into());
    }

    if matches!(config.driver, ScaleSimulatorDriver::Toledo8540) {
        toledo8540_encoder::settings(config.division_kg, config.capacity_kg, &config.unit)?;
        if !matches!(config.toledo8540_stop_bits, None | Some(1) | Some(2)) {
            return Err("Toledo 8540: selecione 1 ou 2 stop bits.".into());
        }
        if !config.initial_weight_kg.is_finite() || !config.variation_kg.is_finite() {
            return Err("Toledo 8540: pesos devem ser finitos.".into());
        }
    }
    if config.name.trim().is_empty() {
        return Err("Informe o nome do simulador.".to_string());
    }
    if config.mode.is_tcp() {
        if config.host.trim().is_empty() {
            return Err("Informe o host de escuta TCP.".to_string());
        }
        if config.port == 0 {
            return Err("Informe uma porta TCP válida.".to_string());
        }
    }
    if config.mode.is_serial() {
        let serial_port = config.serial_port.trim();
        if serial_port.is_empty() {
            return Err("Informe a porta serial RS232, exemplo: COM1, COM2 ou COM10.".to_string());
        }
        if config.baud_rate == 0 {
            return Err("Informe um baud rate serial válido.".to_string());
        }
        let available_ports = list_serial_ports_with_usage()?;
        if !available_ports
            .iter()
            .any(|port| port.port_name.eq_ignore_ascii_case(serial_port))
        {
            let detected = if available_ports.is_empty() {
                "nenhuma porta COM foi identificada pelo Windows".to_string()
            } else {
                available_ports
                    .iter()
                    .map(|port| port.port_name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            return Err(format!(
                "A porta serial {serial_port} não existe ou não foi identificada neste computador. Portas detectadas: {detected}. Para simular em ambiente local, crie um par virtual COM, por exemplo COM10/COM11."
            ));
        }
    }
    if config.capacity_kg <= 0.0 {
        return Err("A capacidade deve ser maior que zero.".to_string());
    }
    if config.division_kg <= 0.0 {
        return Err("A divisão deve ser maior que zero.".to_string());
    }
    if config.interval_ms < 50 {
        return Err("O intervalo mínimo de transmissão é 50 ms.".to_string());
    }
    if config.initial_weight_kg.abs() > config.capacity_kg {
        return Err("O peso inicial não pode exceder a capacidade configurada.".to_string());
    }
    Ok(())
}

async fn run_tcp_server(
    listener: TcpListener,
    config: ScaleSimulatorConfig,
    state: Arc<Mutex<ScaleState>>,
    clients: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    logs: Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>,
) {
    let bind_addr = format!("{}:{}", config.host, config.port);
    push_log(&logs, "info", format!("Escutando em tcp://{bind_addr}."));

    loop {
        if shutdown.load(Ordering::SeqCst) {
            push_log(&logs, "info", "Servidor TCP finalizado por solicitação do usuário.".to_string());
            break;
        }

        match tokio::time::timeout(Duration::from_millis(250), listener.accept()).await {
            Ok(Ok((stream, addr))) => {
                clients.fetch_add(1, Ordering::SeqCst);
                push_log(&logs, "info", format!("Cliente conectado: {addr}."));
                let client_config = config.clone();
                let client_state = state.clone();
                let client_clients = clients.clone();
                let client_shutdown = shutdown.clone();
                let client_logs = logs.clone();
                tokio::spawn(async move {
                    handle_tcp_client(
                        stream,
                        client_config,
                        client_state,
                        client_clients,
                        client_shutdown,
                        client_logs,
                    )
                    .await;
                });
            }
            Ok(Err(err)) => {
                push_log(&logs, "error", format!("Falha ao aceitar conexão: {err}"));
                sleep(Duration::from_millis(250)).await;
            }
            Err(_) => {}
        }
    }
}

async fn handle_tcp_client(
    mut stream: TcpStream,
    config: ScaleSimulatorConfig,
    state: Arc<Mutex<ScaleState>>,
    clients: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    logs: Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>,
) {
    let mut read_buffer = [0_u8; 1024];
    loop {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        if config.mode.is_command() {
            match tokio::time::timeout(Duration::from_millis(250), stream.read(&mut read_buffer)).await {
                Ok(Ok(size)) if size > 0 => {
                    let command = String::from_utf8_lossy(&read_buffer[..size]).to_string();
                    let response = handle_protocol_command(&command, &config, &state, &logs);
                    if let Some(frame) = response {
                        if stream.write_all(frame.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                }
                Ok(Ok(_)) => break,
                Ok(Err(err)) if err.kind() == io::ErrorKind::WouldBlock => {}
                Ok(Err(_)) => break,
                Err(_) => {}
            }
        } else {
            apply_variation(&config, &state);
            let frame = frame_and_update_last(&config, &state);
            if stream.write_all(frame.as_bytes()).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(config.interval_ms)).await;
        }
    }

    clients.fetch_sub(1, Ordering::SeqCst);
    push_log(&logs, "info", "Cliente TCP desconectado.".to_string());
}

fn run_serial_loop(
    config: ScaleSimulatorConfig,
    state: Arc<Mutex<ScaleState>>,
    clients: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    logs: Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>,
) {
    let serial_name = config.serial_port.trim().to_string();
    let builder = serial_builder(&config);
    let mut port = match builder.open() {
        Ok(port) => port,
        Err(err) => {
            push_log(
                &logs,
                "error",
                format!(
                    "Falha ao abrir porta serial {serial_name}: {err}. Para simular RS232, use uma porta COM real ou par virtual como com0com/VSPE."
                ),
            );
            return;
        }
    };

    clients.store(1, Ordering::SeqCst);
    push_log(
        &logs,
        "info",
        format!("Porta serial {serial_name} aberta em {} bps.", config.baud_rate),
    );

    let mut read_buffer = [0_u8; 1024];
    while !shutdown.load(Ordering::SeqCst) {
        if config.mode.is_command() {
            match port.read(&mut read_buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&read_buffer[..size]).to_string();
                    if let Some(frame) = handle_protocol_command(&command, &config, &state, &logs) {
                        if let Err(err) = port.write_all(frame.as_bytes()) {
                            push_log(&logs, "error", format!("Falha ao escrever em {serial_name}: {err}"));
                            break;
                        }
                    }
                }
                Ok(_) => {}
                Err(err) if err.kind() == io::ErrorKind::TimedOut => {}
                Err(err) => {
                    push_log(&logs, "error", format!("Falha ao ler {serial_name}: {err}"));
                    break;
                }
            }
        } else {
            apply_variation(&config, &state);
            let frame = frame_and_update_last(&config, &state);
            if let Err(err) = port.write_all(frame.as_bytes()) {
                push_log(&logs, "error", format!("Falha ao escrever em {serial_name}: {err}"));
                break;
            }
            std::thread::sleep(Duration::from_millis(config.interval_ms));
        }
    }

    clients.store(0, Ordering::SeqCst);
    push_log(&logs, "info", format!("Porta serial {serial_name} finalizada."));
}

fn handle_protocol_command(
    raw_command: &str,
    config: &ScaleSimulatorConfig,
    state: &Arc<Mutex<ScaleState>>,
    logs: &Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>,
) -> Option<String> {
    let normalized = raw_command.trim_matches(char::from(0)).trim().to_uppercase();
    if normalized.is_empty()
        || normalized == "P"
        || normalized == "PESO"
        || normalized == "READ"
        || normalized == "ENQ"
        || raw_command.as_bytes().contains(&0x05)
    {
        return Some(frame_and_update_last(config, state));
    }
    if normalized == "ZERO" || normalized == "Z" {
        if let Ok(mut guard) = state.lock() {
            guard.gross_kg = 0.0;
            guard.tare_kg = 0.0;
        }
        push_log(logs, "info", "Comando remoto ZERO recebido.".to_string());
        return Some(frame_and_update_last(config, state));
    }
    if normalized == "TARE" || normalized == "TARA" || normalized == "T" {
        if let Ok(mut guard) = state.lock() {
            guard.tare_kg = guard.gross_kg.max(0.0);
            guard.gross_mode = false;
        }
        push_log(logs, "info", "Comando remoto TARA recebido.".to_string());
        return Some(frame_and_update_last(config, state));
    }
    if normalized == "GROSS" || normalized == "BRUTO" {
        if let Ok(mut guard) = state.lock() {
            guard.gross_mode = true;
        }
        return Some(frame_and_update_last(config, state));
    }
    if normalized == "NET" || normalized == "LIQUIDO" || normalized == "LÍQUIDO" {
        if let Ok(mut guard) = state.lock() {
            guard.gross_mode = false;
        }
        return Some(frame_and_update_last(config, state));
    }
    if let Some(value) = normalized.strip_prefix("WEIGHT ") {
        if let Ok(weight) = value.trim().replace(',', ".").parse::<f64>() {
            if let Ok(mut guard) = state.lock() {
                guard.gross_kg = normalize_weight(weight, config.division_kg);
            }
            return Some(frame_and_update_last(config, state));
        }
    }
    push_log(logs, "warn", format!("Comando remoto ignorado: {normalized}."));
    Some(frame_and_update_last(config, state))
}

fn apply_variation(config: &ScaleSimulatorConfig, state: &Arc<Mutex<ScaleState>>) {
    if config.variation_kg.abs() <= f64::EPSILON {
        return;
    }
    if let Ok(mut guard) = state.lock() {
        let seed = Utc::now().timestamp_millis() as f64;
        let wave = (seed / 1000.0).sin() * config.variation_kg;
        let next = guard.gross_kg + wave;
        guard.gross_kg = normalize_weight(next.clamp(-config.capacity_kg, config.capacity_kg), config.division_kg);
        guard.stable = config.stable && wave.abs() < config.division_kg;
    }
}

fn frame_kind(driver: &ScaleSimulatorDriver) -> frame_test_overrides::Kind {
    match driver {
        ScaleSimulatorDriver::Toledo8540 => frame_test_overrides::Kind::Toledo8540,
        ScaleSimulatorDriver::LiderLd2052 => frame_test_overrides::Kind::Lider,
        ScaleSimulatorDriver::ToledoPrix3 => frame_test_overrides::Kind::StxEtx,
        _ => frame_test_overrides::Kind::Line,
    }
}

fn transmission_frame(config: &ScaleSimulatorConfig, state: &ScaleState) -> Result<String, String> {
    let frame = build_frame(config, state);
    if !config.frame_override_enabled {
        return Ok(frame);
    }
    frame_test_overrides::apply(
        &frame,
        frame_kind(&config.driver),
        config.frame_prefix_hex.as_deref(),
        config.frame_suffix_hex.as_deref(),
        config.frame_checksum_mode.as_deref().unwrap_or("protocol"),
    )
}

fn frame_and_update_last(config: &ScaleSimulatorConfig, state: &Arc<Mutex<ScaleState>>) -> String {
    let frame = match state.lock() {
        Ok(guard) => transmission_frame(config, &guard).unwrap_or_default(),
        Err(_) => "ERR\r\n".to_string(),
    };
    if let Ok(mut guard) = state.lock() {
        guard.last_frame = frame.clone();
    }
    frame
}

fn build_frame(config: &ScaleSimulatorConfig, state: &ScaleState) -> String {
    let net_kg = state.gross_kg - state.tare_kg;
    let display_kg = if state.gross_mode { state.gross_kg } else { net_kg };
    let sign = if display_kg < 0.0 { '-' } else { '+' };
    let unit = config.unit.trim();
    let scaled = display_kg.abs().round() as i64;
    let tare_scaled = state.tare_kg.abs().round() as i64;
    let mode = if state.gross_mode { "GS" } else { "NT" };
    let stability = if state.stable { "ST" } else { "US" };

    match config.driver {
        // Compatível com o parser do All Driver Platform:
        // parts[0] == "0" significa estável; parts[1]=bruto; parts[2]=tara; parts[3]=líquido.
        ScaleSimulatorDriver::Wt1000n => {
            let status = if state.stable { "0" } else { "1" };
            format!("{status},{},{},{}\r\n", signed_value(state.gross_kg), signed_value(state.tare_kg), signed_value(net_kg))
        }
        ScaleSimulatorDriver::Wt27r => format!("={sign}{scaled:06}{unit}\r\n"),
        // Compatível com o parser WT3000IR do ADP: status em parts[0] e números em parts[2].
        ScaleSimulatorDriver::Wt3000ir => {
            format!("{stability},{mode},{sign}{scaled:06}{unit} {tare_scaled:06}{unit}\r\n")
        }
        ScaleSimulatorDriver::Toledo8540 => toledo8540_encoder::encode(
            state.gross_kg,
            state.tare_kg,
            state.stable,
            state.gross_mode,
            config.capacity_kg,
            config.division_kg,
            &config.unit,
        ),
        ScaleSimulatorDriver::ToledoPrix3 => format!("\u{0002}{sign}{scaled:06}{unit}\u{0003}"),
        // Compatível com parser A12e do ADP.
        ScaleSimulatorDriver::A12e => format!("wn{sign}{scaled:06}{unit}\r\n"),
        // Compatível com parser LIDERLD2052 do ADP: 0x01 + 7 ASCII + espaço + E/I + 0x02.
        ScaleSimulatorDriver::LiderLd2052 => {
            let status = if state.stable { 'E' } else { 'I' };
            format!("\u{0001}{scaled:>07} {status}\u{0002}")
        }
        ScaleSimulatorDriver::Filizola => format!("P{sign}{scaled:06}{unit}\r\n"),
        ScaleSimulatorDriver::GenericAscii => format!("{sign}{scaled:06} {unit}\r\n"),
    }
}

fn signed_value(value: f64) -> String {
    if value.fract().abs() <= f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.3}")
    }
}

fn status_from_runtime(id: &str, runtime: &SimulatorRuntime) -> Result<ScaleSimulatorStatus, String> {
    let state = runtime
        .state
        .lock()
        .map_err(|_| "Falha ao ler estado do simulador.".to_string())?;
    let net_kg = state.gross_kg - state.tare_kg;
    Ok(ScaleSimulatorStatus {
        id: id.to_string(),
        name: runtime.config.name.clone(),
        driver: runtime.config.driver.clone(),
        mode: runtime.config.mode.clone(),
        host: runtime.config.host.clone(),
        port: runtime.config.port,
        serial_port: runtime.config.serial_port.clone(),
        baud_rate: runtime.config.baud_rate,
        serial_data_bits: serial_settings(&runtime.config).0,
        serial_parity: serial_settings(&runtime.config).1.to_string(),
        serial_stop_bits: serial_settings(&runtime.config).2,
        frame_override_enabled: runtime.config.frame_override_enabled,
        running: !runtime.shutdown.load(Ordering::SeqCst),
        clients: runtime.clients.load(Ordering::SeqCst),
        weight_kg: if state.gross_mode { state.gross_kg } else { net_kg },
        gross_kg: state.gross_kg,
        tare_kg: state.tare_kg,
        net_kg,
        stable: state.stable,
        last_frame: state.last_frame.clone(),
        started_at: runtime.started_at,
    })
}

fn normalize_weight(value: f64, division: f64) -> f64 {
    if division <= 0.0 {
        return value;
    }
    (value / division).round() * division
}

fn push_log(logs: &Arc<Mutex<VecDeque<ScaleSimulatorLogEntry>>>, level: &str, message: String) {
    if let Ok(mut guard) = logs.lock() {
        if guard.len() >= 200 {
            guard.pop_front();
        }
        guard.push_back(ScaleSimulatorLogEntry {
            at: Utc::now(),
            level: level.to_string(),
            message,
        });
    }
}
