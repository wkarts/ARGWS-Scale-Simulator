# Publicação 0.3.0 como pré-release

Decisão do proprietário: versão de produto e tag sem sufixo, `0.3.0` e `v0.3.0`. O canal é representado pelo campo **Pre-release** do GitHub, não pelo texto da versão. `develop` nunca promove para estável. Não há incremento automático de beta nem de patch.

## Causa comprovada

No run 41263619530 o Windows x86 terminou a compilação Rust e gerou o executável. O WiX rejeitou `0.3.0-beta.1` ao gerar o MSI. Nove arquivos de macOS, Linux, Web e fontes ficaram preservados no rascunho antigo. Não foram apagados nem renomeados como se fossem outra versão.

## Correção

- O coordenador deriva a origem de `workflow_run`, preservando `develop` mesmo quando o workflow executa a partir de `main`.
- Planejamento usa SemVer normal e `prerelease=true` para develop.
- Versão repetida não cria outra tag. Uma tag existente de outro source não é movida nem recebe binários de código diferente.
- `workflow_dispatch` em develop aceita `resume_tag` para retomar um rascunho pré-release numérico existente. Os jobs fazem checkout da tag, não do novo código de aplicação.
- Configuração WiX numérica gerada apenas no runner, sem modificar identidade, upgradeCode, dados ou código do aplicativo.
- Assets completos são reutilizados mediante checksum; não há `--clobber`.

## Transição e aceite

Os binários antigos incorporam beta.1 internamente. É necessário construir os instaladores 0.3.0 novamente; mudar só o nome do arquivo seria incorreto. O rascunho/tag antigo permanece intacto até a publicação nova estar íntegra e a limpeza ser autorizada. A entrega final desejada é uma única pré-release v0.3.0. Não publicar parcialmente nem afirmar sucesso antes de todos os alvos e manifestos estarem prontos.

Executar testes de release, pipeline local real Git/Cargo, actionlint e CI do SHA final. Merge apenas em develop, mediante autorização. A promoção estável exige aprovação separada.
