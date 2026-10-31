# Migração do Simulador WT .NET para Rust/Tauri

## Origem analisada

O projeto original `Simulador_WT` é uma aplicação WinForms/.NET de simulador de indicador de pesagem. A base possui:

- `frmSimuladorWT.cs`: tela principal com peso, tara, zero, comunicação TCP/serial, transmissão e monitoramento.
- `WTSocket.cs`: wrapper de socket assíncrono para escuta TCP, envio e recebimento de dados.
- `mdlWt21.cs`: estado global do indicador, incluindo peso real, divisão, tara, capacidade, zero manual, modo de comunicação e parâmetros seriais.
- `Simulate.cs`: utilitários compatíveis com comportamento VB/VB.NET, como `MidStatement`, `IsNumeric`, `IsDate` e reversão de string.

## Conversão aplicada nesta etapa

Foi criada uma nova aplicação baseada no `template-app-tauri-desktop-web-1.1.3`, preservando a arquitetura do template:

- frontend Vue/TypeScript;
- backend Rust/Tauri;
- commands Tauri para toda lógica crítica;
- layout, autenticação, sessão, logs e menus do template;
- script `.bat` de build para Windows.

## Funcionalidades implementadas

### Backend Rust

Arquivo principal:

- `src-tauri/src/commands/scale_simulator.rs`

Commands adicionados:

- `scale_simulator_presets`
- `scale_simulator_preview_frame`
- `scale_simulator_start`
- `scale_simulator_stop`
- `scale_simulator_stop_all`
- `scale_simulator_list`
- `scale_simulator_logs`
- `scale_simulator_set_weight`
- `scale_simulator_action`

Recursos:

- múltiplas instâncias simultâneas de balança;
- uma porta TCP por simulador;
- modo TCP contínuo;
- modo TCP sob comando;
- controle de peso bruto, tara, líquido, zero e estabilidade;
- logs por instância;
- frames por driver/preset;
- comandos remotos básicos: `P`, `PESO`, `READ`, `ENQ`, `ZERO`, `TARE`, `GROSS`, `NET`, `WEIGHT 1234`.

### Drivers/presets iniciais

- WT1000N TCP contínuo;
- WT27R TCP sob comando;
- Toledo/Prix 3 ASCII;
- Filizola ASCII simples;
- Genérico ASCII.

### Frontend Vue

Arquivos principais:

- `src/pages/ScaleSimulatorPage.vue`
- `src/services/scaleSimulator.ts`
- `src/config/navigation.ts`
- `src/router/index.ts`
- `src/styles.css`

Tela criada:

- menu `Ferramentas > Simulador de Balanças`;
- seleção de preset;
- configuração de host, porta, capacidade, divisão, peso inicial, oscilação e intervalo;
- preview do frame;
- lista de simuladores ativos;
- painel de peso estilo indicador;
- aplicar peso manual;
- zerar, tarar, limpar tara, bruto/líquido, estabilizar/oscilação;
- logs da instância selecionada.

## Scripts Windows

- `build-windows.bat`: valida Node/npm/Rust, instala dependências, roda fmt/clippy e gera build Tauri.
- `dev-windows.bat`: inicia o ambiente de desenvolvimento Tauri no Windows.

## Próximas etapas recomendadas

1. Adicionar driver serial real com crate `serialport`, mantendo opção TCP atual.
2. Adicionar persistência SQLite das configurações de simuladores.
3. Implementar mais protocolos específicos conforme documentação real dos indicadores.
4. Criar importação/exportação JSON dos presets.
5. Criar API interna para consultar/alterar simuladores por HTTP.
6. Integrar com a plataforma ADP/All Driver Platform como ambiente de homologação.
