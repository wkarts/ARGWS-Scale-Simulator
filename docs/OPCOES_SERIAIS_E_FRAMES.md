# Parâmetros seriais e quadros de teste

## Serial

COM e baud continuam configuráveis. Novos campos opcionais permitem `serial_data_bits` (5, 6, 7, 8), `serial_parity` (`none`, `even`, `odd`) e `serial_stop_bits` (1 ou 2). Campo ausente mantém 8N1 nos modelos anteriores; Toledo 8540 usa 7E1 e respeita o campo legado `toledo8540_stop_bits` quando `serial_stop_bits` está ausente. Não foi adicionado controle de fluxo.

Os parâmetros devem coincidir nas duas pontas. Escolher 5/6 bits ou paridade incorreta pode alterar a comunicação e é útil apenas em testes deliberados; não muda o formato produzido pelo encoder. Os parâmetros efetivos são devolvidos no status. Aplicar preset ou trocar modelo restaura os defaults daquele modelo.

## Override de frame (avançado)

`frame_override_enabled` é falso por padrão. Desligado, nenhum override altera bytes, mesmo que existam valores antigos nos campos. Não há mudança de encoder dos outros modelos.

- `frame_prefix_hex`: `null` preserva prefixo; string vazia remove; string hexadecimal substitui. Exemplos: `02` (STX), `01` (SOH).
- `frame_suffix_hex`: `null` preserva terminador; string vazia remove; hexadecimal substitui. Exemplos: `03` (ETX), `0D` (CR), `0A` (LF), `0D 0A` (CRLF).
- Cada campo aceita até oito bytes, somente 00–7F, para preservar o transporte ASCII/String da aplicação. Rejeita hex incompleto, caracteres inválidos e bytes acima de 7F.
- Em protocolos sem prefixo de controle, o prefixo solicitado é adicionado antes do payload. Em 8540/Prix substitui STX; em Líder substitui SOH.
- O terminador original é CR no 8540, STX no Líder, ETX no Prix e CRLF nos demais.
- `frame_checksum_mode`: `protocol` (padrão), `corrupt` ou `omit`. Corrupção/omissão são aceitas apenas no 8540. `protocol` recalcula CKS sobre o frame modificado; `corrupt` inverte o bit menos significativo; `omit` remove CKS. O checksum fica depois do terminador.

O checkbox e o status identificam o quadro personalizado. Mudar STX, terminador ou checksum pode fazer o consumidor rejeitar a leitura: esse é um teste de falha, não um novo protocolo homologado. Desative as opções avançadas para retornar ao formato normal. Os controles de peso, tara e oscilação continuam independentes.

## Testes executados

`python tests/check_serial_config.py` compila o módulo real sem macros Tauri e testa defaults retrocompatíveis, 48 combinações seriais explícitas, overrides desligados/ligados em nove modelos, validação de hex, checksum corrompido/omitido, TCP loopback com NUL e propriedades da oscilação. Não abre COM física ou virtual.

## Prévia HEX

A prévia e o último frame mostram também os bytes UTF-8 enviados pelo código ao transporte, incluindo 00, 02, 0D e 0A. São bytes gerados, não uma captura do fio/UART; parâmetros inadequados de bits/paridade ainda podem impedir a recepção correta. Uma falha de validação da prévia é mostrada como erro, sem apresentar a mensagem como quadro hexadecimal.
