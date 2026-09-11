$AppName = "AgentOS Studio"
$InstallDir = "$env:LOCALAPPDATA\Programs\AgentOS"

Write-Host ">>> 正在卸载 $AppName ..." -ForegroundColor Yellow

# 1. 移除快捷方式
Remove-Item -Path "$env:USERPROFILE\Desktop\$AppName.lnk" -Force -ErrorAction SilentlyContinue
Remove-Item -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\AgentOS" -Recurse -Force -ErrorAction SilentlyContinue

# 2. 移除 Windows 注册表卸载信息
Remove-Item -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\AgentOS" -Recurse -Force -ErrorAction SilentlyContinue

# 3. 从 PATH 移除
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -like "*$InstallDir*") {
    $CleanPath = ($UserPath.Split(';') | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $CleanPath, "User")
}

# 4. 清理安装目录
Write-Host ">>> 正在清除应用安装目录..."
Start-Process cmd.exe -ArgumentList "/c timeout /t 1 >nul & rmdir /s /q `"$InstallDir`"" -WindowStyle Hidden
Write-Host ">>> $AppName 已从您的 Windows 计算机中成功卸载。" -ForegroundColor Green
