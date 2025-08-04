@echo off
chcp 65001 >nul
title Prompt Builder GUI - Instalador v1.0

echo.
echo ╔══════════════════════════════════════╗
echo ║    🚀 PROMPT BUILDER GUI v1.0        ║
echo ║         Instalador Automático        ║
echo ╚══════════════════════════════════════╝
echo.

:: Verificar se executável release existe
if not exist "..\target\release\prompt-builder-gui.exe" (
    echo ❌ Executável release não encontrado!
    echo.
    echo 💡 Execute primeiro:
    echo    cargo build --release
    echo.
    echo 📍 Ou compile o projeto antes de instalar.
    echo.
    pause
    exit /b 1
)

echo � Verificando sistema...
echo    ✅ Executável encontrado
echo    📊 Preparando instalação
echo.

:: Executar script PowerShell com melhor tratamento de erros
echo 🚀 Iniciando instalação...
echo.
powershell.exe -ExecutionPolicy Bypass -NoProfile -File "%~dp0install.ps1"

if %errorlevel% neq 0 (
    echo.
    echo ❌ Erro durante a instalação!
    echo    Código de erro: %errorlevel%
    echo.
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ Instalação concluída com sucesso!
echo.
echo 💡 O programa foi instalado e está pronto para uso.
echo    Procure o ícone na área de trabalho ou Menu Iniciar.
echo.
pause
