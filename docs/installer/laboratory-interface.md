# Interface do laboratório de pesagem

A navegação principal apresenta Simulação, Playground e Visão geral no grupo Laboratório. Configurações reúne ambiente, acesso e diagnóstico; Ajuda reúne guia, API e ficha técnica. Cadastros empresariais e serviços genéricos do template não ocupam o menu operacional. As rotas e dados existentes continuam disponíveis, sem alterações de permissões ou exclusão de recursos.

Cartões de protocolos têm disposição vertical explícita, evitando herdar o layout horizontal dos botões globais. Os indicadores usam estilos próprios de métrica. Formulários, quadros de peso, controles e Playground adaptam-se à largura efetiva do conteúdo com container queries. A seleção e o carregamento de modelos continuam sem iniciar, parar ou reconfigurar instâncias existentes. A navegação entre guias conserva a rolagem da própria guia, evitando herdar a posição de outra tela.

A ficha técnica passa a usar o SHA do código compilado, obtido do checkout, em vez do SHA da branch que hospeda o workflow.

## Verificação

- Typecheck e build Web.
- Testes de navegação e estilos do produto.
- Testes das páginas Vue reais em Chromium, com API sintética isolada, em larguras 320, 390, 768 e 1366.
- Verificação de ausência de overflow horizontal, nove cartões de protocolo, quatro métricas, cartões verticais, carregamento de modelo sem comandos de runtime e início explícito no Playground.
- Capturas sintéticas anexadas ao job de frontend por um dia.

Esses testes não substituem validação de hardware serial/TCP real. Não há alteração dos serviços ou protocolos do simulador.
