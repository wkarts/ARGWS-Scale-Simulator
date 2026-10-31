# Scale × ADP 1.9.1 — auditoria factual somente leitura

Verificado em 17/10/2026, 20:57–21:00 UTC. Nenhum encoder ou parser de produção foi alterado por esta auditoria.

## Base efetivamente comparada

- GitHub `wkarts/ALL-DRIVER-PLATFORM`: `main` e tag `v1.9.1` resolvidos separadamente para **8c339264ab799006af4eec59833c6cc7530f1cbf**. Release publicada 20:22:38 UTC; PR #51 incorporada às 20:11:39 UTC, merge `b8fe392e7f3ca94032da062aba37eeb2b074c7ab`.
- Árvore remota nesse SHA: `src-tauri/src/commands/scale_serial.rs` blob `e4e3bb39df828fc4e8beefbae1cb0f03a97b4290`; `toledo8540.rs` blob `09cbdc4d4cf5f8db2555f17975117b864fb62682`. `git hash-object` comprovou igualdade exata com os arquivos locais usados nos testes. Não dependemos de um checkout antigo por suposição.
- Original: ZIP `scale-simulator-tauri-v0.1.9-adp-drivers-rs232-fix.zip`.
- Corrigido: `build_frame` da entrega final 0.1.10, revalidado às 21:01 UTC; contém o encoder 8540 corrigido. Overrides opcionais de testes não participam desta comparação de encoders; sua camada desligada/ligada é validada separadamente em `tests/check_serial_config.py`.

Fontes fixadas: [commit](https://github.com/wkarts/ALL-DRIVER-PLATFORM/commit/8c339264ab799006af4eec59833c6cc7530f1cbf), [parser e transporte](https://github.com/wkarts/ALL-DRIVER-PLATFORM/blob/8c339264ab799006af4eec59833c6cc7530f1cbf/src-tauri/src/commands/scale_serial.rs), [8540](https://github.com/wkarts/ALL-DRIVER-PLATFORM/blob/8c339264ab799006af4eec59833c6cc7530f1cbf/src-tauri/src/commands/toledo8540.rs), [release](https://github.com/wkarts/ALL-DRIVER-PLATFORM/releases/tag/v1.9.1).

## Resultado executivo

Não é factual dizer que somente 8540 diverge. O 8540 original é incompatível com o parser atual, e o corrigido resolve os casos válidos ensaiados. Outros modelos têm limitações semânticas comprovadas, embora recebam e produzam números. Receber bytes, aceitar um quadro e interpretar todos os estados corretamente são critérios diferentes.

O ADP atual lista exatamente cinco equipamentos: WT1000N, WT3000IR, TOLEDO8540, A12e, LIDERLD2052. WT27R, Toledo Prix3, Filizola e Generic ASCII existem no Scale, mas não possuem driver correspondente nesse ADP. Isso é ausência de suporte, não prova de defeito desses quatro simuladores.

## Matriz de protocolo

`display` = bruto ou bruto−tara conforme seleção no Scale. Nas linhas não 8540, arredondamento refere-se ao encoder; a normalização prévia do estado pela divisão da balança é uma etapa separada. Larguras Rust são mínimas: valores grandes podem expandir campos.

| Modelo | Quadro Scale | Resultado ADP atual: campos, sinal, precisão, tara, estados, unidade | Integridade / classificação |
|---|---|---|---|
| WT1000N | `status,bruto,tara,líquido CR LF`; status 0 estável/1 instável; números com sinal e até 3 decimais | Preserva bruto/tara/líquido e estabilidade em kg. Troca GS/NT não altera quadro, pois leva os três campos. Unidade configurada não é emitida; ADP assume kg. Encoder não emite OL ao superar capacidade, embora parser reconheça OL em qualquer campo. | CRLF; sem checksum. Compatível nos casos normais kg, com lacunas de overload/unidade/precisão acima de 3 casas. |
| WT3000IR | `ST ou US,GS ou NT,±display_arredondado unidade tara_abs_arredondada unidade CR LF` | ADP ignora GS/NT, toma primeiro número como bruto e calcula líquido subtraindo tara novamente. Sinal do display preservado; sinal da tara descartado pelo encoder. Frações perdidas. Estabilidade ST/US funciona; encoder não emite OL. ADP ignora rótulos de unidade e assume kg. | CRLF; sem checksum. Parcial em GS inteiro/kg; semanticamente incorreto em NT com tara. |
| Toledo 8540 original | STX + estados antigos + bruto abs inteiro[6] + tara abs inteira[6] + CRLF | ADP 1.9.1 rejeitou todos os 12 casos ensaiados: checksum inválido ou layout inválido. Estados antigos também não satisfazem contrato atual; instabilidade antiga usa bit agora identificado como sobrecarga. Não há sinal/decimais/NT/unidade corretamente codificados. | LF ocupava posição de CKS, sem calcular checksum. Incompatível. |
| Toledo 8540 corrigido | STX + SWA/SWB/SWC + peso[6] + tara[6] + CR + CKS (18 bytes) | SWA escala decimal; SWB modo líquido, sinal, overload, movimento e kg/lb. ADP reconstrói bruto/tara/líquido. Ensaios positivos/negativos, decimal, tara GS/NT, instável, zero e lb corretos. Conversão kg→lb feita pelo encoder quando lb selecionado. Sobrecarga rejeitada como leitura indisponível. g e configurações fora da capacidade de codificação não são casos válidos de configuração. | CKS = `(-sum(STX..CR)) & 0x7f`; CR no índice 16, CKS no 17. Compatibilidade demonstrada para variante com STX/checksum habilitados; não certificação de hardware. |
| A12e | `wn±display_arredondado unidade CR LF` | Sinal preservado, frações perdidas. ADP fixa estável=true, bruto=líquido=display e tara=0. Não transporta modo GS/NT nem instabilidade. Aceita kg; com g divide o número por 1000 mas mantém unit="g". lb rejeitado. Parser aplica limite fixo positivo >10000, independente da capacidade Scale; não verifica valor absoluto. | CRLF; sem checksum. Aceita peso simples, não representa todo o estado do simulador. |
| Lider LD2052 | SOH(01) + abs(display) inteiro[7] + espaço + E/I + STX(02) | Encoder perde sinal e frações. ADP interpreta campo como líquido, copia-o também para bruto, tara=0, unit=kg; E/I funciona. Scale em GS com tara envia bruto onde ADP documenta líquido. Não há overload emitido. Campo com 8 dígitos gera 12 bytes e ADP rejeita: espera 11. | Delimitadores 01/02; sem checksum. Parcial para líquido não negativo inteiro/kg até 7 dígitos. |
| WT27R | `=±display_arredondado unidade CR LF` | Nenhum parser WT27R no ADP atual. Sinal/unidade emitidos; tara, estabilidade, overload não emitidos; frações perdidas. | CRLF; sem checksum. Não suportado pelo ADP. |
| Toledo Prix3 | STX(02) + `±display_arredondado unidade` + ETX(03) | Nenhum parser Prix3 no ADP atual. Tara/estabilidade/overload não emitidos; frações perdidas. O splitter CRLF genérico ADP nem entrega esse quadro. Não selecionar TOLEDO8540 como substituto. | STX/ETX; sem checksum. Não suportado pelo ADP. |
| Filizola | `P±display_arredondado unidade CR LF` | Nenhum parser Filizola no ADP atual; tara/estabilidade/overload não emitidos; frações perdidas. | CRLF; sem checksum. Não suportado pelo ADP. |
| Generic ASCII | `±display_arredondado espaço unidade CR LF` | Nenhum parser genérico no ADP atual; tara/estabilidade/overload não emitidos; frações perdidas. | CRLF; sem checksum. Não suportado pelo ADP. |

## Exemplos reproduzidos, sem parser fictício

- WT3000IR: bruto 1234, tara 200, modo NT → quadro `ST,NT,+001034kg 000200kg` → ADP bruto **1034**, tara **200**, líquido **834**. O esperado semanticamente seria 1234/200/1034.
- Lider: −1234 → +1234; 12,345 → 12. Bruto 1234/tara200 em GS → ADP líquido1234, embora líquido de estado seja1034. Em NT positivo → líquido1034 corretamente, mas tara não é recuperável.
- A12e: instável → ADP estável; tara200 não aparece. Configurado g com estado12 kg → envia `wn+000012g` → ADP número0,012 com unit g; unidade e magnitude não reproduzem o estado de origem.
- WT1000N: 80001 kg com capacidade80000 → ADP sobrecarga=false. A entrada inicial acima da capacidade é bloqueada pela validação, mas alteração de peso posterior/comando permite esse estado; o teste de encoder isolado não deve ser confundido com passar pela UI inicial.
- 8540 corrigido: bruto1234/tara200 em GS e NT → ambos retornam1234/200/1034; bruto100/tara200 em NT →100/200/−100. 12,345 kg com divisão0,001 →12,345. 4,5359237 kg e tara0,90718474 kg em lb →10/2/8 lb.

## Transporte e modo de operação

- Todos os nove modelos Scale usam a mesma infraestrutura: servidor TCP contínuo ou por comando; serial contínua ou por comando. `write_all(frame.as_bytes())` preserva bytes de controle. Não existe encoder alternativo de formato só para TCP.
- Entrada de balança ADP é **serial**, em `start_worker`/`read_weight_once`; nenhuma conexão TCP cliente para equipamento nesses drivers. Servidor HTTP/API do ADP não é entrada TCP de protocolo da balança. Scale TCP → ADP exige ponte TCP–serial externa; esta auditoria não a configurou nem a testou.
- Em serial, usar portas distintas de um par virtual ou conexão física adequada; parâmetros baud/data bits/paridade/stop bits devem coincidir. ADP base 9600/8N1 é configurável; não deduzir parâmetros físicos de todos os modelos desse default.
- 8540 corrigido Scale adota 7E1/7E2 conforme configuração, além de overrides experimentais em desenvolvimento. Ajustar ADP para os mesmos parâmetros. Os outros modelos preservam comportamento serial anterior salvo override explicitamente habilitado.
- ADP `start_worker` lê fluxo continuamente e **não envia polling**, mesmo se a configuração diz polling. `read_weight_once` envia `P\r` quando mode=polling. Scale comando reconhece P após trim, portanto o comando nominal corresponde; para recepção contínua, Scale deve estar em contínuo ou haver solicitação explícita.
- Scale trata cada leitura como comando e não tem remontagem completa de comandos segmentados; comandos desconhecidos também devolvem o quadro. Não afirmar compatibilidade integral com todos os comandos dos aparelhos físicos.
- Em outros drivers, ADP pode publicar resultados com erros; supressão de publicação por erro está específica ao Toledo8540. Números zero presentes em diagnóstico com errors não provam pesagem zero.

## Testes realizados e limites

1. **216 registros**: nove modelos ×12 cenários ×2 versões. Cenários base, negativo, decimal, tara/GS, tara/NT, líquido negativo, instável, sobrecarga, zero, lb, g e campo largo.
2. `audit.py` extrai sem reescrever a lógica as funções reais build_frame/signed_value do ZIP e do snapshot, além de parse_number/splitters/parsers/convert_frame do ADP. Módulo Toledo de produção incluído por path. Apenas a infraestrutura de tipos é mínima e o timestamp é substituído por String vazia para dispensar chrono; nenhum algoritmo de parser foi recriado.
3. Testados todos os cortes possíveis de dois quadros concatenados para cada caso em que o splitter entregou quadro; contagem e entrega confirmadas. Isso verifica framing por bytes, não UART, cabo, serial real ou ponte TCP.
4. As saídas dos oito encoders não 8540 são **byte-idênticas** entre ZIP e snapshot em todos os 96 estados ensaiados. Isso preserva também as limitações encontradas; não equivale a certificar que estão corretos.
5. **11 testes unitários reais do parser Toledo ADP passaram**, incluindo checksum, sinal, decimal, unidades, framing, corrupção e recuperação. Não confundir 216 registros de comparação com 216 testes de compatibilidade aprovados: há rejeições e divergências esperadas e documentadas.
6. Os testes isolados chamam o encoder diretamente; casos g/capacidade irrepresentável do 8540 são diagnósticos adversariais, não configurações válidas aprovadas pela validação do app. Estado e divisão podem normalizar pesos antes do encoder no uso normal.
7. Não executado aqui: aplicação Tauri completa, bancada física, firmware/modelos reais, portas COM reais, manuais de todos os modelos ou homologação metrológica. A documentação de 8540 no ADP aponta fontes de protocolo/manuais, mas não foi realizada nesta rodada uma nova validação documental independente; para os demais modelos, fidelidade a manual **não comprovada**. Compatibilidade com o código ADP e fidelidade ao hardware são conclusões distintas.

## Decisão sugerida

Manter correção funcional restrita ao 8540, como autorizado, e registrar as demais diferenças para decisão explícita. Para validar os outros modelos sem mudança, restringir cenários ao contrato realmente representado (por exemplo, WT3000IR GS/kg/inteiro; LD2052 NT/kg/não negativo/inteiro; A12e peso simples). Não anunciar suporte ADP aos quatro modelos ausentes. Se houver autorização futura, separar correções por modelo e confrontar cada uma com documentação/capturas confiáveis, sem fazer um lado imitar silenciosamente um erro do outro.
