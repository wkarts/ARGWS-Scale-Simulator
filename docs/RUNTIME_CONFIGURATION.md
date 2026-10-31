# Configuração runtime, portas e serviços

A porta padrão do Tauri permanece controlada por `src-tauri/tauri.conf.json` (`devUrl` em `http://localhost:1420`) e não deve ser alterada pela configuração runtime.

## Prioridade de configuração

1. Valores padrão do template.
2. `.env` local criado no diretório de dados da aplicação, ou em `TEMPLATE_APP_ENV_FILE` quando informado.
3. Configuração persistida no banco local (`app_settings`, chave `runtime.settings`).
4. Variáveis de ambiente reais do processo, que podem sobrescrever os valores persistidos em ambientes automatizados.

A tela **Sistema e parâmetros** permite alterar, validar e persistir as portas no banco local e no `.env`. O template usa por padrão a faixa alta `61001-61004` para reduzir conflito com bancos, servidores de desenvolvimento e aplicações comuns da máquina.

## Portas padrão

| Serviço | Env host | Env porta | Padrão | Finalidade |
| --- | --- | --- | ---: | --- |
| API interna | `TEMPLATE_APP_API_HOST` | `TEMPLATE_APP_API_PORT` | `61001` | API local Axum para desktop/headless. |
| Servidor web local | `TEMPLATE_APP_WEB_HOST` | `TEMPLATE_APP_WEB_PORT` | `61002` | Servidor web/preview fora da porta Tauri. |
| Serviços auxiliares | `TEMPLATE_APP_AUX_HOST` | `TEMPLATE_APP_AUX_PORT` | `61003` | Workers, filas, webhooks e jobs locais. |
| Bridge/core local | `TEMPLATE_APP_BRIDGE_HOST` | `TEMPLATE_APP_BRIDGE_PORT` | `61004` | Ponte local entre UI, backend embarcado e integrações nativas. |

## Validação e fallback

- A aplicação bloqueia salvamento quando duas entradas usam a mesma porta configurada.
- Ao iniciar a API interna, se a porta configurada estiver ocupada, o backend tenta um fallback seguro na próxima porta livre.
- A tela mostra avisos quando uma porta configurada não está livre no momento da validação.

## Tray, autostart e serviços

- `TEMPLATE_APP_TRAY_ENABLED`: habilita tray icon.
- `TEMPLATE_APP_TRAY_MINIMIZE_TO_TRAY`: preferência para minimizar para a bandeja.
- `TEMPLATE_APP_TRAY_CLOSE_TO_TRAY`: intercepta fechamento e oculta a janela.
- `TEMPLATE_APP_START_WITH_WINDOWS`: preferência de iniciar com Windows; no Windows a ação usa `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- `TEMPLATE_APP_SERVICES_AUTO_START`: inicia serviços internos junto com a aplicação.
- Instalação/remoção do backend como serviço usa os comandos nativos já expostos na tela de runtime/API e exige permissões do sistema operacional.

## Instalação inicial limpa

1. Inicie a aplicação.
2. O diretório de dados e o banco local são criados automaticamente.
3. Um `.env` padrão é criado se ainda não existir.
4. Abra **Sistema e parâmetros** para revisar portas, tray e autostart.
5. Reinicie serviços em execução após alterar portas.

## Assets e logo

Os assets de branding são importados pelo Vite para garantir empacotamento no build. Caso algum asset falhe, a UI aplica fallback visual e registra erro nos logs da aplicação.
