# Compatibilidade do instalador Windows

A versão 0.2.1 usa publisher `WK Arts`, productName `Scale Simulator`, identifier `br.com.wkarts.scale-simulator-tauri` e executável `scale_simulator_tauri.exe`.

A migração para 0.3.0 removeu publisher. Esse campo não é somente apresentação: no NSIS do Tauri, MANUPRODUCTKEY inclui fabricante/produto e guarda o diretório da instalação. A lista de desinstalação é identificada pelo produto. Com fabricante diferente, uma instalação anterior pode ser detectada sem recuperar o diretório correto enviado ao desinstalador. A correção restaura exatamente os metadados anteriores, sem remover dados ou alterar a identidade.

Referência: https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/windows/nsis/installer.nsi (MANUPRODUCTKEY, PageLeaveReinstall e RestorePreviousInstallLocation).

## Verificação

O teste Node fixa os campos da instalação anterior e o modo currentUser. O novo job Windows constrói o NSIS real, baixa o instalador oficial 0.2.1 com SHA-256 fixado, instala em diretório sintético com espaços, consulta a chave usada pelo novo instalador, executa a remoção anterior com o parâmetro de diretório correspondente, instala a versão atual e a remove. Um marcador de dados deve sobreviver a ambas as remoções.

O teste executa somente no runner Windows isolado do GitHub Actions. Ele não simula cliques nas páginas interativas; valida o handoff de diretório e os instaladores/desinstaladores reais em modo silencioso. O resultado remoto deve ser conferido antes de afirmar que o cenário passou. Não altera as preferências de remoção de dados da aplicação.

## Distribuição

A tag v0.3.0 já publicada não pode ser silenciosamente movida ou receber binários diferentes. A publicação da correção depende da escolha explícita entre patch SemVer e substituição coordenada da pré-release existente. Esta PR não altera versão nem exclui a release. Dados locais e modelos salvos não devem ser apagados para contornar a falha.
