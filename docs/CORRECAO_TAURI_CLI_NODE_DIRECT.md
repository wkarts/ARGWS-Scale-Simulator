# Correção do build Windows com caminho contendo espaço

## Problema

A etapa final do build falhava ao executar o Tauri CLI quando o projeto ficava em um caminho como:

```text
C:\Users\Wallace Kleiton\Downloads\...
```

O erro visto era:

```text
'\"C:\Users\Wallace Kleiton\...\node_modules\.bin\tauri.cmd\"' não é reconhecido
```

Isso ocorria porque o script tentava chamar `tauri.cmd` por `cmd.exe`, e o caminho acabava chegando ao Windows com aspas escapadas literalmente.

## Correção

O arquivo `scripts/ci/run-tauri.mjs` agora evita chamar `node_modules/.bin/tauri.cmd`.

Quando o pacote local `@tauri-apps/cli/tauri.js` existe, o script chama diretamente:

```text
node node_modules/@tauri-apps/cli/tauri.js build
```

A chamada é feita com `spawnSync(command, args, { shell: false })`, passando o caminho e os argumentos separadamente. Isso elimina problemas de aspas e espaços em diretórios do Windows.

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
