# Scale Simulator

Laboratório de balanças TCP e RS232 em Tauri/Rust + Vue. Esta branch contém a migração para o template 1.2.0 e a identidade V2, destinada ao canal beta.

- Simulador completo com presets originais, modelos salvos e controles independentes.
- Playground para selecionar um modelo pronto e iniciar uma instância, sem editar presets ou alterar outras instâncias.
- Guia pesquisável e exportável dentro da aplicação em Documentação.
- Desktop e backend Web autenticado; SQLite local com migração conservadora.

## Desenvolvimento
`npm ci`, `npm run build:web`, `npm run tauri:dev`. Para backend sem interface, use os scripts headless/CloudPanel fornecidos. Não há senha pública embutida; siga o primeiro acesso do template.

## Documentação
- [Modelos salvos](docs/MODELOS-SALVOS.md)
- [Migração, distribuição e rollback](docs/MIGRACAO-1.2.0.md)

main é estável; develop recebe pré-releases beta. Nunca use uma pré-release sobre a única cópia do banco. A publicação exige os gates de CI e os artefatos completos da matriz do produto.
