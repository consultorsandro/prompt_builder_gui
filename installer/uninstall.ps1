# 🗑️ Prompt Builder GUI - Script de Desinstalação

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\PromptBuilderGUI"
)

Write-Host "🗑️ Desinstalando Prompt Builder GUI..." -ForegroundColor Yellow

# Remover ícone da área de trabalho
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "Prompt Builder GUI.lnk"
if (Test-Path $ShortcutPath) {
    Remove-Item $ShortcutPath -Force
    Write-Host "✅ Ícone removido da área de trabalho" -ForegroundColor Green
}

# Remover entrada do Menu Iniciar
$StartMenuPath = [Environment]::GetFolderPath("StartMenu")
$ProgramsPath = Join-Path $StartMenuPath "Programs"
$StartMenuShortcut = Join-Path $ProgramsPath "Prompt Builder GUI.lnk"
if (Test-Path $StartMenuShortcut) {
    Remove-Item $StartMenuShortcut -Force
    Write-Host "✅ Entrada removida do Menu Iniciar" -ForegroundColor Green
}

# Remover registro do sistema
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PromptBuilderGUI"
if (Test-Path $UninstallKey) {
    Remove-Item $UninstallKey -Force
    Write-Host "✅ Registro removido do sistema" -ForegroundColor Green
}

# Perguntar se quer remover arquivos de instalação
$Choice = Read-Host "❓ Deseja remover todos os arquivos de instalação? (s/N)"
if ($Choice -eq "s" -or $Choice -eq "S") {
    if (Test-Path $InstallPath) {
        Remove-Item $InstallPath -Recurse -Force
        Write-Host "✅ Arquivos de instalação removidos" -ForegroundColor Green
    }
} else {
    Write-Host "💾 Arquivos mantidos em: $InstallPath" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "✅ Desinstalação concluída!" -ForegroundColor Green
