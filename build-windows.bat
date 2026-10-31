@echo off
setlocal EnableExtensions EnableDelayedExpansion

cd /d "%~dp0"

set "APP_NAME=SCALE SIMULATOR TAURI"
set "BUNDLE_DIR=src-tauri\target\release\bundle"

if "%1"=="--debug" set "TAURI_BUILD_SCRIPT=tauri:build:debug"
if not defined TAURI_BUILD_SCRIPT set "TAURI_BUILD_SCRIPT=tauri:build"

echo ============================================================
echo  %APP_NAME% - BUILD WINDOWS
echo ============================================================
echo.

echo [1/7] Verificando Node.js...
where node >nul 2>nul
if errorlevel 1 (
  echo ERRO: Node.js nao encontrado no PATH.
  echo Instale o Node.js LTS, feche o terminal e execute novamente.
  exit /b 1
)
node --version
if errorlevel 1 exit /b 1

echo.
echo [2/7] Verificando npm...
where npm >nul 2>nul
if errorlevel 1 (
  echo ERRO: npm nao encontrado no PATH.
  echo Reinstale o Node.js LTS marcando a opcao de adicionar ao PATH.
  exit /b 1
)
call npm --version
if errorlevel 1 exit /b 1

echo.
echo [3/7] Verificando Rust/Cargo...
where cargo >nul 2>nul
if errorlevel 1 (
  echo ERRO: Cargo/Rust nao encontrado no PATH.
  echo Instale pelo rustup: https://rustup.rs/
  echo Depois feche e abra o PowerShell/CMD novamente.
  exit /b 1
)
cargo --version
if errorlevel 1 exit /b 1

echo.
echo [4/7] Instalando dependencias NPM...
if not exist package-lock.json (
  echo ERRO: package-lock.json ausente. Extraia novamente o pacote completo.
  exit /b 1
)
if not exist src-tauri\Cargo.lock (
  echo ERRO: Cargo.lock ausente. Extraia novamente o pacote completo.
  exit /b 1
)
call npm ci
if errorlevel 1 (
  echo ERRO: falha ao instalar dependencias NPM.
  exit /b 1
)

echo.
echo [5/7] Validando frontend Vue/TypeScript...
call npm run ci:tauri
if errorlevel 1 exit /b 1
call npm run build:web
if errorlevel 1 (
  echo ERRO: falha no build web/typecheck.
  exit /b 1
)

echo.
echo [6/7] Formatando e validando Rust...
call npm run fmt:rust
if errorlevel 1 (
  echo ERRO: falha ao formatar Rust.
  exit /b 1
)
call npm run fmt:rust:check
if errorlevel 1 (
  echo ERRO: cargo fmt --check encontrou diferencas.
  exit /b 1
)
call npm run lint:rust
if errorlevel 1 (
  echo ERRO: cargo clippy encontrou erros.
  exit /b 1
)

echo.
echo [7/7] Gerando build Tauri Windows...
call npm run %TAURI_BUILD_SCRIPT%
if errorlevel 1 (
  echo ERRO: falha ao gerar build Tauri.
  exit /b 1
)

echo.
echo ============================================================
echo  BUILD CONCLUIDO COM SUCESSO
echo  Artefatos: %BUNDLE_DIR%
echo ============================================================
endlocal
exit /b 0
