# Correção do Tauri CLI em caminho Windows com espaço

## Problema

O build chegava na etapa final e falhava com:

```text
\'\"C:\Users\Wallace Kleiton\...\node_modules\.bin\tauri.cmd\"\' não é reconhecido como um comando interno
```

A causa era a montagem manual do comando do Tauri CLI com aspas escapadas. Em caminhos Windows com espaço, o `cmd.exe` passou a receber `\"C:\...\"` como parte literal do comando.

## Correção

O arquivo `scripts/ci/run-tauri.mjs` agora chama o `tauri.cmd` usando:

```cmd
cmd.exe /d /c call "caminho\node_modules\.bin\tauri.cmd" "build"
```

Isso preserva corretamente caminhos como:

```text
C:\Users\Wallace Kleiton\Downloads\...
```

## Arquivo alterado

- `scripts/ci/run-tauri.mjs`

## Validação recomendada

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --all
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
.\build-windows.bat
```
