# Ajuste do menu e foco RS232

## Objetivo

Esta etapa ajusta a aplicação para o cenário real do projeto: simular balanças ligadas por porta serial RS232, usando portas Windows `COM1` até `COMXXX`.

## Alterações aplicadas

- menu **Cadastro** agora exibe somente **Empresa**;
- criado novo grupo de menu **Simulador**;
- tela de balança movida para **Simulador > Balança**;
- simulador passa a iniciar por padrão em modo serial `RS232 COM contínuo`;
- adicionados modos:
  - `serial_continuous`;
  - `serial_command`;
  - `tcp_server`;
  - `tcp_server_command`;
- adicionados campos de configuração serial:
  - porta serial (`COM1`, `COM2`, `COM10`, etc.);
  - baud rate;
- adicionada listagem de portas seriais disponíveis via Rust;
- mantido TCP como apoio para homologação sem par virtual serial.

## Observação operacional

Um aplicativo comum não cria fisicamente uma nova porta COM sozinho. Para simular uma balança RS232 para outro sistema na mesma máquina, use:

- porta serial física real; ou
- par virtual de portas COM, como `com0com`, `VSPE` ou equivalente.

Exemplo:

- simulador abre `COM10`;
- sistema testado lê `COM11`;
- `COM10` e `COM11` precisam estar conectadas por um par virtual serial.

## Arquivos principais alterados

- `src/config/navigation.ts`
- `src/layouts/AppLayout.vue`
- `src/pages/ScaleSimulatorPage.vue`
- `src/services/scaleSimulator.ts`
- `src-tauri/src/commands/scale_simulator.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/Cargo.toml`
