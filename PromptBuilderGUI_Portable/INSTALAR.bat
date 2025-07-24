@echo off 
echo. 
echo ================================ 
echo   Prompt Builder GUI - Installer 
echo ================================ 
echo. 
echo Instalando Prompt Builder GUI... 
echo. 
cd installer 
powershell.exe -ExecutionPolicy Bypass -File install_simple.ps1 
echo. 
echo Instalacao concluida! 
pause 
