//! 윈도 exe 에 아이콘 · 버전 정보를 넣는다(탐색기 · 작업 표시줄 · 설치 프로그램 목록에 보인다).
//! 다른 OS 로 빌드할 때는 아무것도 하지 않는다.
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/icon.ico")
        .set("ProductName", "DOCX_2_HWPX_OR_HLT")
        .set("FileDescription", "특허 명세서 변환 (DOCX → HLT · HWPX)")
        .set("CompanyName", "lmg6983-dotcom")
        .set("LegalCopyright", "© 2026 lmg6983-dotcom (MIT)");
    if let Err(e) = res.compile() {
        // 리소스 컴파일러가 없어도 빌드는 이어 간다(아이콘만 빠진다).
        println!("cargo:warning=exe 아이콘을 넣지 못했습니다: {e}");
    }
}
