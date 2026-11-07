#define AppName "SonoraX"
#define AppPublisher "SonoraX"
#define AppExeName "SonoraX.exe"
#define AppVersion GetEnv("SONORA_VERSION")
#define SourceExe GetEnv("SONORA_EXE")
#define OutputDir GetEnv("SONORA_DIST")
; x64compatible for the x64 build, arm64 for the ARM one
#define Arch GetEnv("SONORA_ARCH")
; SonoraX-Setup for the x64 build, SonoraX-Setup-arm64 for the ARM one
#define SetupName GetEnv("SONORA_SETUP")

[Setup]
AppId={{D0EE49E6-4F4A-4AA0-B126-96A2E22FFEA0}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\{#AppExeName}
OutputDir={#OutputDir}
OutputBaseFilename={#SetupName}
SetupIconFile=..\..\assets\windows\sonora.ico
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed={#Arch}
ArchitecturesInstallIn64BitMode={#Arch}
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=commandline dialog
WizardStyle=modern
ChangesAssociations=yes

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"

[Files]
Source: "{#SourceExe}"; DestDir: "{app}"; DestName: "{#AppExeName}"; Flags: ignoreversion
Source: "..\..\COPYING"; DestDir: "{app}"; DestName: "LICENSE"; Flags: ignoreversion
Source: "..\..\THIRD-PARTY.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon; Check: not SilentUpgrade

; Lists SonoraX in "Open With" and in Settings > Apps > Default apps for the file types
; it plays, without automatically becoming the default handler for any of them.
; MultiSelectModel=Player is the key Explorer honors to invoke the app once
; with every selected file passed as its own argument, instead of once per file.
[Registry]
Root: HKA; Subkey: "Software\SonoraX"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\SonoraX\Capabilities"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\SonoraX\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "{#AppName}"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "A native, familiar music streaming client"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities"; ValueType: string; ValueName: "ApplicationIcon"; ValueData: "{app}\{#AppExeName},0"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "{#AppName}"; ValueData: "Software\SonoraX\Capabilities"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "{#AppName}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
; .mp3
Root: HKA; Subkey: "Software\Classes\.mp3"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mp3\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mp3\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.mp3"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.mp3"; ValueType: string; ValueName: ""; ValueData: "MP3 Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.mp3"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.mp3\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.mp3\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".mp3"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mp3"; ValueData: "SonoraX.mp3"; Flags: uninsdeletevalue
; .flac
Root: HKA; Subkey: "Software\Classes\.flac"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.flac\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.flac\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.flac"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.flac"; ValueType: string; ValueName: ""; ValueData: "FLAC Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.flac"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.flac\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.flac\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".flac"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".flac"; ValueData: "SonoraX.flac"; Flags: uninsdeletevalue
; .m4a
Root: HKA; Subkey: "Software\Classes\.m4a"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.m4a\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.m4a\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.m4a"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.m4a"; ValueType: string; ValueName: ""; ValueData: "MPEG-4 Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.m4a"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.m4a\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.m4a\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".m4a"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".m4a"; ValueData: "SonoraX.m4a"; Flags: uninsdeletevalue
; .mp4 (no default apps entry, since most .mp4 files are videos)
Root: HKA; Subkey: "Software\Classes\.mp4"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mp4\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mp4\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.mp4"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.mp4"; ValueType: string; ValueName: ""; ValueData: "MPEG-4 Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.mp4"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.mp4\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.mp4\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".mp4"; ValueData: ""; Flags: uninsdeletevalue
; .aac
Root: HKA; Subkey: "Software\Classes\.aac"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aac\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aac\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.aac"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.aac"; ValueType: string; ValueName: ""; ValueData: "AAC Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.aac"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.aac\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.aac\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".aac"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".aac"; ValueData: "SonoraX.aac"; Flags: uninsdeletevalue
; .ogg
Root: HKA; Subkey: "Software\Classes\.ogg"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.ogg\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.ogg\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.ogg"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.ogg"; ValueType: string; ValueName: ""; ValueData: "Ogg Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.ogg"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.ogg\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.ogg\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".ogg"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ogg"; ValueData: "SonoraX.ogg"; Flags: uninsdeletevalue
; .oga
Root: HKA; Subkey: "Software\Classes\.oga"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.oga\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.oga\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.oga"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.oga"; ValueType: string; ValueName: ""; ValueData: "Ogg Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.oga"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.oga\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.oga\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".oga"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".oga"; ValueData: "SonoraX.oga"; Flags: uninsdeletevalue
; .opus
Root: HKA; Subkey: "Software\Classes\.opus"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.opus\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.opus\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.opus"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.opus"; ValueType: string; ValueName: ""; ValueData: "Opus Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.opus"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.opus\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.opus\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".opus"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".opus"; ValueData: "SonoraX.opus"; Flags: uninsdeletevalue
; .wav
Root: HKA; Subkey: "Software\Classes\.wav"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.wav\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.wav\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.wav"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.wav"; ValueType: string; ValueName: ""; ValueData: "WAVE Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.wav"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.wav\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.wav\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".wav"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".wav"; ValueData: "SonoraX.wav"; Flags: uninsdeletevalue
; .aiff
Root: HKA; Subkey: "Software\Classes\.aiff"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aiff\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aiff\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.aiff"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.aiff"; ValueType: string; ValueName: ""; ValueData: "AIFF Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.aiff"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.aiff\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.aiff\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".aiff"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".aiff"; ValueData: "SonoraX.aiff"; Flags: uninsdeletevalue
; .aif
Root: HKA; Subkey: "Software\Classes\.aif"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aif\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aif\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.aif"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.aif"; ValueType: string; ValueName: ""; ValueData: "AIFF Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.aif"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.aif\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.aif\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".aif"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".aif"; ValueData: "SonoraX.aif"; Flags: uninsdeletevalue
; .aifc
Root: HKA; Subkey: "Software\Classes\.aifc"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aifc\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.aifc\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.aifc"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.aifc"; ValueType: string; ValueName: ""; ValueData: "AIFF-C Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.aifc"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.aifc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.aifc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".aifc"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".aifc"; ValueData: "SonoraX.aifc"; Flags: uninsdeletevalue
; .webm
Root: HKA; Subkey: "Software\Classes\.webm"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.webm\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.webm\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.webm"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.webm"; ValueType: string; ValueName: ""; ValueData: "WebM Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.webm"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.webm\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.webm\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".webm"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".webm"; ValueData: "SonoraX.webm"; Flags: uninsdeletevalue
; .mka
Root: HKA; Subkey: "Software\Classes\.mka"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mka\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.mka\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.mka"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.mka"; ValueType: string; ValueName: ""; ValueData: "Matroska Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.mka"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.mka\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.mka\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".mka"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".mka"; ValueData: "SonoraX.mka"; Flags: uninsdeletevalue
; .wv
Root: HKA; Subkey: "Software\Classes\.wv"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.wv\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.wv\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.wv"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.wv"; ValueType: string; ValueName: ""; ValueData: "WavPack Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.wv"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.wv\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.wv\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".wv"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".wv"; ValueData: "SonoraX.wv"; Flags: uninsdeletevalue
; .ape
Root: HKA; Subkey: "Software\Classes\.ape"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.ape\OpenWithProgids"; Flags: uninsdeletekeyifempty
Root: HKA; Subkey: "Software\Classes\.ape\OpenWithProgids"; ValueType: string; ValueName: "SonoraX.ape"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraX.ape"; ValueType: string; ValueName: ""; ValueData: "Monkey's Audio"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraX.ape"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"
Root: HKA; Subkey: "Software\Classes\SonoraX.ape\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraX.ape\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".ape"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\SonoraX\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ape"; ValueData: "SonoraX.ape"; Flags: uninsdeletevalue
; sonorax, which no other app claims, so SonoraX owns the scheme outright
Root: HKA; Subkey: "Software\Classes\sonorax"; ValueType: string; ValueName: ""; ValueData: "URL:SonoraX Link"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\sonorax"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\sonorax\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\sonorax\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
; spotify
Root: HKA; Subkey: "Software\SonoraX\Capabilities\UrlAssociations"; ValueType: string; ValueName: "spotify"; ValueData: "SonoraXSpotify"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraXSpotify"; ValueType: string; ValueName: ""; ValueData: "Spotify Link"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraXSpotify"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\SonoraXSpotify\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraXSpotify\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
; deezer
Root: HKA; Subkey: "Software\SonoraX\Capabilities\UrlAssociations"; ValueType: string; ValueName: "deezer"; ValueData: "SonoraXDeezer"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraXDeezer"; ValueType: string; ValueName: ""; ValueData: "Deezer Link"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraXDeezer"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\SonoraXDeezer\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraXDeezer\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
; tidal
Root: HKA; Subkey: "Software\SonoraX\Capabilities\UrlAssociations"; ValueType: string; ValueName: "tidal"; ValueData: "SonoraXTidal"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\SonoraXTidal"; ValueType: string; ValueName: ""; ValueData: "TIDAL Link"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SonoraXTidal"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\SonoraXTidal\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\SonoraXTidal\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""

; Setup runs elevated, and a [Run] entry inherits that unless it says otherwise: postinstall
; entries default to runasoriginaluser, the relaunch after a silent update does not, and an
; elevated SonoraX is out of reach for tools like FancyZones that manage windows unelevated.
[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch {#AppName}"; Flags: nowait postinstall skipifsilent
Filename: "{app}\{#AppExeName}"; Flags: nowait runasoriginaluser; Check: RelaunchRequested

[Code]
function RelaunchRequested: Boolean;
begin
  Result := ExpandConstant('{param:relaunch|0}') = '1';
end;

// A silent run over an existing install is an update from the app or a package manager. It
// leaves the desktop alone, so a shortcut the user deleted or replaced stays that way.
function SilentUpgrade: Boolean;
begin
  Result := WizardSilent and (WizardForm.PrevAppDir <> '');
end;
