<div align="center">

<img src="docs/images/banner.png" alt="DOCX_2_HWPX_OR_HLT — Word 특허 명세서를 특허청 전자출원 편집기 파일로" width="100%">

<br>

**Word로 쓴 특허 명세서(`.docx`)를 NK-Editor(`.hlt`) · MOIP-Editor(`.hwpx`) 파일로 한 번에 바꿉니다.**<br>
수식 · 표 · 그림 · 도형 · 변경 추적 · 특수 문자까지 — 손으로 다시 옮겨 적던 시간을 없애 줍니다.

<br>

[![Release](https://img.shields.io/github/v/release/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT?style=for-the-badge&logo=github&label=%EC%B5%9C%EC%8B%A0%20%EB%B2%84%EC%A0%84&color=2563eb)](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/total?style=for-the-badge&logo=windows&label=%EB%8B%A4%EC%9A%B4%EB%A1%9C%EB%93%9C&color=10b981)](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/windows.yml?branch=main&style=for-the-badge&logo=githubactions&logoColor=white&label=%EB%B9%8C%EB%93%9C)](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/actions/workflows/windows.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-f59e0b?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-000?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

[**⬇️ 설치 파일 받기**](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest/download/docx2hwpx-hlt-setup-x64.exe) &nbsp;·&nbsp;
[쓰는 법](#-30초-만에-시작하기) &nbsp;·&nbsp;
[자동으로 처리하는 것](#-자동으로-처리하는-것) &nbsp;·&nbsp;
[자주 묻는 질문](#-자주-묻는-질문) &nbsp;·&nbsp;
[CLI](#%EF%B8%8F-cli) &nbsp;·&nbsp;
[변경 기록](CHANGELOG.md) &nbsp;·&nbsp;
[English](#-english)

<br>

<img src="docs/images/app-light.png" alt="화면 앱 — 여러 명세서를 한꺼번에 변환한 모습" width="49%">
<img src="docs/images/app-dark.png" alt="화면 앱 — 어두운 테마" width="49%">

</div>

<br>

| 만드는 파일 | 여는 프로그램 | 서식 |
|:---:|---|---|
| **`.hlt`** | NK-Editor (통합명세서작성기) | 국내 출원 · PCT 국내진입 |
| **`.hwpx`** | MOIP-Editor | 지식재산처 명세서 서식 (A4 · 바탕체 12pt · 줄 간격 280%) |

> [!IMPORTANT]
> HLT · HWPX 는 **공개 사양이 없는 형식**입니다. 바꾼 파일은 반드시 **NK-Editor 나 MOIP-Editor 에서 열어 확인한 뒤** 제출하세요.
> 앱이 「편집기에서 확인할 곳」을 `【0012】 부근 그림`처럼 식별 번호로 짚어 줍니다.

## ✨ 왜 쓰나요?

| 😩 지금까지 | 🚀 DOCX_2_HWPX_OR_HLT |
|---|---|
| 명세서를 편집기에 문단마다 복사 · 붙여넣기 | 파일을 **끌어다 놓고 버튼 한 번** |
| 수식을 편집기 수식 도구로 다시 입력 | 워드 수식(OMML · 옛 수식 편집기)을 **그대로 그림으로** |
| `x²`, `–`, `ü` 같은 글자가 저장 안 돼서 하나씩 찾아 고침 | CP949 밖 글자를 **자동으로 첨자 서식 · 비슷한 글자 · 글자 그림으로** |
| 편집기가 못 읽는 표 · 도형 때문에 변환이 멈춤 | **23가지 표 구조 · 도형 그룹 · SVG 를 고쳐 넣고** 어디를 고쳤는지 알려 줌 |
| 변경 추적 · 메모를 지우는 걸 깜빡 | **변경은 모두 반영, 메모는 모두 삭제** 후 변환 |
| 파일 하나씩 | **여러 파일 · 폴더를 한꺼번에** 일괄 변환 |

- 🔒 **완전 오프라인** — 명세서가 컴퓨터 밖으로 나가지 않습니다. 네트워크를 전혀 쓰지 않습니다.
- 🆓 **무료 · 오픈소스(MIT)** · 회원 가입 · 인증 없음
- ⚡ **빠름** — Rust 로 만든 단일 실행 파일, 수십 쪽 명세서도 몇 초
- 🤖 **자동화 친화적인 CLI** — `--json` 한 줄 출력 · 고정된 오류/경고 코드 ([AGENTS.md](AGENTS.md))

## 📦 내려받기

> Windows 10 / 11 (64비트)

| 파일 | 이런 분께 | |
|---|---|---|
| 🧩 **설치 파일** `docx2hwpx-hlt-setup-x64.exe` | **대부분은 이것.** 시작 메뉴 · 탐색기 우클릭 메뉴까지 한 번에 | [**받기**](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest/download/docx2hwpx-hlt-setup-x64.exe) |
| 🗂️ 휴대용 zip `docx2hwpx-hlt-windows-x64.zip` | 설치 없이 USB · 공용 PC 에서 바로 | [받기](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest/download/docx2hwpx-hlt-windows-x64.zip) |
| ⌨️ CLI zip `docx2hwpx-hlt-cli-windows-x64.zip` | 자동화 · 스크립트 · PCT 국내진입 서식 | [받기](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest/download/docx2hwpx-hlt-cli-windows-x64.zip) |

<details>
<summary><b>설치 프로그램이 해 주는 것</b></summary>

<br>

- **관리자 권한 없이** 내 사용자 폴더에 설치됩니다(설치 창에서 「모든 사용자」를 고르면 Program Files).
- 시작 메뉴에 **특허 명세서 변환** 바로가기
- (선택) 바탕화면 바로가기
- 탐색기에서 `.docx` 를 우클릭 → **HLT · HWPX 로 변환** (Windows 11 은 「더 많은 옵션 표시」 안에 있습니다)
- 탐색기 **보내기** 메뉴 — 여러 파일을 골라 한 번에 보낼 때 편합니다
- 제어판 「앱」에서 깔끔하게 제거됩니다(메뉴 · 바로가기 함께 정리)

워드의 `.docx` 기본 연결 프로그램은 **건드리지 않습니다.**

</details>

> [!TIP]
> 처음 실행할 때 Windows 가 「Windows의 PC 보호」 창을 띄우면 **추가 정보 → 실행** 을 누르세요.
> 이 저장소의 GitHub Actions 가 소스에서 바로 빌드한 파일입니다. 코드 서명 계획은 [CODE_SIGNING.md](CODE_SIGNING.md)에 있습니다.

## 🚀 30초 만에 시작하기

```
 ① 파일 놓기          ② 형식 고르기            ③ 변환하기
┌────────────┐     ┌──────┐ ┌───────┐     ┌──────────────┐
│  📄 .docx   │ ──▶ │ HLT  │ │ HWPX  │ ──▶ │  3개 변환하기  │ ──▶  ✅ 같은 폴더에 저장
└────────────┘     └──────┘ └───────┘     └──────────────┘
```

1. **파일 놓기** — `.docx` 를 창에 끌어다 놓습니다. 여러 개 · 폴더째 놓아도 됩니다. (눌러서 고르기 · `Ctrl+O`)
2. **형식 고르기** — **HLT**(NK-Editor · 통합명세서작성기) 또는 **HWPX**(MOIP-Editor)
3. **변환하기** — `Enter` 또는 큰 단추. 끝나면 원본 옆에 `명세서.hlt` · `명세서.hwpx` 가 생깁니다.

파일마다 결과가 표시됩니다.

| 표시 | 뜻 |
|:---:|---|
| 🟢 | 변환 완료 |
| 🟠 | 변환 완료, **편집기에서 확인할 곳**이 있음 — 「자세히」에 목록 |
| 🔴 | 실패 — 이유와 해결 방법을 보여 줍니다 |

> 같은 이름의 파일이 이미 있으면 덮어쓰지 않고 `명세서 (2).hlt` 로 새로 만듭니다. 「덮어쓰기」를 켜면 바꿔 넣으며, 변환이 실패해도 원래 파일은 남습니다.

## 🪄 자동으로 처리하는 것

| | 항목 | 하는 일 |
|:---:|---|---|
| 🧮 | **수식** | 워드 수식(옛 수식 편집기 개체 포함)을 최소 14pt의 깔끔한 그림으로 넣습니다. |
| 📝 | **변경 내용 추적 · 메모** | 추적한 변경은 모두 반영(수락)하고, 메모는 모두 지운 뒤 바꿉니다. |
| 🖼️ | **표와 그림** | 셀을 합친 표, WMF/EMF 그림을 옮깁니다. ChemDraw 개체는 미리보기 그림으로 넣습니다. HLT 에서는 그림을 300dpi 상한으로 다듬어 파일을 줄입니다. |
| 🔷 | **Word 도형 · 그룹** | 그림 · 선 · 화살표 · 글상자를 묶은 그룹(「도형으로 변환」한 ChemDraw 반응식 등)을 원래 배치대로 그림 한 장으로 바꿉니다. |
| Ω | **기호 글꼴** | Symbol · Wingdings · MT Extra 글꼴로 쓴 글(화면에는 α · × · ® · ™ · ✓, 실제로는 `a` · `´` · `Ò`)을 진짜 기호로 바꿉니다. 스타일로 지정한 글꼴도 따라가고, 줄 안 나뉘는 하이픈(Ctrl+Shift+-)도 `-`로 살립니다. |
| 🧩 | **표 구조** | 칸이 모자란 줄, 줄 앞뒤 빈 자리, 표 밖으로 넘친 칸 합치기, 위가 없는 세로 병합처럼 편집기 형식에 맞지 않는 표를 고쳐 넣고 몇 번째 표인지 알려 줍니다. |
| 🩹 | **그림 문제** | SVG는 PNG로 바꿉니다. 파일이 없거나, 읽을 수 없거나, 「파일에 연결」로만 넣은 그림은 「그림 없음」 자리표를 넣고 알려 줍니다. |
| 📦 | **큰 문서** | HLT 변환 한도(64MB)를 넘으면 그림 해상도를 200 → 150 → 100 → 72dpi로 낮춰 맞추고 알려 줍니다. |
| 📐 | **HWPX 서식** | 지식재산처가 정한 명세서 서식(A4, 바탕체 12pt, 줄 간격 280% 등)에 맞춥니다. |

<details>
<summary><b>🔤 HLT에서 쓸 수 없는 글자는 이렇게 바꿉니다</b> (펼치기)</summary>

<br>

NK-Editor는 완성형 한글(**CP949**)에 있는 글자만 저장할 수 있습니다.

| 원래 글자 | 예 | 바꾸는 방법 |
|---|---|---|
| 첨자 모양 글자 | `x²`, `H₂O` | 보통 글자에 위/아래첨자 서식을 붙입니다 |
| 모양이 비슷한 글자 | `–`, `−`, `≈`, `・`, `⋅` | 저장할 수 있는 글자로 바꿉니다 (`-`, `≒`, `·`) |
| 바꿀 수 없는 글자 | `x̂`, `ü` | 글자 모양 그대로 그림으로 넣습니다 *(편집기에서 글자로 고칠 수는 없습니다)* |

</details>

## ⚠️ 알아 둘 점

- 그룹 밖 글상자 안의 글은 옮기지 않습니다(그룹 안 글상자는 그림에 함께 그려집니다).
- 워드 **스타일에만** 정해 둔 서식(글꼴 · 크기 등)은 옮겨지지 않을 수 있습니다. 크기가 없는 글은 12pt 로 처리합니다.
- HLT 는 표 한 칸에 문단을 하나만 넣을 수 있어, 칸 안의 여러 문단은 하나로 합치고 칸 안의 표는 글로 풉니다.
- 화면 앱은 HLT 를 **국내 출원 서식**으로 만듭니다. **PCT 국내진입 서식**은 [CLI](#%EF%B8%8F-cli)의 `--pct` 로 만듭니다.

## ❓ 자주 묻는 질문

<details>
<summary><b>명세서가 외부 서버로 전송되나요?</b></summary>
<br>
아니요. 변환은 전부 내 컴퓨터 안에서 일어나며 프로그램은 네트워크를 전혀 쓰지 않습니다. 인터넷이 끊긴 PC 에서도 동작합니다.
</details>

<details>
<summary><b><code>.doc</code> 파일은 안 되나요?</b></summary>
<br>
<code>.docx</code> 만 읽습니다. Word 에서 열어 <b>다른 이름으로 저장 → Word 문서(*.docx)</b> 로 저장한 뒤 변환하세요.
</details>

<details>
<summary><b>「편집기에서 확인할 곳」은 무엇인가요?</b></summary>
<br>
변환은 됐지만 사람이 눈으로 봐야 하는 곳입니다. 예: 고쳐 넣은 표, 「그림 없음」 자리표, 그리지 못해 글로 남은 수식, 글자 그림으로 넣은 특수 문자(편집기에서 글자로 고칠 수 없음) 등. 파일 줄의 「자세히」를 누르면 <code>【0012】 부근 그림</code>처럼 위치와 함께 목록이 나옵니다.
</details>

<details>
<summary><b>우클릭 메뉴가 Windows 11 에서 안 보여요.</b></summary>
<br>
Windows 11 은 새 메뉴에 등록되지 않은 항목을 <b>「더 많은 옵션 표시」(Shift+F10)</b> 안에 보여 줍니다. 여러 파일을 골라 우클릭하거나 <b>보내기 → 특허 명세서 변환</b> 을 쓰면 한 창에 모입니다.
</details>

<details>
<summary><b>변환이 실패했어요. 어떻게 알려 주면 되나요?</b></summary>
<br>
실패한 파일을 펼치면 맨 아래에 「기술 정보」가 있습니다. 그 내용과 함께 <a href="https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/issues/new/choose">이슈</a>로 알려 주세요. <b>실제 출원 전 명세서는 올리지 말고</b>, 문제를 재현하는 최소한의 예시 문서나 내용을 지운 문서를 올려 주세요.
</details>

<details>
<summary><b>수식 안 한글 글꼴이 이상해요.</b></summary>
<br>
수식 안의 한글과 글자 그림은 시스템 글꼴을 씁니다. CLI 에서 <code>--font-dir 폴더</code> 로 글꼴을 더 지정할 수 있습니다.
</details>

---

## ⌨️ CLI

```bash
docx2hwpx-hlt 명세서.docx --to hlt                       # → 명세서.hlt (국내 출원 서식)
docx2hwpx-hlt 명세서.docx --to hwpx                      # → 명세서.hwpx
docx2hwpx-hlt 명세서.docx --to hlt --pct -o 번역문.hlt   # PCT 국내진입 서식
docx2hwpx-hlt 명세서.docx --to hlt --json                # 결과를 JSON 한 줄로
```

<details>
<summary><b>옵션 전체 보기</b></summary>

<br>

| 옵션 | 뜻 |
|---|---|
| `-t`, `--to <hwpx\|hlt>` | 만들 형식 **(필수)** |
| `-o`, `--output <파일>` | 출력 경로. 없으면 입력 이름에 확장자만 바꿉니다 |
| `--pct` | HLT를 PCT 국내진입 서식으로 (`--to hlt`에만) |
| `--image-dpi <N>` | HLT 그림 해상도 상한 (기본 300, `0`이면 끔) |
| `--overwrite` | 출력이 이미 있으면 덮어씁니다. 끝까지 쓴 뒤 바꿔 넣으므로 실패해도 원래 파일이 남습니다 |
| `--json` | 결과를 표준 출력에 JSON 한 줄로 |
| `--font-dir <폴더>` | 수식 안 한글 · 글자 그림에 쓸 글꼴 폴더 (여러 번 줄 수 있음) |
| `-h`, `--help` · `-V`, `--version` | 도움말 · 판 번호 |

</details>

<details>
<summary><b>폴더 안 모든 파일 한꺼번에 (PowerShell)</b></summary>

```powershell
Get-ChildItem *.docx | ForEach-Object { .\docx2hwpx-hlt.exe $_.FullName --to hlt --overwrite }
```

</details>

| 종료 코드 | 뜻 |
|:---:|---|
| `0` | 성공 (경고가 있어도 0) |
| `1` | 변환 실패 — JSON의 `error.code`를 보세요 |
| `2` | 인자가 잘못됨 |

> [!NOTE]
> 자동화나 AI 에이전트로 쓸 때의 JSON 필드 · 오류 코드 · 경고 코드는 [AGENTS.md](AGENTS.md)에 계약으로 정리돼 있습니다.

## 🏗️ 직접 빌드하기

[Rust](https://rustup.rs) (stable, 2024 edition)가 필요합니다.

```bash
cargo build --release                                          # CLI
cargo build --release --features gui --bin docx2hwpx-hlt-gui   # 화면 앱
cargo test                                                     # 시험
```

<details>
<summary><b>📦 설치 파일 만들기 (Windows · Inno Setup 6)</b></summary>

```powershell
cargo build --release --locked --features gui --bins
mkdir exe; copy target\release\*.exe exe\
iscc /DAppVersion=0.2.0 installer\docx2hwpx-hlt.iss   # → dist\docx2hwpx-hlt-setup-x64.exe
```

</details>

<details>
<summary><b>🗺️ 변환이 흘러가는 길</b></summary>

<br>

```mermaid
flowchart LR
    A[📄 .docx] --> B[normalize<br/>변경 추적 · 메모 · 표 · 그림 정리]
    B --> C[omml · render<br/>수식 → Typst → PNG]
    B --> D[shapes<br/>도형 · 그룹 → SVG → PNG]
    B --> E[chars<br/>CP949 글자 규칙]
    C & D & E --> F{형식}
    F -->|HLT| G[hlt-core<br/>📘 .hlt]
    F -->|HWPX| H[HwpForge<br/>📗 .hwpx]
```

| 파일 | 하는 일 |
|---|---|
| `src/lib.rs` | 변환 핵심 `convert` (바이트 → 바이트 + 보고 · 진행 알림) |
| `src/main.rs` · `src/bin/gui.rs` | CLI · 화면 앱(egui) |
| `src/normalize.rs` | DOCX 고쳐 쓰기 — 수식, WMF/EMF, 변경 추적, 칸 합치기, 글자 규칙 |
| `src/tables.rs` · `src/shapes.rs` | 표 구조 바로잡기 · Word 도형을 그림으로 |
| `src/hwpx.rs` | 고쳐 쓴 DOCX → HWPX |
| `installer/` · `assets/` · `build.rs` | 설치 파일 · 아이콘 · exe 버전 정보 |

</details>

**릴리스:** `v*` 태그를 올리거나 Actions의 [`windows`](.github/workflows/windows.yml) 워크플로를 수동으로 실행하면서 `release`에 버전(예: `v0.2.1`)을 넣으면 설치 파일 · 휴대용 zip · CLI zip 이 붙은 릴리스가 자동으로 만들어집니다.
자세한 순서와 커밋 규칙은 [CONTRIBUTING.md](CONTRIBUTING.md)를 보세요.

## 🤝 기여하기

버그 제보와 제안을 환영합니다. [이슈 열기](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/issues/new/choose)

> [!CAUTION]
> **실제 출원 전 명세서는 이슈에 올리지 마세요.** 문제를 재현하는 최소한의 예시 문서나, 내용을 지운 문서를 올려 주세요.

## 🙏 바탕이 된 오픈소스

| 이름 | 쓰는 곳 | 라이선스 |
|---|---|---|
| [seoan-editor](https://github.com/tornado1014/seoan-editor) `hlt-core` | DOCX 읽기, HLT 쓰기 | MIT |
| [HwpForge](https://github.com/ai-screams/HwpForge) | HWPX 쓰기 | MIT / Apache-2.0 |
| [Typst](https://github.com/typst/typst) | 수식, 글자 그림 | Apache-2.0 |
| [wmf-rs](https://github.com/mythrnr/wmf-rs), emf-core, [resvg](https://github.com/linebender/resvg) | WMF/EMF · SVG 그림 변환 | MIT, MIT / Apache-2.0 |
| [egui / eframe](https://github.com/emilk/egui) | 화면 앱 | MIT / Apache-2.0 |
| [Inno Setup](https://jrsoftware.org/isinfo.php) | 설치 파일 | Inno Setup License |

## 📄 라이선스

[MIT](LICENSE) © 2026 lmg6983-dotcom

---

## 🌐 English

**DOCX_2_HWPX_OR_HLT** converts Microsoft Word patent specifications (`.docx`) into the two file formats used by the Korean Intellectual Property Office e-filing editors:

| Output | Editor |
|---|---|
| `.hlt` | **NK-Editor** (통합명세서작성기) — domestic & PCT national-phase layouts |
| `.hwpx` | **MOIP-Editor** — the official specification template |

**Highlights**

- Drag & drop **batch conversion** of many files or whole folders, with per-file status and a "things to check" list that points to paragraph numbers like `【0012】`
- Word equations (OMML and legacy OLE Equation objects) → crisp images; WMF/EMF, SVG and grouped Word shapes → PNG
- Repairs 23 kinds of table structures the editors can't read; tracked changes accepted and comments removed automatically
- For HLT (CP949 only): superscript/subscript look-alikes become real script formatting, look-alike characters are substituted, symbol fonts are mapped, anything else is embedded as a glyph image
- 100 % offline, single native binary (Rust), light & dark theme, MIT licensed
- Windows installer (no admin rights needed) with Start-menu entry, optional desktop shortcut, `.docx` right-click verb and *Send to* entry

**Download:** grab [`docx2hwpx-hlt-setup-x64.exe`](https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/latest/download/docx2hwpx-hlt-setup-x64.exe) from the latest release. A portable zip and a CLI build (`--json` output for automation, see [AGENTS.md](AGENTS.md)) are attached as well.

> Both formats have no public specification. Always open the result in NK-Editor / MOIP-Editor and check it before filing.

<div align="center">
<br>
<sub>특허 실무자의 반복 작업을 덜기 위해 만들었습니다 · 문제가 있으면 <a href="https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/issues">이슈</a>로 알려 주세요</sub>
</div>
