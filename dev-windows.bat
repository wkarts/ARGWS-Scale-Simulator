@echo off
setlocal EnableExtensions EnableDelayedExpansion

cd /d "%~dp0"

echo ============================================================
echo  SCALE SIMULATOR TAURI - DEV WINDOWS
echo ============================================================
echo.

echo [1/3] Verificando Node/npm...
where node >nul 2>nul
if errorlevel 1 (
  echo ERRO: Node.js nao encontrado no PATH.
  exit /b 1
)
node --version
if errorlevel 1 exit /b 1

where npm >nul 2>nul
if errorlevel 1 (
  echo ERRO: npm nao encontrado no PATH.
  exit /b 1
)
call npm --version
if errorlevel 1 exit /b 1

echo.
echo [2/3] Verificando Rust/Cargo...
where cargo >nul 2>nul
if errorlevel 1 (
  echo ERRO: Cargo/Rust nao encontrado no PATH.
  echo Instale pelo rustup: https://rustup.rs/
  exit /b 1
)
cargo --version
if errorlevel 1 exit /b 1

echo.
echo [3/3] Preparando dependencias e iniciando Tauri...
call npm ci
if errorlevel 1 exit /b 1
call npm run ci:tauri
if errorlevel 1 exit /b 1

call npm run tauri:dev
if errorlevel 1 exit /b 1

endlocal
exit /b 0
