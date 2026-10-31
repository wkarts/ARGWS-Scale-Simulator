# Etapa v0.1.9 — Correção RS232 e compatibilidade com All Driver Platform

## Problema tratado

A aplicação podia manter um simulador serial registrado mesmo quando a abertura real da porta falhava. Isso causava mensagens falsas como:

```text
A porta serial COM1 já está em uso por outro simulador nesta aplicação.
```

Além disso, o frame WT1000N original do Scale Simulator não era compatível com o parser WT1000N do All Driver Platform.

## Correções

- limpeza automática de simuladores finalizados antes de iniciar nova instância;
- validação real da abertura da porta serial antes de registrar o simulador;
- erro mais claro quando a COM está ocupada ou inválida;
- inclusão dos drivers/protocolos do ADP como presets;
- ajuste do frame WT1000N para o formato esperado pelo ADP.

## Drivers compatíveis com o ADP

- WT1000N
- WT3000IR
- TOLEDO8540
- A12e
- LIDERLD2052

## Uso correto com par virtual COM

Para testar o All Driver Platform no mesmo computador, não use a mesma COM nos dois sistemas.

Exemplo correto:

```text
Scale Simulator abre COM10
All Driver Platform lê COM11
COM10 <-> COM11 via com0com/VSPE/null modem
```

Exemplo incorreto:

```text
Scale Simulator abre COM1
All Driver Platform também tenta abrir COM1
```

Uma porta serial física ou virtual só pode ser aberta por um processo por vez no Windows.
