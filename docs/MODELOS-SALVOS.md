# Modelos salvos
Na área de simulação, configure todos os parâmetros, informe um nome e use **Salvar como novo**. Pode haver vários modelos do mesmo driver. Os modelos são independentes dos presets originais.

Selecione um modelo e use **Carregar**: somente o formulário é preenchido. Use o botão **Iniciar** existente quando desejar criar uma instância. Nenhuma instância já em execução é alterada, parada ou substituída.

Para editar, carregue o modelo, ajuste o formulário e use **Atualizar selecionado**. **Salvar como novo** cria outra configuração. **Remover** pede confirmação e exclui somente o modelo. A revisão impede sobrescrever alterações de outra janela. Uma COM indisponível permanece visível; não é substituída silenciosamente.

Os modelos persistem no SQLite local, em tabela adicional, sem remover ou modificar tabelas existentes. Eles acompanham o backup do banco. Salvar/carregar não abre portas, não cria pares seriais nem instala drivers.

Esta versão mantém o visual e os controles atuais. Playground e identidade V2 pertencem à migração futura, não a esta inclusão.
