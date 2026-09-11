$ErrorActionPreference = "Stop"
$AppName = "AgentOS Studio"
$InstallDir = "$env:LOCALAPPDATA\Programs\AgentOS"
$SourceDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not (Test-Path "$SourceDir\myagent.toml")) { $SourceDir = $PSScriptRoot }

Write-Host ">>> 正在使用 Windows 原生安装程序部署 $AppName ..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Files = @("myagent_ui.exe", "myagent_runtime.exe", "myagent.toml", "*.dll")
foreach ($f in $Files) {
    Get-ChildItem -Path $SourceDir -Filter $f -ErrorAction SilentlyContinue | Copy-Item -Destination $InstallDir -Force
}

# 1. 创建开始菜单与桌面快捷方式 (使用 Windows 原生 WScript.Shell)
$WshShell = New-Object -ComObject WScript.Shell
$ExePath = "$InstallDir\myagent_ui.exe"

$StartMenuDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\AgentOS"
New-Item -ItemType Directory -Force -Path $StartMenuDir | Out-Null
$Shortcut = $WshShell.CreateShortcut("$StartMenuDir\$AppName.lnk")
$Shortcut.TargetPath = $ExePath
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.Description = "100% Pure Rust Autonomous AgentOS Studio"
$Shortcut.Save()

$DesktopShortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\$AppName.lnk")
$DesktopShortcut.TargetPath = $ExePath
$DesktopShortcut.WorkingDirectory = $InstallDir
$DesktopShortcut.Description = "100% Pure Rust Autonomous AgentOS Studio"
$DesktopShortcut.Save()

# 2. 注册至 Windows 原生 "已安装的应用" (添加/删除程序)
$RegKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\AgentOS"
New-Item -Path $RegKey -Force | Out-Null
Set-ItemProperty -Path $RegKey -Name "DisplayName" -Value "$AppName (Pure Rust)"
Set-ItemProperty -Path $RegKey -Name "DisplayVersion" -Value "0.1.0"
Set-ItemProperty -Path $RegKey -Name "Publisher" -Value "AgentOS Team"
Set-ItemProperty -Path $RegKey -Name "InstallLocation" -Value $InstallDir
Set-ItemProperty -Path $RegKey -Name "UninstallString" -Value "powershell.exe -ExecutionPolicy Bypass -File `"$InstallDir\uninstall.ps1`""
Set-ItemProperty -Path $RegKey -Name "DisplayIcon" -Value "$ExePath,0"

# 3. 复制卸载程序至安装目录
Copy-Item -Path "$PSScriptRoot\uninstall.ps1" -Destination "$InstallDir\uninstall.ps1" -Force -ErrorAction SilentlyContinue

# 4. 追加至用户 PATH 环境变量
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
}

Write-Host ">>> 安装成功！已生成桌面快捷方式与开始菜单，并已注册到 Windows 系统应用列表。" -ForegroundColor Green
