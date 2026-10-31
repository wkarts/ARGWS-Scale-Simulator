# Publicação do Scale Simulator

## Canais

- `main`: versão estável indicada por VERSION e pelos manifests sincronizados. Primeira publicação: 0.1.11.
- `develop`: beta da próxima versão, sempre maior que a última estável. Exemplo: estável 0.1.11 → 0.1.12-beta.1. Se o projeto já estiver em 0.2.0, publicar 0.2.0-beta.N. Sem tags anteriores, utilizar o próximo patch da versão atual.
- Demais branches e PRs: somente CI, sem publicação.

A release automática aguarda o CI de um push em main/develop. Disparo manual permite repetir o processo nessas branches, com validações de frontend e builds. A versão beta é aplicada em commit exclusivo da tag: não altera os arquivos da develop. Tags existentes nunca são movidas. Uma versão estável precisa ser incrementada explicitamente nos manifests para publicar mudanças após a primeira release.

## Artefatos

Windows x64/x86, Linux x64, macOS Intel/Apple Silicon, Web e fontes. A publicação ocorre num único job somente após todos os builds. SHA256SUMS.txt e RELEASE-MANIFEST.json registram integridade, versão e commit. Não é publicada uma release parcial. Reexecução reutiliza a tag exata; releases já publicadas não têm assets sobrescritos.

## Cache

Limpeza diária, manual e após CI/Release. Adia limpeza se outros workflows estiverem em execução ou fila. Remove entradas sem uso há sete dias e as mais antigas acima de 6 GiB. Não remove releases, artefatos publicados nem dados da aplicação. Uma execução adiada será reconsiderada pelo próximo gatilho.

## Recuperação

Se um build falhar, corrigir a causa; repetir o workflow para uma tag estável já preparada não muda o código dessa tag. Se a correção precisar mudar os fontes, incrementar a versão e publicar uma nova tag. Nunca forçar tags ou reutilizar uma versão pública para código diferente.

## Etapas

Esta infraestrutura publica a aplicação existente 0.1.11. A migração para template 1.2.0, nova marca e manual interno devem ocorrer depois, preservando protocolos e dados, numa nova PR para develop.
