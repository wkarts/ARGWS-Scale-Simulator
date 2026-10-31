> Atualização 0.1.11: correção das dependências Tauri. Veja [instruções e validação](docs/CORRECAO_TAURI_0.1.11.md).

# Scale Simulator 0.1.10

Simulador desktop de balanças e indicadores para testar integrações TCP e RS232, desenvolvido em Rust, Tauri 2, Vue 3 e TypeScript.

## Esta entrega

- Correção exclusiva do encoder Toledo 8540: estados, sinal, bruto/líquido, tara, casas decimais, kg/lb e checksum em quadros de 18 bytes.
- Os outros oito encoders foram preservados byte a byte; não se afirma que todo comportamento de hardware foi homologado.
- Configuração de porta, baud, bits de dados, paridade e stop bits. Defaults anteriores preservados quando os novos campos não são informados.
- Oscilação, tara, estabilidade, modos de leitura e múltiplas instâncias continuam disponíveis.
- Opções avançadas, desligadas por padrão, permitem substituir prefixo/terminador em hexadecimal e testar checksum do Toledo 8540. Elas podem gerar quadros inválidos propositalmente.
- Identidade visual própria, versão sincronizada, dashboard baseado em instâncias reais e administração preservada em área secundária.

## Executar

Requisitos: Node.js/npm, Rust estável e pré-requisitos nativos do Tauri para o sistema operacional.

```
npm ci
npm run validate:web
npm run tauri:dev
```

Somente interface web: `npm run dev`. A simulação serial depende do aplicativo desktop e das permissões do sistema; o navegador não recebe acesso automático à COM.

Build desktop: `npm run tauri:build`. Este ZIP contém código-fonte; não contém instalador Windows certificado.

## Testar com ADP

Em um único computador, use um par serial virtual como COM10 ↔ COM11. Abra COM10 no Scale Simulator e COM11 no ADP; use os mesmos parâmetros seriais nas duas pontas. Nunca abra a mesma COM nos dois programas.

O preset 8540 aplica 4800/7E1, com opção 7E2 ou outras configurações explícitas para testes. O modo contínuo 8540 espera STX e checksum habilitados no consumidor. TCP serve para testes de bytes sem hardware; não implica que o driver serial do ADP aceite conexão TCP diretamente.

## Documentação

- [Correção e uso do Toledo 8540](README-TOLEDO8540.md)
- [Validação de protocolo e limitações](docs/VALIDACAO_TOLEDO8540.md)
- [Comparação factual com o ADP atual](docs/COMPARACAO_ADP_1.9.1.md)
- [Identidade visual e validação da interface](docs/IDENTIDADE_SCALE_SIMULATOR.md)
- [Parâmetros seriais e quadros de teste](docs/OPCOES_SERIAIS_E_FRAMES.md)
- [Pull Request preparada, não publicada](PR_INFO.md)
- [Primeiro uso](docs/FIRST_USE.md)
- [Configuração de runtime](docs/RUNTIME_CONFIGURATION.md)

Identificadores de instalação, diretórios de dados, rotas, autenticação e recursos administrativos existentes foram mantidos. Documentação técnica antiga foi conservada para referência; algumas páginas históricas ainda descrevem a base original do projeto.

## Modelos salvos
Consulte [o guia](docs/MODELOS-SALVOS.md) para salvar, editar e carregar configurações sem alterar instâncias em execução.
