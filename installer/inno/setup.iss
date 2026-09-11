#define MyAppName "AgentOS Studio"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "AgentOS Team"
#define MyAppExeName "myagent_ui.exe"

[Setup]
AppId={{8D9A4B3A-8B6C-4D2A-9E1F-2B3C4D5E6F7C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={localappdata}\Programs\AgentOS
DefaultGroupName={#MyAppName}
OutputDir=dist-windows
OutputBaseFilename=AgentOS-Setup-x64
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest
DisableProgramGroupPage=yes

[Languages]
Name: "chinesesimp"; MessagesFile: "compiler:Languages\ChineseSimplified.isl,compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "创建桌面快捷方式"; GroupDescription: "附加快捷方式:"; Flags: unchecked

[Files]
Source: "dist-windows\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs; Excludes: "*.exe,*.msi,*.zip"
Source: "dist-windows\myagent_ui.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "dist-windows\myagent_runtime.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "立即运行 {#MyAppName}"; Flags: nowait postinstall skipifsilent
