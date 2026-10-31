# Serviço Linux/systemd

Exemplo de unidade systemd:

```ini
[Unit]
Description=Scale Simulator Server
After=network.target

[Service]
ExecStart=/opt/scale-simulator/app-server --mode=headless-api
Restart=always
User=templateapp
Environment=APP_ENV=production

[Install]
WantedBy=multi-user.target
```
