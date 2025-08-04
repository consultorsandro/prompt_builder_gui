# 🗑️ Prompt Builder GUI - Script de Desinstalação Profissional
# Versão 1.0 - Remove completamente o programa do sistema

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\PromptBuilderGUI",
    [switch]$KeepUserData
)

Write-Host ""
Write-Host "🗑️ PROMPT BUILDER GUI - DESINSTALADOR" -ForegroundColor Red
Write-Host "======================================" -ForegroundColor Red
Write-Host ""

# Confirmar desinstalação
if (-not $KeepUserData) {
    $response = Read-Host "⚠️  Deseja remover TODOS os dados incluindo prompts salvos? (s/N)"
    if ($response -ne 's' -and $response -ne 'S') {
        $KeepUserData = $true
        Write-Host "📁 Dados do usuário serão preservados" -ForegroundColor Yellow
    }
}

Write-Host "🔍 Removendo componentes do sistema..." -ForegroundColor Yellow

# Remover atalhos
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "Prompt Builder GUI.lnk"
if (Test-Path $ShortcutPath) {
    Remove-Item $ShortcutPath -Force
    Write-Host "   ✅ Atalho removido da área de trabalho" -ForegroundColor Gray
}

$StartMenuPath = [Environment]::GetFolderPath("StartMenu")
$ProgramsPath = Join-Path $StartMenuPath "Programs"
$StartMenuShortcut = Join-Path $ProgramsPath "Prompt Builder GUI.lnk"
if (Test-Path $StartMenuShortcut) {
    Remove-Item $StartMenuShortcut -Force
    Write-Host "   ✅ Entrada removida do Menu Iniciar" -ForegroundColor Gray
}

# Remover registro do sistema
try {
    $UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PromptBuilderGUI"
    if (Test-Path $UninstallKey) {
        Remove-Item $UninstallKey -Force
        Write-Host "   ✅ Registro removido do Painel de Controle" -ForegroundColor Gray
    }
} catch {
    Write-Host "   ⚠️  Não foi possível remover registro do sistema" -ForegroundColor Yellow
}

# Remover arquivos do programa
Write-Host "📁 Removendo arquivos do programa..." -ForegroundColor Yellow
if (Test-Path $InstallPath) {
    if ($KeepUserData) {
        # Remover apenas executável e scripts, manter dados do usuário
        $filesToRemove = @(
            "prompt-builder-gui.exe",
            "uninstall.ps1"
        )
        
        foreach ($file in $filesToRemove) {
            $fullPath = Join-Path $InstallPath $file
            if (Test-Path $fullPath) {
                Remove-Item $fullPath -Force
                Write-Host "   ✅ $file removido" -ForegroundColor Gray
            }
        }
        
        Write-Host "   📂 Dados do usuário preservados" -ForegroundColor Green
    } else {
        # Remover tudo
        Remove-Item $InstallPath -Recurse -Force
        Write-Host "   ✅ Todos os arquivos removidos" -ForegroundColor Gray
    }
} else {
    Write-Host "   ℹ️  Diretório de instalação não encontrado" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✅ DESINSTALAÇÃO CONCLUÍDA!" -ForegroundColor Green
Write-Host "==========================" -ForegroundColor Green
Write-Host ""

if ($KeepUserData -and (Test-Path $InstallPath)) {
    Write-Host "� Dados preservados em: $InstallPath" -ForegroundColor Cyan
    Write-Host "   (Você pode remover manualmente se desejar)" -ForegroundColor Gray
}

Write-Host "🎯 Prompt Builder GUI foi removido do sistema." -ForegroundColor White
Write-Host ""
Write-Host "Pressione Enter para finalizar..." -ForegroundColor Gray
Read-Host
