# 📜 변경 기록

이 프로젝트의 눈에 띄는 변경은 모두 이 파일에 적습니다.

형식은 [Keep a Changelog](https://keepachangelog.com/ko/1.1.0/)를, 판 번호는 [유의적 버전(SemVer)](https://semver.org/lang/ko/)을 따릅니다.
분류: ✨ **추가** · 🔧 **바뀜** · 🐛 **고침** · 🗑️ **없앰** · 🔒 **보안**

## [Unreleased]

## [0.2.0] — 2026-11-20

> **설치 파일**과 **일괄 변환** 화면 앱을 낸 판입니다. 변환 결과는 0.1.5와 같습니다.

### ✨ 추가

- **윈도 설치 파일**(Inno Setup) — 관리자 권한 없이 설치, 시작 메뉴 · (선택) 바탕화면 바로가기,
  탐색기 `.docx` 우클릭 「HLT · HWPX 로 변환」, 「보내기」 메뉴, 제거 시 정리. 릴리스에 `docx2hwpx-hlt-setup-x64.exe` 로 붙습니다.
- 화면 앱 **일괄 변환** — 여러 파일 · 폴더를 놓고 한 번에 변환, 파일마다 상태 · 보고, 「멈추기」.
- 화면 앱 **저장 위치**(원본 폴더 / 다른 폴더)와 덮어쓰기 선택. 같은 이름이 있으면 `이름 (2).hlt` 로 만듭니다.
- 우클릭 · 보내기로 여러 파일을 열면 이미 열린 창 하나로 모읍니다.
- 앱 · exe 아이콘과 버전 정보, 화면 앱에 만든이 표시.

### 🔧 바뀜

- 화면 앱을 단계 카드 화면으로 새로 짰습니다. 밝게/어둡게 테마를 따르고, 맑은 고딕 · 굵은 글씨를 씁니다.
- 화면 앱이 저장 창 대신 고른 폴더에 바로 저장합니다.
- README를 배너 · 스크린샷 · 자주 묻는 질문 · 영어 요약으로 다시 꾸미고, 변경 기록 · 기여 안내 · 이슈/PR 양식을 더했습니다.

## [0.1.5] — 2026-11-05

> 표 · 그림이 많은 명세서가 HLT 변환 중 **문서 전체가 멈추던 문제**를 고친 판입니다.

### 🐛 고침

- **HLT 표 구조** — 표가 많은 명세서가 변환 87%에서 `geometry_invalid (docx_table)`로 실패하던 문제.
  칸이 모자라거나 남는 줄, 줄 앞뒤 빈 자리, 표 밖으로 넘친 칸 합치기, 위가 없는 세로 병합,
  소수 · 빠진 열 너비, 콘텐츠 컨트롤로 감싼 칸 등 23가지 표 구조를 고쳐 넣습니다.
- **HLT 그림 관계** — `structure_invalid (docx_relationships · docx_document)`로 실패하던 문제.
  - SVG 그림은 PNG로 바꿉니다(PNG 대체 그림이 함께 있어도 실패했습니다).
  - 파일이 없거나 · 깨졌거나 · 읽을 수 없거나 · 「파일에 연결」로만 넣은 그림은 「그림 없음」 자리표를 넣습니다.
- **HLT 64MB 한도** — `size_limit_exceeded`로 실패하던 큰 문서는 그림 해상도를 200 → 150 → 100 → 72dpi로 낮춰 맞춥니다.
- 행이 없는 빈 표는 HLT · HWPX 모두 지웁니다.

### ✨ 추가

- 경고 `table_repaired`(「표 n: …」), `images_downscaled`.

### 🔧 바뀜

- 경고 · 오류 문구를 **어디서 — 무엇이 — 어떻게** 순서의 사용자 말로 다듬었습니다.
  위치는 `【0012】 부근 그림`, `【0015】 부근 표(문서의 3번째 표)`처럼 식별 번호로 알려 주고,
  개발자용 내용은 맨 뒤 「기술 정보」에 둡니다.
- `image_missing` 경고의 뜻을 넓혔습니다(없는 파일 · 깨진 그림 · 링크만 있는 그림).

## [0.1.4] — 2026-11-04

### ✨ 추가

- **Word 그룹 도형 · 벡터 도형을 그림 한 장으로** — 「도형으로 변환」한 ChemDraw 반응식 · 메커니즘이
  구조식째 빠지던 문제를 해결했습니다. 그룹 안 그림 · 경로 · 기본 도형 · 화살표 · 글상자를 300dpi PNG로 굽습니다.
- JSON `shapes`, 경고 `shape_partial` · `shape_failed`.

## [0.1.3] — 2026-11-04

### 🐛 고침

- **기호 글꼴** — Symbol · Wingdings · MT Extra 글꼴로 쓴 글(α · × · ® · ™ · μ · ✓ …)이 HLT에서 깨지던 문제.
  대응표로 진짜 유니코드 기호로 바꾸고, 스타일(basedOn 포함)로만 받은 기호 글꼴도 따라갑니다.
- 줄 안 나뉘는 하이픈(`w:noBreakHyphen`)이 사라져 `1‑2`가 `12`가 되던 문제.

### 🔧 바뀜

- HLT: 풀어쓴 한글(NFD)은 완성형으로 합치고, 옛한글은 음절 하나를 그림 하나로 넣습니다.
- HLT: `≈` → `≒`, `・ ･ ‧ ⋅ ⸱` → `·`, `⁄ ∕` → `/`.
- 기호 글자 그림은 흑백 기호 글꼴(Segoe UI Symbol 등)로 그립니다(컬러 이모지로 그려지지 않게).

## [0.1.2] — 2026-11-04

### ✨ 추가

- [MIT 라이선스](LICENSE)와 SignPath 코드 서명 준비([CODE_SIGNING.md](CODE_SIGNING.md)).
- 릴리스 zip에 `LICENSE`를 넣습니다.

### 🔧 바뀜

- 저장소 새 이름(`DOCX_2_HWPX_OR_HLT`)에 맞춰 링크와 제목을 고쳤습니다.

## [0.1.1] — 2026-11-04

> 태그 없이 판 번호만 올렸습니다. 변환 결과는 0.1.0과 같습니다.

### 🔧 바뀜

- 화면 앱 zip과 명령줄 zip(`docx2hwpx-hlt-cli-windows-x64.zip`)을 따로 냅니다.
- 릴리스 빌드를 최적화(`lto = "fat"`, `codegen-units = 1`, `strip`)해 내려받을 크기를 줄였습니다.
- README를 사용자 중심으로 다시 썼습니다.

## [0.1.0] — 2026-11-04

첫 공개 판입니다.

### ✨ 추가

- **DOCX → HLT(NK-Editor) · HWPX(MOIP-Editor)** 변환 명령줄 도구와 화면 앱(끌어다 놓기 · 게이지 · 다른 이름으로 저장).
- 수식(OMML · 옛 OLE 수식)을 그림으로, WMF/EMF를 PNG로.
- 변경 추적은 모두 받아들이고, 메모는 모두 지운 뒤 변환.
- HLT: CP949 밖 글자 처리 — 첨자 서식 · 같은 모양 글자 · 글자 그림.
- HLT 그림 다듬기(`--image-dpi`), 수식 최소 14pt.
- HWPX를 지식재산처 명세서 서식(A4 · 바탕체 · 줄 간격 280%)에 맞춤.
- `--json` 결과 · 고정 실패/경고 코드 · `--overwrite` · [AGENTS.md](AGENTS.md).
- `v*` 태그로 윈도 빌드와 GitHub 릴리스를 자동으로 만드는 워크플로.

[Unreleased]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.0...v0.1.2
[0.1.1]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/compare/v0.1.0...v0.1.2
[0.1.0]: https://github.com/lmg6983-dotcom/DOCX_2_HWPX_OR_HLT/releases/tag/v0.1.0
