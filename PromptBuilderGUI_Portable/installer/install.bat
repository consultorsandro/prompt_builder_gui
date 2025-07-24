@echo off
echo.
echo ===================================
echo   Prompt Builder GUI - Installer
echo ===================================
echo.

:: Verificar se executavel existe
if not exist "..\target\release\prompt-builder-gui.exe" (
    echo ❌ Executavel nao encontrado!
    echo    Execute "cargo build --release" primeiro.
    pause
    exit /b 1
)

echo 🚀 Instalando Prompt Builder GUI...
echo.

:: Executar script PowerShell
powershell.exe -ExecutionPolicy Bypass -File "%~dp0install.ps1"

echo.
echo ✅ Instalacao concluida!
echo    Voce pode usar o programa clicando no icone da area de trabalho.
echo.
pause
