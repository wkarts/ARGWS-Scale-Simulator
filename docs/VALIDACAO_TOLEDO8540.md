# Validação Toledo 8540 — 17/10/2026

## Código exercitado

- Gerador real: `src-tauri/src/commands/toledo8540_encoder.rs`, chamado por `build_frame` real extraído do módulo de produção.
- Parser real ADP corrigido: `src-tauri/src/commands/toledo8540.rs` (arquivo externo fornecido ao teste).
- SHA-256 do parser: `c3aadd3f604f9c144db36c09d47166ee0d9f4e64f4cb4e0760075db9b2d79bb9`.
- Fixture dos demais modelos: função original `build_frame` do ZIP v0.1.9, preservada em `tests/fixtures/legacy_frames.rs`.
- Compilador: rustc 1.99.1 (58cf8f9c3 2026-10-13), distribuição oficial verificada por SHA-256 do manifesto oficial.

## Execuções aprovadas

1. `python tests/check_toledo8540.py --adp-parser CAMINHO_DO_PARSER`:
   - 15 testes Rust, 0 falhas (11 internos do parser ADP e 4 integração).
   - 192 combinações de escala, incremento, sinal, modo bruto/líquido, tara e estabilidade.
   - 512 comparações byte a byte: oito modelos não Toledo, quatro pesos, duas taras, dois estados, dois modos e duas unidades.
   - Checksum NUL/STX/LF/CR, cortes em cada posição de dois frames concatenados, corrupção, zero real, sobrecarga e entradas inválidas.
   - 16 frames produzidos pelo Rust e decodificados independentemente por Python; checados comprimento, checksum, escala, sinal, tara, modo, unidade e estabilidade.
2. `python tests/check_serial_config.py`:
   - 5 testes Rust, 0 falhas.
   - Compila o módulo completo do simulador, retirando somente atributos de macro `tauri::command`; libudev desativado somente no harness para não depender da descoberta física Linux.
   - Deserializa configuração antiga sem o campo novo; compara builder serial real 7E1/7E2 e defaults das outras oito opções.
   - Servidor TCP real na interface de loopback recebe conexão e transmite dois frames de 18 bytes contendo checksum NUL. Nenhuma COM foi aberta.
3. Clippy do harness de módulo completo: `cargo clippy --all-targets -- -D warnings`, aprovado após integração final de controles seriais/frames e testes.
4. Instalação npm limpa e `npm run validate:web` aprovados após integração de identidade/versionamento. 11 testes de identidade/UI/apresentação HEX aprovados após a integração final.
5. Auditoria final com 216 registros contra fontes reais ADP main/tag1.9.1 SHA `8c339264ab799006af4eec59833c6cc7530f1cbf`; diferenças existentes e cenários rejeitados documentados em `COMPARACAO_ADP_1.9.1.md`. Outros oito encoders permaneceram byte-idênticos.
6. Defaults de override desligados preservam bytes; habilitados testam prefixos/terminadores, CKS omitido/corrompido e rejeição de hex inválido. Foram exercitadas 48 combinações seriais explícitas e propriedades de quantização/capacidade da oscilação existente.

## Limitações observadas

- `cargo check` da aplicação completa alcançou build de bibliotecas nativas e parou: `gobject-2.0` ausente no Linux. Não foi certificado build Tauri completo nem instalador Windows.
- Não executados: COM física/virtual no Windows, equipamento Toledo real, instalador/assinatura, teste visual de aplicativo desktop.
- Problemas do ZIP-base, registrados antes da integração de identidade: `npm ci` retornou HTTP 404 em `duplexer2-0.1.9.tgz`; `validate:web` detectou versões 0.1.9/0.1.0 divergentes; `cargo fmt --check` encontrou formatação antiga em scale_simulator.rs. A validação final da entrega deve considerar os ajustes posteriores feitos fora do patch de protocolo.

## Roteiro Windows recomendado

Crie par COM10↔COM11. Escolha Toledo 8540, 4800 baud, 7E1 (ou 7E2 nas duas pontas), intervalo 500 ms, divisão 1 kg, capacidade 80000 kg. Abra simulador em COM10 e ADP corrigido em COM11. Confira 1234 kg; tara 200 kg → líquido 1034 kg; peso bruto -100 kg → líquido -300 kg; alterne estabilidade. Repita com capacidade 100 kg/divisão 0,001 kg e unidade lb, considerando conversão e arredondamento conforme SWA. Não use a mesma COM nos dois processos.
