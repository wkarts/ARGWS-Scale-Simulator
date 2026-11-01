# Pré-release contínua em develop

A versão de produto continua `0.3.0`, com uma única tag `v0.3.0` marcada como
pré-release no GitHub. Uma execução de CI aprovada em develop pode atualizar
esse canal. A main e releases estáveis permanecem imutáveis.

## Substituição e recuperação

1. Confere origem do evento, SHA do checkout, anotação da tag anterior e avanço
   na linhagem de develop. Releases estáveis, imutáveis ou sem origem comprovada
   são recusadas.
2. Coloca a mesma release em rascunho. Seus downloads ficam temporariamente
   indisponíveis enquanto a matriz nova está em construção.
3. Salva um manifesto dos assets anteriores e renomeia os arquivos antigos com
   prefixo `previous-<objeto-da-tag>-`, preservando seus bytes no próprio rascunho.
4. Atualiza somente a tag com lease do objeto anterior, nunca a branch com force.
5. Compila o novo código. Retentativas da mesma origem reaproveitam apenas bundles
   novos cujo digest SHA-256 remoto corresponde aos bytes baixados. Arquivos
   anteriores não participam dessa seleção.
6. Exige os 13 bundles: Windows x64/x86 NSIS e MSI, Linux AppImage/deb/rpm,
   macOS Intel/ARM DMG e app.tar.gz, web e fontes. Gera manifesto e checksums.
7. Confirma hashes remotos de todo o conjunto, remove os backups da geração
   anterior e publica a mesma pré-release. Não promove latest nem main.

Se o build falhar, a release continua em rascunho e os arquivos anteriores ficam
preservados. Reexecutar os jobs falhos continua o mesmo conjunto; não apagar o
rascunho nem reconstruir a partir de uma tag de outra origem. O manifesto de
backup identifica a tag anterior para recuperação manual, se necessária.

## Evidências

Testes locais verificam limites do canal, linhagem, ordem rascunho/backup/rename,
lease da tag e recusa de alteração de releases estáveis. O CI valida a execução
com GitHub e as plataformas reais. Testes locais não comprovam publicação.
