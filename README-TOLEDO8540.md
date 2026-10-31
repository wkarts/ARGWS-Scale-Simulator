# Correção do simulador Toledo 8540

Pacote-fonte baseado no ZIP `scale-simulator-tauri-v0.1.9-adp-drivers-rs232-fix.zip`.
A entrega foi identificada como versão 0.1.10, junto da revisão visual do produto. Este pacote não contém instalador ou executável Windows.

## Alteração

Somente o protocolo Toledo 8540 foi corrigido: STX + SWA/SWB/SWC + 6 dígitos de peso + 6 de tara + CR + checksum. São exatamente 18 bytes; o checksum é `(-soma(STX..CR)) & 0x7f`, sem LF adicional. Status contém sinal, modo bruto/líquido, movimento, unidade e sobrecarga. A posição decimal/incremento vem da divisão.

- kg: divisões 1, 2 ou 5 vezes potência de dez, entre 0,00001 e 500 kg, desde que capacidade/resolução caiba em seis dígitos.
- lb: a tela e as ações continuam em kg; o frame converte kg para lb usando 0,45359237 kg/lb. A resolução transmitida é arredondada para potência de dez, limitada por capacidade e até cinco casas decimais. O decoder deve usar a escala SWA, e não assumir inteiros.
- Peso fora da capacidade, tara inválida ou valor que não caiba em seis dígitos gera um frame de sobrecarga. Os campos são zerados, mas o bit de sobrecarga impede interpretar esses zeros como pesagem válida.
- Valores de checksum NUL, STX, LF e CR são possíveis. Consumidores devem ler por posição, nunca dividir exclusivamente por CR/LF.
- O simulador gera frames; o parser testado é o parser real corrigido do ADP, não uma implementação nova dentro deste aplicativo.

Referência: https://devser.com.br/downloads/protocolo-toledo-8540.pdf (8540 analógico/numérico; transmissão contínua com STX e checksum habilitados, C44A=1).

## Uso

1. Selecione o preset Toledo 8540. Ele aplica 4800 baud e 7E1.
2. Se necessário, escolha 7E2 no controle exclusivo do Toledo 8540. Baud continua ajustável. Configure a outra ponta com o mesmo baud, 7 bits, paridade par e quantidade de stop bits.
3. No mesmo Windows, use um par virtual/null modem: simulador COM10, ADP COM11, conectadas entre si. Nunca abra a mesma COM nos dois aplicativos.
4. Inicie o simulador, altere peso, aplique tara, alterne bruto/líquido e estável/instável. A tara e os controles existentes foram preservados.
5. Para testar sem serial, escolha TCP contínuo e valide os bytes de rede com o parser corrigido. Nenhuma balança ou câmera é necessária para os testes automatizados.

As outras oito opções de protocolo mantêm seus frames e defaults seriais anteriores. O campo legado `toledo8540_stop_bits` aceita 1 ou 2; ausente significa 1 somente no Toledo. Os novos campos seriais opcionais permitem overrides explícitos em qualquer modelo, sem mudar defaults quando ausentes. Consulte `docs/OPCOES_SERIAIS_E_FRAMES.md`.

## Validação reproduzível

Com Python 3 e Rust instalado, a partir desta pasta:

```
python tests/check_toledo8540.py --adp-parser /caminho/ADP/src-tauri/src/commands/toledo8540.rs
python tests/check_serial_config.py
npm ci
npm run typecheck
npm run build:web
npm run fmt:rust:check
npm run lint:rust
npm run test:rust
```

O script extrai o `build_frame` real, usa o módulo encoder real e importa o parser ADP indicado. Testa 192 combinações de escala/incremento/sinal/tara/estabilidade/modo, 512 comparações de bytes dos outros oito modelos contra a fixture original, CKS 0/2/10/13 com todos os cortes de recepção, corrupção e sobrecarga. Também executa os testes internos do parser e passa 16 frames gerados em Rust a um decoder Python independente.

## Evidência e limites desta entrega

- 15 testes Rust aprovados (4 de integração e 11 do parser ADP).
- 16 frames reais Rust aprovados pelo decoder Python independente.
- Módulo completo do simulador compilado em harness sem macros Tauri/libudev; 5 testes adicionais aprovados: serde/builder serialport real (config antiga sem campo novo, 7E1/7E2 e defaults dos outros oito modelos) e transporte TCP loopback real com dois frames completos/checksum NUL. Também foram testados overrides de frame/serial e propriedades da oscilação. Nenhuma porta serial foi aberta.
- Clippy do módulo isolado com `-D warnings`: aprovado (não substitui Clippy de toda a aplicação).
- `npm run typecheck` e `npm run build:web`: aprovados com dependências já disponíveis no ambiente. O build web não substitui build desktop.
- Instalação limpa `npm ci`: o lockfile original retornava HTTP 404 em `duplexer2-0.1.9.tgz`; a integração final corrigiu para a versão 0.1.4 e a instalação limpa foi aprovada.
- Versões antes divergentes 0.1.9/0.1.0 foram sincronizadas em 0.1.10 durante a revisão da identidade do produto. Consulte a validação final da entrega.
- `cargo fmt --check`: arquivo original scale_simulator.rs já contém trechos fora da formatação. O novo encoder foi formatado e validado isoladamente; a entrega evita reformatação global.
- `cargo check` online baixou as dependências, mas parou na biblioteca nativa Linux ausente `gobject-2.0` (pkg-config/gobject-sys). Cargo/Tauri completo, instalador Windows e comunicação em portas COM reais/virtuais não estão certificados por este pacote.

Rollback: restaure o ZIP original. Não há mudança de banco, API HTTP, câmera, licenciamento ou configuração de outros drivers.
