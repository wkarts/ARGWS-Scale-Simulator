# Etapa RS232 — Detecção de portas COM existentes

## Objetivo

Ajustar o simulador para identificar as portas seriais RS232/COM existentes no computador antes de iniciar uma balança simulada.

## Referência analisada

Foi usado o projeto `ALL-DRIVER-PLATFORM-1.2.2` apenas como referência de arquitetura para leitura de portas seriais. O simulador não foi acoplado ao ADP.

## Implementado

- command Rust `scale_simulator_serial_ports` agora retorna metadados completos das portas;
- identificação via `serialport::available_ports()`;
- exibição de COM, tipo, descrição, fabricante, VID/PID e status de uso pelo simulador;
- tela com botão `Atualizar COM`;
- seleção por `select` quando portas reais são detectadas;
- validação antes de iniciar simulador serial;
- mensagem orientando criação de par virtual COM10/COM11 quando nenhuma porta existir;
- ordenação natural de portas COM, evitando COM10 aparecer antes de COM2.

## Observação operacional

Para simular uma balança RS232 para outro sistema na mesma máquina, normalmente é necessário instalar/criar um par virtual serial:

```text
Simulador abre COM10
Sistema leitor abre COM11
COM10 <-> COM11 ligados por null modem virtual
```

Ferramentas comuns:

- com0com;
- VSPE;
- Null-modem emulator.

## Arquivos alterados

- `src-tauri/src/commands/scale_simulator.rs`;
- `src/services/scaleSimulator.ts`;
- `src/pages/ScaleSimulatorPage.vue`;
- `src/styles.css`.
