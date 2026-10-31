# Scale Simulator - Release CloudPanel/Linux sem instalador

Pacote para **Debian/Ubuntu**, **CloudPanel**, **CLI** e **navegador**, sem instalador gráfico e sem abrir janela desktop/Tauri.

O release oficial gera dois artefatos:

- `linux-x64`: `x86_64-unknown-linux-gnu`
- `linux-x86`: `i686-unknown-linux-gnu`

## Uso no CloudPanel como Node.js Application

1. Envie/descompacte o `.tar.gz` gerado no GitHub Release.
2. Copie o ambiente padrão:
   ```bash
   cp .env.example .env
   ```
3. Confirme permissão do binário:
   ```bash
   chmod +x bin/scale_simulator_tauri *.sh
   ```
4. No CloudPanel, use:
   ```bash
   npm start
   ```

O CloudPanel normalmente injeta a variável `PORT`. Por padrão, o launcher usa essa porta como porta do WebPort/browser.

## Uso direto por terminal Linux

```bash
cp .env.example .env
chmod +x bin/scale_simulator_tauri *.sh
./start.sh
./status.sh
./logs.sh
```

Parar/reiniciar:

```bash
./stop.sh
./restart.sh
```

CLI:

```bash
./cli.sh
```

Worker:

```bash
./worker.sh
```

Ver portas configuradas:

```bash
./ports.sh
# ou
npm run ports
```

Health check:

```bash
npm run health
```

Cron opcional para checagem:

```cron
* * * * * cd /caminho/do/app && ./check.sh >/dev/null 2>&1
```

## Portas padrão

| Serviço | Variável | Padrão seguro | Porta |
|---|---|---:|---:|
| API Headless | `SCALE_SIMULATOR_API_HOST` / `SCALE_SIMULATOR_API_PORT` | `127.0.0.1` | `61001` |
| Web/browser/WebPort | `SCALE_SIMULATOR_WEB_HOST` / `SCALE_SIMULATOR_WEB_PORT` | `127.0.0.1` | `61002` |
| Webhook | `SCALE_SIMULATOR_WEBHOOK_HOST` / `SCALE_SIMULATOR_WEBHOOK_PORT` | `127.0.0.1` | `61003` |
| WebSocket | `SCALE_SIMULATOR_WEBSOCKET_HOST` / `SCALE_SIMULATOR_WEBSOCKET_PORT` | `127.0.0.1` | `61004` |

## Alterar portas

Edite o `.env`:

```env
SCALE_SIMULATOR_API_PORT=62001
SCALE_SIMULATOR_WEB_PORT=62002
SCALE_SIMULATOR_WEBHOOK_PORT=62003
SCALE_SIMULATOR_WEBSOCKET_PORT=62004
```

No CloudPanel, se ele injetar `PORT`, essa porta terá prioridade sobre `SCALE_SIMULATOR_WEB_PORT`. Para forçar o uso da porta do `.env`:

```env
SCALE_SIMULATOR_RESPECT_CLOUDPANEL_PORT=false
```

## Publicar todas as portas da aplicação

Por padrão, o release é seguro e mantém tudo em `127.0.0.1`. Para publicar API, Web, Webhook e WebSocket diretamente em todas as interfaces de rede, use:

```bash
cp .env.public.example .env
```

Ou ajuste manualmente:

```env
SCALE_SIMULATOR_PUBLISH_ALL_PORTS=true
SCALE_SIMULATOR_START_ALL_PORTS=true
SCALE_SIMULATOR_RESPECT_CLOUDPANEL_PORT=false
SCALE_SIMULATOR_PUBLIC_BIND_HOST=0.0.0.0

SCALE_SIMULATOR_API_HOST=0.0.0.0
SCALE_SIMULATOR_API_PORT=61001

SCALE_SIMULATOR_WEB_HOST=0.0.0.0
SCALE_SIMULATOR_WEB_PORT=61002

SCALE_SIMULATOR_WEBHOOK_ENABLED=true
SCALE_SIMULATOR_WEBHOOK_AUTO_START=true
SCALE_SIMULATOR_WEBHOOK_HOST=0.0.0.0
SCALE_SIMULATOR_WEBHOOK_PORT=61003

SCALE_SIMULATOR_WEBSOCKET_ENABLED=true
SCALE_SIMULATOR_WEBSOCKET_AUTO_START=true
SCALE_SIMULATOR_WEBSOCKET_HOST=0.0.0.0
SCALE_SIMULATOR_WEBSOCKET_PORT=61004
```

Depois libere as portas no firewall/Nginx/proxy conforme sua infraestrutura.

## Segurança

Se for publicar portas diretamente:

- troque `SCALE_SIMULATOR_API_TOKEN`, `SCALE_SIMULATOR_WEBHOOK_TOKEN` e `SCALE_SIMULATOR_WEBSOCKET_TOKEN`;
- mantenha `SCALE_SIMULATOR_API_REQUIRE_TOKEN=true`;
- use firewall para liberar apenas IPs confiáveis quando possível;
- prefira HTTPS/reverse proxy para tráfego público.

Para CloudPanel convencional, a recomendação é expor publicamente apenas o WebPort pelo painel e manter API/Webhook/WebSocket em `127.0.0.1`.
