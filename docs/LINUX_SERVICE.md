# Serviço Linux/systemd

Exemplo de unidade systemd:

```ini
[Unit]
Description=Template App Server
After=network.target

[Service]
ExecStart=/opt/template-app/app-server --mode=headless-api
Restart=always
User=templateapp
Environment=APP_ENV=production

[Install]
WantedBy=multi-user.target
```
