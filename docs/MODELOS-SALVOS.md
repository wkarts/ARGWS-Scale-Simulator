# Modelos salvos

A área de simulação usa uma barra compacta: **Modelo**, **Carregar**, **Salvar…** e **⋯ Gerenciar**. Em espaços estreitos, os controles se distribuem em duas linhas sem rolagem horizontal.

## Salvar ou atualizar
Configure o formulário normalmente e clique em **Salvar…**. Na janela, informe o nome e escolha **Salvar como novo** ou **Atualizar selecionado**. Vários modelos podem usar o mesmo driver. Atualizar grava os valores atuais do formulário no modelo selecionado; não altera uma instância iniciada.

## Carregar e editar
Selecione um modelo e clique em **Carregar**. Somente o formulário muda. Edite seus parâmetros e use **Salvar… → Atualizar selecionado** quando desejar persistir a edição. Inicie uma nova instância pelo botão **Iniciar** existente, separadamente.

## Gerenciar
O botão **⋯** abre a lista e as ações menos frequentes: atualizar a lista, carregar/editar e remover. Remover exige confirmação e exclui somente o modelo. O campo de nome e as ações de manutenção não ficam ocupando o painel principal.

## Preservação e dados
Presets de fábrica e instâncias em execução não são alterados ao salvar, carregar ou remover. A COM salva indisponível continua identificada, sem troca silenciosa. Os modelos persistem no SQLite local, em tabela adicional, com revisão para impedir sobrescrita de alteração concorrente. Faça backup do banco antes de atualizar.
