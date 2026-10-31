# Pull Request

## Branch
`fix/scale-simulator-toledo8540-identity-serial-tests` — sugerida; nenhuma branch, commit ou PR foi publicada.

## Título
`fix(scale-simulator): corrige Toledo 8540 e refina identidade e testes seriais`

## Descrição
Entrega completa em código-fonte Scale Simulator 0.1.10. Corrige exclusivamente o encoder Toledo 8540; preserva os outros oito encoders. Acrescenta parâmetros seriais opcionais e overrides explícitos de frame para testes de falha, além de identidade visual própria e dashboard baseado em dados reais.

## Contexto
O 8540 original tinha status incorreto, perda de sinal/casas decimais e LF no lugar do checksum. A interface ainda apresentava marca/versão de template e métricas administrativas sem relação com a simulação. A instalação limpa também tinha uma referência inválida de duplexer2 no lockfile.

## Objetivo
Permitir testes verificáveis com o parser 8540 real do ADP 1.9.1, manter compatibilidade e oferecer controles de bancada explícitos sem alterar o protocolo padrão dos outros modelos.

## Escopo incluído e fora do escopo
Incluídos: 8540, parâmetros de UART, overrides de prefixo/terminador/checksum, prévia de bytes, identidade, versão 0.1.10, documentação e testes. Fora: correções de outros encoders ou do ADP, câmeras, mudanças de API/banco/autenticação, instalador certificado e homologação de hardware.

## Decisões técnicas e arquitetura
Encoder 8540 puro em módulo isolado. Overrides em módulo separado aplicado após o encoder, desligado por padrão. Configuração nova opcional e retrocompatível; defaults anteriores 8N1, exceto 8540 corrigido 7E1/legado7E2. Controle de fluxo não foi alterado. Strings preservam bytes ASCII de controle, inclusive checksum NUL.

## Alterações realizadas
- Frame 8540 de 18 bytes com SWA/SWB/SWC, sinal, bruto/líquido, tara, casas decimais, kg/lb, sobrecarga e CKS.
- Serial explícita 5–8 bits, none/even/odd, 1/2 stop bits; COM/baud/oscilação preservados.
- Overrides avançados de até oito bytes ASCII/hex por prefixo/terminador; CKS8540 normal/corrompido/omitido.
- Identidade Scale Simulator, autoria existente, marca de balança e ícones; dashboard sem métricas fictícias.
- Sincronização 0.1.10 e correção pontual duplexer2 para 0.1.4 dentro da faixa já requerida.

## Arquivos adicionados, modificados e removidos
Adicionados: módulos `toledo8540_encoder.rs`, `frame_test_overrides.rs`; testes em `tests/`; relatórios em `docs/`; README específico do 8540 e ativos da marca. Modificados: `scale_simulator.rs`, Page/services do simulador, shell/config/dashboard/guia/Sobre, metadata/versionamento, ativos e cinco strings públicas de fallback/documentação Rust. Lista completa em `docs/ARQUIVOS_ALTERADOS.md`. Nenhum arquivo original removido.

## Banco de dados
Sem migração ou exclusão. Identificadores de aplicação, diretórios e namespaces de armazenamento preservados.

## APIs, contratos e integrações
Comandos existentes preservados; config/status recebem campos opcionais/efetivos de serial e testes de frame. ADP não alterado. Consumidor 8540 deve corresponder ao modo contínuo com STX/checksum habilitados. TCP do simulador não implica entrada TCP no driver serial ADP.

## Dependências, configurações e variáveis de ambiente
Sem dependência nova de produção. Correção pontual do lockfile npm. Nenhuma variável de ambiente removida ou renomeada. Versões sincronizadas em 0.1.10.

## Segurança e privacidade
Sem alteração de credenciais, autorização, licenciamento, permissões, câmera ou compartilhamento. Hex validado e limitado; overrides identificados como teste de quadro potencialmente inválido. ZIP sem caches, node_modules, target ou segredos locais.

## Desempenho e observabilidade
Frames 8540 têm tamanho fixo no modo padrão. Dashboard atualiza a cada cinco segundos enquanto ativo, evita sobreposição e encerra timer ao sair. Status informa parâmetros efetivos e override; prévia textual/hex permite inspecionar controles.

## Testes implementados e executados
- 15 testes Rust de protocolo/ADP (11 do parser e 4 integração), 16 frames reais Rust decodificados por Python.
- 5 testes Rust do módulo completo isolado: defaults seriais, 48 combinações explícitas, overrides/validação, transporte TCP loopback com NUL e propriedades da oscilação.
- 512 comparações byte a byte dos outros oito encoders contra o ZIP original.
- Auditoria adicional: 216 registros reais contra ADP main/tag1.9.1 SHA `8c339264ab799006af4eec59833c6cc7530f1cbf`; confirma preservação em 96 estados dos oito encoders e registra divergências existentes. Não são 216 testes de compatibilidade aprovados.
- Instalação npm limpa, sincronização de versões, TypeScript e build web aprovados. 11 testes de identidade/UI/apresentação HEX aprovados.
- Clippy do módulo isolado com `-D warnings` aprovado. Ver relatório final para comandos e ressalvas.

## Testes recomendados / validação manual
Em Windows, validar app nativo, instalador, login, dados existentes, tema/menu/permissões e dashboard com instâncias reais. Usar COM10↔COM11 com parâmetros iguais, testar peso negativo/decimal/tara/estabilidade no ADP. Repetir com overrides para confirmar rejeição de quadro inválido e desativá-los ao concluir. Comparar HEX com bytes capturados.

## Impactos, compatibilidade, riscos e mitigação
Padrão 8540 deixa de reproduzir o formato antigo inválido e exige parser compatível. Overrides alteram comunicação somente quando habilitados. Configs antigas mantêm defaults. Outros encoders preservam também suas limitações: não se promete suporte integral a todos os estados. A auditoria factual está em `docs/COMPARACAO_ADP_1.9.1.md`.

## Deploy
Descompactar fontes, `npm ci`, `npm run validate:web`, instalar pré-requisitos Tauri, executar testes e `npm run tauri:build` no sistema-alvo. Revisar configurações antes de usar com equipamentos.

## Rollback
Restaurar ZIP anterior; sem migrações de dados. Conservar backup dos dados locais antes de qualquer atualização de aplicativo.

## Build, release e distribuição
Código-fonte completo 0.1.10. Build web verificado. `cargo check` Tauri completo bloqueado pela biblioteca Linux `gobject-2.0` ausente; janela nativa/instalador Windows não certificados. `cargo fmt --check` aponta formatação antiga no módulo original; evitada reformatação global fora de escopo. Nenhum push, tag, release ou PR remoto foi realizado.

## Documentação
README do produto; guia 8540; opções seriais/frames; identidade; relatório de validação; comparação factual ADP e resultados TSV; manifesto de arquivos alterados.

## Checklist
- [x] Encoder8540 corrigido e testado contra parser real
- [x] Outros oito encoders preservados byte a byte
- [x] Overrides explícitos desligados por padrão
- [x] Instalação limpa, typecheck e build web
- [x] Identificadores de dados mantidos
- [ ] App/instalador Windows e UART/COM física ou virtual
- [ ] Homologação de todos os protocolos em hardware

## Commit sugerido
`fix(scale-simulator): corrige 8540 e adiciona controles explícitos de teste serial`

## Merge sugerido
Squash após revisão e validação Windows. Nenhum merge executado.

## Versão sugerida
0.1.10 (patch), já sincronizada no pacote; não publicada remotamente.
