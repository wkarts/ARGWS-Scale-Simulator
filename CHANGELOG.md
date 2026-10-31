# Scale Simulator

## 0.1.11 — 2026-10-17 (pacote-fonte, não publicado)

- Corrigido mismatch: Tauri Rust e API JavaScript fixados em 2.12.4.
- CLI 2.11.1 e tauri-build 2.7.3 fixados nas versões próprias; Cargo.lock incluído.
- Builds e desenvolvimento usam Cargo --locked; instalação Windows usa npm ci.
- Adicionados guarda de versões e testes do encaminhamento de argumentos.
- Protocolos, drivers, telas e identidade de aplicação preservados; somente metadados de versão atualizados.

## 0.1.10 — 2026-10-17 (pacote-fonte, não publicado)

- Corrigido protocolo Toledo 8540 contínuo de 18 bytes, incluindo status, sinal, escala, tara, unidade e checksum.
- Preservados os outros oito encoders; diferenças preexistentes frente ao ADP 1.9.1 registradas em auditoria factual.
- Acrescentados parâmetros seriais opcionais e overrides explícitos de prefixo/terminador/checksum para testes de falha.
- Identidade própria, versão sincronizada, ícones de balança e dashboard com instâncias reais.
- Corrigida referência inválida de duplexer2 no lockfile; adicionados testes e documentação.

# Histórico herdado da base técnica

As entradas abaixo pertencem ao projeto-base anterior. Não representam versões publicadas do Scale Simulator.

## [1.1.3](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.1.2...v1.1.3) (2026-05-14)


### Bug Fixes

* **ci:** garantir cargo válido no build Tauri ([4160ef6](https://github.com/wkarts/template-app-tauri-desktop-web/commit/4160ef6f40c2f3ae7e575f66b3253e39fa71ca1e))

## [1.1.2](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.1.1...v1.1.2) (2026-05-14)


### Bug Fixes

* concluir ajustes da API interna Scalar ([7bbad5f](https://github.com/wkarts/template-app-tauri-desktop-web/commit/7bbad5fb7cf8452293af1f3863fa1d4e17781d77))

## [1.1.1](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.1.0...v1.1.1) (2026-05-14)


### Bug Fixes

* configura API interna, autostart e página dedicada do Scalar ([571b74b](https://github.com/wkarts/template-app-tauri-desktop-web/commit/571b74be33c4fbd7bfd26ca8f0e7ff4457e0c8bc))

# [1.1.0](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.0.2...v1.1.0) (2026-05-14)


### Bug Fixes

* ajusta portas runtime para faixa dedicada ([b66405f](https://github.com/wkarts/template-app-tauri-desktop-web/commit/b66405f702d7f37d7cbcbbe8eaae9921247eeb36))
* resolve clippy port item arguments ([3b30e9d](https://github.com/wkarts/template-app-tauri-desktop-web/commit/3b30e9d3691632d9d10d5f0b285e665e2c518a1c))


### Features

* padroniza configuração runtime Tauri ([8846896](https://github.com/wkarts/template-app-tauri-desktop-web/commit/884689603790737f71cfd3ee5b2abcb7fb004cdf))

## [1.0.2](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.0.1...v1.0.2) (2026-05-14)


### Bug Fixes

* **template:** stabilize desktop-web runtime, branding, tray and user page ([fa0b9b9](https://github.com/wkarts/template-app-tauri-desktop-web/commit/fa0b9b9692c3efe5be7efd16d156876fd2770abe))

## [1.0.1](https://github.com/wkarts/template-app-tauri-desktop-web/compare/v1.0.0...v1.0.1) (2026-05-14)


### Bug Fixes

* **template:** preserve storage compatibility in desktop-web runtime ([b4edb48](https://github.com/wkarts/template-app-tauri-desktop-web/commit/b4edb48b42666cd4109f90e026d8243057a26847))
* **template:** preserve storage compatibility in desktop-web runtime ([ddf63e1](https://github.com/wkarts/template-app-tauri-desktop-web/commit/ddf63e1f6ad57341f08c98716d7401327edaa41b))

# 1.0.0 (2026-05-14)


### Bug Fixes

* **template:** preserve local storage compatibility in desktop-web runtime ([e249c70](https://github.com/wkarts/template-app-tauri-desktop-web/commit/e249c7045f14e5e17c8844b003af94362a582f75))


### Features

* **template:** add unified Web/PWA/Tauri desktop-web architecture ([5790e14](https://github.com/wkarts/template-app-tauri-desktop-web/commit/5790e141e61e31f2dd3e65ac1a8e98d7e4ef961f))

# 1.0.0 (2026-05-13)


### Bug Fixes

* **template:** completa validação Rust e base DatabaseProvider ([c634e2d](https://github.com/wkarts/template-app-tauri-desktop/commit/c634e2d5ed4875de8ff38a1d5b6e0cd083b85d08))
* **template:** corrige build Rust e lockfile do npm ([5ad80fc](https://github.com/wkarts/template-app-tauri-desktop/commit/5ad80fc8aa3f5a8c361f5510068dec296867a58e))
* **template:** corrige clippy e acelera validação de PR ([f03977d](https://github.com/wkarts/template-app-tauri-desktop/commit/f03977dc78028f9d9fb0b85ddfcd007cc5c731a1))
* **template:** corrige falhas de clippy no core Rust ([a801d23](https://github.com/wkarts/template-app-tauri-desktop/commit/a801d23b26e66ee2b10e49b91386b0b28b185fd3))
* **template:** corrige repositoryUrl do semantic-release ([2cde230](https://github.com/wkarts/template-app-tauri-desktop/commit/2cde2301a59a2347655477fd60d0af5b4bbdaed3))
* **template:** corrige rustfmt e otimiza validação CI ([88630c2](https://github.com/wkarts/template-app-tauri-desktop/commit/88630c297d6100ed67d52fe8dd774a495d320bb8))
* **template:** estabiliza rustfmt e otimiza CI Rust ([237b7e4](https://github.com/wkarts/template-app-tauri-desktop/commit/237b7e45b53b5b3ccfbb81c5190ec5e6406756c6))


### Features

* **template:** inicia CRUD provider-based multi-banco ([e2d119c](https://github.com/wkarts/template-app-tauri-desktop/commit/e2d119c58054beefb378e96a3312e5ae46942a28))

## 0.1.19 - Patch release repository autodetect

- Corrige `semantic-release` para não usar repositório hardcoded.
- Usa `REPOSITORY_URL` quando informado ou deriva automaticamente de `GITHUB_SERVER_URL` + `GITHUB_REPOSITORY`.
- Evita falha `Repository not found` ao reutilizar o template em outro repositório GitHub.


## 0.1.19 - CI rápido para PR e correção Clippy

- Corrige falha de `cargo fmt --check` por linha em branco final em `types.rs`.
- Remove função SQLite não utilizada que quebrava `clippy -D warnings`.
- Ajusta CI para PR rápida sem instalar dependências gráficas do Tauri desnecessariamente.
- Mantém validação Rust completa em push/main/release e `workflow_dispatch`.
- Adiciona cache de pacotes apt do Tauri na validação completa.

## 0.1.19 - Patch Clippy provider/template

- Corrige `clippy -D warnings` com `--all-features`.
- Remove atributo `dead_code` duplicado no módulo core.
- Remove `cfg` interno inexistente `sqlite-provider-value`.
- Remove imports não utilizados.
- Corrige seed do usuário `admin/admin123` tratando `hash_password` como `Result`.

## 0.1.19 - Patch CI Rust rápido e rustfmt

- Corrige `rustfmt.toml` para evitar compactação extrema e falhas recorrentes de `cargo fmt --check`.
- Separa CI Rust em `rust-format` e `rust-check`, reduzindo tempo de falha em PR.
- Remove necessidade de Node/npm no job de `cargo fmt`.
- Adiciona cache npm nos workflows e usa `npm ci --prefer-offline`.
- Mantém validações reais de rustfmt, clippy, test e builds com features.
- Adiciona `docs/CI_PERFORMANCE.md`.

## 0.1.14 - Patch build CI

- Corrige falha de `cargo fmt --check` causada por arquivos Rust não formatados conforme configuração do template.
- Ajusta `rustfmt.toml` para uma política consistente do template sem remover `cargo fmt --check`.
- Remove URLs internas do `package-lock.json`, evitando download quebrado em GitHub Actions.
- Mantém Node.js 20 nos jobs de validação e preserva setup explícito no workflow Rust.
- Mantém CRUD provider-based multi-banco iniciado na 0.1.13 sem remover o CRUD SQLite antigo.

## 0.1.13 - Etapa 2.5

- Corrige workflows para instalar Node.js 20 explicitamente.
- Corrige formatação Rust apontada pelo CI.
- Evolui o DatabaseProvider para iniciar CRUD multi-banco real.
- Mantém SQLite como provider funcional padrão.
- Implementa fluxo provider-based para list/get/create/update/delete sem remover CRUD antigo.
- Implementa execução/query MySQL por feature `mysql-db`.
- Implementa execução/query PostgreSQL por feature `postgres-db`.
- Mantém Firebird fora do escopo funcional por compatibilidade.

## 0.1.12 - Etapa 2.4.1

- adiciona scripts locais de correção/validação Rust (`fix-rust-format`, `validate:*`)
- adiciona `.editorconfig`, `rustfmt.toml` e hook de pre-commit
- completa a base `DatabaseProvider` com `DbValue`, `DbRow`, `execute`, `query`, `query_one` e transações iniciais
- mantém `SQLiteProvider` funcional para health/query/query_one/execute
- adiciona comandos `provider_entity_*` sem remover os CRUDs antigos em SQLite
- mantém MySQL/PostgreSQL preparados por feature com mensagens claras para CRUD provider completo
- mantém Firebird fora do escopo funcional

## 0.1.11 - Etapa 2.4

- Adicionados scripts locais `fmt:rust`, `fmt:rust:check`, `lint:rust` e `test:rust`.
- Reforçado CI para validar `cargo fmt --check`, Clippy com `-D warnings` e testes Rust.
- Criada documentação de Rust formatting/CI para humanos e IA.
- Criada camada inicial `DatabaseProvider`.
- Implementado `SQLiteProvider` funcional.
- Preparados `MySqlProvider` e `PostgresProvider` por feature.
- Iniciado CRUD provider-based com `entity_provider_list` sem remover CRUD antigo.
- Firebird mantido fora do escopo funcional.

# Changelog

## 0.1.10 - Etapa 2.3

- Mantém Firebird fora do escopo funcional por compatibilidade.
- Remove Firebird como opção funcional no frontend e ajusta a tela de banco.
- Completa migrations MySQL/MariaDB e PostgreSQL para o schema central equivalente ao SQLite.
- Protege `/docs` e `/openapi.json` por token quando `requireToken=true`, salvo configuração explícita `TEMPLATE_APP_API_DOCS_PUBLIC=true`.
- Documenta claramente que SQLite segue como banco principal do desktop e que MySQL/PostgreSQL exigem camada de repositórios para virar banco principal dos CRUDs.


## 0.1.8 - Etapa 2.1

- Corrige typecheck em Dashboard, System e EntityPage.
- Implementa segurança efetiva na API interna com token, header configurável, CORS opcional e bloqueio de bind público por padrão.
- Faz `/docs` e `/openapi.json` respeitarem `exposeDocs`.
- Ajusta tray para respeitar configuração de runtime e fechamento opcional para tray.
- Renderiza HTML real no preview de impressão em janela independente.
- Etapa 2.1 alinhou mensagens; a Etapa 2.2 adiciona MySQL/PostgreSQL reais por feature e ignora Firebird.
- Implementa comandos de serviço Windows via `sc.exe` e Linux via `systemd`.


## 0.1.3

- Remove referências e artefatos do projeto de origem.
- Mantém somente a base genérica Tauri para novos projetos.
- Preserva autenticação, usuários, perfis, permissões, empresas, licenciamento, logs, configurações e módulos opcionais de cadastro.

## 0.1.1

- Limpa menu principal e remove cadastros opcionais da navegação padrão.

## 0.1.0

- Cria template corporativo Tauri com Vue, Rust e SQLite.

## 0.1.4 - Splash screen inicial e janela maximizada

- adiciona splash screen inicial exclusivo antes da tela de login
- adiciona barra de progresso de carregamento com mensagens de inicialização
- restaura sessão somente durante o bootstrap inicial
- exibe login/aplicação apenas após concluir carregamento
- configura janela principal para abrir maximizada
- reforça maximização e foco no setup do backend Tauri



## Etapa 2 funcional - 0.1.8

Implementações adicionadas nesta versão:

- API interna real em Rust com Axum: `/health`, `/version`, `/status`, `/app/meta`, `/features`, `/logs`, `/openapi.json` e `/docs`.
- Documentação Scalar em tema claro fixo.
- Modo runtime por argumentos: `--mode=desktop`, `--mode=headless-api`, `--mode=cli`, `--mode=worker`, `--host`, `--port`, `--database-driver`, `--data-dir`.
- Estrutura de CLI/headless sem abrir WebView.
- Camada de banco com SQLite funcional; MySQL/PostgreSQL reais por feature Rust; Firebird ignorado por compatibilidade.
- Migrations centrais ampliadas: feature flags, integrações, logs de integração e tokens de API.
- Integrações externas funcionais: cadastro, token protegido, teste de conexão e logs.
- Tray Tauri 2 com restaurar, status e sair definitivamente.
- Comandos de serviço Windows executam `sc.exe` para instalar/controlar o executável atual em modo `--mode=headless-api`.
- Preview de impressão em janela própria.
- Dashboard com dados reais de logs, banco, API e integrações.

## 0.1.10 - Etapa 2.2

- Implementa suporte real feature-gated para MySQL/MariaDB (`mysql-db`) e PostgreSQL (`postgres-db`).
- Mantém SQLite como banco padrão do desktop.
- Ignora Firebird nesta etapa por compatibilidade.
- Adiciona migrations iniciais para MySQL/PostgreSQL nas tabelas centrais.
- Adiciona controle desktop da API interna: start, stop, restart e status.
- Implementa instalação/controle real de serviço Windows via `sc.exe`.
- Implementa instalação/controle real de serviço Linux via `systemd`.
- Mantém API interna com segurança por token, host, CORS e documentação controlável.
- Melhora preview de impressão em janela independente com HTML real.
