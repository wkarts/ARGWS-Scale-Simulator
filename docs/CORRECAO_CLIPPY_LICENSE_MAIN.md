# Correção do clippy/build Windows - v0.1.3

## Problemas corrigidos

1. `src-tauri/src/main.rs` ainda referenciava o crate antigo `template_app_tauri_lib`, mesmo após o projeto ter sido renomeado para `scale_simulator_tauri`.

2. `src-tauri/generic-license-tauri/src/device.rs` gerava warnings no Windows:
   - import não utilizado de `std::fs`;
   - expressões inalcançáveis após blocos `return` condicionais por plataforma.

## Correções aplicadas

- Troca de `template_app_tauri_lib` para `scale_simulator_tauri_lib`;
- Import de `std::fs` limitado ao Linux com `#[cfg(target_os = "linux")]`;
- Reescrita das funções de coleta de dados do dispositivo para retornar expressões por plataforma, sem código inalcançável;
- Atualização da versão para `0.1.3`.

## Comandos recomendados

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --all
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
.\build-windows.bat
```
