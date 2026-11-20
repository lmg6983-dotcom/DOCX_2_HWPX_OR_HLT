//! 화면 앱: DOCX 여러 개 고르기(끌어다 놓기 · 누르기 · Ctrl+O · exe/우클릭 메뉴/보내기) →
//! HLT/HWPX 고르기 → 저장 위치 고르기 → 일괄 변환(파일마다 상태 · 전체 게이지).
//!
//! 만든 파일은 원본과 같은 폴더(또는 고른 폴더)에 `이름.hlt` · `이름.hwpx` 로 쓴다.
//! 같은 이름이 있으면 덮어쓰지 않고 `이름 (2).hlt` 로 새로 만든다(덮어쓰기를 켜면 바꿔 넣는다).
#![cfg_attr(windows, windows_subsystem = "windows")]

use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::time::Duration;

use docx2hwpx_hlt::{Failure, Target};
use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, FontFamily, FontId, Frame, Id, Key,
    KeyboardShortcut, Layout, Margin, Modifiers, Pos2, Rect, RichText, Sense, Stroke, StrokeKind,
    TextureHandle, Ui, Vec2,
};
use serde_json::Value;

const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");
const APP_TITLE: &str = "특허 명세서 변환";
const AUTHOR: &str = "lmg6983@gmail.com";

// ───────────────────────────── 색 ─────────────────────────────

/// 밝은 · 어두운 테마 색. 토스(Toss) 앱처럼 회색 바탕 위 테두리 없는 흰 카드, 파랑 하나로 강조한다.
/// 본문 대비 4.5:1 이상을 맞췄다(큰 단추 글자는 3:1).
#[derive(Clone, Copy)]
struct Pal {
    /// 창 바탕(grey100)
    bg: Color32,
    /// 카드
    card: Color32,
    /// 카드 안 옅은 바탕(grey50) — 펼친 줄 · 놓는 곳
    sunken: Color32,
    /// 마우스를 올린 줄 · 회색 단추 바탕(grey100)
    hover: Color32,
    ink: Color32,
    /// 회색 단추 글자 · 둘째 글(grey700)
    ink2: Color32,
    muted: Color32,
    line: Color32,
    /// 꺼진 체크 원(grey300)
    outline: Color32,
    accent: Color32,
    accent_ink: Color32,
    /// 고른 것 · 파랑 옅은 바탕
    accent_soft: Color32,
    /// accent_soft 위 글자
    accent_deep: Color32,
    ok: Color32,
    ok_soft: Color32,
    warn: Color32,
    warn_soft: Color32,
    err: Color32,
    err_soft: Color32,
}

const LIGHT: Pal = Pal {
    bg: Color32::from_rgb(0xf2, 0xf4, 0xf6),
    card: Color32::WHITE,
    sunken: Color32::from_rgb(0xf9, 0xfa, 0xfb),
    hover: Color32::from_rgb(0xf2, 0xf4, 0xf6),
    ink: Color32::from_rgb(0x19, 0x1f, 0x28),
    ink2: Color32::from_rgb(0x4e, 0x59, 0x68),
    muted: Color32::from_rgb(0x6b, 0x76, 0x84),
    line: Color32::from_rgb(0xe5, 0xe8, 0xeb),
    outline: Color32::from_rgb(0xd1, 0xd6, 0xdb),
    accent: Color32::from_rgb(0x31, 0x82, 0xf6),
    accent_ink: Color32::WHITE,
    accent_soft: Color32::from_rgb(0xe8, 0xf3, 0xff),
    accent_deep: Color32::from_rgb(0x1b, 0x64, 0xda),
    ok: Color32::from_rgb(0x0b, 0x7a, 0x55),
    ok_soft: Color32::from_rgb(0xe5, 0xf8, 0xef),
    warn: Color32::from_rgb(0xa6, 0x53, 0x00),
    warn_soft: Color32::from_rgb(0xff, 0xf3, 0xe0),
    err: Color32::from_rgb(0xd2, 0x20, 0x30),
    err_soft: Color32::from_rgb(0xff, 0xee, 0xee),
};

const DARK: Pal = Pal {
    bg: Color32::from_rgb(0x17, 0x17, 0x1c),
    card: Color32::from_rgb(0x20, 0x20, 0x27),
    sunken: Color32::from_rgb(0x26, 0x26, 0x2e),
    hover: Color32::from_rgb(0x2c, 0x2c, 0x35),
    ink: Color32::from_rgb(0xf2, 0xf2, 0xf4),
    ink2: Color32::from_rgb(0xc3, 0xc3, 0xc6),
    muted: Color32::from_rgb(0x9e, 0x9e, 0xa4),
    line: Color32::from_rgb(0x34, 0x34, 0x3e),
    outline: Color32::from_rgb(0x4d, 0x4d, 0x59),
    accent: Color32::from_rgb(0x31, 0x82, 0xf6),
    accent_ink: Color32::WHITE,
    accent_soft: Color32::from_rgb(0x1f, 0x30, 0x4d),
    accent_deep: Color32::from_rgb(0x9d, 0xc2, 0xff),
    ok: Color32::from_rgb(0x3f, 0xd5, 0x99),
    ok_soft: Color32::from_rgb(0x18, 0x3a, 0x2e),
    warn: Color32::from_rgb(0xff, 0xb3, 0x4d),
    warn_soft: Color32::from_rgb(0x3d, 0x2e, 0x14),
    err: Color32::from_rgb(0xff, 0x8a, 0x93),
    err_soft: Color32::from_rgb(0x44, 0x21, 0x2a),
};

fn pal(ui: &Ui) -> Pal {
    if ui.visuals().dark_mode { DARK } else { LIGHT }
}

fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("bold".into()))
}

// ───────────────────────────── 시작 ─────────────────────────────

fn main() -> eframe::Result {
    // exe 에 끌어다 놓기 · 우클릭 메뉴 · 보내기 메뉴로 열면 경로가 인자로 온다(여러 개일 수 있다).
    let inputs: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();

    // 한 창으로 모으기: 우클릭 메뉴에서 여러 파일을 고르면 파일마다 실행된다.
    // 먼저 뜬 창이 받는 쪽이 되고, 나중에 뜬 것은 파일을 넘기고 바로 끝난다.
    let listener = TcpListener::bind(instance_addr()).ok();
    if listener.is_none() && !inputs.is_empty() && hand_off(&inputs) {
        return Ok(());
    }

    let icon = image::load_from_memory(ICON_PNG).map(|i| i.to_rgba8()).ok();
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(format!("{APP_TITLE} (DOCX를 HLT, HWPX로)"))
        .with_inner_size([580.0, 720.0])
        .with_min_inner_size([460.0, 560.0])
        .with_drag_and_drop(true);
    if let Some(i) = &icon {
        viewport = viewport.with_icon(egui::IconData {
            rgba: i.as_raw().clone(),
            width: i.width(),
            height: i.height(),
        });
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "docx2hwpx-hlt",
        options,
        Box::new(move |cc| {
            let ctx = &cc.egui_ctx;
            install_fonts(ctx);
            style(ctx);
            let logo = icon.map(|i| {
                let size = [i.width() as usize, i.height() as usize];
                ctx.load_texture(
                    "logo",
                    egui::ColorImage::from_rgba_unmultiplied(size, i.as_raw()),
                    egui::TextureOptions::LINEAR,
                )
            });
            let (handoff_tx, handoff_rx) = channel();
            if let Some(listener) = listener {
                let ctx = ctx.clone();
                std::thread::spawn(move || serve(listener, handoff_tx, ctx));
            }
            let mut app = App::new(logo, handoff_rx);
            app.add(inputs);
            Ok(Box::new(app))
        }),
    )
}

/// 한글 글꼴을 첫 글꼴로 넣는다. 없으면 eframe 의 시스템 글꼴 대체에 맡긴다.
/// 굵은 글씨용 `bold` 글꼴 묶음도 만든다(굵은 글꼴이 없으면 보통 글꼴).
fn install_fonts(ctx: &egui::Context) {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| r"C:\Windows".into());
    let win = |f: &str| (PathBuf::from(&windir).join("Fonts").join(f), 0u32);
    let regular = [
        win("malgun.ttf"),
        ("/System/Library/Fonts/AppleSDGothicNeo.ttc".into(), 0),
        ("/usr/share/fonts/truetype/nanum/NanumGothic.ttf".into(), 0),
        (
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".into(),
            1,
        ),
        ("/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc".into(), 0),
    ];
    let heavy = [
        win("malgunbd.ttf"),
        (
            "/usr/share/fonts/truetype/nanum/NanumGothicBold.ttf".into(),
            0,
        ),
        (
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc".into(),
            1,
        ),
    ];
    let load = |list: &[(PathBuf, u32)]| {
        list.iter().find_map(|(path, index)| {
            let bytes = std::fs::read(path).ok()?;
            let mut data = egui::FontData::from_owned(bytes);
            data.index = *index;
            Some(data)
        })
    };

    let mut fonts = egui::FontDefinitions::default();
    let base = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let mut proportional = base.clone();
    let mut heavy_list = base;
    if let Some(data) = load(&regular) {
        fonts.font_data.insert("ko".into(), Arc::new(data));
        proportional.insert(0, "ko".into());
        heavy_list.insert(0, "ko".into());
    }
    if let Some(data) = load(&heavy) {
        fonts.font_data.insert("ko-bold".into(), Arc::new(data));
        heavy_list.insert(0, "ko-bold".into());
    }
    fonts
        .families
        .insert(FontFamily::Proportional, proportional);
    fonts
        .families
        .insert(FontFamily::Name("bold".into()), heavy_list);
    ctx.set_fonts(fonts);
}

/// 밝은 · 어두운 테마를 모두 다듬고, 시스템 테마를 따른다.
fn style(ctx: &egui::Context) {
    for (theme, p) in [(egui::Theme::Light, LIGHT), (egui::Theme::Dark, DARK)] {
        let mut v = match theme {
            egui::Theme::Light => egui::Visuals::light(),
            egui::Theme::Dark => egui::Visuals::dark(),
        };
        v.panel_fill = p.bg;
        v.window_fill = p.card;
        v.window_stroke = Stroke::NONE;
        v.window_corner_radius = CornerRadius::same(16);
        v.window_shadow = egui::Shadow {
            offset: [0, 4],
            blur: 16,
            spread: 0,
            color: Color32::from_black_alpha(if p.card == Color32::WHITE { 28 } else { 90 }),
        };
        v.popup_shadow = v.window_shadow;
        v.extreme_bg_color = p.sunken;
        v.faint_bg_color = p.sunken;
        v.hyperlink_color = p.accent;
        v.override_text_color = Some(p.ink);
        v.selection.bg_fill = p.accent_soft;
        v.selection.stroke = Stroke::new(2.0, p.accent_deep);
        v.text_cursor.stroke = Stroke::new(2.0, p.accent);
        // 화면의 단추 · 칸은 모두 손으로 그린다(button · choice). 여기는 그 밖의 기본값.
        let r = CornerRadius::same(10);
        for w in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.corner_radius = r;
            w.expansion = 0.0;
        }
        v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.muted);
        v.widgets.inactive.bg_fill = p.card;
        v.widgets.inactive.weak_bg_fill = p.card;
        v.widgets.inactive.bg_stroke = Stroke::NONE;
        v.widgets.inactive.fg_stroke = Stroke::new(1.0, p.ink2);
        v.widgets.hovered.weak_bg_fill = p.hover;
        v.widgets.hovered.bg_fill = p.hover;
        v.widgets.hovered.bg_stroke = Stroke::NONE;
        v.widgets.hovered.fg_stroke = Stroke::new(1.0, p.ink);
        v.widgets.active.weak_bg_fill = p.accent_soft;
        v.widgets.active.bg_fill = p.accent_soft;
        v.widgets.active.bg_stroke = Stroke::NONE;
        v.widgets.active.fg_stroke = Stroke::new(1.0, p.accent_deep);
        ctx.set_visuals_of(theme, v);
    }
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(8.0, 8.0);
        s.spacing.button_padding = egui::vec2(16.0, 8.0);
        s.spacing.interact_size.y = 32.0;
        s.spacing.scroll = egui::style::ScrollStyle::floating();
        s.text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(15.0));
        s.text_styles.insert(egui::TextStyle::Button, bold(13.5));
        s.text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(12.0));
    });
    // 기본은 시스템 테마. D2H_THEME=light|dark 로 고정할 수 있다.
    ctx.set_theme(match std::env::var("D2H_THEME").as_deref() {
        Ok("dark") => egui::ThemePreference::Dark,
        Ok("light") => egui::ThemePreference::Light,
        _ => egui::ThemePreference::System,
    });
}

// ───────────────────────────── 한 창으로 모으기 ─────────────────────────────

const HANDOFF_MAGIC: &str = "docx2hwpx-hlt/1";

fn instance_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 47631))
}

/// 이미 열린 창에 파일을 넘긴다. 그 창이 `ok` 로 답해야 넘긴 것으로 친다
/// (다른 프로그램이 그 포트를 쓰고 있으면 이 창이 따로 뜬다).
fn hand_off(paths: &[PathBuf]) -> bool {
    let Ok(mut s) = TcpStream::connect_timeout(&instance_addr(), Duration::from_millis(400)) else {
        return false;
    };
    let _ = s.set_read_timeout(Some(Duration::from_millis(1500)));
    let mut msg = format!("{HANDOFF_MAGIC}\n");
    for p in paths {
        let abs = std::path::absolute(p).unwrap_or_else(|_| p.clone());
        msg.push_str(&abs.to_string_lossy());
        msg.push('\n');
    }
    if s.write_all(msg.as_bytes()).is_err() || s.shutdown(Shutdown::Write).is_err() {
        return false;
    }
    let mut reply = String::new();
    s.read_to_string(&mut reply).is_ok() && reply.starts_with("ok")
}

fn serve(listener: TcpListener, tx: Sender<Vec<PathBuf>>, ctx: egui::Context) {
    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        let _ = s.set_read_timeout(Some(Duration::from_millis(1500)));
        let mut text = String::new();
        if s.read_to_string(&mut text).is_err() {
            continue;
        }
        let mut lines = text.lines();
        if lines.next() != Some(HANDOFF_MAGIC) {
            continue;
        }
        let paths: Vec<PathBuf> = lines.filter(|l| !l.is_empty()).map(PathBuf::from).collect();
        let _ = s.write_all(b"ok");
        if tx.send(paths).is_err() {
            return;
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.request_repaint();
    }
}

// ───────────────────────────── 상태 ─────────────────────────────

enum JobState {
    Waiting,
    Running,
    Done { output: PathBuf, report: Value },
    Failed(Failure),
}

struct Job {
    input: PathBuf,
    state: JobState,
    open: bool,
}

impl Job {
    /// 편집기에서 사람이 확인할 곳 수(경고 + 글자 그림).
    fn checks(&self) -> usize {
        match &self.state {
            JobState::Done { report, .. } => {
                let n = |k: &str| report[k].as_array().map_or(0, Vec::len);
                n("warnings") + n("glyph_images")
            }
            _ => 0,
        }
    }
}

enum Msg {
    Start(usize),
    Progress(f32, String),
    Finished(usize, Result<PathBuf, Failure>, Value),
}

struct Batch {
    rx: Receiver<Msg>,
    cancel: Arc<AtomicBool>,
    total: usize,
    finished: usize,
    current: Option<usize>,
    fraction: f32,
    stage: String,
}

struct App {
    jobs: Vec<Job>,
    hlt: bool,
    /// `None` 이면 원본과 같은 폴더.
    out_dir: Option<PathBuf>,
    overwrite: bool,
    batch: Option<Batch>,
    /// 전체 게이지가 그리는 값 — 목표값을 부드럽게 따라간다.
    shown: f32,
    notice: Option<String>,
    logo: Option<TextureHandle>,
    handoff: Receiver<Vec<PathBuf>>,
}

impl App {
    fn new(logo: Option<TextureHandle>, handoff: Receiver<Vec<PathBuf>>) -> Self {
        Self {
            jobs: Vec::new(),
            hlt: true,
            out_dir: None,
            overwrite: false,
            batch: None,
            shown: 0.0,
            notice: None,
            logo,
            handoff,
        }
    }

    fn working(&self) -> bool {
        self.batch.is_some()
    }

    fn target(&self) -> Target {
        if self.hlt {
            Target::Hlt {
                pct: false,
                max_dpi: docx2hwpx_hlt::image_opt::DEFAULT_MAX_DPI,
            }
        } else {
            Target::Hwpx
        }
    }

    fn pending(&self) -> usize {
        self.jobs
            .iter()
            .filter(|j| matches!(j.state, JobState::Waiting | JobState::Failed(_)))
            .count()
    }

    fn add(&mut self, paths: Vec<PathBuf>) {
        if paths.is_empty() {
            return;
        }
        if self.working() {
            self.notice =
                Some("변환 중에는 파일을 더할 수 없습니다. 끝난 뒤 다시 놓아 주세요.".into());
            return;
        }
        let mut rejected = Vec::new();
        for path in paths {
            if path.is_dir() {
                // 폴더를 놓으면 그 안의 .docx(하위 폴더 제외)를 모두 넣는다.
                let mut found: Vec<PathBuf> = std::fs::read_dir(&path)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| is_docx(p))
                    .collect();
                found.sort();
                if found.is_empty() {
                    rejected.push(file_name(&path));
                }
                for p in found {
                    self.push(p);
                }
            } else if is_docx(&path) {
                self.push(path);
            } else {
                rejected.push(file_name(&path));
            }
        }
        self.notice = match rejected.len() {
            0 => None,
            1 => Some(format!(
                "{} 파일은 Word 문서(.docx)가 아니어서 넣지 않았습니다.",
                rejected[0]
            )),
            n => Some(format!(
                "Word 문서(.docx)가 아닌 {n}개는 넣지 않았습니다: {}",
                rejected.join(", ")
            )),
        };
    }

    fn push(&mut self, path: PathBuf) {
        let path = std::path::absolute(&path).unwrap_or(path);
        if self.jobs.iter().any(|j| j.input == path) {
            return;
        }
        self.jobs.push(Job {
            input: path,
            state: JobState::Waiting,
            open: false,
        });
    }

    fn browse(&mut self) {
        if let Some(paths) = rfd::FileDialog::new()
            .set_title("변환할 Word 문서 고르기")
            .add_filter("Word 문서", &["docx"])
            .pick_files()
        {
            self.add(paths);
        }
    }

    fn browse_out_dir(&mut self) {
        let mut dialog = rfd::FileDialog::new().set_title("저장할 폴더 고르기");
        if let Some(dir) = self.out_dir.clone().or_else(|| {
            self.jobs
                .first()
                .and_then(|j| j.input.parent().map(Path::to_path_buf))
        }) {
            dialog = dialog.set_directory(dir);
        }
        if let Some(dir) = dialog.pick_folder() {
            self.out_dir = Some(dir);
        }
    }

    /// 형식을 바꾸면 이전 결과는 맞지 않는다 — 모두 다시 기다림으로.
    fn reset_results(&mut self) {
        for j in &mut self.jobs {
            j.state = JobState::Waiting;
            j.open = false;
        }
        self.shown = 0.0;
    }

    fn start(&mut self, ctx: &egui::Context) {
        let todo: Vec<(usize, PathBuf)> = self
            .jobs
            .iter()
            .enumerate()
            .filter(|(_, j)| matches!(j.state, JobState::Waiting | JobState::Failed(_)))
            .map(|(i, j)| (i, j.input.clone()))
            .collect();
        if todo.is_empty() {
            return;
        }
        for (i, _) in &todo {
            self.jobs[*i].state = JobState::Waiting;
            self.jobs[*i].open = false;
        }
        let (tx, rx) = channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let total = todo.len();
        let target = self.target();
        let out_dir = self.out_dir.clone();
        let overwrite = self.overwrite;
        let stop = cancel.clone();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            run_batch(todo, target, out_dir, overwrite, &stop, &tx, &ctx);
        });
        self.shown = 0.0;
        self.notice = None;
        self.batch = Some(Batch {
            rx,
            cancel,
            total,
            finished: 0,
            current: None,
            fraction: 0.0,
            stage: "준비하는 중".into(),
        });
    }

    fn poll(&mut self) {
        let Some(batch) = &mut self.batch else { return };
        let mut ended = false;
        loop {
            match batch.rx.try_recv() {
                Ok(Msg::Start(i)) => {
                    batch.current = Some(i);
                    batch.fraction = 0.0;
                    batch.stage = "준비하는 중".into();
                    self.jobs[i].state = JobState::Running;
                }
                Ok(Msg::Progress(f, s)) => {
                    batch.fraction = f;
                    batch.stage = s;
                }
                Ok(Msg::Finished(i, result, report)) => {
                    batch.finished += 1;
                    batch.current = None;
                    batch.fraction = 0.0;
                    let single = self.jobs.len() == 1;
                    let job = &mut self.jobs[i];
                    job.state = match result {
                        Ok(output) => JobState::Done { output, report },
                        Err(f) => JobState::Failed(f),
                    };
                    // 실패했거나 파일이 하나뿐이면 펼쳐 둔다(여럿이면 「자세히」로).
                    job.open = matches!(job.state, JobState::Failed(_)) || single;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    ended = true;
                    break;
                }
            }
        }
        if ended {
            // 멈추기를 눌렀거나 변환 스레드가 끝났다 — 「변환 중」에 남은 것은 기다림으로.
            for j in &mut self.jobs {
                if matches!(j.state, JobState::Running) {
                    j.state = JobState::Failed(docx2hwpx_hlt::fail(
                        "convert_failed",
                        docx2hwpx_hlt::messages::with_tech(
                            "변환 중에 예기치 않은 오류로 멈췄습니다. 문서를 다시 저장한 뒤 한 번 더 변환해 보고, 그래도 안 되면 이 메시지와 함께 개발자에게 알려 주세요.",
                            "변환 스레드가 결과 없이 끝남",
                        ),
                    ));
                }
            }
            self.batch = None;
        }
    }

    fn overall(&self) -> f32 {
        match &self.batch {
            Some(b) => {
                let current = if b.current.is_some() { b.fraction } else { 0.0 };
                ((b.finished as f32 + current) / b.total.max(1) as f32).min(1.0)
            }
            None if self
                .jobs
                .iter()
                .any(|j| matches!(j.state, JobState::Done { .. })) =>
            {
                1.0
            }
            None => 0.0,
        }
    }

    fn shortcuts(&mut self, ctx: &egui::Context) {
        let open = KeyboardShortcut::new(Modifiers::COMMAND, Key::O);
        let open = ctx.input_mut(|i| i.consume_shortcut(&open));
        // Enter = 변환. 단추에 초점이 있으면 그 단추가 Enter 를 받으니 겹치지 않게.
        let enter = ctx.memory(|m| m.focused().is_none())
            && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter));
        if open && !self.working() {
            self.browse();
        }
        if enter && !self.working() && self.pending() > 0 {
            self.start(ctx);
        }
    }
}

/// 변환 스레드: 파일을 하나씩 바꿔 저장한다. 한 파일의 panic 은 그 파일의 실패로 바꾼다.
fn run_batch(
    todo: Vec<(usize, PathBuf)>,
    target: Target,
    out_dir: Option<PathBuf>,
    overwrite: bool,
    stop: &AtomicBool,
    tx: &Sender<Msg>,
    ctx: &egui::Context,
) {
    let mut written: HashSet<PathBuf> = HashSet::new();
    for (i, input) in todo {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let _ = tx.send(Msg::Start(i));
        ctx.request_repaint();
        let (result, report) = match std::fs::read(&input) {
            Err(e) => (
                Err(docx2hwpx_hlt::fail(
                    "input_unreadable",
                    docx2hwpx_hlt::messages::input_unreadable(
                        &file_name(&input),
                        &format!("{}: {e}", input.display()),
                    ),
                )),
                Value::Null,
            ),
            Ok(docx) => {
                let mut progress = |fraction: f32, stage: &str| {
                    let _ = tx.send(Msg::Progress(fraction, stage.to_string()));
                    ctx.request_repaint();
                };
                let outcome = catch_unwind(AssertUnwindSafe(|| {
                    docx2hwpx_hlt::convert(&docx, target, &[], &mut progress)
                }));
                match outcome {
                    Ok(o) => (o.result, o.report),
                    Err(_) => (
                        Err(docx2hwpx_hlt::fail(
                            "convert_failed",
                            docx2hwpx_hlt::messages::with_tech(
                                "변환 중에 예기치 않은 오류로 멈췄습니다. 문서를 다시 저장한 뒤 한 번 더 변환해 보고, 그래도 안 되면 이 메시지와 함께 개발자에게 알려 주세요.",
                                "변환 중 panic",
                            ),
                        )),
                        Value::Null,
                    ),
                }
            }
        };
        let result = result.and_then(|bytes| {
            let dir = out_dir
                .clone()
                .or_else(|| input.parent().map(Path::to_path_buf))
                .unwrap_or_default();
            let path = output_path(&dir, &input, target.extension(), overwrite, &written);
            docx2hwpx_hlt::write_atomic(&path, &bytes)?;
            written.insert(path.clone());
            Ok(path)
        });
        let _ = tx.send(Msg::Finished(i, result, report));
        ctx.request_repaint();
    }
}

/// `이름.ext` — 이미 있으면(덮어쓰기를 껐거나, 이번 일괄 변환에서 이미 쓴 이름이면) `이름 (2).ext`, `(3)` …
fn output_path(
    dir: &Path,
    input: &Path,
    ext: &str,
    overwrite: bool,
    written: &HashSet<PathBuf>,
) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let taken = |p: &PathBuf| written.contains(p) || (!overwrite && p.exists());
    let first = dir.join(format!("{stem}.{ext}"));
    if !taken(&first) {
        return first;
    }
    (2..)
        .map(|n| dir.join(format!("{stem} ({n}).{ext}")))
        .find(|p| !taken(p))
        .unwrap_or(first)
}

// ───────────────────────────── 화면 ─────────────────────────────

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        while let Ok(paths) = self.handoff.try_recv() {
            self.add(paths);
        }
        self.poll();
        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .map(|f| f.path().to_path_buf())
                .filter(|p| !p.as_os_str().is_empty())
                .collect()
        });
        self.add(dropped);
        self.shortcuts(&ctx);
        let p = pal(ui);

        // 위: 바탕과 같은 색, 나눔선 없이 큰 제목만
        egui::Panel::top("header")
            .show_separator_line(false)
            .frame(Frame::new().fill(p.bg).inner_margin(Margin {
                left: 24,
                right: 24,
                top: 20,
                bottom: 4,
            }))
            .show(ui, |ui| self.header(ui, p));

        // 아래: 상태 줄
        egui::Panel::bottom("footer")
            .show_separator_line(false)
            .frame(Frame::new().fill(p.bg).inner_margin(Margin {
                left: 24,
                right: 24,
                top: 0,
                bottom: 12,
            }))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // 오른쪽부터: 버전 · 만든이(누르면 메일) — 좁으면 왼쪽 안내가 말줄임된다.
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        ui.label(
                            RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                .size(12.0)
                                .color(p.muted),
                        );
                        ui.add_space(4.0);
                        ui.hyperlink_to(
                            RichText::new(AUTHOR).size(12.0),
                            format!("mailto:{AUTHOR}"),
                        )
                        .on_hover_text("만든이에게 메일 보내기");
                        ui.label(RichText::new("만든이").size(12.0).color(p.muted));
                        ui.add_space(10.0);
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(
                                        "만든 파일은 NK-Editor, MOIP-Editor 에서 확인한 뒤 제출하세요",
                                    )
                                    .size(12.0)
                                    .color(p.muted),
                                )
                                .truncate(),
                            );
                        });
                    });
                });
            });

        // 아래: 변환 단추 · 게이지 — 목록이 길어도 늘 보인다.
        egui::Panel::bottom("action")
            .show_separator_line(false)
            .frame(Frame::new().fill(p.bg).inner_margin(Margin {
                left: 24,
                right: 24,
                top: 12,
                bottom: 12,
            }))
            .show(ui, |ui| self.action_bar(ui, &ctx, p));

        egui::CentralPanel::default()
            .frame(Frame::new().fill(p.bg))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        Frame::new()
                            .inner_margin(Margin {
                                left: 24,
                                right: 24,
                                top: 12,
                                bottom: 8,
                            })
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 12.0;
                                self.files_card(ui, p);
                                self.format_card(ui, p);
                                self.output_card(ui, p);
                            });
                    });
            });

        // 파일을 창 위로 끌고 오면 창 전체를 놓는 곳으로 보여 준다.
        if !self.working() && ctx.input(|i| !i.raw.hovered_files.is_empty()) {
            drop_overlay(&ctx, p);
        }
    }
}

impl App {
    fn header(&self, ui: &mut Ui, p: Pal) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 12.0;
            if let Some(logo) = &self.logo {
                ui.add(egui::Image::new((logo.id(), Vec2::splat(40.0))));
            }
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(RichText::new(APP_TITLE).font(bold(22.0)).color(p.ink));
                ui.label(
                    RichText::new(
                        "Word 명세서를 NK-Editor(.hlt), MOIP-Editor(.hwpx) 파일로 바꿉니다",
                    )
                    .size(13.0)
                    .color(p.muted),
                );
            });
        });
    }

    fn files_card(&mut self, ui: &mut Ui, p: Pal) {
        let working = self.working();
        let count = self.jobs.len();
        let title = if count == 0 {
            "변환할 파일".to_string()
        } else {
            format!("변환할 파일 {count}개")
        };
        let mut clear = false;
        card(
            ui,
            p,
            &title,
            |ui| {
                if count > 0 && !working {
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if button(ui, p, Btn::Text, "목록 비우기", true).clicked() {
                            clear = true;
                        }
                    });
                }
            },
            |ui| {
                if self.jobs.is_empty() {
                    if drop_zone(ui, p, 168.0, working, true).clicked() {
                        self.browse();
                    }
                } else {
                    let mut remove = None;
                    let mut reveal_path = None;
                    // 토스 목록처럼 테두리 · 나눔선 없이 줄만 늘어놓는다.
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        for (i, job) in self.jobs.iter_mut().enumerate() {
                            match job_row(ui, p, job, working, self.batch.as_ref()) {
                                RowAction::Remove => remove = Some(i),
                                RowAction::Reveal(path) => reveal_path = Some(path),
                                RowAction::None => {}
                            }
                        }
                    });
                    if let Some(i) = remove {
                        self.jobs.remove(i);
                    }
                    if let Some(path) = reveal_path {
                        reveal(&path);
                    }
                    if drop_zone(ui, p, 52.0, working, false).clicked() {
                        self.browse();
                    }
                }
                if let Some(n) = &self.notice {
                    callout(ui, p.err_soft, p.err, n);
                }
            },
        );
        if clear {
            self.jobs.clear();
            self.notice = None;
            self.shown = 0.0;
        }
    }

    fn format_card(&mut self, ui: &mut Ui, p: Pal) {
        let working = self.working();
        let mut pick = None;
        card(
            ui,
            p,
            "만들 형식",
            |_| {},
            |ui| {
                let gap = 10.0;
                let w = ((ui.available_width() - gap) / 2.0).max(120.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    for (hlt, name, editor) in [
                        (true, "HLT", "NK-Editor (통합명세서작성기)"),
                        (false, "HWPX", "MOIP-Editor"),
                    ] {
                        if format_tile(ui, p, w, name, editor, self.hlt == hlt, !working).clicked()
                        {
                            pick = Some(hlt);
                        }
                    }
                });
                let hint = if self.hlt {
                    "국내 출원 서식입니다. CP949 밖 글자는 첨자 서식, 비슷한 글자, 글자 그림으로 바꿉니다. PCT 국내진입 서식은 명령줄(CLI)에서 만들 수 있습니다."
                } else {
                    "지식재산처 명세서 서식(A4, 바탕체 12pt, 줄 간격 280%)에 맞춥니다."
                };
                ui.add_space(2.0);
                ui.label(RichText::new(hint).size(13.0).color(p.muted));
            },
        );
        if let Some(hlt) = pick
            && hlt != self.hlt
        {
            self.hlt = hlt;
            self.reset_results();
        }
    }

    fn output_card(&mut self, ui: &mut Ui, p: Pal) {
        let working = self.working();
        let mut browse = false;
        card(
            ui,
            p,
            "저장 위치",
            |_| {},
            |ui| {
                ui.add_enabled_ui(!working, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let same = self.out_dir.is_none();
                if radio(ui, p, same, "원본 파일과 같은 폴더").clicked() {
                    self.out_dir = None;
                }
                ui.horizontal(|ui| {
                    if radio(ui, p, !same, "다른 폴더").clicked() && self.out_dir.is_none() {
                        browse = true;
                    }
                    if let Some(dir) = &self.out_dir {
                        ui.add(
                            egui::Label::new(RichText::new(dir.display().to_string()).size(12.5).color(p.muted))
                                .truncate(),
                        );
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if button(ui, p, Btn::Weak, "폴더 고르기", !working).clicked() {
                            browse = true;
                        }
                    });
                });
                ui.add_space(4.0);
                checkbox(ui, p, &mut self.overwrite, "같은 이름의 파일이 있으면 덮어쓰기");
                let ext = self.target().extension();
                let note = if self.overwrite {
                    "같은 이름이 있으면 새 파일로 바꿔 넣습니다. 변환이 실패하면 원래 파일은 그대로 남습니다.".to_string()
                } else {
                    format!("같은 이름이 있으면 이름 뒤에 (2)를 붙여 새 .{ext} 파일로 만듭니다.")
                };
                ui.add_space(2.0);
                Frame::new().inner_margin(Margin { left: 44, ..Default::default() }).show(ui, |ui| {
                    ui.label(RichText::new(note).size(13.0).color(p.muted));
                });
            });
            },
        );
        if browse {
            self.browse_out_dir();
        }
    }

    fn action_bar(&mut self, ui: &mut Ui, ctx: &egui::Context, p: Pal) {
        let goal = self.overall();
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.shown += (goal - self.shown) * (1.0 - (-dt * 10.0).exp());
        if (goal - self.shown).abs() < 0.002 {
            self.shown = goal;
        }
        if self.shown != goal {
            ctx.request_repaint();
        }

        if let Some(batch) = &self.batch {
            // 변환 중: 무엇을 하는지 + 전체 게이지 + 멈추기
            let name = batch
                .current
                .map(|i| file_name(&self.jobs[i].input))
                .unwrap_or_default();
            let mut stop = false;
            ui.horizontal(|ui| {
                ui.add(egui::Spinner::new().size(16.0).color(p.accent));
                let step = (batch.finished + 1).min(batch.total);
                ui.label(
                    RichText::new(format!("변환 중 {step}/{}", batch.total))
                        .font(bold(14.0))
                        .color(p.ink),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let cancelling = batch.cancel.load(Ordering::Relaxed);
                    let label = if cancelling {
                        "멈추는 중"
                    } else {
                        "멈추기"
                    };
                    if button(ui, p, Btn::Weak, label, !cancelling).clicked() {
                        stop = true;
                    }
                    ui.add(
                        egui::Label::new(
                            RichText::new(format!("{name}  {}", batch.stage))
                                .size(12.5)
                                .color(p.muted),
                        )
                        .truncate(),
                    );
                });
            });
            if stop {
                batch.cancel.store(true, Ordering::Relaxed);
            }
            ui.add_space(4.0);
            gauge(ui, p, self.shown);
            ui.label(
                RichText::new(format!("{:.0}%", self.shown * 100.0))
                    .size(12.0)
                    .color(p.muted),
            );
            ctx.request_repaint_after(Duration::from_millis(50));
            return;
        }

        let pending = self.pending();
        let done: Vec<&Job> = self
            .jobs
            .iter()
            .filter(|j| matches!(j.state, JobState::Done { .. }))
            .collect();
        let failed = self
            .jobs
            .iter()
            .filter(|j| matches!(j.state, JobState::Failed(_)))
            .count();

        // 결과 요약 띠
        if !done.is_empty() || failed > 0 {
            let checks: usize = done.iter().map(|j| j.checks()).sum();
            let (fill, ink, text) = if failed > 0 {
                (
                    p.err_soft,
                    p.err,
                    format!(
                        "{}개 변환, {failed}개 실패. 실패한 파일을 눌러 이유를 확인하세요",
                        done.len()
                    ),
                )
            } else if checks > 0 {
                (
                    p.warn_soft,
                    p.warn,
                    format!(
                        "{}개 모두 변환했습니다. 편집기에서 확인할 곳이 {checks}곳 있어요",
                        done.len()
                    ),
                )
            } else {
                (
                    p.ok_soft,
                    p.ok,
                    format!("{}개 모두 변환했습니다", done.len()),
                )
            };
            let mark = if failed > 0 {
                Mark::Err
            } else if checks > 0 {
                Mark::Warn
            } else {
                Mark::Ok
            };
            let last = done.last().and_then(|j| match &j.state {
                JobState::Done { output, .. } => Some(output.clone()),
                _ => None,
            });
            let mut open_folder = false;
            // 알림 띠: 아이콘 + 한 줄 글 + 오른쪽 글자 단추
            Frame::new()
                .fill(fill)
                .corner_radius(16)
                .inner_margin(Margin {
                    left: 16,
                    right: 8,
                    top: 6,
                    bottom: 6,
                })
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.set_min_height(36.0);
                        let (r, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
                        mark_badge(ui.painter(), r.center(), mark, ink, fill);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if last.is_some()
                                && button(ui, p, Btn::Text, "폴더 열기", true).clicked()
                            {
                                open_folder = true;
                            }
                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(text).font(bold(13.5)).color(ink),
                                    )
                                    .truncate(),
                                );
                            });
                        });
                    });
                });
            if open_folder && let Some(path) = last {
                reveal(&path);
            }
        }

        let (label, enabled) = match (self.jobs.len(), pending) {
            (0, _) => ("변환하기".to_string(), false),
            (_, 0) => ("모두 변환했습니다".to_string(), false),
            (_, n) if failed > 0 && n == failed => (format!("실패한 {n}개 다시 변환"), true),
            (_, 1) => ("변환하기".to_string(), true),
            (_, n) => (format!("{n}개 변환하기"), true),
        };
        ui.add_space(4.0);
        let response = button_ex(
            ui,
            p,
            Btn::Filled,
            &label,
            enabled,
            ui.available_width(),
            56.0,
            bold(17.0),
        );
        if enabled {
            response.clone().on_hover_text("Enter");
        }
        if response.clicked() {
            self.start(ctx);
        }
    }
}

// ───────────────────────────── 부품 ─────────────────────────────

/// 테두리 · 그림자 없는 흰 카드(모서리 20). `aside` 는 제목 줄 오른쪽.
fn card(ui: &mut Ui, p: Pal, title: &str, aside: impl FnOnce(&mut Ui), body: impl FnOnce(&mut Ui)) {
    Frame::new()
        .fill(p.card)
        .corner_radius(20)
        .inner_margin(Margin {
            left: 20,
            right: 20,
            top: 18,
            bottom: 20,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.horizontal(|ui| {
                ui.set_min_height(32.0);
                ui.label(RichText::new(title).font(bold(18.0)).color(p.ink));
                aside(ui);
            });
            ui.add_space(4.0);
            body(ui);
        });
}

#[derive(Clone, Copy, PartialEq)]
enum Btn {
    /// 꽉 찬 파랑 — 화면에서 하나뿐인 주 행동
    Filled,
    /// 회색 바탕 — 보조 행동
    Weak,
    /// 파랑 글자만 — 가벼운 행동
    Text,
}

/// 토스식 단추(높이 36 · 모서리 10). 누르면 살짝 작아지고 어두워진다.
fn button(ui: &mut Ui, p: Pal, kind: Btn, label: &str, enabled: bool) -> egui::Response {
    button_ex(ui, p, kind, label, enabled, 0.0, 36.0, bold(14.0))
}

#[allow(clippy::too_many_arguments)]
fn button_ex(
    ui: &mut Ui,
    p: Pal,
    kind: Btn,
    label: &str,
    enabled: bool,
    min_width: f32,
    height: f32,
    font: FontId,
) -> egui::Response {
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, Color32::PLACEHOLDER);
    let pad = if kind == Btn::Text { 10.0 } else { 16.0 };
    let size = egui::vec2((galley.size().x + pad * 2.0).max(min_width), height);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label));
    let hot = enabled && (response.hovered() || response.has_focus());
    if hot {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if ui.is_rect_visible(rect) {
        let pressed = enabled && response.is_pointer_button_down_on();
        let off = p.ink.gamma_multiply(0.3);
        let (fill, ink) = match kind {
            Btn::Filled if enabled => (p.accent, p.accent_ink),
            Btn::Filled => (p.hover, off),
            Btn::Weak => (p.hover, if enabled { p.ink2 } else { off }),
            Btn::Text => (
                Color32::TRANSPARENT,
                if enabled { p.accent_deep } else { off },
            ),
        };
        let r = CornerRadius::same(if height > 44.0 { 16 } else { 10 });
        // 눌렀을 때 살짝 줄어든다(토스의 눌림 효과).
        let shape = if pressed {
            rect.shrink2(rect.size() * 0.02)
        } else {
            rect
        };
        let painter = ui.painter();
        painter.rect_filled(shape, r, fill);
        let layer = match (kind, pressed, hot) {
            (_, true, _) => 0.10,
            (Btn::Filled, false, true) => 0.06,
            (_, false, true) => 0.05,
            _ => 0.0,
        };
        if layer > 0.0 {
            painter.rect_filled(shape, r, Color32::BLACK.gamma_multiply(layer));
        }
        if response.has_focus() {
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same(r.nw + 2),
                Stroke::new(2.0, p.accent),
                StrokeKind::Outside,
            );
        }
        painter.galley(shape.center() - galley.size() / 2.0, galley, ink);
    }
    response
}

#[derive(Clone, Copy)]
enum Icon {
    Close,
    ExpandMore,
    ExpandLess,
    Folder,
}

/// 아이콘 단추(32×32). 마우스를 올리면 둥근 회색 바탕이 깔린다.
fn icon_button(ui: &mut Ui, p: Pal, icon: Icon, tip: &str) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::click());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tip));
    let hot = response.hovered() || response.has_focus();
    if hot {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let painter = ui.painter();
    let c = rect.center();
    let r = CornerRadius::same(8);
    if response.is_pointer_button_down_on() {
        painter.rect_filled(rect, r, p.ink.gamma_multiply(0.12));
    } else if hot {
        painter.rect_filled(rect, r, p.ink.gamma_multiply(0.07));
    }
    if response.has_focus() {
        painter.rect_stroke(rect, r, Stroke::new(2.0, p.accent), StrokeKind::Inside);
    }
    paint_icon(painter, c, icon, p.muted);
    response.on_hover_text(tip)
}

fn paint_icon(painter: &egui::Painter, c: Pos2, icon: Icon, color: Color32) {
    let s = Stroke::new(1.8, color);
    let v = egui::vec2;
    match icon {
        Icon::Close => {
            painter.line_segment([c + v(-4.5, -4.5), c + v(4.5, 4.5)], s);
            painter.line_segment([c + v(4.5, -4.5), c + v(-4.5, 4.5)], s);
        }
        Icon::ExpandMore => {
            painter.add(egui::Shape::line(
                vec![c + v(-5.0, -2.5), c + v(0.0, 2.5), c + v(5.0, -2.5)],
                s,
            ));
        }
        Icon::ExpandLess => {
            painter.add(egui::Shape::line(
                vec![c + v(-5.0, 2.5), c + v(0.0, -2.5), c + v(5.0, 2.5)],
                s,
            ));
        }
        Icon::Folder => {
            painter.add(egui::Shape::closed_line(
                vec![
                    c + v(-7.0, -5.5),
                    c + v(-2.5, -5.5),
                    c + v(-1.0, -3.5),
                    c + v(7.0, -3.5),
                    c + v(7.0, 5.5),
                    c + v(-7.0, 5.5),
                ],
                Stroke::new(1.6, color),
            ));
        }
    }
}

/// 토스 목록의 둥근 네모 아이콘 자리(40×40) 안에 귀 접힌 파란 문서.
fn doc_icon(ui: &mut Ui, p: Pal) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(12), p.accent_soft);
    let body = Rect::from_center_size(rect.center(), egui::vec2(15.0, 19.0));
    let fold = 5.0;
    let (l, t, r, b) = (body.left(), body.top(), body.right(), body.bottom());
    painter.add(egui::Shape::convex_polygon(
        vec![
            Pos2::new(l, t),
            Pos2::new(r - fold, t),
            Pos2::new(r, t + fold),
            Pos2::new(r, b),
            Pos2::new(l, b),
        ],
        p.accent,
        Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        vec![
            Pos2::new(r - fold, t),
            Pos2::new(r, t + fold),
            Pos2::new(r - fold, t + fold),
        ],
        Color32::WHITE.gamma_multiply(0.6),
        Stroke::NONE,
    ));
    let ink = Stroke::new(1.4, Color32::WHITE);
    for (y, w) in [(8.0, 8.0), (11.0, 8.0), (14.0, 5.0)] {
        painter.line_segment(
            [Pos2::new(l + 3.5, t + y), Pos2::new(l + 3.5 + w, t + y)],
            ink,
        );
    }
}

/// 돌아가는 원호(진행 중 표시).
fn spinner_arc(ui: &Ui, painter: &egui::Painter, c: Pos2, r: f32, color: Color32) {
    let t = ui.input(|i| i.time) as f32;
    let start = t * 5.0;
    let sweep = 1.2 + (t * 2.2).sin().abs() * 3.2;
    let pts: Vec<Pos2> = (0..=20)
        .map(|k| {
            let a = start + sweep * k as f32 / 20.0;
            c + egui::vec2(a.cos(), a.sin()) * r
        })
        .collect();
    painter.add(egui::Shape::line(pts, Stroke::new(2.0, color)));
    ui.ctx().request_repaint();
}

/// 끌어다 놓는 곳. `big` 이면 빈 목록용 큰 상자, 아니면 목록 끝의 「파일 추가」 줄.
fn drop_zone(ui: &mut Ui, p: Pal, height: f32, working: bool, big: bool) -> egui::Response {
    let sense = if working {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), height), sense);
    let (title, sub) = if big {
        (
            "DOCX 파일을 여기에 끌어다 놓으세요",
            "여러 파일이나 폴더를 한꺼번에 놓아도 됩니다. 눌러서 고를 수도 있어요 (Ctrl+O)",
        )
    } else {
        ("파일 추가", "끌어다 놓거나 눌러서 고르기 (Ctrl+O)")
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, !working, format!("{title}. {sub}"))
    });
    if response.hovered() && !working {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let lit = !working && (response.hovered() || response.has_focus());
    let painter = ui.painter();
    let ink = if working { p.muted } else { p.ink };
    if big {
        painter.rect_filled(
            rect,
            CornerRadius::same(16),
            if lit { p.accent_soft } else { p.sunken },
        );
        dashed_rect(
            painter,
            rect.shrink(0.5),
            if lit { p.accent } else { p.outline },
            if lit { 1.6 } else { 1.2 },
        );
        // 위로 올리는 화살표
        let c = rect.center() - egui::vec2(0.0, 34.0);
        painter.circle_filled(c, 24.0, if lit { p.card } else { p.accent_soft });
        let s = Stroke::new(2.4, p.accent);
        let v = egui::vec2;
        painter.line_segment([c + v(0.0, 9.0), c + v(0.0, -9.0)], s);
        painter.line_segment([c + v(-7.0, -2.0), c + v(0.0, -9.0)], s);
        painter.line_segment([c + v(7.0, -2.0), c + v(0.0, -9.0)], s);
        painter.text(
            rect.center() + egui::vec2(0.0, 14.0),
            Align2::CENTER_CENTER,
            title,
            bold(16.0),
            ink,
        );
        let sub = painter.layout(
            sub.into(),
            FontId::proportional(13.0),
            p.muted,
            rect.width() - 48.0,
        );
        painter.galley(
            Pos2::new(rect.center().x - sub.size().x / 2.0, rect.center().y + 30.0),
            sub,
            p.muted,
        );
    } else {
        // 목록 줄과 같은 꼴: 왼쪽 둥근 네모 안 + 표시, 파랑 제목, 회색 설명
        if lit {
            painter.rect_filled(rect, CornerRadius::same(14), p.hover);
        }
        let icon = Rect::from_min_size(
            Pos2::new(rect.left() + 8.0, rect.center().y - 20.0),
            Vec2::splat(40.0),
        );
        painter.rect_filled(icon, CornerRadius::same(12), p.hover);
        let c = icon.center();
        let s = Stroke::new(2.0, if working { p.muted } else { p.accent });
        painter.line_segment([c - egui::vec2(7.0, 0.0), c + egui::vec2(7.0, 0.0)], s);
        painter.line_segment([c - egui::vec2(0.0, 7.0), c + egui::vec2(0.0, 7.0)], s);
        let left = icon.right() + 12.0;
        let t = painter.layout_no_wrap(
            title.into(),
            bold(15.0),
            if working { p.muted } else { p.accent_deep },
        );
        let th = t.size().y;
        painter.galley(
            Pos2::new(left, rect.center().y - th / 2.0),
            t.clone(),
            p.accent_deep,
        );
        let g = painter.layout_no_wrap(sub.into(), FontId::proportional(13.0), p.muted);
        painter.galley(
            Pos2::new(left + t.size().x + 10.0, rect.center().y - g.size().y / 2.0),
            g,
            p.muted,
        );
    }
    response
}

fn dashed_rect(painter: &egui::Painter, rect: Rect, color: Color32, width: f32) {
    let stroke = Stroke::new(width, color);
    let r = 16.0;
    let (l, t, rt, b) = (rect.left(), rect.top(), rect.right(), rect.bottom());
    let dash = 6.0;
    let gap = 4.0;
    for (a, z) in [
        (Pos2::new(l + r, t), Pos2::new(rt - r, t)),
        (Pos2::new(l + r, b), Pos2::new(rt - r, b)),
        (Pos2::new(l, t + r), Pos2::new(l, b - r)),
        (Pos2::new(rt, t + r), Pos2::new(rt, b - r)),
    ] {
        painter.extend(egui::Shape::dashed_line(&[a, z], stroke, dash, gap));
    }
    // 모서리는 실선 호
    for (c, start) in [
        (Pos2::new(l + r, t + r), 180.0_f32),
        (Pos2::new(rt - r, t + r), 270.0),
        (Pos2::new(rt - r, b - r), 0.0),
        (Pos2::new(l + r, b - r), 90.0),
    ] {
        let pts: Vec<Pos2> = (0..=8)
            .map(|k| {
                let a = (start + k as f32 * 90.0 / 8.0).to_radians();
                c + egui::vec2(a.cos(), a.sin()) * r
            })
            .collect();
        painter.add(egui::Shape::line(pts, stroke));
    }
}

fn drop_overlay(ctx: &egui::Context, p: Pal) {
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        Id::unique("drop-overlay"),
    ));
    let screen = ctx.content_rect();
    painter.rect_filled(screen, CornerRadius::ZERO, p.bg.gamma_multiply(0.92));
    let inner = screen.shrink(18.0);
    painter.rect_filled(inner, CornerRadius::same(16), p.accent_soft);
    dashed_rect(&painter, inner, p.accent, 2.0);
    painter.text(
        inner.center() - egui::vec2(0.0, 12.0),
        Align2::CENTER_CENTER,
        "여기에 놓으세요",
        bold(22.0),
        p.accent,
    );
    painter.text(
        inner.center() + egui::vec2(0.0, 20.0),
        Align2::CENTER_CENTER,
        "Word 문서(.docx)만 목록에 넣습니다",
        FontId::proportional(13.0),
        p.muted,
    );
}

fn format_tile(
    ui: &mut Ui,
    p: Pal,
    width: f32,
    name: &str,
    editor: &str,
    selected: bool,
    enabled: bool,
) -> egui::Response {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 76.0), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::Role::RadioButton,
            enabled,
            selected,
            format!("{name}, {editor}"),
        )
    });
    if response.hovered() && enabled {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let hot = enabled && (response.hovered() || response.has_focus());
    let painter = ui.painter();
    let r = CornerRadius::same(16);
    // 고른 것: 옅은 파랑 바탕 + 파랑 테두리. 아니면 회색 바탕만.
    if selected {
        painter.rect(
            rect,
            r,
            p.accent_soft,
            Stroke::new(2.0, p.accent),
            StrokeKind::Inside,
        );
    } else {
        painter.rect_filled(rect, r, p.hover);
        if hot {
            painter.rect_filled(rect, r, Color32::BLACK.gamma_multiply(0.04));
        }
    }
    if response.has_focus() {
        painter.rect_stroke(
            rect.expand(2.0),
            CornerRadius::same(18),
            Stroke::new(2.0, p.accent),
            StrokeKind::Outside,
        );
    }
    let left = rect.left() + 18.0;
    painter.text(
        Pos2::new(left, rect.top() + 27.0),
        Align2::LEFT_CENTER,
        name,
        bold(18.0),
        if selected { p.accent_deep } else { p.ink },
    );
    painter.text(
        Pos2::new(left, rect.top() + 51.0),
        Align2::LEFT_CENTER,
        editor,
        FontId::proportional(13.0),
        if selected { p.accent_deep } else { p.muted },
    );
    // 오른쪽: 체크 원
    check_circle(
        painter,
        p,
        Pos2::new(rect.right() - 24.0, rect.top() + 26.0),
        selected,
        enabled,
    );
    response
}

/// 토스식 체크 원: 켜면 파랑, 꺼지면 회색 원 안에 흰 체크.
fn check_circle(painter: &egui::Painter, p: Pal, c: Pos2, on: bool, enabled: bool) {
    let fill = match (on, enabled) {
        (true, true) => p.accent,
        (true, false) => p.accent.gamma_multiply(0.4),
        (false, _) => p.outline,
    };
    painter.circle_filled(c, 11.0, fill);
    check_mark(painter, c, 5.0, Color32::WHITE);
}

/// 하나만 고르는 줄(라디오 역할). 글자까지 줄 전체가 눌린다.
fn radio(ui: &mut Ui, p: Pal, on: bool, label: &str) -> egui::Response {
    choice(ui, p, label, on, true)
}

/// 켜고 끄는 줄(체크박스 역할).
fn checkbox(ui: &mut Ui, p: Pal, value: &mut bool, label: &str) -> egui::Response {
    let mut r = choice(ui, p, label, *value, false);
    if r.clicked() {
        *value = !*value;
        r.mark_changed();
    }
    r
}

fn choice(ui: &mut Ui, p: Pal, label: &str, on: bool, radio: bool) -> egui::Response {
    let enabled = ui.is_enabled();
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        FontId::proportional(15.0),
        Color32::PLACEHOLDER,
    );
    let size = egui::vec2(44.0 + galley.size().x + 12.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            if radio {
                egui::Role::RadioButton
            } else {
                egui::Role::CheckBox
            },
            enabled,
            on,
            label,
        )
    });
    let painter = ui.painter();
    let r = CornerRadius::same(10);
    if enabled && response.is_pointer_button_down_on() {
        painter.rect_filled(rect, r, p.ink.gamma_multiply(0.10));
    } else if enabled && (response.hovered() || response.has_focus()) {
        painter.rect_filled(rect, r, p.hover);
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if response.has_focus() {
        painter.rect_stroke(rect, r, Stroke::new(2.0, p.accent), StrokeKind::Inside);
    }
    check_circle(
        painter,
        p,
        Pos2::new(rect.left() + 20.0, rect.center().y),
        on,
        enabled,
    );
    let ink = if enabled {
        p.ink
    } else {
        p.ink.gamma_multiply(0.38)
    };
    painter.galley(
        Pos2::new(rect.left() + 44.0, rect.center().y - galley.size().y / 2.0),
        galley,
        ink,
    );
    response
}

fn check_mark(painter: &egui::Painter, c: Pos2, s: f32, color: Color32) {
    painter.add(egui::Shape::line(
        vec![
            c + egui::vec2(-s, 0.0),
            c + egui::vec2(-s * 0.3, s * 0.7),
            c + egui::vec2(s, -s * 0.6),
        ],
        Stroke::new(2.0, color),
    ));
}

enum RowAction {
    None,
    Remove,
    Reveal(PathBuf),
}

fn job_row(ui: &mut Ui, p: Pal, job: &mut Job, working: bool, batch: Option<&Batch>) -> RowAction {
    let mut action = RowAction::None;
    let checks = job.checks();
    let expandable = matches!(job.state, JobState::Done { .. } | JobState::Failed(_));
    let fraction = batch.map_or(0.0, |b| b.fraction);
    let (sub, sub_color) = match &job.state {
        JobState::Waiting => (
            job.input
                .parent()
                .map(|d| d.display().to_string())
                .unwrap_or_default(),
            p.muted,
        ),
        JobState::Running => (batch.map(|b| b.stage.clone()).unwrap_or_default(), p.muted),
        JobState::Done { output, .. } => (format!("→ {}", file_name(output)), p.muted),
        JobState::Failed(f) => (failure_title(f).to_string(), p.err),
    };

    // 바탕(상태 층)은 내용을 그린 뒤 크기를 알고 채운다.
    let bg = ui.painter().add(egui::Shape::Noop);
    let inner = Frame::new()
        .inner_margin(Margin {
            left: 8,
            right: 4,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                doc_icon(ui, p);
                ui.add_space(8.0);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if !working && icon_button(ui, p, Icon::Close, "목록에서 빼기").clicked()
                    {
                        action = RowAction::Remove;
                    }
                    if let JobState::Done { output, .. } = &job.state
                        && icon_button(ui, p, Icon::Folder, "저장한 폴더 열기").clicked()
                    {
                        action = RowAction::Reveal(output.clone());
                    }
                    if expandable {
                        let (icon, tip) = if job.open {
                            (Icon::ExpandLess, "접기")
                        } else {
                            (Icon::ExpandMore, "자세히 보기")
                        };
                        if icon_button(ui, p, icon, tip).clicked() {
                            job.open = !job.open;
                        }
                    }
                    ui.add_space(4.0);
                    status_chip(ui, p, &job.state, checks, fraction);
                    ui.add_space(8.0);
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.spacing_mut().item_spacing.y = 1.0;
                        ui.add(
                            egui::Label::new(
                                RichText::new(file_name(&job.input))
                                    .font(bold(15.0))
                                    .color(p.ink),
                            )
                            .truncate(),
                        );
                        ui.add(
                            egui::Label::new(RichText::new(sub).size(13.0).color(sub_color))
                                .truncate(),
                        );
                    });
                });
            });
            if job.open {
                // 글자 줄과 왼쪽을 맞춘다(문서 아이콘 + 틈).
                Frame::new()
                    .inner_margin(Margin {
                        left: 52,
                        right: 12,
                        top: 6,
                        bottom: 6,
                    })
                    .show(ui, |ui| match &job.state {
                        JobState::Done { report, .. } => summary(ui, p, report),
                        JobState::Failed(f) => {
                            let (plain, tech) = docx2hwpx_hlt::split_message(&f.message);
                            ui.label(RichText::new(plain).size(14.0).color(p.ink2));
                            if let Some(tech) = tech {
                                ui.label(
                                    RichText::new(format!(
                                        "기술 정보(문의할 때 함께 보내 주세요): {tech}"
                                    ))
                                    .size(11.5)
                                    .color(p.muted),
                                );
                            }
                        }
                        _ => {}
                    });
            }
        });
    let rect = inner.response.rect;
    let fill = if job.open {
        p.sunken
    } else if expandable && ui.rect_contains_pointer(rect) {
        p.hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().set(
        bg,
        egui::Shape::rect_filled(rect, CornerRadius::same(14), fill),
    );
    if expandable && !matches!(action, RowAction::None) {
        return action;
    }
    // 줄 아무 곳이나 눌러도 펼친다.
    if expandable
        && inner
            .response
            .interact(Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
    {
        job.open = !job.open;
    }
    action
}

/// 줄 오른쪽의 상태 배지(토스의 옅은 바탕 + 진한 글자 배지).
fn status_chip(ui: &mut Ui, p: Pal, state: &JobState, checks: usize, fraction: f32) {
    let (fill, ink, text) = match state {
        JobState::Waiting => (p.hover, p.muted, "대기".to_string()),
        JobState::Running => (
            p.accent_soft,
            p.accent_deep,
            format!("{:.0}%", fraction * 100.0),
        ),
        JobState::Done { .. } if checks == 0 => (p.ok_soft, p.ok, "완료".to_string()),
        JobState::Done { .. } => (p.warn_soft, p.warn, format!("확인 {checks}곳")),
        JobState::Failed(_) => (p.err_soft, p.err, "실패".to_string()),
    };
    let running = matches!(state, JobState::Running);
    let galley = ui
        .painter()
        .layout_no_wrap(text, bold(12.5), Color32::PLACEHOLDER);
    let lead = if running { 18.0 } else { 0.0 };
    let size = egui::vec2(galley.size().x + 16.0 + lead, 26.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(6), fill);
    if running {
        spinner_arc(
            ui,
            painter,
            Pos2::new(rect.left() + 15.0, rect.center().y),
            5.0,
            ink,
        );
    }
    painter.galley(
        Pos2::new(
            rect.left() + 8.0 + lead,
            rect.center().y - galley.size().y / 2.0,
        ),
        galley,
        ink,
    );
}

#[derive(Clone, Copy)]
enum Mark {
    Ok,
    Warn,
    Err,
}

/// 채운 원 안에 체크 · 느낌표 · 엑스. `hole` 은 안쪽 기호 색(바탕색).
fn mark_badge(painter: &egui::Painter, c: Pos2, mark: Mark, ink: Color32, hole: Color32) {
    painter.circle_filled(c, 10.0, ink);
    match mark {
        Mark::Ok => check_mark(painter, c, 4.8, hole),
        Mark::Warn => {
            painter.text(c, Align2::CENTER_CENTER, "!", bold(13.0), hole);
        }
        Mark::Err => {
            let s = Stroke::new(2.0, hole);
            let v = egui::vec2;
            painter.line_segment([c + v(-4.0, -4.0), c + v(4.0, 4.0)], s);
            painter.line_segment([c + v(4.0, -4.0), c + v(-4.0, 4.0)], s);
        }
    }
}

/// 둥근 6px 진행 막대.
fn gauge(ui: &mut Ui, p: Pal, value: f32) {
    let h = 6.0;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), h), Sense::hover());
    let painter = ui.painter();
    let r = CornerRadius::same(3);
    painter.rect_filled(rect, r, p.line);
    let w = rect.width() * value.clamp(0.0, 1.0);
    if w > 0.5 {
        painter.rect_filled(
            Rect::from_min_size(rect.min, egui::vec2(w.max(h), h)),
            r,
            p.accent,
        );
    }
}

fn callout(ui: &mut Ui, fill: Color32, ink: Color32, text: &str) {
    Frame::new()
        .fill(fill)
        .corner_radius(14)
        .inner_margin(Margin::symmetric(16, 12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(text).size(13.5).color(ink));
        });
}

/// 실패 코드 → 제목. 무엇을 하면 되는지는 메시지(사람용 문장)에 있다.
fn failure_title(f: &Failure) -> &'static str {
    match f.code {
        "input_unreadable" => "파일을 열지 못했습니다",
        "invalid_docx" => "Word 문서(.docx)가 아니거나 손상되었습니다",
        "write_failed" => "파일을 저장하지 못했습니다",
        _ => "변환하지 못했습니다",
    }
}

/// 변환 보고. 무엇을 바꿨는지, 사람이 편집기에서 확인할 곳은 어디인지.
fn summary(ui: &mut Ui, p: Pal, report: &Value) {
    let n = |k: &str| report[k].as_u64().unwrap_or(0);
    let list = |k: &str| report[k].as_array().cloned().unwrap_or_default();

    // 숫자 칩
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
        for (label, value) in [
            ("변경 추적 반영", n("revisions_accepted")),
            ("메모 지움", n("comments_removed")),
            ("수식 그림", n("equations")),
            ("WMF/EMF", n("metafiles")),
            ("도형 묶음", n("shapes")),
            ("첨자 서식", n("scripts")),
            ("그림 다듬음", n("images_optimized")),
        ] {
            if value > 0 {
                chip(ui, p, label, value);
            }
        }
    });

    let subs = list("substitutions");
    if !subs.is_empty() {
        ui.add_space(4.0);
        ui.label(RichText::new("바꾼 글자").font(bold(14.0)).color(p.ink));
        for s in subs {
            // 글자 하나는 따옴표 대신 작은 회색 네모에 담아 보인다.
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                glyph_box(ui, p, s["from"].as_str().unwrap_or(""));
                ui.label(RichText::new("→").size(13.0).color(p.muted));
                glyph_box(ui, p, s["to"].as_str().unwrap_or(""));
                ui.label(
                    RichText::new(format!(
                        "{}, {}곳",
                        s["codepoints"].as_str().unwrap_or(""),
                        s["count"]
                    ))
                    .size(13.0)
                    .color(p.muted),
                );
            });
        }
    }

    // 확인할 곳: 경고 + 글자 그림(편집기에서 글로 고칠 수 없다)
    let mut check: Vec<String> = list("warnings")
        .iter()
        .map(|w| {
            docx2hwpx_hlt::split_message(w["message"].as_str().unwrap_or(""))
                .0
                .to_string()
        })
        .collect();
    for g in list("glyph_images") {
        check.push(format!(
            "{} ({}) 글자는 NK-Editor 에 없어 그림으로 넣었습니다({}곳). 편집기에서 글자로 고칠 수 없으니, 고쳐야 하면 Word 원본에서 고친 뒤 다시 변환해 주세요",
            g["text"].as_str().unwrap_or(""),
            g["codepoints"].as_str().unwrap_or(""),
            g["count"]
        ));
    }
    if !check.is_empty() {
        ui.add_space(4.0);
        Frame::new()
            .fill(p.warn_soft)
            .corner_radius(14)
            .inner_margin(Margin::symmetric(16, 14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.label(
                    RichText::new(format!("편집기에서 확인할 곳 {}곳", check.len()))
                        .font(bold(14.0))
                        .color(p.warn),
                );
                for l in &check {
                    ui.label(RichText::new(l).size(13.5).color(p.ink2));
                }
            });
    }
}

/// 이름과 숫자를 담은 회색 칩. 크기를 먼저 알려 줘야 `horizontal_wrapped` 가 줄을 바꾼다.
fn chip(ui: &mut Ui, p: Pal, label: &str, value: u64) {
    let painter = ui.painter();
    let name = painter.layout_no_wrap(label.to_owned(), FontId::proportional(13.0), p.ink2);
    let num = painter.layout_no_wrap(value.to_string(), bold(13.0), p.ink);
    let size = egui::vec2(name.size().x + 6.0 + num.size().x + 24.0, 30.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(8), p.card);
    let y = rect.center().y;
    let x = rect.left() + 12.0;
    let nw = name.size().x;
    painter.galley(Pos2::new(x, y - name.size().y / 2.0), name, p.ink2);
    painter.galley(Pos2::new(x + nw + 6.0, y - num.size().y / 2.0), num, p.ink);
}

/// 글자 하나를 담는 작은 네모.
fn glyph_box(ui: &mut Ui, p: Pal, text: &str) {
    let g = ui
        .painter()
        .layout_no_wrap(text.to_owned(), bold(15.0), p.ink);
    let size = egui::vec2(g.size().x.max(14.0) + 14.0, 26.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(6), p.card);
    painter.galley(rect.center() - g.size() / 2.0, g, p.ink);
}

/// 저장한 파일을 파일 탐색기에서 보여 준다.
fn reveal(path: &Path) {
    #[cfg(windows)]
    let result = std::process::Command::new("explorer")
        .arg("/select,")
        .arg(path)
        .spawn();
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open")
        .arg("-R")
        .arg(path)
        .spawn();
    #[cfg(not(any(windows, target_os = "macos")))]
    let result = std::process::Command::new("xdg-open")
        .arg(path.parent().unwrap_or(Path::new(".")))
        .spawn();
    let _ = result;
}

fn is_docx(p: &Path) -> bool {
    p.is_file()
        && p.extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("docx"))
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
