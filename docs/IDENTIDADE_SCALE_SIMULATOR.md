# Identidade do Scale Simulator — 0.1.10

## Alterações
- Nome público Scale Simulator no shell, janela, instalador, login, Sobre, HTML e manifesto PWA.
- Desenvolvedor Wallace Kleiton, conforme autoria já presente no Cargo.toml. Publisher WK Arts preservado; nenhum contato ou entidade legal nova foi criado.
- Versões alinhadas em VERSION, package.json/package-lock.json, Cargo.toml, tauri.conf.json, projectConfig.ts e brand.json.
- Marca vetorial própria de balança, com ativos PNG, SVG, ICO e ICNS para interface, splash, bandeja, PWA e empacotamento.
- Visão geral mostra instâncias em execução, TCP, RS232 e clientes TCP consultados do comando existente scale_simulator_list. Atualiza a cada cinco segundos enquanto a página está visível; a consulta respeita a permissão config:view, evita sobreposição e é encerrada ao sair da página.
- Web e erro de consulta mostram estado indisponível, sem apresentar zeros como dados reais. O visor da área de apresentação é uma ilustração decorativa.
- Removidos da visão geral contadores administrativos, gráficos vazios e alegações de licenciamento/serviços ativos sem consulta real.
- Administração agrupada e recolhida; recursos avançados preservados em Sistema. Não foram removidas rotas, tabelas, autenticação, licenciamento ou configurações existentes.
- Guia orientado a simulação, TCP/RS232 e Toledo 8540; Sobre focado no produto. Tema claro continua padrão, respeitando escolha local existente.
- Parâmetros RS232 editáveis: bits de dados, paridade e bits de parada; defaults preservados em 7E1 para Toledo 8540 e 8N1 para os demais modelos. Trocar driver/preset restaura os padrões. Campo legado de stop bits preservado. Oscilação e controles de peso mantidos.
- Enquadramento avançado opt-in, desligado por padrão: prefixo/terminador HEX opcionais e checksum correto/corrompido/omitido para testes do Toledo 8540. Trocar driver/preset restaura os padrões. A interface alerta sobre quadros inválidos e identifica instâncias personalizadas.
- Corrigido o único registro inválido de dependência identificado: duplexer2 0.1.9 inexistente para 0.1.4, dentro da faixa ~0.1.0 já exigida pelo ancestral. Sem atualização geral de dependências.

## Compatibilidade
O identificador br.com.wkarts.scale-simulator-tauri, localDataDir scale_simulator_tauri e storagePrefix scale-simulator permanecem inalterados. Nomes técnicos de crates, variáveis de ambiente e caminhos legados não foram renomeados para evitar perda de acesso a dados existentes. Nenhuma migração ou exclusão de dados. Os nove encoders e os controles de oscilação foram preservados; a correção de protocolo é exclusiva do Toledo 8540.

## Verificação executada
- npm ci --cache /tmp/scale-npm-cache --prefix /tmp/scale-clean-install: 373 pacotes instalados em diretório limpo, com scripts de instalação habilitados.
- npm run validate:web usando as dependências dessa instalação limpa: sincronização de versão, typecheck e build Vite aprovados.
- node --test tests/product-identity.test.mjs: 7 testes de contrato aprovados (identidade/versionamento, namespaces preservados, dashboard real, rotas administrativas, nove drivers/seletor 8540, parâmetros seriais e oscilação, enquadramento opt-in).
- SVG renderizado com Inkscape e marca PNG inspecionada visualmente; ativos raster e pacotes de ícones regenerados.

## Limites e validação manual recomendada
Os testes de contrato não substituem interação real. A interface completa não foi testada visualmente neste ambiente: Chromium não iniciou por restrição de sockets do sandbox; a tentativa escalada falhou na infraestrutura. Não há alegação de teste de janela nativa, instalador, hardware ou comunicação RS232.

No Windows, validar login, dashboard vazio e com instâncias reais, atualização/parada, troca entre telas, menu recolhido, tema claro/escuro, permissões, acesso aos recursos avançados e conservação dos dados anteriores. Comparar os contadores do dashboard com as instâncias do simulador. Confirmar marca no instalador, janela e bandeja. Ver relatório específico do Toledo 8540 para testes de protocolo e hardware pendentes.

## Inspeção HEX de quadros
A prévia e o último quadro preservam a representação textual e exibem também os bytes UTF-8 em HEX, equivalentes a String::as_bytes() no transmissor Rust. NUL, STX, CR, LF e checksum deixam de ficar invisíveis. Quadro vazio aparece como “—”. Erros da consulta não são convertidos em HEX; respostas atrasadas não sobrescrevem a prévia mais recente.

Verificação adicional: node --test tests/product-identity.test.mjs tests/frame-presentation.test.mjs passou 11/11, incluindo controles ASCII, vazio, UTF-8 e guarda de validade da prévia.
