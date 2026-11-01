# Scale Simulator 0.3.0 — migração em pré-release

Base arquitetural: template-app-tauri-desktop v1.2.0, commit 1b75a9110ce5b2cd828f66af05fa8844d2a47c8d; 390 arquivos-fonte conferidos por hash antes da derivação. Identidade: kit V2 aprovado de 31/10/2026, balança azul/branca e sinal verde. A base estável e a PR de modelos 0.2.0 permanecem separadas.

## Capacidades e preservação
- Simulador original TCP/RS232, presets, enquadramentos, controles e logs portados. Não há reconfiguração automática de instâncias ao carregar modelos.
- Modelos salvos independentes e revisados, também incluídos na versão atual pela PR4.
- Playground somente nesta migração: modelo → início explícito → controles por instância; edição na tela completa.
- Guia interno pesquisável e exportável em Markdown, inclusive uso serial, TCP, presets, modelos, Playground, backup e primeiro acesso.
- Tema claro Scale Signal, fonte incorporada, ícones V2 no desktop/PWA/tray/instaladores. Escuro opcional.
- Providers Web/Tauri, autenticação, guias persistentes e recursos de configuração do template. Bridge remoto do simulador exige sessão e permissões.
- Licenciamento permanece disponível no código, desativado por padrão.

## Dados e configuração
Destino canônico scale_simulator_tauri/app.db. Origem histórica confirmada template_app_tauri/app.db. O mecanismo de migração usa cópia, integridade, contagens, backup e marcadores; inclui a tabela de modelos na verificação. Usuários e hashes existentes não são substituídos por seed novo. Configuração antiga conhecida é lida sem regravar sua origem e recebe equivalência de prefixo somente se o novo parâmetro não estiver definido. Serviços anteriormente opt-in não são ligados automaticamente por mudança de defaults.

## Distribuição
main continua estável; develop gera beta SemVer superior à versão estável. O coordenador mantém a matriz já distribuída do Scale/ADP: Windows x64/x86, Linux x64, macOS Intel/ARM, Web e fontes, checksums e retomada por assets verificados. Uma release incompleta permanece draft. Os recursos e workflows adicionais de mobile e CloudPanel do template permanecem disponíveis separadamente; não se anunciam esses builds como assets publicados pelo canal desktop.

## Evidências locais
Build/typecheck Web, 22 testes de release, 2 testes de Playground, check Rust headless, teste de modelos SQLite Rust e 19 cenários executáveis de migração passaram. O check headless conserva warnings de código desktop não utilizado do template; não é apresentado como clippy limpo. CI multiplataforma, instalação Windows real e recepção serial física ainda precisam validar o SHA publicado. Não foi gerada pré-release nem feito merge neste checkpoint.

## Rollback
Antes de testar, faça backup do banco e da configuração. Não exclua o banco original para resolver login. Preserve o instalador estável. Se necessário, encerre a pré-release e volte à versão estável com seu banco compatível; dados criados exclusivamente na nova versão precisam de avaliação antes de qualquer restauração. Não usar cópia de arquivo enquanto a aplicação anterior ainda mantém WAL ativo.

## Limite específico de iOS
iOS 15 ou superior não expõe a enumeração serial IOKit de macOS. O build iOS não incorpora essa dependência; operações RS232 locais falham explicitamente antes de criar uma instância. Para serial, use o backend desktop autenticado. Windows/Linux/macOS mantêm o código serial original. TCP e controles remotos continuam disponíveis.

## Publicação beta: contrato do preflight
O comando `ci:tauri` é mantido como contrato com o workflow Release da branch padrão, mesmo quando ele publica um SHA da develop. O CI executa esse mesmo gate antes do build; um teste lista os comandos npm de prepare para detectar ausência antes da publicação. Correção sem alteração de protocolo, persistência ou identidade. Artefatos anteriores de CI não são apagados. CI verde não implica instaladores desktop já produzidos: estes são gerados pela matriz Release após o preflight e ficam em draft com verificação de hash para retomada.


### Integridade do prepare e retomada da mesma beta
O prepare deriva branch/SHA do evento workflow_run validado, não da branch padrão onde o workflow está instalado. A develop exige sufixo beta e prerelease=true. O evento deve ser push aprovado do próprio repositório e coincidir com o checkout. Não se tenta sobrescrever variáveis reservadas GITHUB_*.
A validação real de Cargo metadata mantém --locked e o grafo completo, descarta apenas stdout não utilizado (que ultrapassa1MiB) e preserva falhas, timeout e diagnóstico. O CI executa a preparação beta real em cópia isolada com Cargo verdadeiro, sem criar tag/commit/release. Um retry reutiliza a tag beta anotada somente quando Source coincide com o SHA exato; assets já enviados continuam verificados por hash, sem clobber. Nenhuma release estável é criada a partir de develop.
