!include "MUI2.nsh"

!define PRODUCT_NAME "AgentOS Studio"
!define PRODUCT_VERSION "0.1.0"
!define PRODUCT_PUBLISHER "AgentOS Team"
!define PRODUCT_EXE "myagent_ui.exe"
!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\AgentOS"

Unicode true
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "dist-windows\AgentOS-Setup-x64.exe"
InstallDir "$LOCALAPPDATA\Programs\AgentOS"
RequestExecutionLevel user
SetCompressor /SOLID lzma

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\${PRODUCT_EXE}"
!define MUI_FINISHPAGE_RUN_TEXT "立即运行 ${PRODUCT_NAME}"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "SimpChinese"

Section "MainSection" SEC01
    SetOutPath "$INSTDIR"
    SetOverwrite on

    File /r /x "*.exe" /x "*.msi" /x "*.zip" "dist-windows\*.*"
    File "dist-windows\myagent_ui.exe"
    File "dist-windows\myagent_runtime.exe"

    WriteUninstaller "$INSTDIR\Uninstall.exe"

    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk" "$INSTDIR\${PRODUCT_EXE}" "" "$INSTDIR\${PRODUCT_EXE}" 0
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\卸载 ${PRODUCT_NAME}.lnk" "$INSTDIR\Uninstall.exe" "" "$INSTDIR\Uninstall.exe" 0
    CreateShortCut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\${PRODUCT_EXE}" "" "$INSTDIR\${PRODUCT_EXE}" 0

    WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
    WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
    WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" "$INSTDIR\${PRODUCT_EXE},0"
    WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr HKCU "${UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegStr HKCU "${UNINST_KEY}" "InstallLocation" "$INSTDIR"
SectionEnd

Section "Uninstall"
    Delete "$DESKTOP\${PRODUCT_NAME}.lnk"
    RMDir /r "$SMPROGRAMS\${PRODUCT_NAME}"

    Delete "$INSTDIR\Uninstall.exe"
    Delete "$INSTDIR\myagent_ui.exe"
    Delete "$INSTDIR\myagent_runtime.exe"
    Delete "$INSTDIR\myagent.toml"
    Delete "$INSTDIR\*.dll"
    RMDir /r "$INSTDIR"

    DeleteRegKey HKCU "${UNINST_KEY}"
SectionEnd
