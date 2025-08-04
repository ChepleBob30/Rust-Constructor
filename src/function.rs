//! function.rs is the functional module of the Rust Constructor, including function declarations, struct definitions, and some auxiliary content.
use anyhow::Context;
use eframe::{emath::Rect, epaint::Stroke, epaint::textures::TextureOptions};
use egui::{Color32, FontData, FontDefinitions, FontId, Frame, PointerButton, Pos2, Ui, Vec2};
use json::JsonValue;
use kira::{
    manager::{AudioManager, backend::cpal},
    sound::static_sound::StaticSoundData,
};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
    vec::Vec,
};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{
        Menu, MenuItem, PredefinedMenuItem,
        accelerator::{Accelerator, Modifiers},
    },
};

// import for macos status bar.

// #[cfg(target_os = "macos")]
// use objc2::sel;
// #[cfg(target_os = "macos")]
// use objc2_app_kit::{NSStatusItem};

pub fn load_icon_from_file(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Ok(Icon::from_rgba(rgba, width, height)?)
}

// 创建格式化的JSON文件
#[allow(dead_code)]
pub fn create_json<P: AsRef<Path>>(path: P, data: JsonValue) -> anyhow::Result<()> {
    let parent_dir = path
        .as_ref()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("无效的文件路径"))?;

    // 创建父目录（如果不存在）
    fs::create_dir_all(parent_dir)?;

    // 生成带缩进的JSON字符串（4空格缩进）
    let formatted = json::stringify_pretty(data, 4);

    // 写入文件（自动处理换行符）
    fs::write(path, formatted)?;
    Ok(())
}

// 复制并重新格式化JSON文件
#[allow(dead_code)]
pub fn copy_and_reformat_json<P: AsRef<Path>>(src: P, dest: P) -> anyhow::Result<()> {
    // 读取原始文件
    let content = fs::read_to_string(&src)?;

    // 解析JSON（自动验证格式）
    let parsed = json::parse(&content)?;

    // 使用格式化写入新文件
    create_json(dest, parsed)?;

    Ok(())
}

pub fn check_file_exists<P: AsRef<Path>>(path: P) -> bool {
    let path_ref = path.as_ref();
    if path_ref.exists() {
        true // 文件已存在时直接返回，不执行写入
    } else {
        // 文件不存在时，返回 false
        false
    }
}

// 通用 JSON 写入函数
#[allow(dead_code)]
pub fn write_to_json<P: AsRef<Path>>(path: P, data: JsonValue) -> anyhow::Result<()> {
    let parent_dir = path
        .as_ref()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("无效的文件路径"))?;

    fs::create_dir_all(parent_dir)?;
    let formatted = json::stringify_pretty(data, 4);
    fs::write(path, formatted)?;
    Ok(())
}

// 通用 JSON 读取函数
pub fn read_from_json<P: AsRef<Path>>(path: P) -> anyhow::Result<JsonValue> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("无法读取文件: {}", path.as_ref().display()))?;
    json::parse(&content).with_context(|| format!("解析 JSON 失败: {}", path.as_ref().display()))
}

pub fn play_wav(path: &str) -> anyhow::Result<f64> {
    let mut manager: kira::manager::AudioManager<cpal::CpalBackend> =
        AudioManager::new(kira::manager::AudioManagerSettings::default())?;
    let sound_data = StaticSoundData::from_file(path, Default::default())?;
    let duration = sound_data.duration().as_secs_f64();
    manager.play(sound_data)?;
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
    Ok(duration)
}

pub fn general_click_feedback() {
    std::thread::spawn(|| {
        play_wav("Resources/assets/sounds/Click.wav").unwrap();
    });
}

#[allow(dead_code)]
pub fn count_files_recursive(dir: &Path, target: &str) -> std::io::Result<usize> {
    let mut count = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                count += count_files_recursive(&path, target)?;
            } else if path.file_name().unwrap().to_string_lossy().contains(target) {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[allow(dead_code)]
pub fn list_files_recursive(path: &Path, prefix: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut matches = Vec::new();

    for entry in std::fs::read_dir(path)? {
        // 遍历目录
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // 递归处理子目录
            matches.extend(list_files_recursive(&path, prefix)?);
        } else if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().contains(prefix) {
                matches.push(path);
            }
        }
    }

    Ok(matches)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub language: u8,
    pub amount_languages: u8,
    pub rc_strict_mode: bool,
    pub enable_debug_mode: bool,
}

impl Config {
    pub fn from_json_value(value: &JsonValue) -> Option<Config> {
        Some(Config {
            language: value["language"].as_u8()?,
            amount_languages: value["amount_languages"].as_u8()?,
            rc_strict_mode: value["rc_strict_mode"].as_bool()?,
            enable_debug_mode: value["enable_debug_mode"].as_bool()?,
        })
    }

    #[allow(dead_code)]
    pub fn to_json_value(&self) -> JsonValue {
        json::object! {
            language: self.language,
            amount_languages: self.amount_languages,
            rc_strict_mode: self.rc_strict_mode,
            enable_debug_mode: self.enable_debug_mode,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameText {
    pub game_text: HashMap<String, Vec<String>>,
}

impl GameText {
    pub fn from_json_value(value: &JsonValue) -> Option<GameText> {
        // 检查 game_text 字段是否为对象
        if !value["game_text"].is_object() {
            return None;
        }

        // 遍历对象键值对
        let mut parsed = HashMap::new();
        for (key, val) in value["game_text"].entries() {
            if let JsonValue::Array(arr) = val {
                let str_vec: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                parsed.insert(key.to_string(), str_vec);
            }
        }

        Some(GameText { game_text: parsed })
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Value {
    Bool(bool),
    Int(i32),
    UInt(u32),
    Float(f32),
    Vec(Vec<Value>),
    String(String),
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Value::UInt(u)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Float(f)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::Vec(v.into_iter().map(|x| x.into()).collect())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

#[derive(Clone, Debug)]
pub struct ReportState {
    pub current_page: String,
    pub current_total_runtime: f32,
    pub current_page_runtime: f32,
}

#[derive(Clone, Debug)]
pub struct Problem {
    pub severity_level: SeverityLevel,
    pub problem: String,
    pub annotation: String,
    pub report_state: ReportState,
}

#[derive(Clone, Debug)]
pub enum SeverityLevel {
    #[allow(dead_code)]
    MildWarning,
    SevereWarning,
    Error,
}

pub trait RustConstructorResource {
    #[allow(dead_code)]
    fn name(&self) -> &str;

    fn expose_type(&self) -> &str;

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>);

    fn match_resource(&self, resource_name: &str, resource_type: &str) -> bool {
        resource_name == self.name() && resource_type == self.expose_type()
    }
}

impl RustConstructorResource for PageData {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct PageData {
    pub discern_type: String,
    pub name: String,
    pub forced_update: bool,
    pub change_page_updated: bool,
    pub enter_page_updated: bool,
}

#[derive(Clone, Debug)]
pub struct Timer {
    pub start_time: f32,
    pub total_time: f32,
    pub timer: Instant,
    pub now_time: f32,
}

impl RustConstructorResource for ImageTexture {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    pub discern_type: String,
    pub name: String,
    pub texture: Option<egui::TextureHandle>,
    pub cite_path: String,
}

impl RustConstructorResource for CustomRect {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct CustomRect {
    pub discern_type: String,
    pub name: String,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub rounding: f32,
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub center_display: [bool; 4],
    pub color: [u8; 4],
    pub border_width: f32,
    pub border_color: [u8; 4],
    pub origin_position: [f32; 2],
}

impl RustConstructorResource for Image {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone)]
pub struct Image {
    pub discern_type: String,
    pub name: String,
    pub image_texture: Option<egui::TextureHandle>,
    pub image_position: [f32; 2],
    pub image_size: [f32; 2],
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub center_display: [bool; 4],
    pub alpha: u8,
    pub overlay_color: [u8; 4],
    pub use_overlay_color: bool,
    pub origin_position: [f32; 2],
    pub origin_cite_texture: String,
}

impl RustConstructorResource for Text {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub discern_type: String,
    pub name: String,
    pub text_content: String,
    pub font_size: f32,
    pub rgba: [u8; 4],
    pub position: [f32; 2],
    pub center_display: [bool; 4],
    pub wrap_width: f32,
    pub write_background: bool,
    pub background_rgb: [u8; 4],
    pub rounding: f32,
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub origin_position: [f32; 2],
}

impl RustConstructorResource for ScrollBackground {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct ScrollBackground {
    pub discern_type: String,
    pub name: String,
    pub image_name: Vec<String>,
    pub horizontal_or_vertical: bool,
    pub left_and_top_or_right_and_bottom: bool,
    pub scroll_speed: u32,
    pub boundary: f32,
    pub resume_point: f32,
}

impl RustConstructorResource for Variable {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub discern_type: String,
    pub name: String,
    pub value: Value,
}

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub discern_type: String,
    pub font_definitions: FontDefinitions,
    pub path: String,
}

impl RustConstructorResource for Font {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

impl RustConstructorResource for SplitTime {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
pub struct SplitTime {
    pub discern_type: String,
    pub name: String,
    pub time: [f32; 2],
}

impl RustConstructorResource for Switch {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Switch {
    pub discern_type: String,
    pub name: String,
    pub appearance: Vec<SwitchData>,
    pub switch_image_name: String,
    pub enable_hover_click_image: [bool; 2],
    pub state: u32,
    pub click_method: Vec<SwitchClickAction>,
    pub last_time_hovered: bool,
    pub last_time_clicked: bool,
    pub last_time_clicked_index: usize,
    pub animation_count: u32,
    pub hint_text: Vec<String>,
    pub hint_text_name: String,
}

#[derive(Clone, Debug)]
pub struct RenderResource {
    pub discern_type: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct SwitchData {
    pub texture: String,
    pub color: [u8; 4],
}

#[derive(Clone, Debug)]
pub struct SwitchClickAction {
    pub click_method: PointerButton,
    pub action: bool,
}

#[derive(Clone, Debug)]
pub struct MessageBox {
    pub discern_type: String,
    pub name: String,
    pub box_size: [f32; 2],
    pub box_content_name: String,
    pub box_title_name: String,
    pub box_image_name: String,
    pub box_keep_existing: bool,
    pub box_existing_time: f32,
    pub box_exist: bool,
    pub box_speed: f32,
    pub box_restore_speed: f32,
    pub box_memory_offset: f32,
}

impl RustConstructorResource for MessageBox {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name.to_string(),
        });
    }
}

#[derive(Clone)]
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum RCR {
    Image(Image),
    Text(Text),
    CustomRect(CustomRect),
    ScrollBackground(ScrollBackground),
    Variable(Variable),
    Font(Font),
    SplitTime(SplitTime),
    Switch(Switch),
    MessageBox(MessageBox),
    ImageTexture(ImageTexture),
    PageData(PageData),
}

#[derive(Clone)]
pub struct App {
    pub config: Config,
    pub game_text: GameText,
    pub rust_constructor_resource: Vec<RCR>,
    pub render_resource_list: Vec<RenderResource>,
    pub problem_list: Vec<Problem>,
    pub frame: Frame,
    pub vertrefresh: f32,
    pub page: String,
    pub timer: Timer,
    pub frame_times: Vec<f32>,
    pub last_frame_time: Option<f64>,
    pub tray_icon: Option<tray_icon::TrayIcon>,
    pub tray_icon_created: bool,
}

impl App {
    pub fn new() -> Self {
        let mut config = Config {
            language: 0,
            amount_languages: 0,
            rc_strict_mode: false,
            enable_debug_mode: false,
        };
        let mut game_text = GameText {
            game_text: HashMap::new(),
        };
        if let Ok(json_value) = read_from_json("Resources/config/Preferences.json") {
            if let Some(read_config) = Config::from_json_value(&json_value) {
                config = read_config;
            }
        }
        if let Ok(json_value) = read_from_json("Resources/config/GameText.json") {
            if let Some(read_game_text) = GameText::from_json_value(&json_value) {
                game_text = read_game_text;
            }
        }
        Self {
            config,
            game_text,
            rust_constructor_resource: vec![
                RCR::PageData(PageData {
                    discern_type: "PageData".to_string(),
                    name: "Launch".to_string(),
                    forced_update: true,
                    change_page_updated: false,
                    enter_page_updated: false,
                }),
                RCR::PageData(PageData {
                    discern_type: "PageData".to_string(),
                    name: "Demo_Desktop".to_string(),
                    forced_update: true,
                    change_page_updated: false,
                    enter_page_updated: false,
                }),
                RCR::PageData(PageData {
                    discern_type: "PageData".to_string(),
                    name: "Error".to_string(),
                    forced_update: true,
                    change_page_updated: false,
                    enter_page_updated: false,
                }),
            ],
            render_resource_list: Vec::new(),
            problem_list: Vec::new(),
            frame: Frame {
                ..Default::default()
            },
            vertrefresh: 0.01,
            page: "Launch".to_string(),
            timer: Timer {
                start_time: 0.0,
                total_time: 0.0,
                timer: Instant::now(),
                now_time: 0.0,
            },
            frame_times: Vec::new(),
            last_frame_time: None,
            tray_icon: None,
            tray_icon_created: false,
        }
    }

    // Dangerous!

    // #[cfg(target_os = "macos")]
    // pub fn create_macos_status_bar(&mut self) {
    //     unsafe {
    //         use objc2::{MainThreadMarker, MainThreadOnly};
    //         use objc2_foundation::{NSString};
    //         use objc2_app_kit::{NSApp, NSMenu, NSMenuItem};

    //         // 获取主应用菜单
    //         let main_menu = NSMenu::new(MainThreadMarker::new().unwrap());

    //         // 创建 RC 菜单标题
    //         let rc_menu_title = NSString::from_str("RC");
    //         let rc_menu_item = NSMenuItem::initWithTitle_action_keyEquivalent(
    //             NSMenuItem::alloc(MainThreadMarker::new().unwrap()),
    //             &rc_menu_title,
    //             None,
    //             &NSString::from_str(""),
    //         );

    //         // 创建 RC 菜单
    //         let rc_menu = NSMenu::new(MainThreadMarker::new().unwrap());

    //         // 创建"播放提示音效"菜单项，不设置 action，稍后通过其他方式处理
    //         let play_sound_title = NSString::from_str("播放提示音效");
    //         let play_sound_item = NSMenuItem::initWithTitle_action_keyEquivalent(
    //             NSMenuItem::alloc(MainThreadMarker::new().unwrap()),
    //             &play_sound_title,
    //             Some(sel!(play_sound)), // 暂时不设置 action
    //             &NSString::from_str(""),
    //         );
    //         rc_menu.addItem(&play_sound_item);

    //         // 添加分隔符
    //         let separator = NSMenuItem::separatorItem(MainThreadMarker::new().unwrap());
    //         rc_menu.addItem(&separator);

    //         // 创建"退出"菜单项
    //         let quit_title = NSString::from_str("退出");
    //         let quit_item = NSMenuItem::initWithTitle_action_keyEquivalent(
    //             NSMenuItem::alloc(MainThreadMarker::new().unwrap()),
    //             &quit_title,
    //             Some(sel!(terminate:)),
    //             &NSString::from_str(""),
    //         );
    //         rc_menu.addItem(&quit_item);

    //         // 将 RC 菜单设置到 RC 菜单项
    //         rc_menu_item.setSubmenu(Some(&rc_menu));

    //         // 将 RC 菜单项添加到主菜单
    //         main_menu.addItem(&rc_menu_item);

    //         // 将主菜单设置为应用的主菜单
    //         NSApp(MainThreadMarker::new().unwrap()).setMainMenu(Some(&main_menu));
    //     }
    // }

    pub fn switch_page(&mut self, page: &str) {
        self.page = page.to_string();
        if let Ok(id) = self.get_resource_index("PageData", page) {
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                pd.change_page_updated = false;
                self.timer.start_time = self.timer.total_time;
                self.update_timer();
            };
        };
    }

    pub fn tray_icon_init(&mut self) {
        let icon = load_icon_from_file("Resources/assets/images/tray_icon.png").unwrap();
        // 创建菜单
        let tray_menu = Menu::new();
        let show_window_item = MenuItem::new("播放提示音效！", true, None);
        let quit_item = MenuItem::new(
            "退出",
            true,
            Some(Accelerator::new(
                Some(Modifiers::SUPER),
                tray_icon::menu::accelerator::Code::KeyQ,
            )),
        );
        tray_menu
            .append_items(&[
                &show_window_item,
                &PredefinedMenuItem::separator(),
                &quit_item,
            ])
            .unwrap();
        match TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Rust Constructor")
            .with_icon(icon)
            .build()
        {
            Ok(tray_icon) => {
                self.tray_icon = Some(tray_icon);
                self.tray_icon_created = true;
            }
            Err(e) => {
                eprintln!("Failed to create tray icon: {}", e);
            }
        };
    }

    pub fn launch_page_preload(&mut self, ctx: &egui::Context) {
        self.tray_icon_init();
        self.add_fonts("Title", "Resources/assets/fonts/Title.otf");
        self.add_fonts("Content", "Resources/assets/fonts/Content.ttf");
        self.register_all_fonts(ctx);
        self.add_image_texture(
            "Error",
            "Resources/assets/images/error.png",
            [false, false],
            true,
            ctx,
        );
        self.add_image_texture(
            "RC_Logo",
            "Resources/assets/images/rc.png",
            [false, false],
            true,
            ctx,
        );
        self.add_image(
            "Error",
            [0_f32, 0_f32, 130_f32, 130_f32],
            [1, 2, 1, 2],
            [true, true, true, true, false],
            [255, 0, 0, 0, 0],
            "Error",
        );
        self.add_image(
            "RC_Logo",
            [0_f32, 0_f32, 130_f32, 130_f32],
            [1, 2, 1, 3],
            [false, false, true, true, false],
            [255, 0, 0, 0, 0],
            "RC_Logo",
        );
        self.add_rect(
            "Launch_Background",
            [
                0_f32,
                0_f32,
                ctx.available_rect().width(),
                ctx.available_rect().height(),
                0_f32,
            ],
            [1, 2, 1, 2],
            [false, false, true, true],
            [0, 0, 0, 255, 255, 255, 255, 255],
            0.0,
        );
        std::thread::spawn(|| {
            play_wav("Resources/assets/sounds/Launch.wav").unwrap();
        });
        self.add_rect(
            "Error_Pages_Background",
            [
                0_f32,
                0_f32,
                ctx.available_rect().width(),
                ctx.available_rect().height(),
                0_f32,
            ],
            [1, 2, 1, 2],
            [false, false, true, true],
            [31, 103, 179, 255, 255, 255, 255, 255],
            0.0,
        );
        self.add_text(
            ["Error_Pages_Sorry", ":("],
            [0_f32, 0_f32, 100_f32, 1000_f32, 0.0],
            [255, 255, 255, 255, 0, 0, 0, 255],
            [true, true, false, false],
            false,
            [1, 5, 1, 6],
        );
        self.add_text(
            ["Error_Pages_Reason", ""],
            [0_f32, 0_f32, 40_f32, 1000_f32, 0.0],
            [255, 255, 255, 255, 0, 0, 0, 255],
            [true, true, false, false],
            false,
            [1, 5, 2, 6],
        );
        self.add_text(
            ["Error_Pages_Solution", ""],
            [0_f32, 0_f32, 20_f32, 1000_f32, 0.0],
            [255, 255, 255, 255, 0, 0, 0, 255],
            [true, true, false, false],
            false,
            [1, 5, 3, 6],
        );
        self.add_rect(
            "Cut_To_Background",
            [
                0_f32,
                0_f32,
                ctx.available_rect().width(),
                ctx.available_rect().height(),
                0_f32,
            ],
            [1, 2, 1, 2],
            [false, false, true, true],
            [0, 0, 0, 0, 255, 255, 255, 255],
            0.0,
        );
        self.add_image_texture(
            "Close_Message_Box",
            "Resources/assets/images/close_message_box.png",
            [false, false],
            true,
            ctx,
        );
    }

    pub fn get_resource_index(&self, resource_type: &str, resource_name: &str) -> Result<usize, ()> {
        for i in 0..self.rust_constructor_resource.len() {
            match self.rust_constructor_resource[i].clone() {
                RCR::Image(im) => 
                    if im.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::Text(t) => 
                    if t.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                RCR::CustomRect(cr) => 
                    if cr.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::ScrollBackground(sb) => 
                    if sb.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::Variable(v) => 
                    if v.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::Font(f) => 
                    if f.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::SplitTime(st) => 
                    if st.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::Switch(s) =>
                    if s.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::MessageBox(mb) =>
                    if mb.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::ImageTexture(it) =>
                    if it.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
                RCR::PageData(pd) => 
                    if pd.match_resource(resource_name, resource_type) {
                        return Ok(i)
                    }
            };
        };
        Err(())
    }

    pub fn add_fonts(&mut self, font_name: &str, font_path: &str) {
        let mut fonts = FontDefinitions::default();
        if let Ok(font_read_data) = std::fs::read(font_path) {
            let font_data: Arc<Vec<u8>> = Arc::new(font_read_data);
            fonts.font_data.insert(
                font_name.to_owned(),
                Arc::new(FontData::from_owned(
                    Arc::try_unwrap(font_data).ok().unwrap(),
                )),
            );

            // 将字体添加到字体列表中
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font_name.to_owned());

            self.rust_constructor_resource.push(RCR::Font(Font {
                name: font_name.to_string(),
                discern_type: "Font".to_string(),
                font_definitions: fonts,
                path: font_path.to_string(),
            }));
        } else if self.config.rc_strict_mode {
            panic!(
                "{}: \"{}\"",
                self.game_text.game_text["error_font_read_failed"][self.config.language as usize],
                font_path
            )
        } else {
            self.problem_report(
                &format!(
                    "{}: \"{}\"",
                    self.game_text.game_text["error_font_read_failed"]
                        [self.config.language as usize],
                    font_path
                ),
                SeverityLevel::SevereWarning,
                &self.game_text.game_text["error_font_read_failed_annotation"]
                    [self.config.language as usize]
                    .clone(),
            );
        };
        // 应用字体定义
        // ctx.set_fonts(fonts);
    }

    pub fn font(&mut self, name: &str) -> Result<FontDefinitions, ()> {
        if self.get_resource_index("Font", name).is_err() {
            if self.config.rc_strict_mode {
                panic!(
                    "{}: \"{}\"",
                    self.game_text.game_text["error_font_get_failed"]
                        [self.config.language as usize],
                    name
                )
            } else {
                self.problem_report(
                    &format!(
                        "{}: \"{}\"",
                        self.game_text.game_text["error_font_get_failed"]
                            [self.config.language as usize],
                        name
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_font_get_failed_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            };
        } else if let Ok(id) = self.get_resource_index("Font", name) {
            if let RCR::Font(f) = &mut self.rust_constructor_resource[id] {
                return Ok(f.font_definitions.clone())
            }
        }
        Err(())
    }

    pub fn register_all_fonts(&mut self, ctx: &egui::Context) {
        let mut font_definitions = egui::FontDefinitions::default();
        let mut font_resources = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let RCR::Font(f) = &self.rust_constructor_resource[i] {
                font_resources.push(f.clone());
            };
        };
        for i in &font_resources {
            let font_name = i.name.clone();
            // 获取字体数据（返回 FontDefinitions）
            if let Ok(font_def) = self.font(&font_name) {
                // 从 font_def 中提取对应字体的 Arc<FontData>
                if let Some(font_data) = font_def.font_data.get(&font_name) {
                    font_definitions
                        .font_data
                        .insert(font_name.clone(), Arc::clone(font_data));
                    font_definitions
                        .families
                        .entry(egui::FontFamily::Name(font_name.clone().into()))
                        .or_default()
                        .push(font_name.clone());
                };

                // 将字体添加到字体列表中
                font_definitions
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.to_owned());

                font_definitions
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_name.to_owned());
            };
        };
        ctx.set_fonts(font_definitions);
    }

    pub fn fade(
        &mut self,
        fade_in_or_out: bool,
        ctx: &egui::Context,
        ui: &mut Ui,
        split_time_name: &str,
        resource_name: &str,
        fade_speed: u8,
    ) -> Result<u8, ()> {
        if let Ok(id) = self.get_resource_index("CustomRect", resource_name) {
            if let RCR::CustomRect(mut rect) = self.rust_constructor_resource[id].clone() {
                rect.size =
                [ctx.available_rect().width(), ctx.available_rect().height()];
                if self.timer.now_time - self.split_time(split_time_name).unwrap()[0] >= self.vertrefresh {
                    self.add_split_time(split_time_name, true);
                    if fade_in_or_out {
                        rect.color[3] =
                            if rect.color[3] > 255 - fade_speed {
                                255
                            } else {
                                rect.color[3] + fade_speed
                            };
                    } else {
                        rect.color[3] =
                            rect.color[3].saturating_sub(fade_speed)
                    };
                };
                self.rect(ui, resource_name, ctx);
                self.rust_constructor_resource[id] = RCR::CustomRect(rect.clone());
                Ok(rect.color[3])
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn problem_report(
        &mut self,
        problem: &str,
        severity_level: SeverityLevel,
        annotation: &str,
    ) {
        std::thread::spawn(|| {
            play_wav("Resources/assets/sounds/Error.wav").unwrap();
        });
        self.problem_list.push(Problem {
            severity_level,
            problem: problem.to_string(),
            annotation: annotation.to_string(),
            report_state: ReportState {
                current_page: self.page.clone(),
                current_total_runtime: self.timer.total_time,
                current_page_runtime: self.timer.now_time,
            },
        });
    }

    pub fn check_updated(&mut self, name: &str) -> Result<bool, ()> {
        if let Ok(id) = self.get_resource_index("PageData", name) {
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                if pd.change_page_updated {
                    Ok(true)
                } else {
                    self.new_page_update(name);
                    Ok(false)
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn check_enter_updated(&mut self, name: &str) -> Result<bool, ()> {
        if let Ok(id) = self.get_resource_index("PageData", name) {
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                let return_value = pd.enter_page_updated;
                pd.enter_page_updated = true;
                Ok(return_value)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn new_page_update(&mut self, name: &str) {
        self.timer.start_time = self.timer.total_time;
        self.update_timer();
        if let Ok(id) = self.get_resource_index("PageData", name) {
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                pd.change_page_updated = true;
            };
        };
    }

    pub fn update_frame_stats(&mut self, ctx: &egui::Context) {
        let current_time = ctx.input(|i| i.time);
        if let Some(last) = self.last_frame_time {
            let delta = (current_time - last) as f32;
            self.frame_times.push(delta);
            const MAX_SAMPLES: usize = 120;
            if self.frame_times.len() > MAX_SAMPLES {
                let remove_count = self.frame_times.len() - MAX_SAMPLES;
                self.frame_times.drain(0..remove_count);
            }
        }
        self.last_frame_time = Some(current_time);
    }

    pub fn current_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            1.0 / (self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32)
        }
    }

    pub fn add_split_time(&mut self, name: &str, reset: bool) {
        if reset {
            if let Ok(id) = self.get_resource_index("SplitTime", name) {
                if let RCR::SplitTime(st) = &mut self.rust_constructor_resource[id] {
                    st.time = [self.timer.now_time, self.timer.total_time];
                };
            };
        } else {
            self.rust_constructor_resource.push(RCR::SplitTime(SplitTime {
                discern_type: "SplitTime".to_string(),
                name: name.to_string(),
                time: [self.timer.now_time, self.timer.total_time],
            }));
        };
    }

    pub fn split_time(&mut self, name: &str) -> Result<[f32; 2], ()> {
        if let Ok(id) = self.get_resource_index("SplitTime", name) {
            if let RCR::SplitTime(st) = self.rust_constructor_resource[id].clone() {
                Ok(st.time)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn update_timer(&mut self) {
        let elapsed = self.timer.timer.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();
        self.timer.total_time = seconds as f32 + milliseconds as f32 / 1000.0;
        self.timer.now_time = self.timer.total_time - self.timer.start_time
    }

    pub fn add_rect(
        &mut self,
        name: &str,
        position_size_and_rounding: [f32; 5],
        grid: [u32; 4],
        center_display: [bool; 4],
        color: [u8; 8],
        border_width: f32,
    ) {
        self.rust_constructor_resource.push(RCR::CustomRect(CustomRect {
            discern_type: "CustomRect".to_string(),
            name: name.to_string(),
            position: [position_size_and_rounding[0], position_size_and_rounding[1]],
            size: [position_size_and_rounding[2], position_size_and_rounding[3]],
            rounding: position_size_and_rounding[4],
            x_grid: [grid[0], grid[1]],
            y_grid: [grid[2], grid[3]],
            center_display,
            color: [color[0], color[1], color[2], color[3]],
            border_width,
            border_color: [color[4], color[5], color[6], color[7]],
            origin_position: [position_size_and_rounding[0], position_size_and_rounding[1]],
        }));
    }

    pub fn rect(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("CustomRect", name) {
            if let RCR::CustomRect(cr) = &mut self.rust_constructor_resource[id] {
                cr.reg_render_resource(&mut self.render_resource_list);
                cr.position[0] = match cr.x_grid[1] {
                    0 => cr.origin_position[0],
                    _ => {
                        (ctx.available_rect().width() as f64 / cr.x_grid[1] as f64
                            * cr.x_grid[0] as f64) as f32
                            + cr.origin_position[0]
                    }
                };
                cr.position[1] = match cr.y_grid[1] {
                    0 => cr.origin_position[1],
                    _ => {
                        (ctx.available_rect().height() as f64 / cr.y_grid[1] as f64
                            * cr.y_grid[0] as f64) as f32
                            + cr.origin_position[1]
                    }
                };
                let pos_x;
                let pos_y;
                if cr.center_display[2] {
                    pos_x = cr.position[0] - cr.size[0] / 2.0;
                } else if cr.center_display[0] {
                    pos_x = cr.position[0];
                } else {
                    pos_x = cr.position[0] - cr.size[0];
                };
                if cr.center_display[3] {
                    pos_y = cr.position[1] - cr.size[1] / 2.0;
                } else if cr.center_display[1] {
                    pos_y = cr.position[1];
                } else {
                    pos_y = cr.position[1] - cr.size[1];
                };
                ui.painter().rect(
                    Rect::from_min_max(
                        Pos2::new(pos_x, pos_y),
                        Pos2::new(
                            pos_x + cr.size[0],
                            pos_y + cr.size[1],
                        ),
                    ),
                    cr.rounding,
                    Color32::from_rgba_unmultiplied(
                        cr.color[0],
                        cr.color[1],
                        cr.color[2],
                        cr.color[3],
                    ),
                    Stroke {
                        width: cr.border_width,
                        color: Color32::from_rgba_unmultiplied(
                            cr.border_color[0],
                            cr.border_color[1],
                            cr.border_color[2],
                            cr.border_color[3],
                        ),
                    },
                    egui::StrokeKind::Inside,
                );
            };
        };
    }

    pub fn add_text(
        &mut self,
        name_and_content: [&str; 2],
        position_font_size_wrap_width_rounding: [f32; 5],
        color: [u8; 8],
        center_display: [bool; 4],
        write_background: bool,
        grid: [u32; 4],
    ) {
        self.rust_constructor_resource.push(RCR::Text(Text {
            discern_type: "Text".to_string(),
            name: name_and_content[0].to_string(),
            text_content: name_and_content[1].to_string(),
            font_size: position_font_size_wrap_width_rounding[2],
            rgba: [color[0], color[1], color[2], color[3]],
            position: [
                position_font_size_wrap_width_rounding[0],
                position_font_size_wrap_width_rounding[1],
            ],
            center_display,
            wrap_width: position_font_size_wrap_width_rounding[3],
            write_background,
            background_rgb: [color[4], color[5], color[6], color[7]],
            rounding: position_font_size_wrap_width_rounding[4],
            x_grid: [grid[0], grid[1]],
            y_grid: [grid[2], grid[3]],
            origin_position: [
                position_font_size_wrap_width_rounding[0],
                position_font_size_wrap_width_rounding[1],
            ],
        }));
    }

    pub fn text(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Text", name) {
            if let RCR::Text(t) = &mut self.rust_constructor_resource[id] {
                t.reg_render_resource(&mut self.render_resource_list);
                // 计算文本大小
                let galley = ui.fonts(|f| {
                    f.layout(
                        t.text_content.to_string(),
                        FontId::proportional(t.font_size),
                        Color32::from_rgba_unmultiplied(
                            t.rgba[0],
                            t.rgba[1],
                            t.rgba[2],
                            t.rgba[3],
                        ),
                        t.wrap_width,
                    )
                });
                let text_size = galley.size();
                t.position[0] = match t.x_grid[1] {
                    0 => t.origin_position[0],
                    _ => {
                        (ctx.available_rect().width() as f64 / t.x_grid[1] as f64
                            * t.x_grid[0] as f64) as f32
                            + t.origin_position[0]
                    }
                };
                t.position[1] = match t.y_grid[1] {
                    0 => t.origin_position[1],
                    _ => {
                        (ctx.available_rect().height() as f64 / t.y_grid[1] as f64
                            * t.y_grid[0] as f64) as f32
                            + t.origin_position[1]
                    }
                };
                let pos_x;
                let pos_y;
                if t.center_display[2] {
                    pos_x = t.position[0] - text_size.x / 2.0;
                } else if t.center_display[0] {
                    pos_x = t.position[0];
                } else {
                    pos_x = t.position[0] - text_size.x;
                };
                if t.center_display[3] {
                    pos_y = t.position[1] - text_size.y / 2.0;
                } else if t.center_display[1] {
                    pos_y = t.position[1];
                } else {
                    pos_y = t.position[1] - text_size.y;
                };
                // 使用绝对定位放置文本
                let position = Pos2::new(pos_x, pos_y);
                if t.write_background {
                    let rect = Rect::from_min_size(position, text_size);
                    // 绘制背景颜色
                    ui.painter().rect_filled(
                        rect,
                        t.rounding,
                        Color32::from_rgba_unmultiplied(
                            t.background_rgb[0],
                            t.background_rgb[1],
                            t.background_rgb[2],
                            t.background_rgb[3],
                        ),
                    ); // 背景色
                };
                // 绘制文本
                ui.painter().galley(
                    position,
                    galley,
                    Color32::from_rgba_unmultiplied(
                        t.rgba[0],
                        t.rgba[1],
                        t.rgba[2],
                        t.rgba[3], // 应用透明度
                    ),
                );
            };
        };
    }

    pub fn get_text_size(&mut self, resource_name: &str, ui: &mut Ui) -> Result<[f32; 2], ()> {
        if let Ok(id) = self.get_resource_index("Text", resource_name) {
            if let RCR::Text(t) = self.rust_constructor_resource[id].clone() {
                let galley = ui.fonts(|f| {
                    f.layout(
                        t.text_content.to_string(),
                        FontId::proportional(t.font_size),
                        Color32::from_rgba_unmultiplied(
                            t.rgba[0],
                            t.rgba[1],
                            t.rgba[2],
                            t.rgba[3],
                        ),
                        t.wrap_width,
                    )
                });
                Ok([galley.size().x, galley.size().y])
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn read_image_to_vec(&mut self, path: &str) -> Vec<u8> {
        let mut file =
            File::open(path).unwrap_or(File::open("Resources/assets/images/error.png").unwrap());
        if !check_file_exists(path) {
            if self.config.rc_strict_mode {
                panic!(
                    "{}: {}",
                    self.game_text.game_text["error_image_open_failed"]
                        [self.config.language as usize],
                    path
                );
            } else {
                self.problem_report(
                    &format!(
                        "{}: {}",
                        self.game_text.game_text["error_image_open_failed"]
                            [self.config.language as usize],
                        path
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_image_open_failed_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            };
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    pub fn add_var<T: Into<Value>>(&mut self, name: &str, value: T) {
        self.rust_constructor_resource.push(RCR::Variable(Variable {
            discern_type: "Variable".to_string(),
            name: name.to_string(),
            value: value.into(),
        }));
    }

    pub fn modify_var<T: Into<Value>>(&mut self, name: &str, value: T) {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = &mut self.rust_constructor_resource[id] {
                v.value = value.into();
            };
        };
    }

    #[allow(dead_code)]
    pub fn var(&mut self, name: &str) -> Result<Value, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                Ok(v.clone().value)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_i(&mut self, name: &str) -> Result<i32, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Int(i) => Ok(*i),
                    _ => {
                        if self.config.rc_strict_mode {
                            panic!(
                                "\"{}\" {}",
                                name,
                                self.game_text.game_text["error_variable_not_i32_type"]
                                    [self.config.language as usize]
                                    .clone()
                            );
                        } else {
                            self.problem_report(
                                &format!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_i32_type"]
                                        [self.config.language as usize]
                                        .clone()
                                ),
                                SeverityLevel::SevereWarning,
                                &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                    [self.config.language as usize]
                                    .clone(),
                            );
                        }
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_i32_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_i32_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            }
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_u(&mut self, name: &str) -> Result<u32, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
                if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                    match &v.value {
                        // 直接访问 value 字段
                        Value::UInt(u) => Ok(*u),
                        _ => {
                            if self.config.rc_strict_mode {
                                panic!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_u32_type"]
                                        [self.config.language as usize]
                                        .clone()
                                );
                            } else {
                                self.problem_report(
                                    &format!(
                                        "\"{}\" {}",
                                        name,
                                        self.game_text.game_text["error_variable_not_u32_type"]
                                            [self.config.language as usize]
                                            .clone()
                                    ),
                                    SeverityLevel::SevereWarning,
                                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                        [self.config.language as usize]
                                        .clone(),
                                );
                            }
                            Err(())
                        }
                    }
                } else {
                    Err(())
                }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_u32_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_u32_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            };
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_f(&mut self, name: &str) -> Result<f32, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Float(f) => Ok(*f),
                    _ => {
                        if self.config.rc_strict_mode {
                            panic!(
                                "\"{}\" {}",
                                name,
                                self.game_text.game_text["error_variable_not_f32_type"]
                                    [self.config.language as usize]
                                    .clone()
                            );
                        } else {
                            self.problem_report(
                                &format!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_f32_type"]
                                        [self.config.language as usize]
                                        .clone()
                                ),
                                SeverityLevel::SevereWarning,
                                &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                    [self.config.language as usize]
                                    .clone(),
                            );
                        }
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_f32_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_f32_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            }
            Err(())
        }
    }

    pub fn var_b(&mut self, name: &str) -> Result<bool, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Bool(b) => Ok(*b),
                    _ => {
                        if self.config.rc_strict_mode {
                            panic!(
                                "\"{}\" {}",
                                name,
                                self.game_text.game_text["error_variable_not_bool_type"]
                                    [self.config.language as usize]
                                    .clone()
                            );
                        } else {
                            self.problem_report(
                                &format!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_bool_type"]
                                        [self.config.language as usize]
                                        .clone()
                                ),
                                SeverityLevel::SevereWarning,
                                &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                    [self.config.language as usize]
                                    .clone(),
                            );
                        }
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_bool_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_bool_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            }
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_v(&mut self, name: &str) -> Result<Vec<Value>, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Vec(v) => Ok(v.clone()),
                    _ => {
                        if self.config.rc_strict_mode {
                            panic!(
                                "\"{}\" {}",
                                name,
                                self.game_text.game_text["error_variable_not_vec_type"]
                                    [self.config.language as usize]
                                    .clone()
                            );
                        } else {
                            self.problem_report(
                                &format!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_vec_type"]
                                        [self.config.language as usize]
                                        .clone()
                                ),
                                SeverityLevel::SevereWarning,
                                &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                    [self.config.language as usize]
                                    .clone(),
                            );
                        }
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_vec_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_vec_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            }
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_s(&mut self, name: &str) -> Result<String, ()> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::String(s) => Ok(s.clone()),
                    _ => {
                        if self.config.rc_strict_mode {
                            panic!(
                                "\"{}\" {}",
                                name,
                                self.game_text.game_text["error_variable_not_string_type"]
                                    [self.config.language as usize]
                                    .clone()
                            );
                        } else {
                            self.problem_report(
                                &format!(
                                    "\"{}\" {}",
                                    name,
                                    self.game_text.game_text["error_variable_not_string_type"]
                                        [self.config.language as usize]
                                        .clone()
                                ),
                                SeverityLevel::SevereWarning,
                                &self.game_text.game_text["error_variable_wrong_type_annotation"]
                                    [self.config.language as usize]
                                    .clone(),
                            );
                        }
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            if self.config.rc_strict_mode {
                panic!(
                    "\"{}\" {}",
                    name,
                    self.game_text.game_text["error_variable_not_string_type"]
                        [self.config.language as usize]
                        .clone()
                );
            } else {
                self.problem_report(
                    &format!(
                        "\"{}\" {}",
                        name,
                        self.game_text.game_text["error_variable_not_string_type"]
                            [self.config.language as usize]
                            .clone()
                    ),
                    SeverityLevel::SevereWarning,
                    &self.game_text.game_text["error_variable_wrong_type_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
            }
            Err(())
        }
    }

    #[allow(dead_code)]
    pub fn var_decode_b(&mut self, target: Value) -> bool {
        match target {
            Value::Bool(b) => {
                // 处理布尔值
                b
            }
            _ => {
                if self.config.rc_strict_mode {
                    panic!(
                        "\"{:?}\" {}",
                        target,
                        self.game_text.game_text["error_variable_not_bool_type"]
                            [self.config.language as usize]
                            .clone()
                    );
                } else {
                    self.problem_report(
                        &format!(
                            "\"{:?}\" {}",
                            target,
                            self.game_text.game_text["error_variable_not_bool_type"]
                                [self.config.language as usize]
                                .clone()
                        ),
                        SeverityLevel::SevereWarning,
                        &self.game_text.game_text["error_variable_wrong_type_annotation"]
                            [self.config.language as usize]
                            .clone(),
                    );
                    false
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn var_decode_i(&mut self, target: Value) -> i32 {
        match target {
            Value::Int(i) => {
                // 处理i32整型
                i
            }
            _ => {
                if self.config.rc_strict_mode {
                    panic!(
                        "\"{:?}\" {}",
                        target,
                        self.game_text.game_text["error_variable_not_i32_type"]
                            [self.config.language as usize]
                            .clone()
                    );
                } else {
                    self.problem_report(
                        &format!(
                            "\"{:?}\" {}",
                            target,
                            self.game_text.game_text["error_variable_not_i32_type"]
                                [self.config.language as usize]
                                .clone()
                        ),
                        SeverityLevel::SevereWarning,
                        &self.game_text.game_text["error_variable_wrong_type_annotation"]
                            [self.config.language as usize]
                            .clone(),
                    );
                    0
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn var_decode_u(&mut self, target: Value) -> u32 {
        match target {
            Value::UInt(u) => {
                // 处理u32无符号整型
                u
            }
            _ => {
                if self.config.rc_strict_mode {
                    panic!(
                        "\"{:?}\" {}",
                        target,
                        self.game_text.game_text["error_variable_not_u32_type"]
                            [self.config.language as usize]
                            .clone()
                    );
                } else {
                    self.problem_report(
                        &format!(
                            "\"{:?}\" {}",
                            target,
                            self.game_text.game_text["error_variable_not_u32_type"]
                                [self.config.language as usize]
                                .clone()
                        ),
                        SeverityLevel::SevereWarning,
                        &self.game_text.game_text["error_variable_wrong_type_annotation"]
                            [self.config.language as usize]
                            .clone(),
                    );
                    0
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn var_decode_f(&mut self, target: Value) -> f32 {
        match target {
            Value::Float(f) => {
                // 处理浮点数
                f
            }
            _ => {
                if self.config.rc_strict_mode {
                    panic!(
                        "\"{:?}\" {}",
                        target,
                        self.game_text.game_text["error_variable_not_f32_type"]
                            [self.config.language as usize]
                            .clone()
                    );
                } else {
                    self.problem_report(
                        &format!(
                            "\"{:?}\" {}",
                            target,
                            self.game_text.game_text["error_variable_not_f32_type"]
                                [self.config.language as usize]
                                .clone()
                        ),
                        SeverityLevel::SevereWarning,
                        &self.game_text.game_text["error_variable_wrong_type_annotation"]
                            [self.config.language as usize]
                            .clone(),
                    );
                    0_f32
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn var_decode_s(&mut self, target: Value) -> String {
        match target {
            Value::String(s) => {
                // 处理字符串
                s
            }
            _ => {
                if self.config.rc_strict_mode {
                    panic!(
                        "\"{:?}\" {}",
                        target,
                        self.game_text.game_text["error_variable_not_string_type"]
                            [self.config.language as usize]
                            .clone()
                    );
                } else {
                    self.problem_report(
                        &format!(
                            "\"{:?}\" {}",
                            target,
                            self.game_text.game_text["error_variable_not_string_type"]
                                [self.config.language as usize]
                                .clone()
                        ),
                        SeverityLevel::SevereWarning,
                        &self.game_text.game_text["error_variable_wrong_type_annotation"]
                            [self.config.language as usize]
                            .clone(),
                    );
                    String::new()
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_scroll_background(
        &mut self,
        name: &str,
        image_name: Vec<String>,
        horizontal_or_vertical: bool,
        left_and_top_or_right_and_bottom: bool,
        scroll_speed: u32,
        size_position_boundary: [f32; 5],
    ) {
        let mut image_id = vec![];
        for i in image_name.clone() {
            for u in 0..self.rust_constructor_resource.len() {
                if let RCR::Image(im) = self.rust_constructor_resource[u].clone() {
                    if im.name == i {
                        image_id.push(u);
                    };
                };
            };
        };
        for (count, _) in image_id.clone().into_iter().enumerate() {
            if let RCR::Image(im) = &mut self.rust_constructor_resource[image_id[count]] {
                im.x_grid = [0, 0];
                im.y_grid = [0, 0];
                im.center_display = [true, true, false, false];
                im.image_size =
                    [size_position_boundary[0], size_position_boundary[1]];
                let mut temp_position;
                if horizontal_or_vertical {
                    temp_position = size_position_boundary[2];
                } else {
                    temp_position = size_position_boundary[3];
                };
                if horizontal_or_vertical {
                    for _ in 0..count {
                        if left_and_top_or_right_and_bottom {
                            temp_position += size_position_boundary[0];
                        } else {
                            temp_position -= size_position_boundary[0];
                        };
                    }
                    im.origin_position =
                        [temp_position, size_position_boundary[3]];
                } else {
                    for _ in 0..count {
                        if left_and_top_or_right_and_bottom {
                            temp_position += size_position_boundary[1];
                        } else {
                            temp_position -= size_position_boundary[1];
                        };
                    }
                    im.origin_position =
                        [size_position_boundary[2], temp_position];
                };
            };
        };
        if let RCR::Image(im) = self.rust_constructor_resource[image_id[image_id.len() - 1]].clone() {
            let resume_point = if horizontal_or_vertical {
                im.origin_position[0]
            } else {
                im.origin_position[1]
            };
            self.rust_constructor_resource.push(RCR::ScrollBackground(ScrollBackground {
                discern_type: "ScrollBackground".to_string(),
                name: name.to_string(),
                image_name,
                horizontal_or_vertical,
                left_and_top_or_right_and_bottom,
                scroll_speed,
                boundary: size_position_boundary[4],
                resume_point,
            }));
        };
    }

    #[allow(dead_code)]
    pub fn scroll_background(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("ScrollBackground", name) {
            if let RCR::ScrollBackground(sb) = self.rust_constructor_resource[id].clone() {
                sb.reg_render_resource(&mut self.render_resource_list);
                if self.get_resource_index("SplitTime", name).is_err() {
                    self.add_split_time(name, false);
                };
                for i in 0..sb.image_name.len() {
                    self.image(
                        ui,
                        &sb.image_name[i].clone(),
                        ctx,
                    );
                }
                if self.timer.now_time - self.split_time(name).unwrap()[0] >= self.vertrefresh {
                    self.add_split_time(name, true);
                    for i in 0..sb.image_name.len() {
                        if let Ok(id2) = self.get_resource_index("Image", &sb.image_name[i].clone()) {
                            if let RCR::Image(mut im) = self.rust_constructor_resource[id2].clone() {
                                if sb.horizontal_or_vertical {
                                    if sb.left_and_top_or_right_and_bottom {
                                        for _ in 0..sb.scroll_speed {
                                            im.origin_position[0] -= 1_f32;
                                            self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                            self.scroll_background_check_boundary(id, id2);
                                        }
                                    } else {
                                        for _ in 0..sb.scroll_speed {
                                            im.origin_position[0] += 1_f32;
                                            self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                            self.scroll_background_check_boundary(id, id2);
                                        }
                                    };
                                } else if sb.left_and_top_or_right_and_bottom {
                                    for _ in 0..sb.scroll_speed {
                                        im.origin_position[1] -= 1_f32;
                                        self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                        self.scroll_background_check_boundary(id, id2);
                                    }
                                } else {
                                    for _ in 0..sb.scroll_speed {
                                        im.origin_position[1] += 1_f32;
                                        self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                        self.scroll_background_check_boundary(id, id2);
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
    }

    fn scroll_background_check_boundary(&mut self, id: usize, id2: usize) {
        if let RCR::ScrollBackground(sb) = self.rust_constructor_resource[id].clone() {
            if let RCR::Image(mut im) = self.rust_constructor_resource[id2].clone() {
                if sb.horizontal_or_vertical {
                    if sb.left_and_top_or_right_and_bottom {
                        if im.origin_position[0]
                            <= sb.boundary
                        {
                            im.origin_position[0] =
                                sb.resume_point;
                        };
                    } else if im.origin_position[0]
                        >= sb.boundary
                    {
                        im.origin_position[0] =
                            sb.resume_point;
                    };
                } else if sb.left_and_top_or_right_and_bottom {
                    if im.origin_position[1]
                        <= sb.boundary
                    {
                        im.origin_position[1] =
                            sb.resume_point;
                    };
                } else if im.origin_position[1]
                    >= sb.boundary
                {
                    im.origin_position[1] =
                        sb.resume_point;
                };
                self.rust_constructor_resource[id2] = RCR::Image(im);
            };
        };
    }

    pub fn add_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        create_new_resource: bool,
        ctx: &egui::Context,
    ) {
        let img_bytes = self.read_image_to_vec(path);
        let img = image::load_from_memory(&img_bytes).unwrap();
        let rgba_data = match flip {
            [true, true] => img.fliph().flipv().into_rgba8(),
            [true, false] => img.fliph().into_rgba8(),
            [false, true] => img.flipv().into_rgba8(),
            _ => img.into_rgba8(),
        };
        let (w, h) = (rgba_data.width(), rgba_data.height());
        let raw_data: Vec<u8> = rgba_data.into_raw();

        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
        let image_texture = Some(ctx.load_texture(name, color_image, TextureOptions::LINEAR));
        if let Ok(id) = self.get_resource_index("ImageTexture", name) {
            if let RCR::ImageTexture(it) = &mut self.rust_constructor_resource[id] {
                if !create_new_resource {
                    it.texture = image_texture;
                    it.cite_path = path.to_string();
                };
            };
        } else {
            self.rust_constructor_resource.push(RCR::ImageTexture(ImageTexture {
                discern_type: "ImageTexture".to_string(),
                name: name.to_string(),
                texture: image_texture,
                cite_path: path.to_string(),
            }));
        };
    }

    pub fn add_image(
        &mut self,
        name: &str,
        position_size: [f32; 4],
        grid: [u32; 4],
        center_display_and_use_overlay: [bool; 5],
        alpha_and_overlay_color: [u8; 5],
        image_texture_name: &str,
    ) {
        if let Ok(id) = self.get_resource_index("ImageTexture", image_texture_name) {
            if let RCR::ImageTexture(it) = self.rust_constructor_resource[id].clone() {
                self.rust_constructor_resource.push(RCR::Image(Image {
                    discern_type: "Image".to_string(),
                    name: name.to_string(),
                    image_texture: it.texture.clone(),
                    image_position: [position_size[0], position_size[1]],
                    image_size: [position_size[2], position_size[3]],
                    x_grid: [grid[0], grid[1]],
                    y_grid: [grid[2], grid[3]],
                    center_display: [
                        center_display_and_use_overlay[0],
                        center_display_and_use_overlay[1],
                        center_display_and_use_overlay[2],
                        center_display_and_use_overlay[3],
                    ],
                    alpha: alpha_and_overlay_color[0],
                    overlay_color: [
                        alpha_and_overlay_color[1],
                        alpha_and_overlay_color[2],
                        alpha_and_overlay_color[3],
                        alpha_and_overlay_color[4],
                    ],
                    use_overlay_color: center_display_and_use_overlay[4],
                    origin_position: [position_size[0], position_size[1]],
                    origin_cite_texture: image_texture_name.to_string(),
                }));
            };
        };
    }

    pub fn image(&mut self, ui: &Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Image", name) {
            if let RCR::Image(im) = &mut self.rust_constructor_resource[id] {
                im.reg_render_resource(&mut self.render_resource_list);
                im.image_position[0] = match im.x_grid[1] {
                    0 => im.origin_position[0],
                    _ => {
                        (ctx.available_rect().width() as f64 / im.x_grid[1] as f64
                            * im.x_grid[0] as f64) as f32
                            + im.origin_position[0]
                    }
                };
                im.image_position[1] = match im.y_grid[1] {
                    0 => im.origin_position[1],
                    _ => {
                        (ctx.available_rect().height() as f64 / im.y_grid[1] as f64
                            * im.y_grid[0] as f64) as f32
                            + im.origin_position[1]
                    }
                };
                if im.center_display[2] {
                    im.image_position[0] -=
                        im.image_size[0] / 2.0;
                } else if !im.center_display[0] {
                    im.image_position[0] -= im.image_size[0];
                };
                if im.center_display[3] {
                    im.image_position[1] -=
                        im.image_size[1] / 2.0;
                } else if !im.center_display[1] {
                    im.image_position[1] -= im.image_size[1];
                };
                if let Some(texture) = &im.image_texture {
                    let rect = Rect::from_min_size(
                        Pos2::new(
                            im.image_position[0],
                            im.image_position[1],
                        ),
                        Vec2::new(
                            im.image_size[0],
                            im.image_size[1],
                        ),
                    );
                    let color = if im.use_overlay_color {
                        // 创建颜色覆盖
                        Color32::from_rgba_unmultiplied(
                            im.overlay_color[0],
                            im.overlay_color[1],
                            im.overlay_color[2],
                            // 将图片透明度与覆盖颜色透明度相乘
                            (im.alpha as f32
                                * im.overlay_color[3] as f32
                                / 255.0) as u8,
                        )
                    } else {
                        Color32::from_white_alpha(im.alpha)
                    };

                    ui.painter().image(
                        texture.into(),
                        rect,
                        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        color,
                    );
                };
            };
        };
    }

    #[allow(dead_code)]
    pub fn add_message_box(
        &mut self,
        box_itself_title_content_image_name: [&str; 4],
        box_size: [f32; 2],
        box_keep_existing: bool,
        box_existing_time: f32,
        box_normal_and_restore_speed: [f32; 2],
    ) {
        if self.get_resource_index("MessageBox", box_itself_title_content_image_name[0]).is_err()
        {
            if let Ok(id) = self.get_resource_index("Image", box_itself_title_content_image_name[3]) {
                if let RCR::Image(im) = &mut self.rust_constructor_resource[id] {
                    im.image_size = [box_size[1] - 15_f32, box_size[1] - 15_f32];
                    im.center_display = [true, false, false, true];
                    im.x_grid = [1, 1];
                    im.y_grid = [0, 1];    
                    im.name = format!("MessageBox_{}", im.name);  
                };
            };
            if let Ok(id) = self.get_resource_index("Text", box_itself_title_content_image_name[1]) {
                if let RCR::Text(t) = &mut self.rust_constructor_resource[id] {
                    t.x_grid = [1, 1];
                    t.y_grid = [0, 1];
                    t.center_display = [true, true, false, false];
                    t.wrap_width = box_size[0] - box_size[1] + 5_f32;
                    t.name = format!("MessageBox_{}", t.name);
                };
            };
            if let Ok(id) = self.get_resource_index("Text", box_itself_title_content_image_name[2]) {
                if let RCR::Text(t) = &mut self.rust_constructor_resource[id] {
                    t.center_display = [true, true, false, false];
                    t.x_grid = [1, 1];
                    t.y_grid = [0, 1];
                    t.wrap_width = box_size[0] - box_size[1] + 5_f32;
                    t.name = format!("MessageBox_{}", t.name);
                };
            };
            self.rust_constructor_resource.push(RCR::MessageBox(MessageBox {
                discern_type: "MessageBox".to_string(),
                name: box_itself_title_content_image_name[0].to_string(),
                box_size,
                box_title_name: format!("MessageBox_{}", box_itself_title_content_image_name[1]),
                box_content_name: format!("MessageBox_{}", box_itself_title_content_image_name[2]),
                box_image_name: format!("MessageBox_{}", box_itself_title_content_image_name[3]),
                box_keep_existing,
                box_existing_time,
                box_exist: true,
                box_speed: box_normal_and_restore_speed[0],
                box_restore_speed: box_normal_and_restore_speed[1],
                box_memory_offset: 0_f32,
            }));
            if !box_keep_existing {
                self.add_split_time(
                    &format!("MessageBox_{}", box_itself_title_content_image_name[0]),
                    false,
                );
            };
            self.add_split_time(
                &format!(
                    "MessageBox_{}_animation",
                    box_itself_title_content_image_name[0]
                ),
                false,
            );
            self.add_rect(
                &format!("MessageBox_{}", box_itself_title_content_image_name[0]),
                [0_f32, 0_f32, box_size[0], box_size[1], 20_f32],
                [1, 1, 0, 1],
                [true, true, false, false],
                [100, 100, 100, 125, 240, 255, 255, 255],
                0.0,
            );
            self.add_image(
                &format!(
                    "MessageBox_{}_Close",
                    box_itself_title_content_image_name[0]
                ),
                [0_f32, 0_f32, 30_f32, 30_f32],
                [0, 0, 0, 0],
                [false, false, true, true, false],
                [255, 0, 0, 0, 0],
                "Close_Message_Box",
            );
            self.add_switch(
                [
                    &format!(
                        "MessageBox_{}_Close",
                        box_itself_title_content_image_name[0]
                    ),
                    &format!(
                        "MessageBox_{}_Close",
                        box_itself_title_content_image_name[0]
                    ),
                ],
                vec![
                    SwitchData {
                        texture: "Close_Message_Box".to_string(),
                        color: [255, 255, 255, 0],
                    },
                    SwitchData {
                        texture: "Close_Message_Box".to_string(),
                        color: [180, 180, 180, 200],
                    },
                    SwitchData {
                        texture: "Close_Message_Box".to_string(),
                        color: [255, 255, 255, 200],
                    },
                    SwitchData {
                        texture: "Close_Message_Box".to_string(),
                        color: [180, 180, 180, 200],
                    },
                ],
                [false, true, true],
                2,
                vec![SwitchClickAction {
                    click_method: PointerButton::Primary,
                    action: true,
                }],
                vec![
                    format!(
                        "{}: \"{}\"",
                        self.game_text.game_text["close_message_box"]
                            [self.config.language as usize],
                        box_itself_title_content_image_name[0]
                    ),
                    "".to_string(),
                ],
            );
        } else if self.config.rc_strict_mode {
            panic!(
                "{}{}",
                box_itself_title_content_image_name[0],
                self.game_text.game_text["error_message_box_already_exists"]
                    [self.config.language as usize]
            );
        } else {
            self.problem_report(
                &format!(
                    "{}{}",
                    box_itself_title_content_image_name[0],
                    self.game_text.game_text["error_message_box_already_exists"]
                        [self.config.language as usize]
                ),
                SeverityLevel::SevereWarning,
                &self.game_text.game_text["error_message_box_already_exists_annotation"]
                    [self.config.language as usize]
                    .clone(),
            );
        };
    }

    pub fn message_box_display(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let mut offset = 0_f32;
        let mut delete_count = 0;
        let mut index_list = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let RCR::MessageBox(_) = self.rust_constructor_resource[i] {
                index_list.push(i);
            };
        };
        for u in 0..index_list.len() {
            let mut deleted = false;
            let i = u - delete_count;
            if let RCR::MessageBox(mut mb) = self.rust_constructor_resource[index_list[i]].clone() {
                if let Ok(id1) = self.get_resource_index("Image", &mb.box_image_name) {
                    if let RCR::Image(mut im1) = self.rust_constructor_resource[id1].clone() {
                        if let Ok(id2) = self.get_resource_index("CustomRect", &format!("MessageBox_{}", mb.name)) {
                            if let RCR::CustomRect(mut cr) = self.rust_constructor_resource[id2].clone() {
                                if let Ok(id3) = self.get_resource_index("Text", &mb.box_title_name) {
                                    if let RCR::Text(mut t1) = self.rust_constructor_resource[id3].clone() {
                                        if let Ok(id4) = self.get_resource_index("Text", &mb.box_content_name) {
                                            if let RCR::Text(mut t2) = self.rust_constructor_resource[id4].clone() {
                                                if let Ok(id5) = self.get_resource_index("Switch", &format!("MessageBox_{}_Close", mb.name)) {
                                                    if let RCR::Switch(mut s) = self.rust_constructor_resource[id5].clone() {
                                                        if let Ok(id6) = self.get_resource_index("Image", &format!("MessageBox_{}_Close", mb.name)) {
                                                            if let RCR::Image(mut im2) = self.rust_constructor_resource[id6].clone() {
                                                                if mb.box_size[1]
                                                                    < self.get_text_size(&mb.box_title_name.clone(), ui).unwrap()[1]
                                                                        + self.get_text_size(&mb.box_content_name.clone(), ui).unwrap()
                                                                            [1]
                                                                        + 10_f32
                                                                {
                                                                    mb.box_size[1] = self
                                                                        .get_text_size(&mb.box_title_name.clone(), ui).unwrap()[1]
                                                                        + self
                                                                            .get_text_size(&mb.box_content_name.clone(), ui).unwrap()
                                                                            [1]
                                                                        + 10_f32;
                                                                    cr.size[1] = mb.box_size[1];
                                                                    im1.image_size = [
                                                                        mb.box_size[1] - 15_f32,
                                                                        mb.box_size[1] - 15_f32,
                                                                    ];
                                                                    t1.wrap_width = mb.box_size[0]
                                                                        - mb.box_size[1]
                                                                        + 5_f32;
                                                                    t2.wrap_width = mb.box_size[0]
                                                                        - mb.box_size[1]
                                                                        + 5_f32;
                                                                };
                                                                if self.timer.total_time
                                                                    - self.split_time(&format!(
                                                                        "MessageBox_{}_animation",
                                                                        mb.name
                                                                    )).unwrap()[1]
                                                                    >= self.vertrefresh
                                                                {
                                                                    self.add_split_time(
                                                                        &format!("MessageBox_{}_animation", mb.name),
                                                                        true,
                                                                    );
                                                                    if offset != mb.box_memory_offset {
                                                                        if mb.box_memory_offset < offset {
                                                                            if mb.box_memory_offset
                                                                                + mb.box_restore_speed
                                                                                >= offset
                                                                            {
                                                                                mb.box_memory_offset = offset;
                                                                            } else {
                                                                                mb.box_memory_offset +=
                                                                                    mb.box_restore_speed;
                                                                            };
                                                                        } else if mb.box_memory_offset
                                                                            - mb.box_restore_speed
                                                                            <= offset
                                                                        {
                                                                            mb.box_memory_offset = offset;
                                                                        } else {
                                                                            mb.box_memory_offset -=
                                                                                mb.box_restore_speed;
                                                                        };
                                                                    };
                                                                    if cr.origin_position[0]
                                                                        != -mb.box_size[0] - 5_f32
                                                                    {
                                                                        if mb.box_exist {
                                                                            if cr.origin_position[0]
                                                                                - mb.box_speed
                                                                                <= -mb.box_size[0] - 5_f32
                                                                            {
                                                                                cr.origin_position[0] =
                                                                                    -mb.box_size[0] - 5_f32;
                                                                                self.add_split_time(
                                                                                    &format!("MessageBox_{}", mb.name),
                                                                                    true,
                                                                                );
                                                                            } else {
                                                                                cr.origin_position[0] -=
                                                                                    mb.box_speed;
                                                                            };
                                                                        } else if cr.origin_position[0]
                                                                            + mb.box_speed
                                                                            >= 15_f32
                                                                        {
                                                                            cr.origin_position[0] = 15_f32;
                                                                            delete_count += 1;
                                                                            deleted = true;
                                                                        } else {
                                                                            cr.origin_position[0] +=
                                                                                mb.box_speed;
                                                                        };
                                                                    };
                                                                };
                                                                cr.origin_position[1] =
                                                                    mb.box_memory_offset + 20_f32;
                                                                im1.origin_position = [
                                                                    cr.origin_position[0] + 5_f32,
                                                                    cr.origin_position[1]
                                                                        + mb.box_size[1] / 2_f32,
                                                                ];
                                                                t1.origin_position = [
                                                                    im1.origin_position[0]
                                                                        + im1.image_size[0]
                                                                        + 5_f32,
                                                                    cr.origin_position[1] + 5_f32,
                                                                ];
                                                                t2.origin_position = [
                                                                    im1.origin_position[0]
                                                                        + im1.image_size[0]
                                                                        + 5_f32,
                                                                    t1.origin_position[1]
                                                                        + self.get_text_size(&mb.box_title_name.clone(), ui).unwrap()
                                                                            [1],
                                                                ];
                                                                im2.origin_position = cr.position;
                                                                if !mb.box_keep_existing
                                                                    && self.timer.total_time
                                                                        - self.split_time(&format!("MessageBox_{}", mb.name)).unwrap()
                                                                            [1]
                                                                        >= mb.box_existing_time
                                                                    && cr.origin_position[0]
                                                                        == -mb.box_size[0] - 5_f32
                                                                {
                                                                    mb.box_exist = false;
                                                                    if cr.origin_position[0]
                                                                        + mb.box_speed
                                                                        >= 15_f32
                                                                    {
                                                                        cr.origin_position[0] = 15_f32;
                                                                    } else {
                                                                        cr.origin_position[0] +=
                                                                            mb.box_speed;
                                                                    };
                                                                };
                                                                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                                                    let rect = egui::Rect::from_min_size(
                                                                        Pos2 {
                                                                            x: im2.image_position[0],
                                                                            y: im2.image_position[1],
                                                                        },
                                                                        Vec2 {
                                                                            x: cr.size[0] + 25_f32,
                                                                            y: cr.size[1] + 25_f32,
                                                                        },
                                                                    );
                                                                    if rect.contains(mouse_pos) {
                                                                        s.appearance[0].color[3] = 200;
                                                                    } else {
                                                                        s.appearance[0].color[3] = 0;
                                                                    };
                                                                };
                                                                self.rust_constructor_resource[index_list[i]] = RCR::MessageBox(mb.clone());
                                                                self.rust_constructor_resource[id1] = RCR::Image(im1.clone());
                                                                self.rust_constructor_resource[id2] = RCR::CustomRect(cr.clone());
                                                                self.rust_constructor_resource[id3] = RCR::Text(t1.clone());
                                                                self.rust_constructor_resource[id4] = RCR::Text(t2.clone());
                                                                self.rust_constructor_resource[id5] = RCR::Switch(s.clone());
                                                                self.rust_constructor_resource[id6] = RCR::Image(im2.clone());
                                                                self.rect(
                                                                    ui,
                                                                    &format!("MessageBox_{}", mb.name),
                                                                    ctx,
                                                                );
                                                                self.image(
                                                                    ui,
                                                                    &mb.box_image_name.clone(),
                                                                    ctx,
                                                                );
                                                                self.text(ui, &t1.name.clone(), ctx);
                                                                self.text(ui, &t2.name.clone(), ctx);
                                                                if self.switch(
                                                                    &format!("MessageBox_{}_Close", mb.name),
                                                                    ui,
                                                                    ctx,
                                                                    s.state == 0 && mb.box_exist,
                                                                    true,
                                                                ).unwrap()[0] == 0
                                                                {
                                                                    mb.box_exist = false;
                                                                    if cr.origin_position[0]
                                                                        + mb.box_speed
                                                                        >= 15_f32
                                                                    {
                                                                        cr.origin_position[0] = 15_f32;
                                                                    } else {
                                                                        cr.origin_position[0] +=
                                                                            mb.box_speed;
                                                                    };
                                                                    self.rust_constructor_resource[id2] = RCR::CustomRect(cr.clone());
                                                                    self.rust_constructor_resource[index_list[i]] = RCR::MessageBox(mb.clone());
                                                                };
                                                                if deleted {
                                                                    if let Ok(id) = self.get_resource_index("Image", &mb.box_image_name) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("CustomRect", &format!("MessageBox_{}", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("Text", &mb.box_title_name) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("Text", &mb.box_content_name) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("Switch", &format!("MessageBox_{}_Close", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("Image", &format!("MessageBox_{}_Close", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("Text", &format!("MessageBox_{}_Close_hint", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("SplitTime", &format!("MessageBox_{}_animation", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("SplitTime", &format!("MessageBox_{}", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("SplitTime", &format!("MessageBox_{}_Close_hint_fade_animation", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("SplitTime", &format!("MessageBox_{}_Close_start_hover_time", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("MessageBox", &mb.name) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                } else {
                                                                    offset += mb.box_size[1] + 15_f32;
                                                                };
                                                            };
                                                        };
                                                    };
                                                };
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
    }

    pub fn add_switch(
        &mut self,
        name_switch_and_image_name: [&str; 2],
        mut appearance: Vec<SwitchData>,
        enable_hover_click_image_and_use_overlay: [bool; 3],
        switch_amounts_state: u32,
        click_method: Vec<SwitchClickAction>,
        mut hint_text: Vec<String>,
    ) {
        let mut count = 1;
        if enable_hover_click_image_and_use_overlay[0] {
            count += 1;
        };
        if enable_hover_click_image_and_use_overlay[1] {
            count += 1;
        };
        if appearance.len() as u32 != count * switch_amounts_state
            || hint_text.len() as u32 != switch_amounts_state
        {
            if self.config.rc_strict_mode {
                panic!(
                    "{}{}:{}",
                    name_switch_and_image_name[0],
                    self.game_text.game_text["error_switch_vec_mismatch"]
                        [self.config.language as usize],
                    count * switch_amounts_state - appearance.len() as u32
                );
            } else {
                self.problem_report(
                    &format!(
                        "{}{}:{}",
                        name_switch_and_image_name[0],
                        self.game_text.game_text["error_switch_vec_mismatch"]
                            [self.config.language as usize],
                        if appearance.len() as u32 != count * switch_amounts_state {
                            count * switch_amounts_state - appearance.len() as u32
                        } else {
                            switch_amounts_state - hint_text.len() as u32
                        }
                    ),
                    SeverityLevel::MildWarning,
                    &self.game_text.game_text["error_switch_vec_mismatch_annotation"]
                        [self.config.language as usize]
                        .clone(),
                );
                for _ in 0..count * switch_amounts_state - appearance.len() as u32 {
                    appearance.push(SwitchData {
                        texture: "Error".to_string(),
                        color: [255, 255, 255, 255],
                    });
                }
                for _ in 0..count * switch_amounts_state - hint_text.len() as u32 {
                    hint_text.push("Error".to_string());
                }
            };
        };
        if let Ok(id) = self.get_resource_index("Image", name_switch_and_image_name[1]) {
            if let RCR::Image(im) = &mut self.rust_constructor_resource[id] {
                im.use_overlay_color = true;
            };
        };
        if !hint_text.is_empty() {
            self.add_text(
                [
                    &format!("{}_hint", name_switch_and_image_name[0]),
                    &hint_text[0],
                ],
                [0_f32, 0_f32, 25_f32, 300_f32, 10_f32],
                [255, 255, 255, 0, 0, 0, 0, 0],
                [true, true, false, false],
                true,
                [0, 0, 0, 0],
            );
            self.add_split_time(
                &format!("{}_start_hover_time", name_switch_and_image_name[0]),
                false,
            );
            self.add_split_time(
                &format!("{}_hint_fade_animation", name_switch_and_image_name[0]),
                false,
            );
        };
        self.rust_constructor_resource.push(RCR::Switch(Switch {
            discern_type: "Switch".to_string(),
            name: name_switch_and_image_name[0].to_string(),
            appearance,
            switch_image_name: name_switch_and_image_name[1].to_string(),
            enable_hover_click_image: [
                enable_hover_click_image_and_use_overlay[0],
                enable_hover_click_image_and_use_overlay[1],
            ],
            state: 0,
            click_method,
            last_time_hovered: false,
            last_time_clicked: false,
            last_time_clicked_index: 0,
            animation_count: count,
            hint_text: hint_text.clone(),
            hint_text_name: if !hint_text.is_empty() {
                format!("{}_hint", name_switch_and_image_name[0])
            } else {
                "".to_string()
            },
        }));
    }

    pub fn switch(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &egui::Context,
        enable: bool,
        play_sound: bool,
    ) -> Result<[usize; 2], ()> {
        let mut activated = [5, 0];
        if let Ok(id) = self.get_resource_index("Switch", name) {
            if let RCR::Switch(mut s) = self.rust_constructor_resource[id].clone() {
                if let Ok(id2) = self.get_resource_index("Image", &s.switch_image_name.clone()) {
                    if let RCR::Image(mut im) = self.rust_constructor_resource[id2].clone()  {
                        if let Ok(id3) = self.get_resource_index("Text", &s.hint_text_name) {
                            if let RCR::Text(mut t) = self.rust_constructor_resource[id3].clone()  {
                                s.reg_render_resource(&mut self.render_resource_list);
                                let rect = Rect::from_min_size(
                                    Pos2::new(
                                        im.image_position[0],
                                        im.image_position[1],
                                    ),
                                    Vec2::new(
                                        im.image_size[0],
                                        im.image_size[1],
                                    ),
                                );
                                let mut hovered = false;
                                if enable {
                                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                        // 判断是否在矩形内
                                        if rect.contains(mouse_pos) {
                                            if !s.last_time_hovered {
                                                self.add_split_time(
                                                    &format!("{}_start_hover_time", s.name),
                                                    true,
                                                );
                                            } else if self.timer.total_time
                                                - self.split_time(&format!(
                                                    "{}_start_hover_time",
                                                    s.name
                                                )).unwrap()[1]
                                                >= 2_f32
                                                || t.rgba[3] != 0
                                            {
                                                t.rgba[3] = 255;
                                                t.origin_position = [mouse_pos.x, mouse_pos.y];
                                            };
                                            hovered = true;
                                            let mut clicked = vec![];
                                            let mut active = false;
                                            for u in 0..s.click_method.len() as u32 {
                                                clicked.push(ui.input(|i| {
                                                    i.pointer.button_down(
                                                        s.click_method[u as usize].click_method,
                                                    )
                                                }));
                                                if clicked[u as usize] {
                                                    active = true;
                                                    s.last_time_clicked_index = u as usize;
                                                    break;
                                                };
                                            }
                                            if active {
                                                s.last_time_clicked = true;
                                                if s.enable_hover_click_image[1] {
                                                    if s.enable_hover_click_image[0] {
                                                        im.overlay_color = s
                                                            .appearance[(s
                                                            .state
                                                            * s.animation_count
                                                            + 2) as usize]
                                                            .color;
                                                        if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                                        .state
                                                                        * s.animation_count
                                                                        + 2)
                                                                        as usize]
                                                                        .texture
                                                                        .clone()) {
                                                            if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                                im.image_texture = it.texture.clone();
                                                            };
                                                        };
                                                    } else {
                                                        im.overlay_color = s
                                                            .appearance[(s
                                                            .state
                                                            * s.animation_count
                                                            + 1) as usize]
                                                            .color;
                                                        if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                                        .state
                                                                        * s.animation_count
                                                                        + 1)
                                                                        as usize]
                                                                        .texture
                                                                        .clone()) {
                                                            if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                                im.image_texture = it.texture.clone();
                                                            };
                                                        };
                                                    };
                                                } else if !s.enable_hover_click_image[0] {
                                                    im.overlay_color =
                                                        s.appearance[(s.state
                                                            * s.animation_count)
                                                            as usize]
                                                            .color;
                                                    if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                                    .state
                                                                    * s.animation_count)
                                                                    as usize]
                                                                    .texture
                                                                    .clone()) {
                                                        if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                            im.image_texture = it.texture.clone();
                                                        };
                                                    };
                                                };
                                            } else {
                                                if s.last_time_clicked {
                                                    if play_sound {
                                                        general_click_feedback();
                                                    };
                                                    let mut count = 1;
                                                    if s.enable_hover_click_image[0] {
                                                        count += 1;
                                                    };
                                                    if s.enable_hover_click_image[1] {
                                                        count += 1;
                                                    };
                                                    if s.click_method
                                                        [s.last_time_clicked_index]
                                                        .action
                                                    {
                                                        if s.state
                                                            < (s.appearance.len() / count - 1) as u32
                                                        {
                                                            s.state += 1;
                                                        } else {
                                                            s.state = 0;
                                                        };
                                                    };
                                                    activated[0] = s.last_time_clicked_index;
                                                    s.last_time_clicked = false;
                                                };
                                                if s.enable_hover_click_image[0] {
                                                    im.overlay_color = s
                                                        .appearance[(s
                                                        .state
                                                        * s.animation_count
                                                        + 1) as usize]
                                                        .color;
                                                    if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                                    .state
                                                                    * s.animation_count
                                                                    + 1)
                                                                    as usize]
                                                                    .texture
                                                                    .clone()) {
                                                        if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                            im.image_texture = it.texture.clone();
                                                        };
                                                    };
                                                } else {
                                                    im.overlay_color =
                                                        s.appearance[(s.state
                                                            * s.animation_count)
                                                            as usize]
                                                            .color;
                                                    if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                                    .state
                                                                    * s.animation_count)
                                                                    as usize]
                                                                    .texture
                                                                    .clone()) {
                                                        if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                            im.image_texture = it.texture.clone();
                                                        };
                                                    };
                                                };
                                            };
                                        } else {
                                            s.last_time_clicked = false;
                                            im.overlay_color = s.appearance
                                                [(s.state * s.animation_count)
                                                    as usize]
                                                .color;
                                            if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                            .state
                                                            * s.animation_count)
                                                            as usize]
                                                            .texture
                                                            .clone()) {
                                                if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                                    im.image_texture = it.texture.clone();
                                                };
                                            };
                                        };
                                    };
                                } else {
                                    s.last_time_clicked = false;
                                    im.overlay_color =
                                        s.appearance[(s.state
                                            * s.animation_count)
                                            as usize]
                                            .color;
                                    if let Ok(id4) = self.get_resource_index("ImageTexture", &s.appearance[(s
                                                    .state
                                                    * s.animation_count)
                                                    as usize]
                                                    .texture
                                                    .clone()) {
                                        if let RCR::ImageTexture(it) = self.rust_constructor_resource[id4].clone() {
                                            im.image_texture = it.texture.clone();
                                        };
                                    };
                                };
                                if !hovered {
                                    if s.last_time_hovered {
                                        self.add_split_time(
                                            &format!("{}_hint_fade_animation", s.name),
                                            true,
                                        );
                                    };
                                    if self.timer.total_time
                                        - self.split_time(&format!(
                                            "{}_hint_fade_animation",
                                            s.name
                                        )).unwrap()[1]
                                        >= self.vertrefresh
                                    {
                                        t.rgba[3] =
                                            t.rgba[3].saturating_sub(1);
                                    };
                                };
                                t.background_rgb[3] = t.rgba[3];
                                s.last_time_hovered = hovered;
                                t.text_content =
                                    s.hint_text[s.state as usize].clone();
                                activated[1] = s.state as usize;
                                self.rust_constructor_resource[id] = RCR::Switch(s.clone());
                                self.rust_constructor_resource[id2] = RCR::Image(im);
                                self.rust_constructor_resource[id3] = RCR::Text(t);
                                self.image(ui, &s.switch_image_name.clone(), ctx);
                                self.text(ui, &s.hint_text_name.clone(), ctx);
                                Ok(activated)
                            } else {
                                Err(())
                            }
                        } else {
                            Err(())
                        }
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
