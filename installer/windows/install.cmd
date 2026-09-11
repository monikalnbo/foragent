@echo off
title AgentOS Studio 安装向导
chcp 65001 >nul
echo ====================================================
echo    正在启动 AgentOS Studio Windows 原生安装程序...
echo ====================================================
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1"
echo.
pause
