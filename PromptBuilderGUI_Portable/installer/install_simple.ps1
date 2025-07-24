# Prompt Builder GUI - Script de Instalacao Simples
param(
    [string]$InstallPath = "$env:LOCALAPPDATA\PromptBuilderGUI"
)

Write-Host "Instalando Prompt Builder GUI..." -ForegroundColor Green
Write-Host "Local de instalacao: $InstallPath" -ForegroundColor Cyan

# Criar diretorio de instalacao
if (!(Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    Write-Host "Diretorio de instalacao criado" -ForegroundColor Green
}

# Copiar executavel
$ExePath = Join-Path $PSScriptRoot "..\target\release\prompt-builder-gui.exe"
$DestPath = Join-Path $InstallPath "PromptBuilderGUI.exe"

if (Test-Path $ExePath) {
    Copy-Item $ExePath $DestPath -Force
    Write-Host "Executavel copiado" -ForegroundColor Green
} else {
    Write-Host "Executavel nao encontrado. Execute 'cargo build --release' primeiro." -ForegroundColor Red
    Read-Host "Pressione Enter para continuar..."
    exit 1
}

# Criar pasta para prompts salvos
$PromptsPath = Join-Path $InstallPath "prompts salvos"
if (!(Test-Path $PromptsPath)) {
    New-Item -ItemType Directory -Path $PromptsPath -Force | Out-Null
    Write-Host "Pasta para prompts criada" -ForegroundColor Green
}

# Copiar script de desinstalacao
$UninstallScript = Join-Path $PSScriptRoot "uninstall.ps1"
$UninstallDest = Join-Path $InstallPath "uninstall.ps1"
if (Test-Path $UninstallScript) {
    Copy-Item $UninstallScript $UninstallDest -Force
}

# Criar icone na area de trabalho
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "Prompt Builder GUI.lnk"

$WScript = New-Object -ComObject WScript.Shell
$Shortcut = $WScript.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $DestPath
$Shortcut.WorkingDirectory = $InstallPath
$Shortcut.Description = "Prompt Builder GUI - Construtor de Prompts para IA"
$Shortcut.Save()

Write-Host "Icone criado na area de trabalho" -ForegroundColor Green

# Criar entrada no menu Iniciar
$StartMenuPath = [Environment]::GetFolderPath("StartMenu")
$ProgramsPath = Join-Path $StartMenuPath "Programs"
$StartMenuShortcut = Join-Path $ProgramsPath "Prompt Builder GUI.lnk"

$StartShortcut = $WScript.CreateShortcut($StartMenuShortcut)
$StartShortcut.TargetPath = $DestPath
$StartShortcut.WorkingDirectory = $InstallPath
$StartShortcut.Description = "Prompt Builder GUI - Construtor de Prompts para IA"
$StartShortcut.Save()

Write-Host "Entrada no Menu Iniciar criada" -ForegroundColor Green

# Registrar no sistema
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PromptBuilderGUI"
try {
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
    
    Write-Host "Registrado no sistema para desinstalacao" -ForegroundColor Green
} catch {
    Write-Host "Aviso: Nao foi possivel registrar no sistema" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "INSTALACAO CONCLUIDA COM SUCESSO!" -ForegroundColor Green
Write-Host "Local: $InstallPath" -ForegroundColor Cyan
Write-Host "Voce pode iniciar o programa:" -ForegroundColor Yellow
Write-Host "  • Clicando no icone da area de trabalho" -ForegroundColor White
Write-Host "  • Pelo Menu Iniciar > Prompt Builder GUI" -ForegroundColor White
Write-Host "  • Executando: $DestPath" -ForegroundColor White
Write-Host ""
Write-Host "Divirta-se construindo prompts incriveis!" -ForegroundColor Magenta
Write-Host ""
Read-Host "Pressione Enter para finalizar"
