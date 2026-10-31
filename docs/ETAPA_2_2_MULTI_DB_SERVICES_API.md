# Etapa 2.2 — Multi-banco, serviços e API controlável

Esta etapa transforma os recursos preparados da Etapa 2.1 em implementações reais mínimas e seguras.

## Banco de dados

SQLite continua sendo o padrão e permanece obrigatório para o desktop local.

MySQL/MariaDB e PostgreSQL foram adicionados como suporte real por feature Rust:

```bash
cargo build --manifest-path src-tauri/Cargo.toml --features mysql-db
cargo build --manifest-path src-tauri/Cargo.toml --features postgres-db
cargo build --manifest-path src-tauri/Cargo.toml --features mysql-db,postgres-db
```

Variáveis aceitas:

```bash
SCALE_SIMULATOR_DB_HOST=127.0.0.1
SCALE_SIMULATOR_DB_PORT=3306
SCALE_SIMULATOR_DB_DATABASE=scale_simulator
SCALE_SIMULATOR_DB_USERNAME=root
SCALE_SIMULATOR_DB_PASSWORD=senha
```

Ou prefixos específicos:

```bash
SCALE_SIMULATOR_MYSQL_HOST=127.0.0.1
SCALE_SIMULATOR_MYSQL_PORT=3306
SCALE_SIMULATOR_MYSQL_DATABASE=scale_simulator
SCALE_SIMULATOR_MYSQL_USERNAME=root
SCALE_SIMULATOR_MYSQL_PASSWORD=senha

SCALE_SIMULATOR_POSTGRES_HOST=127.0.0.1
SCALE_SIMULATOR_POSTGRES_PORT=5432
SCALE_SIMULATOR_POSTGRES_DATABASE=scale_simulator
SCALE_SIMULATOR_POSTGRES_USERNAME=postgres
SCALE_SIMULATOR_POSTGRES_PASSWORD=senha
```

Firebird foi intencionalmente ignorado nesta etapa por compatibilidade.

## Headless/API

Exemplo:

```bash
cargo run --manifest-path src-tauri/Cargo.toml -- --mode=headless-api --host 127.0.0.1 --port 61001
```

Com MySQL:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --features mysql-db -- --mode=headless-api --database-driver mysql
```

Com PostgreSQL:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --features postgres-db -- --mode=headless-api --database-driver postgres
```

## API controlável pelo desktop

Comandos Tauri adicionados:

```text
internal_api_status
internal_api_start
internal_api_stop
internal_api_restart
```

A API respeita segurança por token, bind host, CORS e exposição da documentação.

## Serviço Windows

Os comandos agora executam `sc.exe` em Windows:

```text
app_service_install
app_service_uninstall
app_service_start
app_service_stop
app_service_restart
app_service_status
```

O serviço instala o executável atual com `--mode=headless-api`.

## Serviço Linux/systemd

Em Linux, os comandos usam systemd e criam/removem uma unit em `/etc/systemd/system`.

Requer permissão administrativa.

## Impressão

`openReportPreview(html, title)` abre uma janela independente via `WebviewWindow` usando o HTML real recebido.

