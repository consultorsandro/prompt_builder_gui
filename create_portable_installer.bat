@echo off
echo.
echo ========================================
echo   Prompt Builder GUI - Portable Installer Creator
echo ========================================
echo.

:: Definir pasta de saida
set OUTPUT_DIR=PromptBuilderGUI_Portable
set INSTALLER_NAME=PromptBuilderGUI_Installer.zip

echo Criando instalador portátil...
echo.

:: Limpar pasta anterior se existir
if exist "%OUTPUT_DIR%" (
    echo Limpando pasta anterior...
    rmdir /s /q "%OUTPUT_DIR%"
)

:: Criar estrutura do instalador
mkdir "%OUTPUT_DIR%"
mkdir "%OUTPUT_DIR%\installer"
mkdir "%OUTPUT_DIR%\assets"

echo Copiando arquivos essenciais...

:: Copiar executável (se existir)
if exist "target\release\prompt-builder-gui.exe" (
    copy "target\release\prompt-builder-gui.exe" "%OUTPUT_DIR%\"
    echo ✅ Executável copiado
) else (
    echo ❌ Executável não encontrado! Execute "cargo build --release" primeiro.
    pause
    exit /b 1
)

:: Copiar scripts de instalação
copy "installer\install_simple.ps1" "%OUTPUT_DIR%\installer\"
copy "installer\install.bat" "%OUTPUT_DIR%\installer\"
copy "installer\uninstall.ps1" "%OUTPUT_DIR%\installer\"

:: Copiar documentação
copy "COMO_INSTALAR.md" "%OUTPUT_DIR%\"
copy "README.md" "%OUTPUT_DIR%\"
copy "LICENSE" "%OUTPUT_DIR%\"

:: Copiar interface Slint
copy "ui\app-window.slint" "%OUTPUT_DIR%\assets\"

:: Criar script de instalação simplificado para o portable
echo @echo off > "%OUTPUT_DIR%\INSTALAR.bat"
echo echo. >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo ================================ >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo   Prompt Builder GUI - Installer >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo ================================ >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo. >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo Instalando Prompt Builder GUI... >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo. >> "%OUTPUT_DIR%\INSTALAR.bat"
echo cd installer >> "%OUTPUT_DIR%\INSTALAR.bat"
echo powershell.exe -ExecutionPolicy Bypass -File install_simple.ps1 >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo. >> "%OUTPUT_DIR%\INSTALAR.bat"
echo echo Instalacao concluida! >> "%OUTPUT_DIR%\INSTALAR.bat"
echo pause >> "%OUTPUT_DIR%\INSTALAR.bat"

:: Criar README para o instalador portátil
echo # 🚀 Prompt Builder GUI - Instalador Portátil > "%OUTPUT_DIR%\LEIA-ME.txt"
echo. >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo Este pacote contém tudo que você precisa para instalar o Prompt Builder GUI! >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo. >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo ## Como Instalar: >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo 1. Duplo clique em "INSTALAR.bat" >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo 2. Aguarde a instalação >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo 3. Use o ícone na área de trabalho! >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo. >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo ## Tamanho do Programa: ~10MB >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo ## Compatibilidade: Windows 10/11 >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo. >> "%OUTPUT_DIR%\LEIA-ME.txt"
echo Divirta-se criando prompts incríveis! 🎉 >> "%OUTPUT_DIR%\LEIA-ME.txt"

echo.
echo ✅ Instalador portátil criado em: %OUTPUT_DIR%
echo.

:: Calcular tamanho do instalador
for /f %%A in ('dir "%OUTPUT_DIR%" /s /-c ^| find "bytes"') do set FOLDER_SIZE=%%A
echo 📊 Tamanho do instalador: %FOLDER_SIZE%
echo.

:: Criar ZIP se possível (requer PowerShell 5+)
echo Criando arquivo ZIP...
powershell.exe -Command "Compress-Archive -Path '%OUTPUT_DIR%\*' -DestinationPath '%INSTALLER_NAME%' -Force"

if exist "%INSTALLER_NAME%" (
    echo ✅ Arquivo ZIP criado: %INSTALLER_NAME%
    
    :: Mostrar tamanho do ZIP
    for %%A in ("%INSTALLER_NAME%") do echo 📦 Tamanho do ZIP: %%~zA bytes (%%~nxA)
) else (
    echo ℹ️  Não foi possível criar ZIP automaticamente
    echo    Você pode compactar a pasta "%OUTPUT_DIR%" manualmente
)

echo.
echo 🎉 SUCESSO! Instalador portátil criado!
echo.
echo 📁 Pasta: %OUTPUT_DIR%\
echo 📦 ZIP:   %INSTALLER_NAME%
echo.
echo Para distribuir:
echo 1. Envie apenas o arquivo %INSTALLER_NAME% (ou pasta %OUTPUT_DIR%)
echo 2. O destinatário extrai e executa INSTALAR.bat
echo 3. Programa instalado com ~10MB total!
echo.
pause
