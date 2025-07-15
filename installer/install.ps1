# 🦀⚡ Prompt Builder GUI - Script de Instalação
# Instala o Prompt Builder GUI no Windows com ícone na área de trabalho

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\PromptBuilderGUI"
)

Write-Host "Instalando Prompt Builder GUI..." -ForegroundColor Green
Write-Host "Local de instalacao: $InstallPath" -ForegroundColor Cyan

# Criar diretório de instalação
if (!(Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    Write-Host "✅ Diretório de instalação criado" -ForegroundColor Green
}

# Copiar executável
$ExePath = Join-Path $PSScriptRoot "..\target\release\prompt-builder-gui.exe"
$DestPath = Join-Path $InstallPath "PromptBuilderGUI.exe"

if (Test-Path $ExePath) {
    Copy-Item $ExePath $DestPath -Force
    Write-Host "✅ Executável copiado" -ForegroundColor Green
} else {
    Write-Host "❌ Executável não encontrado. Execute 'cargo build --release' primeiro." -ForegroundColor Red
    exit 1
}

# Criar pasta para prompts salvos
$PromptsPath = Join-Path $InstallPath "prompts salvos"
if (!(Test-Path $PromptsPath)) {
    New-Item -ItemType Directory -Path $PromptsPath -Force | Out-Null
    Write-Host "✅ Pasta para prompts criada" -ForegroundColor Green
}

# Criar ícone na área de trabalho
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "Prompt Builder GUI.lnk"

$WScript = New-Object -ComObject WScript.Shell
$Shortcut = $WScript.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $DestPath
$Shortcut.WorkingDirectory = $InstallPath
$Shortcut.Description = "Prompt Builder GUI - Construtor de Prompts para IA"
$Shortcut.Save()

Write-Host "✅ Ícone criado na área de trabalho" -ForegroundColor Green

# Criar entrada no menu Iniciar
$StartMenuPath = [Environment]::GetFolderPath("StartMenu")
$ProgramsPath = Join-Path $StartMenuPath "Programs"
$StartMenuShortcut = Join-Path $ProgramsPath "Prompt Builder GUI.lnk"

$StartShortcut = $WScript.CreateShortcut($StartMenuShortcut)
$StartShortcut.TargetPath = $DestPath
$StartShortcut.WorkingDirectory = $InstallPath
$StartShortcut.Description = "Prompt Builder GUI - Construtor de Prompts para IA"
$StartShortcut.Save()

Write-Host "✅ Entrada no Menu Iniciar criada" -ForegroundColor Green

# Registrar no sistema (opcional - para desinstalação)
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PromptBuilderGUI"
if (!(Test-Path $UninstallKey)) {
    New-Item -Path $UninstallKey -Force | Out-Null
}

Set-ItemProperty -Path $UninstallKey -Name "DisplayName" -Value "Prompt Builder GUI"
Set-ItemProperty -Path $UninstallKey -Name "DisplayVersion" -Value "1.0.0"
Set-ItemProperty -Path $UninstallKey -Name "Publisher" -Value "Sandro - Prompt Builder GUI"
Set-ItemProperty -Path $UninstallKey -Name "InstallLocation" -Value $InstallPath
Set-ItemProperty -Path $UninstallKey -Name "UninstallString" -Value "powershell.exe -ExecutionPolicy Bypass -File `"$InstallPath\uninstall.ps1`""
Set-ItemProperty -Path $UninstallKey -Name "NoModify" -Value 1
Set-ItemProperty -Path $UninstallKey -Name "NoRepair" -Value 1

Write-Host "✅ Registrado no sistema para desinstalação" -ForegroundColor Green

Write-Host ""
Write-Host "🎉 Instalação concluída com sucesso!" -ForegroundColor Green
Write-Host "📍 Local: $InstallPath" -ForegroundColor Cyan
Write-Host "🖥️  Você pode iniciar o programa:" -ForegroundColor Yellow
Write-Host "   • Clicando no ícone da área de trabalho" -ForegroundColor White
Write-Host "   • Pelo Menu Iniciar > Prompt Builder GUI" -ForegroundColor White
Write-Host "   • Executando: $DestPath" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Divirta-se construindo prompts incríveis!" -ForegroundColor Magenta
