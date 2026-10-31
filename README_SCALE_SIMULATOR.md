# Scale Simulator Tauri

Aplicação desktop Rust/Tauri baseada no template corporativo para simular balanças e indicadores de pesagem.

## Executar em desenvolvimento no Windows

```bat
dev-windows.bat
```

## Gerar build Windows

```bat
build-windows.bat
```

## Login inicial

- Usuário: `admin`
- Senha: `admin123`

## Tela principal do módulo

Após o login, acesse:

```text
Ferramentas > Simulador de Balanças
```

## Teste rápido via PowerShell

Para um simulador contínuo na porta `9101`:

```powershell
$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 9101)
$stream = $client.GetStream()
$buffer = New-Object byte[] 1024
while ($true) {
  $read = $stream.Read($buffer, 0, $buffer.Length)
  [Text.Encoding]::ASCII.GetString($buffer, 0, $read)
}
```

Para um simulador sob comando na porta `9102`:

```powershell
$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 9102)
$stream = $client.GetStream()
$cmd = [Text.Encoding]::ASCII.GetBytes("PESO`r`n")
$stream.Write($cmd, 0, $cmd.Length)
$buffer = New-Object byte[] 1024
$read = $stream.Read($buffer, 0, $buffer.Length)
[Text.Encoding]::ASCII.GetString($buffer, 0, $read)
```

## Atualização v0.1.7 — Menu e RS232

A aplicação agora deixa o menu **Cadastro** apenas com **Empresa** e cria o grupo **Simulador**, contendo a opção **Balança**.

O foco principal do simulador passa a ser RS232/porta serial Windows, permitindo configurar portas `COM1` até `COMXXX`. O modo TCP continua disponível apenas como alternativa de teste.

Para simular uma balança serial para outro software no mesmo computador, utilize uma porta física real ou um par virtual de portas COM.

## Compatibilidade com All Driver Platform

A partir da versão 0.1.9, o simulador inclui presets compatíveis com os parsers seriais do All Driver Platform:

- WT1000N
- WT3000IR
- TOLEDO8540
- A12e
- LIDERLD2052

Para o ADP conseguir obter pesagem no mesmo computador, crie um par virtual COM. O simulador deve abrir uma ponta e o ADP deve ler a outra.

Exemplo:

```text
Simulador: COM10
ADP:       COM11
```

Não use a mesma porta nos dois programas, porque o Windows bloqueia acesso concorrente à mesma porta serial.
