use crate::{
    app_state::SharedState,
    core::database::{config::DatabaseConfig, health, migrations as db_migrations},
    internal_api,
};

use super::args::{CliArgs, RuntimeMode};

pub fn should_run_without_tauri(args: &CliArgs) -> bool {
    matches!(
        args.mode,
        RuntimeMode::HeadlessApi | RuntimeMode::Cli | RuntimeMode::Worker
    )
}

pub fn run(args: CliArgs) -> Result<(), String> {
    let state = SharedState::new();
    if let Some(data_dir) = args.data_dir.clone() {
        state.init_with_data_dir(data_dir)?;
    } else {
        state.init()?;
    }

    let db_config = DatabaseConfig::from_env_with_driver(&args.database_driver);
    let sqlite_path = state.db_path()?;
    if db_config.driver.is_external() {
        db_migrations::migrate(&db_config, &sqlite_path)?;
    }

    match args.mode {
        RuntimeMode::HeadlessApi => {
            let config = internal_api::config::InternalApiConfig::default()
                .with_host_port(args.host.clone(), args.port);
            internal_api::server::run_blocking_with_config(&state, config)
        }
        RuntimeMode::Cli => {
            let db_path = state.db_path()?;
            let data_dir = state.data_dir()?;
            println!("Template App CLI");
            println!("Versão: {}", env!("CARGO_PKG_VERSION"));
            let health = health::report(&db_config, &db_path);
            println!("Banco SQLite local: {}", db_path.display());
            println!("Driver configurado: {}", health.driver);
            println!("Destino: {}", health.target);
            println!("Status banco: {} - {}", health.ok, health.message);
            println!("Dados: {}", data_dir.display());
            Ok(())
        }
        RuntimeMode::Worker => {
            println!("Worker inicializado. Nenhuma fila ativa configurada neste template.");
            Ok(())
        }
        RuntimeMode::Desktop => Ok(()),
    }
}
