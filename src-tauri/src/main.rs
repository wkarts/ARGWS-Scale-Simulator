#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args = scale_simulator_tauri_lib::cli::args::CliArgs::parse();
    if scale_simulator_tauri_lib::cli::runner::should_run_without_tauri(&args) {
        if let Err(err) = scale_simulator_tauri_lib::cli::runner::run(args) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        return;
    }
    scale_simulator_tauri_lib::run();
}
