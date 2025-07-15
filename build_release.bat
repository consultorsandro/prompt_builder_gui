@echo off
echo.
echo ========================================
echo   Prompt Builder GUI - Build Release
echo ========================================
echo.

echo 🛠️ Compilando versao otimizada...
echo.

:: Limpar builds anteriores
if exist "target\release" (
    echo 🧹 Limpando builds anteriores...
    cargo clean --release
)

:: Build otimizado
echo 🚀 Iniciando compilacao release...
cargo build --release

if %ERRORLEVEL% neq 0 (
    echo.
    echo ❌ Erro na compilacao!
    pause
    exit /b 1
)

echo.
echo ✅ Compilacao concluida!
echo 📦 Executavel gerado: target\release\prompt-builder-gui.exe
echo 📊 Tamanho do arquivo:

:: Mostrar tamanho do arquivo
for %%A in ("target\release\prompt-builder-gui.exe") do echo    %%~zA bytes (%%~zAk KB)

echo.
echo 🎯 Proximo passo: Execute o instalador em "installer\install.bat"
echo.
pause
