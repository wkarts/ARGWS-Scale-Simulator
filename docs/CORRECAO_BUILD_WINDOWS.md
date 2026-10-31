# Correção do build Windows

## Problema identificado

O `build-windows.bat` encerrava logo após exibir a versão do npm:

```text
[1/6] Verificando Node.js...
v22.12.0

[2/6] Verificando npm...
10.9.0
```

A causa era a chamada direta ao `npm` dentro de um arquivo `.bat`. No Windows, `npm` normalmente aponta para `npm.cmd`. Quando um `.bat` chama outro `.cmd` sem `call`, o controle não volta para o script original.

## Correção aplicada

Todas as chamadas ao npm dentro dos scripts Windows foram ajustadas para usar `call`, por exemplo:

```bat
call npm --version
call npm ci
call npm run build:web
call npm run tauri:build
```

Também foram adicionadas validações por etapa, mensagens de erro mais claras e saída explícita com `exit /b`.

## Comandos recomendados

Build final:

```bat
build-windows.bat
```

Build debug:

```bat
build-windows.bat --debug
```

Modo desenvolvimento:

```bat
dev-windows.bat
```
