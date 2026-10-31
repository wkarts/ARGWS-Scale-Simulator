# Scale Simulator 0.1.11 — dependências Tauri

## Correção
O pacote anterior fixava indiretamente a API JavaScript em 2.11.0, mas aceitava qualquer Tauri Rust 2.x e não distribuía Cargo.lock. Uma instalação nova resolveu Rust 2.12.4 e encontrou o mismatch.

- tauri Rust e @tauri-apps/api: 2.12.4 exatos.
- tauri-build: 2.7.3 exato; @tauri-apps/cli: 2.11.1 exato. Esses componentes possuem numeração própria.
- package-lock.json atualizado; correção anterior de duplexer2 0.1.4 preservada.
- Cargo.lock incluído, com checksums das dependências transitivas. unescaper foi resolvido em 0.1.10, disponível no índice, pois o lock preliminar local referenciava 0.1.11 inexistente no índice consultado.
- Wrapper repassa --locked depois do separador -- para Cargo em build/dev, preservando argumentos da aplicação. Scripts Cargo também usam --locked.
- Windows: npm ci obrigatório; dev reinstala a árvore pelo lock para não reutilizar dependências antigas.
- Metadados e cache PWA sincronizados em 0.1.11; identificador e armazenamento mantidos.

## Usar no Windows
1. Extraia este ZIP em uma pasta nova. Não sobreponha uma instalação antiga com node_modules.
2. Tenha Node.js, Rust MSVC, Visual Studio Build Tools (C++) e WebView2 instalados.
3. Execute build-windows.bat para compilar ou dev-windows.bat para desenvolver.
4. Os artefatos de build ficam em src-tauri/target/release/bundle.

Também é possível executar npm ci, npm run ci:tauri e npm run tauri:build. Não exclua os lockfiles nem utilize --ignore-version-mismatches. Para atualização deliberada de dependências, atualize ambos os manifestos e locks e execute as validações.

## Validação executada neste ambiente Linux
- npm ci: aprovado (373 pacotes).
- npm run validate:web: aprovado (versões, guarda Tauri, typecheck e build Vite).
- node --test tests/*.test.mjs: 13/13 aprovados, incluindo os dois novos testes de dependências/wrapper.
- npm run tauri -- info: confirmou Rust/API 2.12.4, tauri-build 2.7.3 e CLI 2.11.1.
- cargo metadata --locked --offline --no-deps: aprovado.
- Testes Toledo 8540: 15/15 testes Rust e 16 quadros no decoder Python aprovados, usando o parser ADP apenas como referência de leitura.
- Testes isolados de configuração serial: 5/5 aprovados, sem abertura de porta física.
- cargo check --locked: resolução aceita; compilação bloqueada pela falta de glib-2.0/gobject-2.0 no Linux.
- cargo fmt --check: detectou diferenças de formatação preexistentes em scale_simulator.rs; não aplicadas para preservar os fontes de protocolos byte a byte. O build-windows.bat mantém sua etapa anterior de formatação.
- Build Windows, instaladores e hardware físico: não executados. Clippy completo e testes do aplicativo inteiro não validados por causa das bibliotecas nativas ausentes.

## Preservação e limites
Todos os 280 arquivos do pacote 0.1.10 permanecem presentes. Todos os arquivos em src-tauri/src, componentes e páginas Vue, serviços e testes anteriores permanecem byte a byte idênticos. A alteração não modifica drivers, quadros, parsing, protocolos nem lógica de UI. O ADP não foi alterado. FILE_MANIFEST.sha256 permite verificar cada arquivo; o SHA256 externo verifica o ZIP. Os locks tornam reproduzível a resolução de dependências, não garantem binários idênticos entre diferentes toolchains ou sistemas.

# Pull Request
- Branch sugerida: fix/scale-tauri-version-alignment
- Título: fix(build): alinha dependências Tauri do Scale Simulator
- Descrição: corrige divergência Rust/JS e distribui lockfiles com execução --locked.
- Contexto/objetivo: recuperar compilação que parava por mismatch 2.12.4 versus 2.11.0.
- Escopo: manifestos, lockfiles, scripts, testes de build, metadados e documentação; ADP, drivers, protocolos e UI fora do escopo.
- Decisão técnica: fixar cada pacote em versão própria, sem ignorar a verificação de compatibilidade.
- Alterações/arquivos: consulte ARQUIVOS_DEPENDENCIAS_0.1.11.md.
- Banco de dados: não se aplica a esta alteração.
- APIs, contratos e integrações: nenhum contrato de negócio alterado.
- Dependências/configurações: versões e locks acima; nenhuma nova variável de ambiente.
- Segurança/privacidade: sem segredos e sem nova permissão; dependências com checksums de registro. Não foi realizada auditoria abrangente de vulnerabilidades.
- Desempenho/observabilidade: não se aplica a esta alteração.
- Testes implementados: guarda dos locks e encaminhamento --locked; executados e limitações acima.
- Testes recomendados/validação manual: executar build-windows.bat no Windows, iniciar aplicativo e confirmar versões e operação de simuladores existentes.
- Impactos/compatibilidade: mudança de build sem breaking change intencional; validação nativa Windows pendente.
- Riscos/mitigações: dependências do sistema continuam obrigatórias; uso dos locks evita nova resolução inesperada.
- Deploy/build/distribuição: extrair ZIP completo e compilar localmente. Nenhum executável ou instalador é entregue.
- Rollback: reextrair o ZIP 0.1.10 em outra pasta; ele conserva o problema original de dependências.
- Documentação: relatório, changelog e manifesto de arquivos atualizados.
- Checklist: fontes preservados; testes web e isolados aprovados; limites nativos declarados; checksums verificados.
- Commit sugerido: fix(build): pin Scale Simulator Tauri dependencies
- Merge sugerido: squash após validação Windows.
- Versão: 0.1.11 (patch).
- Publicação: nenhuma branch, PR, commit, tag ou release enviada ao GitHub; informações acima são uma minuta local.
