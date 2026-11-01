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

## Tema padrão V2

O pacote de identidade aprovado fornece os tokens versionados em `src/assets/branding/theme.json`: azul profundo #102E49, azul-petróleo #176B87, verde-sinal #13B89D, fundo #F3F6FA, superfície #FFFFFF e texto secundário #65758A. A fonte local DejaVu Sans é carregada como ScaleSans. O tema Scale Signal é claro por padrão; o verde escuro #087F6D é reservado a texto com contraste, não substitui o verde-sinal da marca. Temas alternativos continuam opcionais. O teste Chromium confere os tokens computados, além da geometria dos controles, para detectar elementos recortados mesmo quando o contêiner oculta overflow.
