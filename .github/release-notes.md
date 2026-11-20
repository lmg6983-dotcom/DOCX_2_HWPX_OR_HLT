<div align="center">

**워드 명세서(`.docx`)를 특허청 편집기용 파일로 바꾸는 도구**

`.docx` ➜ `.hlt` (NK-Editor) · `.hwpx` (MOIP-Editor)

</div>

## 🎯 이번 판 한눈에 — v0.2.0

> **설치 파일**이 생기고, 화면 앱이 **여러 파일을 한꺼번에** 바꾸는 새 화면으로 바뀌었습니다.

### ✨ 추가

- 🧩 **설치 파일** `docx2hwpx-hlt-setup-x64.exe`
  - 관리자 권한 없이 설치 · 제어판에서 깔끔하게 제거
  - 시작 메뉴 바로가기, (선택) 바탕화면 바로가기
  - 탐색기에서 `.docx` 우클릭 → **HLT · HWPX 로 변환**, 「보내기」 메뉴
- 📚 **일괄 변환** — 여러 파일 · 폴더를 한꺼번에 놓고 한 번에 변환. 파일마다 상태(🟢 완료 · 🟠 확인할 곳 · 🔴 실패)와 자세한 보고, 「멈추기」
- 💾 **저장 위치** — 원본과 같은 폴더 / 다른 폴더. 같은 이름이 있으면 `이름 (2).hlt` 로 새로 만들거나 덮어쓰기
- 🪟 우클릭으로 여러 파일을 열어도 **한 창에 모입니다**

### 🎨 바뀜

- 새 화면 — 단계 카드(① 파일 ② 형식 ③ 저장 위치), 큰 형식 선택, 창 전체에 끌어다 놓기, 늘 보이는 변환 단추
- 밝게/어둡게 테마 자동, 맑은 고딕 · 굵은 글씨, 앱 · exe 아이콘, 만든이 표시
- 저장 창을 띄우지 않고 고른 폴더에 바로 저장합니다

변환 결과(HLT · HWPX 내용)는 v0.1.5 와 같습니다. 전체 기록은 [CHANGELOG.md](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/blob/main/CHANGELOG.md)에 있습니다.

## 📥 받기 — Windows 10/11 64비트

| 파일 | 안에 든 것 | 이럴 때 |
|---|---|---|
| **`docx2hwpx-hlt-setup-x64.exe`** | 설치 파일(화면 앱 + CLI) | ⭐ 대부분 이것 |
| `docx2hwpx-hlt-windows-x64.zip` | 화면 앱 `docx2hwpx-hlt-gui.exe` | 설치 없이 바로(휴대용) |
| `docx2hwpx-hlt-cli-windows-x64.zip` | CLI `docx2hwpx-hlt.exe` | 자동화 · PCT 서식(`--pct`) |

> [!TIP]
> 처음 실행할 때 Windows SmartScreen이 **「알 수 없는 게시자」** 라고 막으면 **추가 정보 → 실행**을 누릅니다.

| 출력 | 여는 곳 |
|---|---|
| `.hlt` | NK-Editor (통합명세서작성기) — 국내 출원 · PCT 국내진입 서식 |
| `.hwpx` | MOIP-Editor — 지식재산처가 고친 명세서 서식 |

> [!WARNING]
> HLT · HWPX는 공개 사양이 없는 형식입니다. 만든 파일은 **반드시 NK-Editor · MOIP-Editor에서 열어 확인한 뒤** 제출하세요.
