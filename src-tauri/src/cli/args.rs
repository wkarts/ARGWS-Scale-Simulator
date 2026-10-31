use std::{env, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeMode {
    Desktop,
    HeadlessApi,
    Cli,
    Worker,
}

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub mode: RuntimeMode,
    pub host: String,
    pub port: u16,
    pub database_driver: String,
    pub data_dir: Option<PathBuf>,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            mode: RuntimeMode::Desktop,
            host: env::var("TEMPLATE_APP_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("TEMPLATE_APP_API_PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(61001),
            database_driver: "sqlite".to_string(),
            data_dir: None,
        }
    }
}

impl CliArgs {
    pub fn parse() -> Self {
        let mut parsed = Self::default();
        let args: Vec<String> = env::args().skip(1).collect();
        let mut index = 0usize;
        while index < args.len() {
            let arg = &args[index];
            if let Some(value) = arg.strip_prefix("--mode=") {
                parsed.mode = parse_mode(value);
            } else if arg == "--mode" {
                if let Some(value) = args.get(index + 1) {
                    parsed.mode = parse_mode(value);
                    index += 1;
                }
            } else if let Some(value) = arg.strip_prefix("--host=") {
                if !value.trim().is_empty() {
                    parsed.host = value.trim().to_string();
                }
            } else if arg == "--host" {
                if let Some(value) = args.get(index + 1) {
                    if !value.trim().is_empty() {
                        parsed.host = value.trim().to_string();
                    }
                    index += 1;
                }
            } else if let Some(value) = arg.strip_prefix("--port=") {
                if let Ok(port) = value.parse::<u16>() {
                    parsed.port = port;
                }
            } else if arg == "--port" {
                if let Some(value) = args.get(index + 1) {
                    if let Ok(port) = value.parse::<u16>() {
                        parsed.port = port;
                    }
                    index += 1;
                }
            } else if let Some(value) = arg.strip_prefix("--database-driver=") {
                parsed.database_driver = value.to_ascii_lowercase();
            } else if arg == "--database-driver" {
                if let Some(value) = args.get(index + 1) {
                    parsed.database_driver = value.to_ascii_lowercase();
                    index += 1;
                }
            } else if let Some(value) = arg.strip_prefix("--data-dir=") {
                if !value.trim().is_empty() {
                    parsed.data_dir = Some(PathBuf::from(value));
                }
            } else if arg == "--data-dir" {
                if let Some(value) = args.get(index + 1) {
                    if !value.trim().is_empty() {
                        parsed.data_dir = Some(PathBuf::from(value));
                    }
                    index += 1;
                }
            }
            index += 1;
        }
        parsed
    }
}

fn parse_mode(value: &str) -> RuntimeMode {
    match value.trim().to_ascii_lowercase().as_str() {
        "headless-api" | "server" | "api" => RuntimeMode::HeadlessApi,
        "cli" => RuntimeMode::Cli,
        "worker" => RuntimeMode::Worker,
        _ => RuntimeMode::Desktop,
    }
}
