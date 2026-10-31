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
    #[cfg(feature = "desktop")]
    scale_simulator_tauri_lib::run();

    #[cfg(not(feature = "desktop"))]
    {
        eprintln!(
            "O modo desktop exige a feature Cargo 'desktop'. Use --headless, --cli ou --worker."
        );
        std::process::exit(2);
    }
}
