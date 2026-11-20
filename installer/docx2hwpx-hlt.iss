; Inno Setup 6 스크립트 — 윈도 설치 파일(docx2hwpx-hlt-setup-x64.exe)
;
;   iscc /DAppVersion=0.2.0 /DBinDir=..\target\release installer\docx2hwpx-hlt.iss
;
; - 관리자 권한 없이 사용자 폴더에 설치한다(설치 창에서 「모든 사용자」를 고르면 Program Files).
; - 시작 메뉴 바로가기 · 바탕화면 바로가기(선택) · .docx 우클릭 메뉴 · 보내기 메뉴.
; - 제거하면 레지스트리 항목 · 바로가기도 함께 지운다.

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#ifndef BinDir
  #define BinDir "..\target\release"
#endif

#define AppName      "DOCX2HWPX-HLT"
#define AppNameKo    "특허 명세서 변환"
#define GuiExe       "docx2hwpx-hlt-gui.exe"
#define CliExe       "docx2hwpx-hlt.exe"
#define AppUrl       "https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT"
#define VerbKey      "Software\Classes\SystemFileAssociations\.docx\shell\Docx2HwpxHlt"

[Setup]
; AppId 는 바꾸지 않는다 — 바꾸면 업그레이드가 아니라 별개 프로그램으로 설치된다.
AppId={{6E0B3F1C-2D7A-4C55-9B8E-D2C4A1F0B7E3}
AppName={#AppName}
AppVerName={#AppNameKo} {#AppVersion}
AppVersion={#AppVersion}
AppPublisher=DOCX2HWPX-HLT
AppPublisherURL={#AppUrl}
AppSupportURL={#AppUrl}/issues
AppUpdatesURL={#AppUrl}/releases
VersionInfoVersion={#AppVersion}
VersionInfoProductName={#AppName}
VersionInfoDescription={#AppNameKo} 설치
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppNameKo}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\{#GuiExe}
UninstallDisplayName={#AppNameKo} ({#AppName})
WizardStyle=modern
WizardSizePercent=110
Compression=lzma2/ultra64
SolidCompression=yes
OutputDir=..\dist
OutputBaseFilename=docx2hwpx-hlt-setup-x64
CloseApplications=yes
ChangesAssociations=yes

[Languages]
Name: "korean";  MessagesFile: "compiler:Languages\Korean.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
korean.TaskDesktop=바탕화면에 바로가기 만들기
english.TaskDesktop=Create a desktop shortcut
korean.TaskContext=탐색기에서 .docx 를 우클릭하면 「HLT · HWPX 로 변환」 메뉴 보이기
english.TaskContext=Add "Convert to HLT · HWPX" to the .docx right-click menu
korean.TaskSendTo=「보내기」 메뉴에 추가 (여러 파일을 한꺼번에 보낼 때)
english.TaskSendTo=Add to the "Send to" menu (for many files at once)
korean.GroupShell=탐색기 메뉴
english.GroupShell=Explorer integration
korean.VerbName=HLT · HWPX 로 변환
english.VerbName=Convert to HLT · HWPX
korean.Uninstall={#AppNameKo} 제거
english.Uninstall=Uninstall {#AppNameKo}

[Tasks]
Name: "desktopicon"; Description: "{cm:TaskDesktop}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "contextmenu"; Description: "{cm:TaskContext}"; GroupDescription: "{cm:GroupShell}"
Name: "sendto";      Description: "{cm:TaskSendTo}";  GroupDescription: "{cm:GroupShell}"

[Files]
Source: "{#BinDir}\{#GuiExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#BinDir}\{#CliExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md";        DestDir: "{app}"; Flags: ignoreversion isreadme
Source: "..\AGENTS.md";        DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppNameKo}";        Filename: "{app}\{#GuiExe}"; Comment: "Word(.docx) → HLT · HWPX"
Name: "{autoprograms}\{cm:Uninstall}";      Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppNameKo}";          Filename: "{app}\{#GuiExe}"; Tasks: desktopicon
Name: "{usersendto}\{#AppNameKo} (HLT · HWPX)"; Filename: "{app}\{#GuiExe}"; Tasks: sendto

[Registry]
; HKA = 관리자 설치면 HKLM, 사용자 설치면 HKCU. SystemFileAssociations 라서 .docx 의 기본 앱(Word)은 건드리지 않는다.
; 여러 파일을 골라 누르면 파일마다 실행되지만, 앱이 이미 열린 창으로 모아 넣는다(한 창에서 일괄 변환).
Root: HKA; Subkey: "{#VerbKey}"; ValueType: string; ValueName: "";     ValueData: "{cm:VerbName}"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKA; Subkey: "{#VerbKey}"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#GuiExe}"",0"; Tasks: contextmenu
Root: HKA; Subkey: "{#VerbKey}"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "Player"; Tasks: contextmenu
Root: HKA; Subkey: "{#VerbKey}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#GuiExe}"" ""%1"""; Tasks: contextmenu

[Run]
Filename: "{app}\{#GuiExe}"; Description: "{cm:LaunchProgram,{#AppNameKo}}"; Flags: nowait postinstall skipifsilent
