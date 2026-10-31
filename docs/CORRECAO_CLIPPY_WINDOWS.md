# Correção do cargo clippy no Windows

## Problema corrigido

O build estava falhando com `-D warnings` por quatro pontos:

1. import `path::PathBuf` não usado no Windows em `runtime.rs`;
2. variável `simulators` marcada como `mut` sem necessidade em `scale_simulator_start`;
3. implementação manual de `Default` para `ScaleSimulatorDriver`;
4. implementação manual de `Default` para `ScaleSimulatorMode`.

## Correção aplicada

- `PathBuf` agora é importado apenas em Linux com `#[cfg(target_os = "linux")]`;
- removido `mut` desnecessário no bloco apenas de validação;
- `Default` dos enums foi convertido para `#[derive(Default)]` com `#[default]` na variante padrão.

## Arquivos alterados

- `src-tauri/src/commands/runtime.rs`
- `src-tauri/src/commands/scale_simulator.rs`
- `VERSION`

## Comandos recomendados

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --all
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
.\build-windows.bat
```
