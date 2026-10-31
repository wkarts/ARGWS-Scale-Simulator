mod app_state;
mod db;
mod migrations;
mod models;
mod security;
mod runtime_config;
#[allow(dead_code)]
mod core;
#[allow(dead_code)]
pub mod internal_api;
#[allow(dead_code)]
mod integrations;
#[allow(dead_code)]
mod service;
#[allow(dead_code)]
pub mod cli;

mod commands {
    pub mod access;
    pub mod app;
    pub mod auth;
    pub mod companies;
    pub mod entities;
    pub mod licensing;
    pub mod support;
    pub mod integrations;
    pub mod runtime;
    pub mod scale_simulator;
    pub mod scale_models;
}

use app_state::SharedState;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if is_tray_enabled() && close_to_tray() {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .manage(SharedState::new())
        .setup(|app| {
            let state = app.state::<SharedState>();
            let _ = crate::runtime_config::init_env_file(None);
            state.init().map_err(|err| -> Box<dyn std::error::Error> {
                Box::new(std::io::Error::other(err))
            })?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.maximize();
                let _ = window.set_focus();
            }

            if is_tray_enabled() {
                setup_tray(app)?;
            }

            if should_auto_start_services(state.inner()) {
                if let Ok(config) =
                    crate::runtime_config::resolve_internal_api_config(state.inner())
                {
                    let _ = crate::internal_api::server::start_background(
                        state.inner().clone(),
                        config,
                    );
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_bootstrap,
            commands::app::app_meta,
            commands::app::system_info,
            commands::app::system_set_data_dir,
            commands::app::app_log_write,
            commands::app::app_log_list,
            commands::app::app_log_clear,
            commands::auth::auth_login,
            commands::auth::auth_restore,
            commands::auth::auth_logout,
            commands::auth::auth_change_password,
            commands::access::permission_catalog,
            commands::access::profile_list,
            commands::access::profile_get,
            commands::access::profile_save,
            commands::access::profile_delete,
            commands::access::user_list,
            commands::access::user_get,
            commands::access::user_policy_get,
            commands::access::user_policy_save,
            commands::access::user_save,
            commands::access::user_delete,
            commands::companies::company_list,
            commands::companies::company_get,
            commands::companies::company_save,
            commands::companies::company_delete,
            commands::companies::company_lookup_cnpj,
            commands::companies::company_lookup_ie,
            commands::entities::entity_list,
            commands::entities::entity_save,
            commands::entities::entity_delete,
            commands::entities::combo_list,
            commands::entities::entity_provider_list,
            commands::entities::provider_entity_list,
            commands::entities::provider_entity_get,
            commands::entities::provider_entity_create,
            commands::entities::provider_entity_update,
            commands::entities::provider_entity_delete,
            commands::licensing::licensing_status,
            commands::licensing::licensing_load_settings,
            commands::licensing::licensing_save_settings,
            commands::licensing::licensing_device_info,
            commands::licensing::licensing_check_runtime,
            commands::licensing::licensing_start_trial,
            commands::support::support_guard_status,
            commands::support::support_guard_provision,
            commands::support::support_guard_enable_totp,
            commands::support::support_guard_unlock,
            commands::integrations::integration_list,
            commands::integrations::integration_save,
            commands::integrations::integration_delete,
            commands::integrations::integration_test,
            commands::integrations::integration_logs,
            commands::runtime::internal_api_status,
            commands::runtime::internal_api_start,
            commands::runtime::internal_api_stop,
            commands::runtime::internal_api_restart,
            commands::runtime::internal_api_test_port,
            commands::runtime::internal_api_test,
            commands::runtime::runtime_settings_load,
            commands::runtime::runtime_settings_save,
            commands::runtime::runtime_env_example,
            commands::runtime::startup_with_windows_set,
            commands::runtime::app_service_install,
            commands::runtime::app_service_uninstall,
            commands::runtime::app_service_start,
            commands::runtime::app_service_stop,
            commands::runtime::app_service_restart,
            commands::runtime::app_service_status,
            commands::runtime::open_print_preview,
            commands::runtime::tray_status,
            commands::runtime::tray_restore_window,
            commands::runtime::tray_exit_app,
            commands::scale_models::scale_models_list,
            commands::scale_models::scale_model_save,
            commands::scale_models::scale_model_delete,
            commands::scale_simulator::scale_simulator_presets,
            commands::scale_simulator::scale_simulator_preview_frame,
            commands::scale_simulator::scale_simulator_start,
            commands::scale_simulator::scale_simulator_stop,
            commands::scale_simulator::scale_simulator_stop_all,
            commands::scale_simulator::scale_simulator_list,
            commands::scale_simulator::scale_simulator_logs,
            commands::scale_simulator::scale_simulator_set_weight,
            commands::scale_simulator::scale_simulator_action,
            commands::scale_simulator::scale_simulator_serial_ports,
        ])
        .run(tauri::generate_context!())
        .expect("erro ao executar a aplicação Tauri");
}

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

    let restore = MenuItem::with_id(app, "restore", "Restaurar", true, None::<&str>)?;
    let api_start = MenuItem::with_id(app, "api_start", "Iniciar API interna", true, None::<&str>)?;
    let api_restart = MenuItem::with_id(
        app,
        "api_restart",
        "Reiniciar API interna",
        true,
        None::<&str>,
    )?;
    let api_stop = MenuItem::with_id(app, "api_stop", "Parar API interna", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Sair definitivamente", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&restore, &api_start, &api_restart, &api_stop, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip(runtime_app_name())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "restore" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            "api_start" => {
                let state = app.state::<SharedState>();
                if let Ok(config) =
                    crate::runtime_config::resolve_internal_api_config(state.inner())
                {
                    let _ = crate::internal_api::server::start_background(
                        state.inner().clone(),
                        config,
                    );
                }
            }
            "api_restart" => {
                let _ = crate::internal_api::server::stop_background();
                let state = app.state::<SharedState>();
                if let Ok(config) =
                    crate::runtime_config::resolve_internal_api_config(state.inner())
                {
                    let _ = crate::internal_api::server::start_background(
                        state.inner().clone(),
                        config,
                    );
                }
            }
            "api_stop" => {
                let _ = crate::internal_api::server::stop_background();
            }
            "quit" => app.exit(0),
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    let _tray = builder.build(app)?;

    Ok(())
}

fn runtime_app_name() -> String {
    std::env::var("TEMPLATE_APP_NAME")
        .or_else(|_| std::env::var("APP_NAME"))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Scale Simulator".to_string())
}

fn env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn is_tray_enabled() -> bool {
    env_bool("TEMPLATE_APP_TRAY_ENABLED", true)
}

fn close_to_tray() -> bool {
    env_bool("TEMPLATE_APP_TRAY_CLOSE_TO_TRAY", false)
}

fn should_auto_start_services(state: &SharedState) -> bool {
    crate::runtime_config::load_settings(state)
        .map(|payload| {
            payload.settings.services_auto_start || payload.settings.internal_api_auto_start
        })
        .unwrap_or_else(|_| {
            env_bool("TEMPLATE_APP_SERVICES_AUTO_START", false)
                || env_bool("TEMPLATE_APP_API_AUTO_START", false)
        })
}
