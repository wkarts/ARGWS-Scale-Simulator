# Correção de build em caminhos com espaço no Windows

## Problema

Em máquinas onde o usuário do Windows possui espaço no nome, por exemplo:

```text
C:\Users\Wallace Kleiton
```

o build Tauri falhava ao chamar o Cargo durante a etapa final:

```text
'C:\Users\Wallace' não é reconhecido como um comando interno ou externo
```

## Causa

O script `scripts/ci/run-tauri.mjs` resolvia corretamente o caminho completo do `cargo.exe`, mas repassava esse caminho absoluto na variável de ambiente `CARGO`:

```text
C:\Users\Wallace Kleiton\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe
```

Em algumas etapas internas do Tauri/Cargo no Windows, esse valor pode ser usado em comando de shell sem aspas corretas. Assim, o Windows interpreta apenas:

```text
C:\Users\Wallace
```

como comando.

## Correção aplicada

O script continua localizando e validando o `cargo.exe` real, mas no Windows repassa ao Tauri apenas:

```text
CARGO=cargo
```

E adiciona a pasta real do Cargo no início do `PATH`.

Dessa forma, o Tauri encontra o Cargo corretamente, sem quebrar por causa do espaço no caminho.

## Arquivo alterado

```text
scripts/ci/run-tauri.mjs
```

## Validação recomendada

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --all
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
.\build-windows.bat
```
