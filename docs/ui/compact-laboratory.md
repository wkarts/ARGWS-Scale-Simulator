# Laboratório compacto e navegação principal

Visão geral pertence à seção Principal. Laboratório contém apenas Simulação e
Playground. O item ativo usa correspondência exata para não manter a página
inicial destacada durante a navegação em uma página filha.

A densidade visual é ajustada para janelas curtas de desktop, incluindo a área
CSS equivalente à escala de 125% no Windows. Cabeçalhos, métricas, espaços entre
campos e indicadores de peso têm limites próprios, sem crescer com a largura
inteira da janela. Uma única instância não ocupa toda a linha no Playground.
O catálogo de protocolos continua disponível por expansão, inicialmente fechado.

A paleta e a família tipográfica V2 são preservadas. Nenhuma mudança nos drivers,
modelos salvos, protocolos, portas ou instâncias em execução.

## Validação

A suíte laboratory-shell.browser.mjs renderiza AppLayout, Simulação, Playground
e Guia reais com serviços sintéticos. Confere menu ativo, organização, limites
de tamanho, quebra de layout, preservação das instâncias ao carregar modelo e
captura screenshots de viewport em 1366×698, 1093×558, 1024×640, 800×600, 390×844
e 320×568. A segunda resolução representa a área útil aproximada de um desktop
1366×720 com escala de 125%; não substitui teste físico do WebView2/Windows.

A suíte anterior das páginas isoladas continua sendo executada. Evidência de
build e testes de código não substitui a revisão das capturas da nova suíte.
