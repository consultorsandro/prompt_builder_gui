# Prompt Builder GUI - Script de Instalacao Profissional
# Versao 1.0 - Atualizado em Agosto 2025
# Instala o Prompt Builder GUI no Windows com integracao completa

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\PromptBuilderGUI"
)

Write-Host ""
Write-Host "PROMPT BUILDER GUI - INSTALADOR" -ForegroundColor Yellow
Write-Host "===============================" -ForegroundColor Yellow
Write-Host "Local de instalacao: $InstallPath" -ForegroundColor Cyan
Write-Host ""

# Verificar se executando como administrador (recomendado mas nao obrigatorio)
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "Executando sem privilegios administrativos" -ForegroundColor Yellow
    Write-Host "Algumas funcionalidades podem ser limitadas" -ForegroundColor Yellow
    Write-Host ""
}

# Criar diretorio de instalacao
Write-Host "Criando estrutura de diretorios..." -ForegroundColor Green
if (!(Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    Write-Host "   Diretorio principal criado" -ForegroundColor Gray
} else {
    Write-Host "   Diretorio principal ja existe" -ForegroundColor Gray
}

# Copiar executavel
Write-Host "Copiando arquivos do programa..." -ForegroundColor Green
$ExePath = Join-Path $PSScriptRoot "..\target\release\prompt-builder-gui.exe"
$DestPath = Join-Path $InstallPath "prompt-builder-gui.exe"

if (Test-Path $ExePath) {
    Copy-Item $ExePath $DestPath -Force
    Write-Host "   Executavel principal copiado" -ForegroundColor Gray
    
    # Verificar se arquivo foi copiado corretamente
    $fileSize = (Get-Item $DestPath).Length
    Write-Host "   Tamanho do arquivo: $([math]::Round($fileSize/1MB, 2)) MB" -ForegroundColor Gray
} else {
    Write-Host "   Executavel nao encontrado em: $ExePath" -ForegroundColor Red
    Write-Host "   Execute 'cargo build --release' primeiro." -ForegroundColor Yellow
    Read-Host "Pressione Enter para sair"
    exit 1
}

# Copiar script de desinstalacao
$UninstallScript = Join-Path $PSScriptRoot "uninstall.ps1"
$UninstallDest = Join-Path $InstallPath "uninstall.ps1"
if (Test-Path $UninstallScript) {
    Copy-Item $UninstallScript $UninstallDest -Force
    Write-Host "   Script de desinstalacao copiado" -ForegroundColor Gray
}

# Criar pasta para prompts salvos
$PromptsPath = Join-Path $InstallPath "prompts_salvos"
if (!(Test-Path $PromptsPath)) {
    New-Item -ItemType Directory -Path $PromptsPath -Force | Out-Null
    Write-Host "   Pasta para prompts criada" -ForegroundColor Gray
} else {
    Write-Host "   Pasta para prompts ja existe" -ForegroundColor Gray
}

# Criar atalhos no sistema
Write-Host "Criando atalhos do sistema..." -ForegroundColor Green

# Icone na area de trabalho
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "Prompt Builder GUI.lnk"

$WScript = New-Object -ComObject WScript.Shell
$Shortcut = $WScript.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $DestPath
$Shortcut.WorkingDirectory = $InstallPath
$Shortcut.Description = "Prompt Builder GUI - Construtor Profissional de Prompts para IA"
$Shortcut.WindowStyle = 1
$Shortcut.Save()

Write-Host "   Atalho criado na area de trabalho" -ForegroundColor Gray

# Entrada no menu Iniciar
$StartMenuPath = [Environment]::GetFolderPath("StartMenu")
$ProgramsPath = Join-Path $StartMenuPath "Programs"
$StartMenuShortcut = Join-Path $ProgramsPath "Prompt Builder GUI.lnk"

$StartShortcut = $WScript.CreateShortcut($StartMenuShortcut)
$StartShortcut.TargetPath = $DestPath
$StartShortcut.WorkingDirectory = $InstallPath
$StartShortcut.Description = "Prompt Builder GUI - Construtor Profissional de Prompts para IA"
$StartShortcut.WindowStyle = 1
$StartShortcut.Save()

Write-Host "   Entrada no Menu Iniciar criada" -ForegroundColor Gray

# Registrar no sistema para controle de programas
Write-Host "Registrando no sistema..." -ForegroundColor Green
try {
    $UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PromptBuilderGUI"
    if (!(Test-Path $UninstallKey)) {
        New-Item -Path $UninstallKey -Force | Out-Null
    }

    Set-ItemProperty -Path $UninstallKey -Name "DisplayName" -Value "Prompt Builder GUI"
    Set-ItemProperty -Path $UninstallKey -Name "DisplayVersion" -Value "1.0.0"
    Set-ItemProperty -Path $UninstallKey -Name "Publisher" -Value "Sandro - Prompt Builder Development"
    Set-ItemProperty -Path $UninstallKey -Name "InstallLocation" -Value $InstallPath
    Set-ItemProperty -Path $UninstallKey -Name "UninstallString" -Value "powershell.exe -ExecutionPolicy Bypass -File `"$InstallPath\uninstall.ps1`""
    Set-ItemProperty -Path $UninstallKey -Name "NoModify" -Value 1
    Set-ItemProperty -Path $UninstallKey -Name "NoRepair" -Value 1
    Set-ItemProperty -Path $UninstallKey -Name "EstimatedSize" -Value $([math]::Round($fileSize/1KB))

    Write-Host "   Registrado no Painel de Controle" -ForegroundColor Gray
} catch {
    Write-Host "   Nao foi possivel registrar no sistema" -ForegroundColor Yellow
}

# Finalizacao
Write-Host ""
Write-Host "INSTALACAO CONCLUIDA COM SUCESSO!" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green
Write-Host ""
Write-Host "Local de instalacao: $InstallPath" -ForegroundColor Cyan
Write-Host "Executavel: prompt-builder-gui.exe" -ForegroundColor Cyan
Write-Host "Prompts salvos: $PromptsPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "Como usar:" -ForegroundColor Yellow
Write-Host "   • Clique no atalho da area de trabalho" -ForegroundColor White
Write-Host "   • Ou procure 'Prompt Builder GUI' no Menu Iniciar" -ForegroundColor White
Write-Host "   • Para desinstalar, execute o uninstall.ps1" -ForegroundColor White
Write-Host ""
Write-Host "Pressione Enter para finalizar..." -ForegroundColor Gray
Read-Host
