//! Rust Constructor V2
//! 一个强大的跨平台GUI框架，在Rust中开发GUI项目的最简单方法。
use anyhow::Context;
use eframe::{emath::Rect, epaint::Stroke, epaint::textures::TextureOptions};
use egui::{
    Color32, FontData, FontDefinitions, FontId, PointerButton, Pos2, Ui, Vec2, text::CCursor,
};
use json::JsonValue;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
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
use tray_icon::{Icon, TrayIconBuilder, menu::Menu};

/// 从文件中加载图标。
pub fn load_icon_from_file(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    let color = image.into_raw();
    Ok(Icon::from_rgba(color, width, height)?)
}

/// 创建格式化的JSON文件。
pub fn create_json<P: AsRef<Path>>(path: P, data: JsonValue) -> anyhow::Result<()> {
    let parent_dir = path
        .as_ref()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path."))?;

    // 创建父目录（如果不存在）。
    fs::create_dir_all(parent_dir)?;

    // 生成带缩进的JSON字符串（4空格缩进）。
    let formatted = json::stringify_pretty(data, 4);

    // 写入文件（自动处理换行符）。
    fs::write(path, formatted)?;
    Ok(())
}

/// 复制并重新格式化JSON文件。
pub fn copy_and_reformat_json<P: AsRef<Path>>(src: P, dest: P) -> anyhow::Result<()> {
    // 读取原始文件。
    let content = fs::read_to_string(&src)?;

    // 解析JSON（自动验证格式）。
    let parsed = json::parse(&content)?;

    // 使用格式化写入新文件。
    create_json(dest, parsed)?;

    Ok(())
}

/// 检查文件是否存在。
pub fn check_file_exists<P: AsRef<Path>>(path: P) -> bool {
    let path_ref = path.as_ref();
    if path_ref.exists() {
        true // 文件已存在时直接返回true。
    } else {
        // 文件不存在时，返回false。
        false
    }
}

/// 通用 JSON 写入函数。
pub fn write_to_json<P: AsRef<Path>>(path: P, data: JsonValue) -> anyhow::Result<()> {
    let parent_dir = path
        .as_ref()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path."))?;

    fs::create_dir_all(parent_dir)?;
    let formatted = json::stringify_pretty(data, 4);
    fs::write(path, formatted)?;
    Ok(())
}

/// 通用 JSON 读取函数。
pub fn read_from_json<P: AsRef<Path>>(path: P) -> anyhow::Result<JsonValue> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Cannot read the file: {}", path.as_ref().display()))?;
    json::parse(&content)
        .with_context(|| format!("Failed to parse JSON: {}", path.as_ref().display()))
}

/// 播放 WAV 文件。
pub fn play_wav(path: &str) -> anyhow::Result<f64> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let sound_data = StaticSoundData::from_file(path)?;
    let duration = sound_data.duration().as_secs_f64();
    manager.play(sound_data)?;
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
    Ok(duration)
}

/// 通用按键点击反馈函数。
pub fn general_click_feedback(sound_path: &str) {
    let sound_path = sound_path.to_string();
    std::thread::spawn(move || {
        play_wav(&sound_path).unwrap_or(0_f64);
    });
}

/// 检查指定目录下有多少个带有特定名称的文件。
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

/// 检查指定目录下有多少个带有特定名称的文件并返回它们的名称。
pub fn list_files_recursive(path: &Path, prefix: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut matches = Vec::new();

    for entry in std::fs::read_dir(path)? {
        // 遍历目录
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // 递归处理子目录
            matches.extend(list_files_recursive(&path, prefix)?);
        } else if let Some(file_name) = path.file_name()
            && file_name.to_string_lossy().contains(prefix)
        {
            matches.push(path);
        }
    }

    Ok(matches)
}

/// 配置文件。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Config {
    /// 显示的语言（注意：此值修改到大于实际语言数目极有可能导致程序崩溃！）。
    pub language: u8,
    /// 总共有多少种语言已被支持（注意：此值修改到大于实际语言数目极有可能导致程序崩溃！）。
    pub amount_languages: u8,
    /// 是否启用严格模式：严格模式下，当遇到无法处理的情况时，将直接panic；若未启用严格模式，则会发出一条问题报告来描述情况。
    pub rc_strict_mode: bool,
    /// 问题反馈音效（留空即可禁用）。
    pub problem_report_sound: String,
}

impl Config {
    pub fn from_json_value(value: &JsonValue) -> Option<Self> {
        Some(Self {
            language: value["language"].as_u8()?,
            amount_languages: value["amount_languages"].as_u8()?,
            rc_strict_mode: value["rc_strict_mode"].as_bool()?,
            problem_report_sound: value["problem_report_sound"].as_str()?.to_string(),
        })
    }

    pub fn to_json_value(&self) -> JsonValue {
        json::object! {
            language: self.language,
            amount_languages: self.amount_languages,
            rc_strict_mode: self.rc_strict_mode,
        }
    }
}

/// 统一的文本调用处。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppText {
    pub app_text: HashMap<String, Vec<String>>,
}

impl AppText {
    pub fn from_json_value(value: &JsonValue) -> Option<Self> {
        // 检查 app_text 字段是否为对象
        if !value["app_text"].is_object() {
            return None;
        }

        // 遍历对象键值对
        let mut parsed = HashMap::new();
        for (key, val) in value["app_text"].entries() {
            if let JsonValue::Array(arr) = val {
                let str_vec: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                parsed.insert(key.to_string(), str_vec);
            }
        }

        Some(Self { app_text: parsed })
    }
}

/// 存储特定值的枚举。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// 报告发生问题时的状态。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ReportState {
    /// 问题发生时所在页面。
    pub current_page: String,
    /// 问题发生时程序总运行时间。
    pub current_total_runtime: f32,
    /// 问题发生时页面运行时间。
    pub current_page_runtime: f32,
}

/// 出现问题时用于存储问题内容、状态及注释的结构体。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Problem {
    /// 问题严重程度。
    pub severity_level: SeverityLevel,
    /// 问题描述。
    pub problem: String,
    /// 问题备注。
    pub annotation: String,
    /// 问题报告状态。
    pub report_state: ReportState,
    /// 问题类型。
    pub problem_type: RustConstructorError,
}

/// 衡量问题的严重等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SeverityLevel {
    /// 弱警告：一般情况下不会产生影响。
    MildWarning,
    /// 强警告：会影响程序正常执行，但一般情况下不会有严重后果。
    SevereWarning,
    /// 错误：会导致程序无法运行。
    Error,
}

/// 核心特征，用于统一管理Rust Constructor资源。
pub trait RustConstructorResource {
    /// 返回资源名称。
    fn name(&self) -> &str;

    /// 返回资源类型。
    fn expose_type(&self) -> &str;

    /// 注册资源。
    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>);

    /// 匹配资源。
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

/// 用于存储页面数据的RC资源。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PageData {
    pub discern_type: String,
    pub name: String,
    /// 是否强制在每帧都刷新页面(使用ctx.request_repaint())。
    pub forced_update: bool,
    /// 是否已经加载完首次进入此页面所需内容。
    pub change_page_updated: bool,
    /// 是否已经加载完进入此页面所需内容。
    pub enter_page_updated: bool,
}

impl Default for PageData {
    fn default() -> Self {
        PageData {
            discern_type: String::from("PageData"),
            name: String::from("PageData"),
            forced_update: true,
            change_page_updated: false,
            enter_page_updated: false,
        }
    }
}

impl PageData {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn forced_update(mut self, forced_update: bool) -> Self {
        self.forced_update = forced_update;
        self
    }
}

/// 用于存储运行时间的计时器。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Timer {
    /// 进入页面的时间。
    pub start_time: f32,
    /// 程序总运行时间。
    pub total_time: f32,
    /// 核心计时器。
    pub timer: Instant,
    /// 当前页面运行时间。
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

/// 用于存储图片纹理的RC资源。
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImageTexture {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub texture: Option<egui::TextureHandle>,
    /// 图片路径。
    pub cite_path: String,
}

impl Default for ImageTexture {
    fn default() -> Self {
        ImageTexture {
            discern_type: String::from("ImageTexture"),
            name: String::from("ImageTexture"),
            texture: None,
            cite_path: String::from(""),
        }
    }
}

impl ImageTexture {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
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

/// RC的矩形资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRect {
    pub discern_type: String,
    pub name: String,
    /// 位置。
    pub position: [f32; 2],
    /// 尺寸。
    pub size: [f32; 2],
    /// 圆角。
    pub rounding: f32,
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 颜色。
    pub color: [u8; 4],
    /// 边框宽度。
    pub border_width: f32,
    /// 边框颜色。
    pub border_color: [u8; 4],
    /// 原始位置。
    pub origin_position: [f32; 2],
}

impl Default for CustomRect {
    fn default() -> Self {
        Self {
            discern_type: String::from("CustomRect"),
            name: String::from("CustomRect"),
            position: [0_f32, 0_f32],
            size: [100_f32, 100_f32],
            rounding: 2_f32,
            x_grid: [0, 0],
            y_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            color: [255, 255, 255, 255],
            border_width: 2_f32,
            border_color: [0, 0, 0, 255],
            origin_position: [0_f32, 0_f32],
        }
    }
}

impl CustomRect {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = [width, height];
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub fn x_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn center_display(
        mut self,
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        self.center_display = (horizontal_align, vertical_align);
        self
    }

    #[inline]
    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = [r, g, b, a];
        self
    }

    #[inline]
    pub fn border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width;
        self
    }

    #[inline]
    pub fn border_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.border_color = [r, g, b, a];
        self
    }

    #[inline]
    pub fn origin_position(mut self, x: f32, y: f32) -> Self {
        self.origin_position = [x, y];
        self
    }
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

/// RC的图片资源。
#[derive(Clone, PartialEq)]
pub struct Image {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub texture: Option<egui::TextureHandle>,
    /// 图片位置。
    pub position: [f32; 2],
    /// 图片大小。
    pub size: [f32; 2],
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 不透明度。
    pub alpha: u8,
    /// 叠加颜色。
    pub overlay_color: [u8; 4],
    /// 是否使用叠加颜色。
    pub use_overlay_color: bool,
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 引用纹理名。
    pub cite_texture: String,
    /// 上一帧引用纹理名。
    pub last_frame_cite_texture: String,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            discern_type: String::from("Image"),
            name: String::from("Image"),
            texture: None,
            position: [0_f32, 0_f32],
            size: [100_f32, 100_f32],
            x_grid: [0, 0],
            y_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            alpha: 255,
            overlay_color: [255, 255, 255, 255],
            use_overlay_color: true,
            origin_position: [0_f32, 0_f32],
            cite_texture: String::from("ImageTexture"),
            last_frame_cite_texture: String::from("ImageTexture"),
        }
    }
}

impl Image {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn origin_position(mut self, x: f32, y: f32) -> Self {
        self.origin_position = [x, y];
        self
    }

    #[inline]
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = [width, height];
        self
    }

    #[inline]
    pub fn x_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn center_display(
        mut self,
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        self.center_display = (horizontal_align, vertical_align);
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn overlay_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.overlay_color = [r, g, b, a];
        self
    }

    #[inline]
    pub fn use_overlay_color(mut self, use_overlay_color: bool) -> Self {
        self.use_overlay_color = use_overlay_color;
        self
    }
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

/// 控制超链接选取方法。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HyperlinkSelectMethod {
    /// 选取所有匹配项。
    All(String),
    /// 选取指定的匹配项。
    Segment(Vec<(usize, String)>),
}

/// RC的文本资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub discern_type: String,
    pub name: String,
    /// 文本内容。
    pub content: String,
    /// 字号。
    pub font_size: f32,
    /// 文本颜色。
    pub color: [u8; 4],
    /// 文本位置。
    pub position: [f32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 单行宽度。
    pub wrap_width: f32,
    /// 是否有背景。
    pub write_background: bool,
    /// 背景颜色。
    pub background_color: [u8; 4],
    /// 圆角。
    pub background_rounding: f32,
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_grid: [u32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 字体。
    pub font: String,
    /// 框选选中的文本。
    pub selection: Option<(usize, usize)>,
    /// 是否可框选。
    pub selectable: bool,
    /// 超链接文本。
    pub hyperlink_text: Vec<(String, HyperlinkSelectMethod)>,
    /// 超链接选取索引值与链接。
    pub hyperlink_index: Vec<(usize, usize, String)>,
    /// 上一帧的文本内容(用于优化超链接文本选取)。
    pub last_frame_content: String,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            discern_type: String::from("Text"),
            name: String::from("Text"),
            content: String::from("Hello world"),
            font_size: 16_f32,
            color: [255, 255, 255, 255],
            position: [0_f32, 0_f32],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            wrap_width: 200_f32,
            write_background: true,
            background_color: [0, 0, 0, 255],
            background_rounding: 2_f32,
            x_grid: [0, 0],
            y_grid: [0, 0],
            origin_position: [0_f32, 0_f32],
            font: String::new(),
            selection: None,
            selectable: true,
            hyperlink_text: Vec::new(),
            hyperlink_index: Vec::new(),
            last_frame_content: String::from("Hello world"),
        }
    }
}

impl Text {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    #[inline]
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    #[inline]
    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = [r, g, b, a];
        self
    }

    #[inline]
    pub fn center_display(
        mut self,
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        self.center_display = (horizontal_align, vertical_align);
        self
    }

    #[inline]
    pub fn wrap_width(mut self, wrap_width: f32) -> Self {
        self.wrap_width = wrap_width;
        self
    }

    #[inline]
    pub fn write_background(mut self, write_background: bool) -> Self {
        self.write_background = write_background;
        self
    }

    #[inline]
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = [r, g, b, a];
        self
    }

    #[inline]
    pub fn background_rounding(mut self, background_rounding: f32) -> Self {
        self.background_rounding = background_rounding;
        self
    }

    #[inline]
    pub fn x_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn origin_position(mut self, x: f32, y: f32) -> Self {
        self.origin_position = [x, y];
        self
    }

    #[inline]
    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_string();
        self
    }

    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    #[inline]
    pub fn hyperlink_text(
        mut self,
        target_text: &str,
        select_method: HyperlinkSelectMethod,
    ) -> Self {
        self.hyperlink_text
            .push((target_text.to_string(), select_method));
        self
    }
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

/// RC的变量资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable {
    pub discern_type: String,
    pub name: String,
    /// 变量的值。
    pub value: Value,
}

impl Default for Variable {
    fn default() -> Self {
        Variable {
            discern_type: String::from("Variable"),
            name: String::from("Variable"),
            value: Value::String(String::from("Hello world")),
        }
    }
}

impl Variable {
    pub fn new<T: Into<Value>>(self, name: &str, value: T) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: value.into(),
        }
    }

    pub fn from_bool(self, name: &str, value: bool) -> Self {
        Self {
            name: String::from(name),
            value: Value::Bool(value),
            ..self
        }
    }

    pub fn from_int(self, name: &str, value: i32) -> Self {
        Self {
            name: String::from(name),
            value: Value::Int(value),
            ..self
        }
    }

    pub fn from_uint(self, name: &str, value: u32) -> Self {
        Self {
            name: String::from(name),
            value: Value::UInt(value),
            ..self
        }
    }

    pub fn from_float(self, name: &str, value: f32) -> Self {
        Self {
            name: String::from(name),
            value: Value::Float(value),
            ..self
        }
    }

    pub fn from_vec(self, name: &str, value: Vec<Value>) -> Self {
        Self {
            name: String::from(name),
            value: Value::Vec(value),
            ..self
        }
    }

    pub fn from_string<T: Into<String>>(self, name: &str, value: T) -> Self {
        Self {
            name: String::from(name),
            value: Value::String(value.into()),
            ..self
        }
    }
}

/// RC的字体资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    pub name: String,
    pub discern_type: String,
    /// 字体定义。
    pub font_definitions: FontDefinitions,
    /// 字体路径。
    pub path: String,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            discern_type: String::from("Font"),
            name: String::from("Font"),
            font_definitions: FontDefinitions::default(),
            path: String::from(""),
        }
    }
}

impl Font {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
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

/// RC的时间分段资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SplitTime {
    pub discern_type: String,
    pub name: String,
    /// 时间点（第一个值为页面运行时间，第二个值为总运行时间）。
    pub time: [f32; 2],
}

impl Default for SplitTime {
    fn default() -> Self {
        Self {
            discern_type: String::from("SplitTime"),
            name: String::from("SplitTime"),
            time: [0_f32, 0_f32],
        }
    }
}

impl SplitTime {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
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

/// RC的开关资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    pub discern_type: String,
    pub name: String,
    /// 外观（包括图片纹理和叠加颜色，数量为开启的动画数量*开关状态总数）。
    pub appearance: Vec<SwitchData>,
    /// 开关使用的图片名称。
    pub switch_image_name: String,
    /// 是否启用鼠标悬浮和点击时的动画。
    pub enable_hover_click_image: [bool; 2],
    /// 开关当前状态。
    pub state: u32,
    /// 可以用于点击开关的方法（包含点击方式和是否改变开关状态两个参数）。
    pub click_method: Vec<SwitchClickAction>,
    /// 上一次渲染是否有鼠标悬浮。
    pub last_time_hovered: bool,
    /// 上一次渲染是否被鼠标点击。
    pub last_time_clicked: bool,
    /// 上一次点击对应的点击方法的索引。
    pub last_time_clicked_index: usize,
    /// 动画总数。
    pub animation_count: u32,
    /// 提示文本资源名。
    pub hint_text_name: String,
    /// 开关上的文本资源名称（不需要可留空）。
    pub text_name: String,
    /// 开关文本资源的原始位置。
    pub text_origin_position: [f32; 2],
    /// 开关点击音效路径。
    pub sound_path: String,
}

impl Default for Switch {
    fn default() -> Self {
        Self {
            discern_type: String::from("Switch"),
            name: String::from("Switch"),
            appearance: vec![],
            switch_image_name: String::from("Image"),
            enable_hover_click_image: [false, false],
            state: 0,
            click_method: vec![],
            last_time_hovered: false,
            last_time_clicked: false,
            last_time_clicked_index: 0,
            animation_count: 0,
            hint_text_name: String::from("HintText"),
            text_name: String::from("Text"),
            text_origin_position: [0_f32, 0_f32],
            sound_path: String::from(""),
        }
    }
}

impl Switch {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn appearance(mut self, appearance: Vec<SwitchData>) -> Self {
        self.appearance = appearance;
        self
    }

    #[inline]
    pub fn enable_hover_click_image(
        mut self,
        enable_hover_image: bool,
        enable_click_image: bool,
    ) -> Self {
        self.enable_hover_click_image = [enable_hover_image, enable_click_image];
        self
    }

    #[inline]
    pub fn click_method(mut self, click_method: Vec<SwitchClickAction>) -> Self {
        self.click_method = click_method;
        self
    }

    #[inline]
    pub fn sound_path(mut self, sound_path: &str) -> Self {
        self.sound_path = sound_path.to_string();
        self
    }
}

/// 渲染的RC资源。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RenderResource {
    pub discern_type: String,
    pub name: String,
}

/// 开关的外观。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwitchData {
    /// 开关的纹理。
    pub texture: String,
    /// 开关的颜色。
    pub color: [u8; 4],
    /// 开关上的文本内容。
    pub text: String,
    /// 开关上的提示文本。
    pub hint_text: String,
}

/// 开关的点击方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchClickAction {
    /// 开关的点击方法。
    pub click_method: PointerButton,
    /// 点击后是否改变开关状态。
    pub action: bool,
}

/// RC的消息框资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageBox {
    pub discern_type: String,
    pub name: String,
    /// 消息框大小。
    pub size: [f32; 2],
    /// 框内内容资源名。
    pub content_name: String,
    /// 框内标题资源名。
    pub title_name: String,
    /// 框内图片资源名。
    pub image_name: String,
    /// 消息框是否持续存在。
    pub keep_existing: bool,
    /// 如果不持续存在，消息框的持续时间。
    pub existing_time: f32,
    /// 消息框是否存在（不等于是否显示）。
    pub exist: bool,
    /// 消息框移动速度。
    pub speed: f32,
    /// 消息框补位速度。
    pub restore_speed: f32,
    /// 消息框上一次渲染时的y轴偏移量（用于实现补位动画）。
    pub memory_offset: f32,
}

impl Default for MessageBox {
    fn default() -> Self {
        Self {
            discern_type: String::from("MessageBox"),
            name: String::from("MessageBox"),
            size: [100_f32, 100_f32],
            content_name: String::from("Content"),
            title_name: String::from("Title"),
            image_name: String::from("Image"),
            keep_existing: false,
            existing_time: 3_f32,
            exist: true,
            speed: 30_f32,
            restore_speed: 10_f32,
            memory_offset: 0_f32,
        }
    }
}

impl MessageBox {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = [width, height];
        self
    }

    #[inline]
    pub fn keep_existing(mut self, keep_existing: bool) -> Self {
        self.keep_existing = keep_existing;
        self
    }

    #[inline]
    pub fn existing_time(mut self, existing_time: f32) -> Self {
        self.existing_time = existing_time;
        self
    }

    #[inline]
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    #[inline]
    pub fn restore_speed(mut self, restore_speed: f32) -> Self {
        self.restore_speed = restore_speed;
        self
    }
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

/// 用于将RC资源存储进vec的枚举。
#[derive(Clone)]
pub enum RCR {
    /// 图片。
    Image(Image),
    /// 文本。
    Text(Text),
    /// 自定义矩形。
    CustomRect(CustomRect),
    /// 变量
    Variable(Variable),
    /// 字体。
    Font(Font),
    /// 时间分段。
    SplitTime(SplitTime),
    /// 开关。
    Switch(Switch),
    /// 消息框。
    MessageBox(MessageBox),
    /// 图片纹理。
    ImageTexture(ImageTexture),
    /// 页面数据。
    PageData(PageData),
}

/// RC资源最基本的错误处理。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustConstructorError {
    /// 图片获取失败。
    ImageGetFailed { image_path: String },
    /// 图片未找到。
    ImageNotFound { image_name: String },
    /// 文本未找到。
    TextNotFound { text_name: String },
    /// 变量未找到。
    VariableNotFound { variable_name: String },
    /// 变量获取失败。
    VariableNotInt { variable_name: String },
    /// 变量获取失败。
    VariableNotUInt { variable_name: String },
    /// 变量获取失败。
    VariableNotFloat { variable_name: String },
    /// 变量获取失败。
    VariableNotVec { variable_name: String },
    /// 变量获取失败。
    VariableNotBool { variable_name: String },
    /// 变量获取失败。
    VariableNotString { variable_name: String },
    /// 分段时间未找到。
    SplitTimeNotFound { split_time_name: String },
    /// 开关外观数量不匹配。
    SwitchAppearanceMismatch { switch_name: String, differ: u32 },
    /// 开关未找到。
    SwitchNotFound { switch_name: String },
    /// 消息框已存在。
    MessageBoxAlreadyExists { message_box_name: String },
    /// 获取字体失败。
    FontGetFailed { font_path: String },
    /// 字体未找到。
    FontNotFound { font_name: String },
    /// 资源未找到。
    ResourceNotFound {
        resource_name: String,
        resource_type: String,
    },
    /// 页面未找到。
    PageNotFound { page_name: String },
}

impl Default for RustConstructorError {
    fn default() -> Self {
        RustConstructorError::ImageGetFailed {
            image_path: "".to_string(),
        }
    }
}

/// 水平对齐方法。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HorizontalAlign {
    /// 左对齐。
    #[default]
    Left,
    /// 居中对齐。
    Center,
    /// 右对齐。
    Right,
}

/// 垂直对齐方法。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerticalAlign {
    /// 顶部对齐。
    #[default]
    Top,
    /// 居中对齐。
    Center,
    /// 底部对齐。
    Bottom,
}

/// 程序主体。
#[derive(Clone)]
pub struct App {
    /// 配置项。
    pub config: Config,
    /// 文本。
    pub app_text: AppText,
    /// RC资源。
    pub rust_constructor_resource: Vec<RCR>,
    /// 渲染资源列表。
    pub render_resource_list: Vec<RenderResource>,
    /// 问题列表。
    pub problem_list: Vec<Problem>,
    /// RC资源刷新率。
    pub vertrefresh: f32,
    /// 当前页面。
    pub page: String,
    /// 计时器。
    pub timer: Timer,
    /// 帧时间。
    pub frame_times: Vec<f32>,
    /// 上一帧时间。
    pub last_frame_time: Option<f64>,
    /// 托盘图标。
    pub tray_icon: Option<tray_icon::TrayIcon>,
    /// 托盘图标是否已创建。
    pub tray_icon_created: bool,
}

impl App {
    /// 初始化程序。
    pub fn new(config: Config, app_text: AppText) -> Self {
        Self {
            config,
            app_text,
            rust_constructor_resource: vec![],
            render_resource_list: Vec::new(),
            problem_list: Vec::new(),
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

    /// 整合所有页面需要一次性处理的功能。
    pub fn page_handler(&mut self, ctx: &egui::Context) {
        // 更新帧数
        self.update_frame_stats(ctx);
        // 更新渲染资源列表
        self.render_resource_list = Vec::new();
        // 更新计时器
        self.update_timer();
        if let Ok(id) = self.get_resource_index("PageData", &self.page.clone())
            && let RCR::PageData(pd) = self.rust_constructor_resource[id].clone()
            && pd.forced_update
        {
            // 请求重新绘制界面
            ctx.request_repaint();
        };
    }

    /// 运行时添加新页面。
    pub fn add_page(&mut self, mut page_data: PageData) {
        page_data.change_page_updated = false;
        page_data.enter_page_updated = false;
        self.rust_constructor_resource
            .push(RCR::PageData(page_data));
    }

    /// 切换页面。
    pub fn switch_page(&mut self, page: &str) {
        if let Ok(id) = self.get_resource_index("PageData", page) {
            self.page = page.to_string();
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                pd.enter_page_updated = false;
                self.timer.start_time = self.timer.total_time;
                self.update_timer();
            };
        };
    }

    /// 初始化托盘图标。
    pub fn tray_icon_init(&mut self, icon_path: &str, tooltip: &str, menu: Box<Menu>) {
        let icon = load_icon_from_file(icon_path).unwrap();
        if let Ok(tray_icon) = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip(tooltip)
            .with_icon(icon)
            .with_icon_as_template(true)
            .build()
        {
            self.tray_icon = Some(tray_icon);
            self.tray_icon_created = true;
        };
    }

    /// 检查是否存在特定资源。
    pub fn check_resource_exists(&mut self, resource_type: &str, resource_name: &str) -> bool {
        for i in 0..self.rust_constructor_resource.len() {
            match self.rust_constructor_resource[i].clone() {
                RCR::Image(im) => {
                    if im.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::Text(t) => {
                    if t.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::CustomRect(cr) => {
                    if cr.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::Variable(v) => {
                    if v.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::Font(f) => {
                    if f.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::SplitTime(st) => {
                    if st.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::Switch(s) => {
                    if s.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::MessageBox(mb) => {
                    if mb.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::ImageTexture(it) => {
                    if it.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
                RCR::PageData(pd) => {
                    if pd.match_resource(resource_name, resource_type) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 获取资源索引。
    pub fn get_resource_index(
        &mut self,
        resource_type: &str,
        resource_name: &str,
    ) -> Result<usize, RustConstructorError> {
        for i in 0..self.rust_constructor_resource.len() {
            match self.rust_constructor_resource[i].clone() {
                RCR::Image(im) => {
                    if im.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::Text(t) => {
                    if t.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::CustomRect(cr) => {
                    if cr.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::Variable(v) => {
                    if v.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::Font(f) => {
                    if f.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::SplitTime(st) => {
                    if st.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::Switch(s) => {
                    if s.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::MessageBox(mb) => {
                    if mb.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::ImageTexture(it) => {
                    if it.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
                RCR::PageData(pd) => {
                    if pd.match_resource(resource_name, resource_type) {
                        return Ok(i);
                    }
                }
            };
        }
        self.problem_report(
            RustConstructorError::ResourceNotFound {
                resource_name: resource_name.to_string(),
                resource_type: resource_type.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::ResourceNotFound {
            resource_name: resource_name.to_string(),
            resource_type: resource_type.to_string(),
        })
    }

    /// 添加字体资源。
    pub fn add_fonts(&mut self, mut font: Font) {
        let mut fonts = FontDefinitions::default();
        if let Ok(font_read_data) = std::fs::read(font.path.clone()) {
            let font_data: Arc<Vec<u8>> = Arc::new(font_read_data);
            fonts.font_data.insert(
                font.name.to_owned(),
                Arc::new(FontData::from_owned(
                    Arc::try_unwrap(font_data).ok().unwrap(),
                )),
            );

            // 将字体添加到字体列表中
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font.name.to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font.name.to_owned());

            font.font_definitions = fonts;
            self.rust_constructor_resource.push(RCR::Font(font));
        } else {
            self.problem_report(
                RustConstructorError::FontGetFailed {
                    font_path: font.path.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
        };
    }

    /// 输出字体资源。
    pub fn font(&mut self, name: &str) -> Result<FontDefinitions, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Font", name)
            && let RCR::Font(f) = &mut self.rust_constructor_resource[id]
        {
            return Ok(f.font_definitions.clone());
        }
        self.problem_report(
            RustConstructorError::FontNotFound {
                font_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::FontNotFound {
            font_name: name.to_string(),
        })
    }

    /// 将所有已添加到RC的字体资源添加到egui中。
    pub fn register_all_fonts(&mut self, ctx: &egui::Context) {
        let mut font_definitions = egui::FontDefinitions::default();
        let mut font_resources = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let RCR::Font(f) = &self.rust_constructor_resource[i] {
                font_resources.push(f.clone());
            };
        }
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
        }
        ctx.set_fonts(font_definitions);
    }

    /// 发生问题时推送报告。
    pub fn problem_report(
        &mut self,
        problem_type: RustConstructorError,
        severity_level: SeverityLevel,
    ) {
        let (problem, annotation) = match problem_type.clone() {
            RustConstructorError::FontGetFailed { font_path } => (
                format!("Font get failed: {}", font_path,),
                "Please check if the font file exists and the path is correct.",
            ),
            RustConstructorError::FontNotFound { font_name } => (
                format!("Font not found: {}", font_name,),
                "Please check whether the font has been added.",
            ),
            RustConstructorError::ImageGetFailed { image_path } => (
                format!("Image get failed: {}", image_path,),
                "Please check whether the image path is correct and whether the image has been added.",
            ),
            RustConstructorError::ImageNotFound { image_name } => (
                format!("Image not found: {}", image_name,),
                "Please check whether the image has been added.",
            ),
            RustConstructorError::TextNotFound { text_name } => (
                format!("Text not found: {}", text_name,),
                "Please check whether the text has been added.",
            ),
            RustConstructorError::MessageBoxAlreadyExists { message_box_name } => (
                format!("Message box already exists: {}", message_box_name),
                "Please check whether the code for generating the message box has been accidentally called multiple times.",
            ),
            RustConstructorError::SplitTimeNotFound { split_time_name } => (
                format!("Split time not found: {}", split_time_name,),
                "Please check whether the split time has been added.",
            ),
            RustConstructorError::SwitchAppearanceMismatch {
                switch_name,
                differ,
            } => (
                format!(
                    "Switch appearance list's number of items is large / small {} more: {}",
                    differ, switch_name
                ),
                "Please check whether the number of appearance list items matches the number of enabled animations.",
            ),
            RustConstructorError::SwitchNotFound { switch_name } => (
                format!("Switch not found: {}", switch_name,),
                "Please check whether the switch has been added.",
            ),
            RustConstructorError::PageNotFound { page_name } => (
                format!("Page not found: {}", page_name,),
                "Please check whether the page has been added.",
            ),
            RustConstructorError::VariableNotFound { variable_name } => (
                format!("Variable not found: {}", variable_name,),
                "Please check whether the variable has been added.",
            ),
            RustConstructorError::VariableNotBool { variable_name } => (
                format!("Variable is not bool: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::VariableNotFloat { variable_name } => (
                format!("Variable is not f32: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::VariableNotInt { variable_name } => (
                format!("Variable is not int: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::VariableNotString { variable_name } => (
                format!("Variable is not string: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::VariableNotUInt { variable_name } => (
                format!("Variable is not uint: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::VariableNotVec { variable_name } => (
                format!("Variable is not vec: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.",
            ),
            RustConstructorError::ResourceNotFound {
                resource_name,
                resource_type,
            } => (
                format!(
                    "Resource not found: {}(\"{}\")",
                    resource_type, resource_name,
                ),
                "Please check whether the resource has been added.",
            ),
        };
        // 如果处于严格模式下，则直接崩溃！
        if self.config.rc_strict_mode {
            panic!("{}", problem);
        } else {
            let sound = self.config.problem_report_sound.clone();
            std::thread::spawn(move || {
                play_wav(&sound).unwrap_or(0_f64);
            });
            self.problem_list.push(Problem {
                severity_level,
                problem,
                annotation: annotation.to_string(),
                report_state: ReportState {
                    current_page: self.page.clone(),
                    current_total_runtime: self.timer.total_time,
                    current_page_runtime: self.timer.now_time,
                },
                problem_type: problem_type.clone(),
            });
        };
    }

    /// 检查页面是否已完成首次加载。
    pub fn check_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("PageData", name)
            && let RCR::PageData(pd) = self.rust_constructor_resource[id].clone()
        {
            if !pd.change_page_updated {
                self.new_page_update(name);
            };
            return Ok(pd.change_page_updated);
        };
        self.problem_report(
            RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::PageNotFound {
            page_name: name.to_string(),
        })
    }

    /// 检查页面是否已完成加载。
    pub fn check_enter_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("PageData", name)
            && let RCR::PageData(pd) = &mut self.rust_constructor_resource[id]
        {
            let return_value = pd.enter_page_updated;
            pd.enter_page_updated = true;
            return Ok(return_value);
        };
        self.problem_report(
            RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::PageNotFound {
            page_name: name.to_string(),
        })
    }

    /// 进入新页面时的更新。
    pub fn new_page_update(&mut self, name: &str) {
        if let Ok(id) = self.get_resource_index("PageData", name) {
            self.timer.start_time = self.timer.total_time;
            self.update_timer();
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                pd.change_page_updated = true;
            };
        };
    }

    /// 更新帧数。
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

    /// 更新帧数显示。
    pub fn current_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            1.0 / (self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32)
        }
    }

    /// 添加分段时间。
    pub fn add_split_time(&mut self, mut split_time: SplitTime) {
        split_time.time = [self.timer.now_time, self.timer.total_time];
        self.rust_constructor_resource
            .push(RCR::SplitTime(split_time));
    }

    /// 重置分段时间。
    pub fn reset_split_time(&mut self, name: &str) {
        if let Ok(id) = self.get_resource_index("SplitTime", name)
            && let RCR::SplitTime(st) = &mut self.rust_constructor_resource[id]
        {
            st.time = [self.timer.now_time, self.timer.total_time];
        };
        self.problem_report(
            RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
    }

    /// 输出分段时间。
    pub fn split_time(&mut self, name: &str) -> Result<[f32; 2], RustConstructorError> {
        if let Ok(id) = self.get_resource_index("SplitTime", name)
            && let RCR::SplitTime(st) = self.rust_constructor_resource[id].clone()
        {
            return Ok(st.time);
        };
        self.problem_report(
            RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::SplitTimeNotFound {
            split_time_name: name.to_string(),
        })
    }

    /// 更新计时器。
    pub fn update_timer(&mut self) {
        let elapsed = self.timer.timer.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();
        self.timer.total_time = seconds as f32 + milliseconds as f32 / 1000.0;
        self.timer.now_time = self.timer.total_time - self.timer.start_time
    }

    /// 添加矩形资源。
    pub fn add_custom_rect(&mut self, custom_rect: CustomRect) {
        self.rust_constructor_resource
            .push(RCR::CustomRect(custom_rect));
    }

    /// 显示矩形资源。
    pub fn custom_rect(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("CustomRect", name)
            && let RCR::CustomRect(cr) = &mut self.rust_constructor_resource[id]
        {
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
            let pos_x = match cr.center_display.0 {
                HorizontalAlign::Left => cr.position[0],
                HorizontalAlign::Center => cr.position[0] - cr.size[0] / 2.0,
                HorizontalAlign::Right => cr.position[0] - cr.size[0],
            };
            let pos_y = match cr.center_display.1 {
                VerticalAlign::Top => cr.position[1],
                VerticalAlign::Center => cr.position[1] - cr.size[1] / 2.0,
                VerticalAlign::Bottom => cr.position[1] - cr.size[1],
            };
            ui.painter().rect(
                Rect::from_min_max(
                    Pos2::new(pos_x, pos_y),
                    Pos2::new(pos_x + cr.size[0], pos_y + cr.size[1]),
                ),
                cr.rounding,
                Color32::from_rgba_unmultiplied(cr.color[0], cr.color[1], cr.color[2], cr.color[3]),
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
    }

    /// 添加文本资源。
    pub fn add_text(&mut self, text: Text) {
        self.rust_constructor_resource.push(RCR::Text(text));
    }

    /// 显示文本资源。
    pub fn text(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Text", name)
            && let RCR::Text(mut t) = self.rust_constructor_resource[id].clone()
        {
            t.reg_render_resource(&mut self.render_resource_list);
            // 计算文本大小
            let galley = ui.fonts(|f| {
                f.layout(
                    t.content.to_string(),
                    if self.check_resource_exists("Font", &t.font.clone()) {
                        FontId::new(t.font_size, egui::FontFamily::Name(t.font.clone().into()))
                    } else {
                        FontId::proportional(t.font_size)
                    },
                    Color32::from_rgba_unmultiplied(t.color[0], t.color[1], t.color[2], t.color[3]),
                    t.wrap_width,
                )
            });
            let text_size = galley.size();
            t.position[0] = match t.x_grid[1] {
                0 => t.origin_position[0],
                _ => {
                    (ctx.available_rect().width() as f64 / t.x_grid[1] as f64 * t.x_grid[0] as f64)
                        as f32
                        + t.origin_position[0]
                }
            };
            t.position[1] = match t.y_grid[1] {
                0 => t.origin_position[1],
                _ => {
                    (ctx.available_rect().height() as f64 / t.y_grid[1] as f64 * t.y_grid[0] as f64)
                        as f32
                        + t.origin_position[1]
                }
            };
            let pos_x = match t.center_display.0 {
                HorizontalAlign::Left => t.position[0],
                HorizontalAlign::Center => t.position[0] - text_size.x / 2.0,
                HorizontalAlign::Right => t.position[0] - text_size.x,
            };
            let pos_y = match t.center_display.1 {
                VerticalAlign::Top => t.position[1],
                VerticalAlign::Center => t.position[1] - text_size.y / 2.0,
                VerticalAlign::Bottom => t.position[1] - text_size.y,
            };
            // 使用绝对定位放置文本
            let position = Pos2::new(pos_x, pos_y);

            if t.write_background {
                let rect = Rect::from_min_size(position, text_size);
                // 绘制背景颜色
                ui.painter().rect_filled(
                    rect,
                    t.background_rounding,
                    Color32::from_rgba_unmultiplied(
                        t.background_color[0],
                        t.background_color[1],
                        t.background_color[2],
                        t.background_color[3],
                    ),
                ); // 背景色
            };
            // 绘制文本
            ui.painter().galley(
                position,
                galley.clone(),
                Color32::from_rgba_unmultiplied(
                    t.color[0], t.color[1], t.color[2], t.color[3], // 应用透明度
                ),
            );

            // 查找超链接索引值
            if t.last_frame_content != t.content {
                t.hyperlink_index.clear();
                for (text, method) in &t.hyperlink_text {
                    let matches: Vec<(usize, &str)> = t.content.match_indices(text).collect();
                    if let HyperlinkSelectMethod::All(url) = method {
                        for (index, _) in matches {
                            t.hyperlink_index
                                .push((index, index + text.len(), url.clone()));
                        }
                    } else if let HyperlinkSelectMethod::Segment(list) = method {
                        for (index, url) in list {
                            if *index >= matches.len() {
                                continue;
                            };
                            t.hyperlink_index.push((
                                matches[*index].0,
                                matches[*index].0 + text.len(),
                                url.clone(),
                            ));
                        }
                    };
                }
            };

            // 绘制超链接
            for (start, end, _) in &t.hyperlink_index {
                // 获取超链接文本的范围
                let start_cursor = galley.pos_from_cursor(CCursor::new(*start));
                let end_cursor = galley.pos_from_cursor(CCursor::new(*end));

                let start_pos = start_cursor.left_top();
                let end_pos = end_cursor.right_top();
                // 绘制超链接下划线
                // 检查超链接是否跨行
                if start_cursor.min.y == end_cursor.min.y {
                    // 单行超链接
                    let underline_y = position.y
                        + start_pos.y
                        + galley.rows.first().map_or(14.0, |row| row.height())
                        - 2.0;

                    // 绘制下划线
                    let color = Color32::from_rgba_unmultiplied(
                        t.color[0], t.color[1], t.color[2], t.color[3],
                    );

                    ui.painter().line_segment(
                        [
                            Pos2::new(position.x + start_pos.x, underline_y),
                            Pos2::new(position.x + end_pos.x, underline_y),
                        ],
                        Stroke::new(t.font_size / 10_f32, color),
                    );
                } else {
                    // 多行超链接
                    let row_height = galley.rows.first().map_or(14.0, |row| row.height()); // 默认行高14.0

                    // 计算起始行和结束行的索引
                    let start_row = (start_pos.y / row_height).round() as usize;
                    let end_row = (end_pos.y / row_height).round() as usize;

                    for row in start_row..=end_row {
                        let row_y = position.y + row as f32 * row_height + row_height - 2.0; // 行底部稍微上移一点绘制下划线

                        // 获取当前行的矩形范围
                        if let Some(current_row) = galley.rows.get(row) {
                            let row_rect = current_row.rect();

                            let color = Color32::from_rgba_unmultiplied(
                                t.color[0], t.color[1], t.color[2], t.color[3],
                            );

                            if row == start_row {
                                // 第一行从文本开始位置到行尾
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(position.x + start_pos.x, row_y),
                                        Pos2::new(position.x + row_rect.max.x, row_y),
                                    ],
                                    Stroke::new(t.font_size / 10_f32, color),
                                );
                            } else if row == end_row {
                                // 最后一行从行首到文本结束位置
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(position.x + row_rect.min.x, row_y),
                                        Pos2::new(position.x + end_pos.x, row_y),
                                    ],
                                    Stroke::new(t.font_size / 10_f32, color),
                                );
                            } else {
                                // 中间整行下划线
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(position.x + row_rect.min.x, row_y),
                                        Pos2::new(position.x + row_rect.max.x, row_y),
                                    ],
                                    Stroke::new(t.font_size / 10_f32, color),
                                );
                            };
                        };
                    }
                };
            }

            if t.selectable {
                let rect = Rect::from_min_size(
                    [position[0] - 20_f32, position[1] - 5_f32].into(),
                    [text_size[0] + 40_f32, text_size[1] + 10_f32].into(),
                );

                let rect2 = Rect::from_min_size(
                    [0_f32, 0_f32].into(),
                    [ctx.available_rect().width(), ctx.available_rect().height()].into(),
                );

                // 创建可交互的区域
                let response = ui.interact(
                    rect,
                    egui::Id::new(format!("text_{}_click_and_drag", t.name)),
                    egui::Sense::click_and_drag(),
                );

                let response2 = ui.interact(
                    rect2,
                    egui::Id::new(format!("text_{}_total", t.name)),
                    egui::Sense::click(),
                );

                // 处理选择逻辑
                let cursor_at_pointer = |pointer_pos: Vec2| -> usize {
                    let relative_pos = pointer_pos - position.to_vec2();
                    let cursor = galley.cursor_from_pos(relative_pos);
                    cursor.index
                };

                if !response.clicked() && response2.clicked() {
                    t.selection = None;
                };

                if response.clicked() || response.drag_started() {
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                        let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                        t.selection = Some((cursor, cursor));
                    };
                    response.request_focus();
                };

                if response.dragged()
                    && t.selection.is_some()
                    && let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos())
                {
                    let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                    if let Some((start, _)) = t.selection {
                        t.selection = Some((start, cursor));
                    };
                };

                // 处理复制操作
                if response.has_focus() {
                    // 处理复制操作 - 使用按键释放事件
                    let copy_triggered = ui.input(|input| {
                        let c_released = input.key_released(egui::Key::C);
                        let cmd_pressed = input.modifiers.command || input.modifiers.mac_cmd;
                        let ctrl_pressed = input.modifiers.ctrl;
                        c_released && (cmd_pressed || ctrl_pressed)
                    });
                    if copy_triggered && let Some((start, end)) = t.selection {
                        let (start, end) = (start.min(end), start.max(end));
                        let chars: Vec<char> = t.content.chars().collect();
                        if start <= chars.len() && end <= chars.len() && start < end {
                            let selected_text: String = chars[start..end].iter().collect();
                            ui.ctx().copy_text(selected_text);
                        };
                    };
                };

                // 绘制选择区域背景
                if let Some((start, end)) = t.selection {
                    let (start, end) = (start.min(end), start.max(end));
                    if start != end {
                        // 获取选择区域的范围
                        let start_cursor = galley.pos_from_cursor(CCursor::new(start));
                        let end_cursor = galley.pos_from_cursor(CCursor::new(end));

                        let start_pos = start_cursor.left_top();
                        let end_pos = end_cursor.right_top();
                        // 选择框绘制
                        if start_pos.y == end_pos.y {
                            // 单行选择
                            // 修复：使用实际行的高度而不是整个文本的高度除以行数
                            let rows = &galley.rows;
                            let row_height = if !rows.is_empty() {
                                // 获取实际行的高度
                                if let Some(row) = rows.first() {
                                    row.height()
                                } else {
                                    text_size.y / t.content.lines().count() as f32
                                }
                            } else {
                                text_size.y / t.content.lines().count() as f32
                            };

                            let selection_rect = Rect::from_min_max(
                                Pos2::new(position.x + start_pos.x, position.y + start_pos.y),
                                Pos2::new(
                                    position.x + end_pos.x,
                                    position.y + start_pos.y + row_height,
                                ),
                            );
                            ui.painter().rect_filled(
                                selection_rect,
                                0.0,
                                Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                            );
                        } else {
                            // 多行选择 - 为每行创建精确的矩形
                            let rows = &galley.rows;
                            let row_height = if !rows.is_empty() {
                                rows[0].height()
                            } else {
                                text_size.y / t.content.lines().count() as f32
                            };

                            // 计算选择的上下边界
                            let selection_top = position.y + start_pos.y.min(end_pos.y);
                            let selection_bottom = position.y + start_pos.y.max(end_pos.y);

                            // 确定起始行和结束行的索引
                            let start_row_index = (start_pos.y / row_height).floor() as usize;
                            let end_row_index = (end_pos.y / row_height).floor() as usize;
                            let (first_row_index, last_row_index) =
                                if start_row_index <= end_row_index {
                                    (start_row_index, end_row_index)
                                } else {
                                    (end_row_index, start_row_index)
                                };

                            for (i, row) in rows.iter().enumerate() {
                                let row_y = position.y + row_height * i as f32;
                                let row_bottom = row_y + row_height;
                                // 检查当前行是否与选择区域相交
                                if row_bottom > selection_top && row_y <= selection_bottom {
                                    let left = if i == first_row_index {
                                        // 首行 - 从选择开始位置开始
                                        position.x + start_pos.x
                                    } else {
                                        // 非首行 - 从行首开始
                                        position.x + row.rect().min.x
                                    };

                                    let right = if i == last_row_index {
                                        // 尾行 - 到选择结束位置结束
                                        position.x + end_pos.x
                                    } else {
                                        // 非尾行 - 到行尾结束
                                        position.x + row.rect().max.x
                                    };

                                    let selection_rect = Rect::from_min_max(
                                        Pos2::new(left, row_y),
                                        Pos2::new(right, row_bottom),
                                    );

                                    // 确保矩形有效
                                    if selection_rect.width() > 0.0 && selection_rect.height() > 0.0
                                    {
                                        ui.painter().rect_filled(
                                            selection_rect,
                                            0.0,
                                            Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                                        );
                                    };
                                };
                            }
                        };
                    };
                };
            };

            // 处理超链接操作
            for (start, end, url) in &t.hyperlink_index {
                // 获取超链接文本的范围
                let start_cursor = galley.pos_from_cursor(CCursor::new(*start));
                let end_cursor = galley.pos_from_cursor(CCursor::new(*end));

                let start_pos = start_cursor.left_top();
                let end_pos = end_cursor.right_top();

                let row_height = galley.rows.first().map_or(14.0, |row| row.height());

                // 为超链接创建交互响应对象
                let link_responses = if start_cursor.min.y == end_cursor.min.y {
                    // 单行超链接
                    let link_rect = Rect::from_min_max(
                        Pos2::new(position.x + start_pos.x, position.y + start_pos.y),
                        Pos2::new(
                            position.x + end_pos.x,
                            position.y + start_pos.y + row_height,
                        ),
                    );
                    vec![ui.interact(
                        link_rect,
                        egui::Id::new(format!("link_{}_{}_{}", t.name, start, end)),
                        egui::Sense::click(),
                    )]
                } else {
                    // 多行超链接
                    let start_row = (start_pos.y / row_height).round() as usize;
                    let end_row = (end_pos.y / row_height).round() as usize;
                    let mut responses = Vec::new();

                    for row in start_row..=end_row {
                        if let Some(current_row) = galley.rows.get(row) {
                            let row_rect = current_row.rect();
                            let row_y = position.y + row as f32 * row_height;

                            let link_rect = if row == start_row {
                                // 第一行从文本开始位置到行尾
                                Rect::from_min_max(
                                    Pos2::new(position.x + start_pos.x, row_y),
                                    Pos2::new(position.x + row_rect.max.x, row_y + row_height),
                                )
                            } else if row == end_row {
                                // 最后一行从行首到文本结束位置
                                Rect::from_min_max(
                                    Pos2::new(position.x + row_rect.min.x, row_y),
                                    Pos2::new(position.x + end_pos.x, row_y + row_height),
                                )
                            } else {
                                // 中间整行
                                Rect::from_min_max(
                                    Pos2::new(position.x + row_rect.min.x, row_y),
                                    Pos2::new(position.x + row_rect.max.x, row_y + row_height),
                                )
                            };

                            responses.push(ui.interact(
                                link_rect,
                                egui::Id::new(format!(
                                    "link_{}_{}_{}_row_{}",
                                    t.name, start, end, row
                                )),
                                egui::Sense::click(),
                            ));
                        };
                    }
                    responses
                };

                // 检查是否正在点击这个超链接
                let mut is_pressing_link = false;
                for link_response in &link_responses {
                    if link_response.is_pointer_button_down_on() && !link_response.drag_started() {
                        t.selection = None;
                        if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            let relative_pos = pointer_pos - position.to_vec2();
                            let cursor = galley.cursor_from_pos(relative_pos.to_vec2());
                            if cursor.index >= *start && cursor.index <= *end {
                                is_pressing_link = true;
                                break;
                            };
                        };
                    };
                }

                // 检查是否释放了鼠标（点击完成）
                let mut clicked_on_link = false;
                for link_response in &link_responses {
                    if link_response.clicked()
                        && let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos())
                    {
                        let relative_pos = pointer_pos - position.to_vec2();
                        let cursor = galley.cursor_from_pos(relative_pos.to_vec2());
                        if cursor.index >= *start && cursor.index <= *end {
                            clicked_on_link = true;
                            break;
                        };
                    };
                }

                if clicked_on_link {
                    // 执行超链接跳转
                    if !url.is_empty() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    };
                };

                // 绘制超链接高亮（如果正在点击或悬停）
                if is_pressing_link {
                    if start_cursor.min.y == end_cursor.min.y {
                        // 单行超链接高亮
                        let selection_rect = Rect::from_min_max(
                            Pos2::new(position.x + start_pos.x, position.y + start_pos.y),
                            Pos2::new(
                                position.x + end_pos.x,
                                position.y
                                    + start_pos.y
                                    + galley.rows.first().map_or(14.0, |row| row.height()),
                            ),
                        );
                        ui.painter().rect_filled(
                            selection_rect,
                            0.0,
                            Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                        );
                    } else {
                        // 多行超链接高亮
                        let row_height = galley.rows.first().map_or(14.0, |row| row.height());
                        let start_row = (start_pos.y / row_height).round() as usize;
                        let end_row = (end_pos.y / row_height).round() as usize;

                        for row in start_row..=end_row {
                            if let Some(current_row) = galley.rows.get(row) {
                                let row_rect = current_row.rect();

                                if row == start_row {
                                    // 第一行从文本开始位置到行尾
                                    let selection_rect = Rect::from_min_max(
                                        Pos2::new(
                                            position.x + start_pos.x,
                                            position.y + row as f32 * row_height,
                                        ),
                                        Pos2::new(
                                            position.x + row_rect.max.x,
                                            position.y + row as f32 * row_height + row_height,
                                        ),
                                    );
                                    ui.painter().rect_filled(
                                        selection_rect,
                                        0.0,
                                        Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                                    );
                                } else if row == end_row {
                                    // 最后一行从行首到文本结束位置
                                    let selection_rect = Rect::from_min_max(
                                        Pos2::new(
                                            position.x + row_rect.min.x,
                                            position.y + row as f32 * row_height,
                                        ),
                                        Pos2::new(
                                            position.x + end_pos.x,
                                            position.y + row as f32 * row_height + row_height,
                                        ),
                                    );
                                    ui.painter().rect_filled(
                                        selection_rect,
                                        0.0,
                                        Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                                    );
                                } else {
                                    // 中间整行高亮
                                    let selection_rect = Rect::from_min_max(
                                        Pos2::new(
                                            position.x + row_rect.min.x,
                                            position.y + row as f32 * row_height,
                                        ),
                                        Pos2::new(
                                            position.x + row_rect.max.x,
                                            position.y + row as f32 * row_height + row_height,
                                        ),
                                    );
                                    ui.painter().rect_filled(
                                        selection_rect,
                                        0.0,
                                        Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                                    );
                                };
                            };
                        }
                    };
                };
            }
            t.last_frame_content = t.content.clone();
            self.rust_constructor_resource[id] = RCR::Text(t);
        };
    }

    /// 获取文本大小。
    pub fn get_text_size(
        &mut self,
        resource_name: &str,
        ui: &mut Ui,
    ) -> Result<[f32; 2], RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Text", resource_name)
            && let RCR::Text(t) = self.rust_constructor_resource[id].clone()
        {
            let galley = ui.fonts(|f| {
                f.layout(
                    t.content.to_string(),
                    FontId::proportional(t.font_size),
                    Color32::from_rgba_unmultiplied(t.color[0], t.color[1], t.color[2], t.color[3]),
                    t.wrap_width,
                )
            });
            return Ok([galley.size().x, galley.size().y]);
        };
        self.problem_report(
            RustConstructorError::TextNotFound {
                text_name: resource_name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::TextNotFound {
            text_name: resource_name.to_string(),
        })
    }

    /// 添加变量资源。
    pub fn add_var(&mut self, variable: Variable) {
        self.rust_constructor_resource.push(RCR::Variable(variable));
    }

    /// 修改变量资源。
    pub fn modify_var<T: Into<Value>>(&mut self, name: &str, value: T) {
        if let Ok(id) = self.get_resource_index("Variable", name)
            && let RCR::Variable(v) = &mut self.rust_constructor_resource[id]
        {
            v.value = value.into();
        };
    }

    /// 取出Value变量。
    pub fn var(&mut self, name: &str) -> Result<Value, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name)
            && let RCR::Variable(v) = self.rust_constructor_resource[id].clone()
        {
            return Ok(v.clone().value);
        };
        self.problem_report(
            RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::VariableNotFound {
            variable_name: name.to_string(),
        })
    }

    /// 取出i32变量。
    pub fn var_i(&mut self, name: &str) -> Result<i32, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Int(i) => Ok(*i),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotInt {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotInt {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出u32资源。
    pub fn var_u(&mut self, name: &str) -> Result<u32, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::UInt(u) => Ok(*u),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotUInt {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotUInt {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出f32资源。
    pub fn var_f(&mut self, name: &str) -> Result<f32, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Float(f) => Ok(*f),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotFloat {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotFloat {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出布尔值资源。
    pub fn var_b(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Bool(b) => Ok(*b),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotBool {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotBool {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出包含Value的Vec资源。
    pub fn var_v(&mut self, name: &str) -> Result<Vec<Value>, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::Vec(v) => Ok(v.clone()),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotVec {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotVec {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出字符串资源。
    pub fn var_s(&mut self, name: &str) -> Result<String, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                match &v.value {
                    // 直接访问 value 字段
                    Value::String(s) => Ok(s.clone()),
                    _ => {
                        self.problem_report(
                            RustConstructorError::VariableNotString {
                                variable_name: name.to_string(),
                            },
                            SeverityLevel::SevereWarning,
                        );
                        Err(RustConstructorError::VariableNotString {
                            variable_name: name.to_string(),
                        })
                    }
                }
            } else {
                // 正常情况下不会触发。
                Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 尝试将Value转换成布尔值。
    pub fn var_decode_b(&mut self, target: Value) -> Result<bool, RustConstructorError> {
        match target {
            Value::Bool(b) => {
                // 处理布尔值
                Ok(b)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotBool {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotBool {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成i32。
    pub fn var_decode_i(&mut self, target: Value) -> Result<i32, RustConstructorError> {
        match target {
            Value::Int(i) => {
                // 处理i32整型
                Ok(i)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotInt {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotInt {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成u32。
    pub fn var_decode_u(&mut self, target: Value) -> Result<u32, RustConstructorError> {
        match target {
            Value::UInt(u) => {
                // 处理u32无符号整型
                Ok(u)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotUInt {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotUInt {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成f32。
    pub fn var_decode_f(&mut self, target: Value) -> Result<f32, RustConstructorError> {
        match target {
            Value::Float(f) => {
                // 处理浮点数
                Ok(f)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotFloat {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotFloat {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成字符串。
    pub fn var_decode_s(&mut self, target: Value) -> Result<String, RustConstructorError> {
        match target {
            Value::String(s) => {
                // 处理字符串
                Ok(s)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotString {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotString {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成Vec。
    pub fn var_decode_v(&mut self, target: Value) -> Result<Vec<Value>, RustConstructorError> {
        match target {
            Value::Vec(v) => {
                // 处理字符串
                Ok(v)
            }
            _ => {
                self.problem_report(
                    RustConstructorError::VariableNotVec {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                );
                Err(RustConstructorError::VariableNotVec {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 添加图片纹理资源。
    pub fn add_image_texture(
        &mut self,
        mut image_texture: ImageTexture,
        path: &str,
        flip: [bool; 2],
        ctx: &egui::Context,
    ) {
        if let Ok(mut file) = File::open(path) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let img_bytes = buffer;
            let img = image::load_from_memory(&img_bytes).unwrap();
            let color_data = match flip {
                [true, true] => img.fliph().flipv().into_rgba8(),
                [true, false] => img.fliph().into_rgba8(),
                [false, true] => img.flipv().into_rgba8(),
                _ => img.into_rgba8(),
            };
            let (w, h) = (color_data.width(), color_data.height());
            let raw_data: Vec<u8> = color_data.into_raw();

            let color_image =
                egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
            let loaded_image_texture = Some(ctx.load_texture(
                image_texture.name.clone(),
                color_image,
                TextureOptions::LINEAR,
            ));
            image_texture.texture = loaded_image_texture;
            image_texture.cite_path = path.to_string();
            self.rust_constructor_resource
                .push(RCR::ImageTexture(image_texture));
        } else {
            self.problem_report(
                RustConstructorError::ImageGetFailed {
                    image_path: path.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
        };
    }

    /// 输出图片纹理。
    pub fn image_texture(
        &mut self,
        name: &str,
    ) -> Result<Option<egui::TextureHandle>, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("ImageTexture", name)
            && let RCR::ImageTexture(it) = &self.rust_constructor_resource[id]
        {
            return Ok(it.texture.clone());
        };
        self.problem_report(
            RustConstructorError::ImageNotFound {
                image_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
        );
        Err(RustConstructorError::ImageNotFound {
            image_name: name.to_string(),
        })
    }

    /// 重置图片纹理。
    pub fn reset_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        ctx: &egui::Context,
    ) {
        if let Ok(id) = self.get_resource_index("ImageTexture", name)
            && let RCR::ImageTexture(it) = &mut self.rust_constructor_resource[id]
        {
            if let Ok(mut file) = File::open(path) {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                let img_bytes = buffer;
                let img = image::load_from_memory(&img_bytes).unwrap();
                let color_data = match flip {
                    [true, true] => img.fliph().flipv().into_rgba8(),
                    [true, false] => img.fliph().into_rgba8(),
                    [false, true] => img.flipv().into_rgba8(),
                    _ => img.into_rgba8(),
                };
                let (w, h) = (color_data.width(), color_data.height());
                let raw_data: Vec<u8> = color_data.into_raw();

                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
                let image_texture =
                    Some(ctx.load_texture(it.name.clone(), color_image, TextureOptions::LINEAR));
                it.texture = image_texture;
                it.cite_path = path.to_string();
            } else {
                self.problem_report(
                    RustConstructorError::ImageGetFailed {
                        image_path: path.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                );
            };
        };
    }

    /// 添加图片资源。
    pub fn add_image(&mut self, mut image: Image, image_texture_name: &str) {
        if let Ok(id) = self.get_resource_index("ImageTexture", image_texture_name)
            && let RCR::ImageTexture(it) = self.rust_constructor_resource[id].clone()
        {
            image.texture = it.texture;
            image.cite_texture = it.name.clone();
            image.last_frame_cite_texture = it.name;
            self.rust_constructor_resource.push(RCR::Image(image));
        };
    }

    /// 显示图片资源。
    pub fn image(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Image", name)
            && let RCR::Image(mut im) = self.rust_constructor_resource[id].clone()
        {
            if im.cite_texture != im.last_frame_cite_texture
                && let Ok(id2) = self.get_resource_index("ImageTexture", &im.cite_texture)
                && let RCR::ImageTexture(it) = self.rust_constructor_resource[id2].clone()
            {
                im.texture = it.texture;
            };
            im.reg_render_resource(&mut self.render_resource_list);
            im.position[0] = match im.x_grid[1] {
                0 => im.origin_position[0],
                _ => {
                    (ctx.available_rect().width() as f64 / im.x_grid[1] as f64
                        * im.x_grid[0] as f64) as f32
                        + im.origin_position[0]
                }
            };
            im.position[1] = match im.y_grid[1] {
                0 => im.origin_position[1],
                _ => {
                    (ctx.available_rect().height() as f64 / im.y_grid[1] as f64
                        * im.y_grid[0] as f64) as f32
                        + im.origin_position[1]
                }
            };
            match im.center_display.0 {
                HorizontalAlign::Left => {}
                HorizontalAlign::Center => im.position[0] -= im.size[0] / 2.0,
                HorizontalAlign::Right => im.position[0] -= im.size[0],
            };
            match im.center_display.1 {
                VerticalAlign::Top => {}
                VerticalAlign::Center => im.position[1] -= im.size[1] / 2.0,
                VerticalAlign::Bottom => im.position[1] -= im.size[1],
            };
            if let Some(texture) = &im.texture {
                let rect = Rect::from_min_size(
                    Pos2::new(im.position[0], im.position[1]),
                    Vec2::new(im.size[0], im.size[1]),
                );
                let color = if im.use_overlay_color {
                    // 创建颜色覆盖
                    Color32::from_rgba_unmultiplied(
                        im.overlay_color[0],
                        im.overlay_color[1],
                        im.overlay_color[2],
                        // 将图片透明度与覆盖颜色透明度相乘
                        (im.alpha as f32 * im.overlay_color[3] as f32 / 255.0) as u8,
                    )
                } else {
                    Color32::from_white_alpha(im.alpha)
                };

                // 直接绘制图片
                egui::Image::new(egui::ImageSource::Texture(texture.into()))
                    .tint(color)
                    .paint_at(ui, rect)
            };
            im.last_frame_cite_texture = im.cite_texture.clone();
            self.rust_constructor_resource[id] = RCR::Image(im);
        };
    }

    /// 添加消息框资源(重要事项：你需要添加一个CloseMessageBox图片纹理资源才可正常使用！)。
    pub fn add_message_box(
        &mut self,
        mut message_box: MessageBox,
        title_name: &str,
        content_name: &str,
        image_name: &str,
        sound_path: &str,
    ) {
        if !self.check_resource_exists("MessageBox", &message_box.name) {
            message_box.exist = true;
            message_box.memory_offset = 0_f32;
            message_box.image_name = String::new();
            if let Ok(id) = self.get_resource_index("Image", image_name)
                && let RCR::Image(im) = &mut self.rust_constructor_resource[id]
            {
                message_box.image_name = image_name.to_string();
                im.size = [message_box.size[1] - 15_f32, message_box.size[1] - 15_f32];
                im.center_display = (HorizontalAlign::Left, VerticalAlign::Center);
                im.x_grid = [1, 1];
                im.y_grid = [0, 1];
                im.name = format!("MessageBox{}", im.name);
            };
            message_box.title_name = String::new();
            if let Ok(id) = self.get_resource_index("Text", title_name)
                && let RCR::Text(t) = &mut self.rust_constructor_resource[id]
            {
                message_box.title_name = title_name.to_string();
                t.x_grid = [1, 1];
                t.y_grid = [0, 1];
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.wrap_width = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
            };
            message_box.content_name = String::new();
            if let Ok(id) = self.get_resource_index("Text", content_name)
                && let RCR::Text(t) = &mut self.rust_constructor_resource[id]
            {
                message_box.content_name = content_name.to_string();
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.x_grid = [1, 1];
                t.y_grid = [0, 1];
                t.wrap_width = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
            };
            self.rust_constructor_resource
                .push(RCR::MessageBox(message_box.clone()));
            if !message_box.keep_existing {
                self.add_split_time(
                    SplitTime::default().name(&format!("MessageBox{}", message_box.name)),
                );
            };
            self.add_split_time(
                SplitTime::default().name(&format!("MessageBox{}Animation", message_box.name)),
            );
            self.add_custom_rect(
                CustomRect::default()
                    .name(&format!("MessageBox{}", message_box.name))
                    .origin_position(0_f32, 0_f32)
                    .size(message_box.size[0], message_box.size[1])
                    .rounding(20_f32)
                    .x_grid(1, 1)
                    .y_grid(0, 1)
                    .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                    .color(100, 100, 100, 125)
                    .border_width(0_f32),
            );
            self.add_image(
                Image::default()
                    .name(&format!("MessageBox{}Close", message_box.name))
                    .use_overlay_color(false)
                    .size(30_f32, 30_f32)
                    .center_display(HorizontalAlign::Center, VerticalAlign::Center),
                "CloseMessageBox",
            );
            self.add_switch(
                Switch::default()
                    .name(&format!("MessageBox{}Close", message_box.name))
                    .appearance(vec![
                        SwitchData {
                            texture: "CloseMessageBox".to_string(),
                            color: [255, 255, 255, 0],
                            text: String::new(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            texture: "CloseMessageBox".to_string(),
                            color: [180, 180, 180, 200],
                            text: String::new(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            texture: "CloseMessageBox".to_string(),
                            color: [255, 255, 255, 200],
                            text: String::new(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            texture: "CloseMessageBox".to_string(),
                            color: [180, 180, 180, 200],
                            text: String::new(),
                            hint_text: String::new(),
                        },
                    ])
                    .enable_hover_click_image(false, true)
                    .click_method(vec![SwitchClickAction {
                        click_method: PointerButton::Primary,
                        action: true,
                    }])
                    .sound_path(sound_path),
                &format!("MessageBox{}Close", message_box.name),
                "",
            );
        } else {
            self.problem_report(
                RustConstructorError::MessageBoxAlreadyExists {
                    message_box_name: message_box.name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
        };
    }

    /// 处理所有已添加的消息框资源。
    pub fn message_box_display(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let mut offset = 0_f32;
        let mut delete_count = 0;
        let mut index_list = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let RCR::MessageBox(_) = self.rust_constructor_resource[i] {
                index_list.push(i);
            };
        }
        for u in 0..index_list.len() {
            let mut deleted = false;
            let i = u - delete_count;
            if let RCR::MessageBox(mut mb) = self.rust_constructor_resource[index_list[i]].clone()
                && let Ok(id1) = self.get_resource_index("Image", &mb.image_name)
                && let RCR::Image(mut im1) = self.rust_constructor_resource[id1].clone()
                && let Ok(id2) =
                    self.get_resource_index("CustomRect", &format!("MessageBox{}", mb.name))
                && let RCR::CustomRect(mut cr) = self.rust_constructor_resource[id2].clone()
                && let Ok(id3) = self.get_resource_index("Text", &mb.title_name)
                && let RCR::Text(mut t1) = self.rust_constructor_resource[id3].clone()
                && let Ok(id4) = self.get_resource_index("Text", &mb.content_name)
                && let RCR::Text(mut t2) = self.rust_constructor_resource[id4].clone()
                && let Ok(id5) =
                    self.get_resource_index("Switch", &format!("MessageBox{}Close", mb.name))
                && let RCR::Switch(mut s) = self.rust_constructor_resource[id5].clone()
                && let Ok(id6) =
                    self.get_resource_index("Image", &format!("MessageBox{}Close", mb.name))
                && let RCR::Image(mut im2) = self.rust_constructor_resource[id6].clone()
            {
                if mb.size[1]
                    < self.get_text_size(&mb.title_name.clone(), ui).unwrap()[1]
                        + self.get_text_size(&mb.content_name.clone(), ui).unwrap()[1]
                        + 10_f32
                {
                    mb.size[1] = self.get_text_size(&mb.title_name.clone(), ui).unwrap()[1]
                        + self.get_text_size(&mb.content_name.clone(), ui).unwrap()[1]
                        + 10_f32;
                    cr.size[1] = mb.size[1];
                    im1.size = [mb.size[1] - 15_f32, mb.size[1] - 15_f32];
                    t1.wrap_width = mb.size[0] - mb.size[1] + 5_f32;
                    t2.wrap_width = mb.size[0] - mb.size[1] + 5_f32;
                };
                if self.timer.total_time
                    - self
                        .split_time(&format!("MessageBox{}Animation", mb.name))
                        .unwrap()[1]
                    >= self.vertrefresh
                {
                    self.reset_split_time(&format!("MessageBox{}Animation", mb.name));
                    if offset != mb.memory_offset {
                        if mb.memory_offset < offset {
                            if mb.memory_offset + mb.restore_speed >= offset {
                                mb.memory_offset = offset;
                            } else {
                                mb.memory_offset += mb.restore_speed;
                            };
                        } else if mb.memory_offset - mb.restore_speed <= offset {
                            mb.memory_offset = offset;
                        } else {
                            mb.memory_offset -= mb.restore_speed;
                        };
                    };
                    if cr.origin_position[0] != -mb.size[0] - 5_f32 {
                        if mb.exist {
                            if cr.origin_position[0] - mb.speed <= -mb.size[0] - 5_f32 {
                                cr.origin_position[0] = -mb.size[0] - 5_f32;
                                if self.check_resource_exists(
                                    "SplitTime",
                                    &format!("MessageBox{}", mb.name),
                                ) {
                                    self.reset_split_time(&format!("MessageBox{}", mb.name));
                                };
                            } else {
                                cr.origin_position[0] -= mb.speed;
                            };
                        } else if cr.origin_position[0] + mb.speed >= 15_f32 {
                            cr.origin_position[0] = 15_f32;
                            delete_count += 1;
                            deleted = true;
                        } else {
                            cr.origin_position[0] += mb.speed;
                        };
                    };
                };
                cr.origin_position[1] = mb.memory_offset + 20_f32;
                im1.origin_position = [
                    cr.origin_position[0] + 5_f32,
                    cr.origin_position[1] + mb.size[1] / 2_f32,
                ];
                t1.origin_position = [
                    im1.origin_position[0] + im1.size[0] + 5_f32,
                    cr.origin_position[1] + 5_f32,
                ];
                t2.origin_position = [
                    im1.origin_position[0] + im1.size[0] + 5_f32,
                    t1.origin_position[1]
                        + self.get_text_size(&mb.title_name.clone(), ui).unwrap()[1],
                ];
                im2.origin_position = cr.position;
                if !mb.keep_existing
                    && self.timer.total_time
                        - self.split_time(&format!("MessageBox{}", mb.name)).unwrap()[1]
                        >= mb.existing_time
                    && cr.origin_position[0] == -mb.size[0] - 5_f32
                {
                    mb.exist = false;
                    if cr.origin_position[0] + mb.speed >= 15_f32 {
                        cr.origin_position[0] = 15_f32;
                    } else {
                        cr.origin_position[0] += mb.speed;
                    };
                };
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let rect = egui::Rect::from_min_size(
                        Pos2 {
                            x: im2.position[0],
                            y: im2.position[1],
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
                self.custom_rect(ui, &format!("MessageBox{}", mb.name), ctx);
                self.image(ui, &mb.image_name.clone(), ctx);
                self.text(ui, &t1.name.clone(), ctx);
                self.text(ui, &t2.name.clone(), ctx);
                if self
                    .switch(
                        &format!("MessageBox{}Close", mb.name),
                        ui,
                        ctx,
                        s.state == 0 && mb.exist,
                        true,
                    )
                    .unwrap()[0]
                    == 0
                {
                    mb.exist = false;
                    if cr.origin_position[0] + mb.speed >= 15_f32 {
                        cr.origin_position[0] = 15_f32;
                    } else {
                        cr.origin_position[0] += mb.speed;
                    };
                    self.rust_constructor_resource[id2] = RCR::CustomRect(cr.clone());
                    self.rust_constructor_resource[index_list[i]] = RCR::MessageBox(mb.clone());
                };
                if deleted {
                    if let Ok(id) = self.get_resource_index("Image", &mb.image_name) {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) =
                        self.get_resource_index("CustomRect", &format!("MessageBox{}", mb.name))
                    {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) = self.get_resource_index("Text", &mb.title_name) {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) = self.get_resource_index("Text", &mb.content_name) {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) =
                        self.get_resource_index("Switch", &format!("MessageBox{}Close", mb.name))
                    {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) =
                        self.get_resource_index("Image", &format!("MessageBox{}Close", mb.name))
                    {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) = self
                        .get_resource_index("SplitTime", &format!("MessageBox{}Animation", mb.name))
                    {
                        self.rust_constructor_resource.remove(id);
                    };
                    if !mb.keep_existing
                        && let Ok(id) =
                            self.get_resource_index("SplitTime", &format!("MessageBox{}", mb.name))
                    {
                        self.rust_constructor_resource.remove(id);
                    };
                    if let Ok(id) = self.get_resource_index("MessageBox", &mb.name) {
                        self.rust_constructor_resource.remove(id);
                    };
                } else {
                    offset += mb.size[1] + 15_f32;
                };
            };
        }
    }

    /// 添加开关资源。
    pub fn add_switch(&mut self, mut switch: Switch, image_name: &str, text_name: &str) {
        let mut count = 1;
        if switch.enable_hover_click_image[0] {
            count += 1;
        };
        if switch.enable_hover_click_image[1] {
            count += 1;
        };
        if switch.appearance.len() < count {
            self.problem_report(
                RustConstructorError::SwitchAppearanceMismatch {
                    switch_name: switch.name.clone(),
                    differ: count as u32 - switch.appearance.len() as u32,
                },
                SeverityLevel::SevereWarning,
            );
            return;
        };
        for _ in 0..switch.appearance.len() % count {
            switch.appearance.pop();
        }
        switch.text_name = String::new();
        if let Ok(id) = self.get_resource_index("Image", image_name)
            && let RCR::Image(mut im) = self.rust_constructor_resource[id].clone()
        {
            im.use_overlay_color = true;
            switch.switch_image_name = image_name.to_string();
            if self.check_resource_exists("Text", text_name)
                && let Ok(id2) = self.get_resource_index("Text", text_name)
                && let RCR::Text(t) = &mut self.rust_constructor_resource[id2]
            {
                switch.text_name = text_name.to_string();
                switch.text_origin_position = t.origin_position;
                t.center_display = (HorizontalAlign::Center, VerticalAlign::Center);
                t.x_grid = [0, 0];
                t.y_grid = [0, 0];
            };
            self.rust_constructor_resource[id] = RCR::Image(im);
        };
        switch.hint_text_name = String::new();
        if switch
            .appearance
            .iter()
            .filter(|x| !x.hint_text.is_empty())
            .count()
            > 0
        {
            switch.hint_text_name = format!("{}Hint", switch.name);
            self.add_text(
                Text::default()
                    .name(&format!("{}Hint", switch.name))
                    .content("")
                    .origin_position(0_f32, 0_f32)
                    .font_size(25_f32)
                    .wrap_width(300_f32)
                    .background_rounding(10_f32)
                    .color(255, 255, 255, 0)
                    .background_color(0, 0, 0, 0)
                    .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                    .selectable(false),
            );
            self.add_split_time(
                SplitTime::default().name(&format!("{}StartHoverTime", switch.name)),
            );
            self.add_split_time(
                SplitTime::default().name(&format!("{}HintFadeAnimation", switch.name)),
            );
        };
        switch.state = 0;
        switch.animation_count = count as u32;
        self.rust_constructor_resource.push(RCR::Switch(switch));
    }

    /// 显示开关资源并返回点击方法和开关状态。
    pub fn switch(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &egui::Context,
        enable: bool,
        play_sound: bool,
    ) -> Result<[usize; 2], RustConstructorError> {
        let mut activated = [5, 0];
        let mut appearance_count = 0;
        if let Ok(id) = self.get_resource_index("Switch", name) {
            if let RCR::Switch(mut s) = self.rust_constructor_resource[id].clone() {
                if let Ok(id2) = self.get_resource_index("Image", &s.switch_image_name.clone()) {
                    if let RCR::Image(mut im) = self.rust_constructor_resource[id2].clone() {
                        s.reg_render_resource(&mut self.render_resource_list);
                        let rect = Rect::from_min_size(
                            Pos2::new(im.position[0], im.position[1]),
                            Vec2::new(im.size[0], im.size[1]),
                        );
                        let mut hovered = false;
                        if enable {
                            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                // 判断是否在矩形内
                                if rect.contains(mouse_pos) {
                                    if !s.hint_text_name.is_empty()
                                        && let Ok(id3) =
                                            self.get_resource_index("Text", &s.hint_text_name)
                                        && let RCR::Text(mut t) =
                                            self.rust_constructor_resource[id3].clone()
                                    {
                                        if !s.last_time_hovered {
                                            self.reset_split_time(&format!(
                                                "{}StartHoverTime",
                                                s.name
                                            ));
                                        } else if self.timer.total_time
                                            - self
                                                .split_time(&format!("{}StartHoverTime", s.name))
                                                .unwrap()[1]
                                            >= 2_f32
                                            || t.color[3] != 0
                                        {
                                            t.color[3] = 255;
                                            t.origin_position = [mouse_pos.x, mouse_pos.y];
                                        };
                                        t.center_display.0 = if mouse_pos.x
                                            + self.get_text_size(&s.hint_text_name, ui).unwrap()[0]
                                            <= ctx.available_rect().width()
                                        {
                                            HorizontalAlign::Left
                                        } else {
                                            HorizontalAlign::Right
                                        };
                                        t.center_display.1 = if mouse_pos.y
                                            + self.get_text_size(&s.hint_text_name, ui).unwrap()[1]
                                            <= ctx.available_rect().height()
                                        {
                                            VerticalAlign::Top
                                        } else {
                                            VerticalAlign::Bottom
                                        };
                                        self.rust_constructor_resource[id3] = RCR::Text(t);
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
                                                appearance_count = 2;
                                            } else {
                                                appearance_count = 1;
                                            };
                                        } else if !s.enable_hover_click_image[0] {
                                            appearance_count = 0;
                                        };
                                    } else {
                                        if s.last_time_clicked {
                                            if play_sound {
                                                general_click_feedback(&s.sound_path);
                                            };
                                            let mut count = 1;
                                            if s.enable_hover_click_image[0] {
                                                count += 1;
                                            };
                                            if s.enable_hover_click_image[1] {
                                                count += 1;
                                            };
                                            if s.click_method[s.last_time_clicked_index].action {
                                                if s.state < (s.appearance.len() / count - 1) as u32
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
                                            appearance_count = 1;
                                        } else {
                                            appearance_count = 0;
                                        };
                                    };
                                } else {
                                    s.last_time_clicked = false;
                                    appearance_count = 0;
                                };
                            } else {
                                s.last_time_clicked = false;
                                appearance_count = 0;
                            };
                        } else {
                            s.last_time_clicked = false;
                            appearance_count = 0;
                        };
                        if !hovered && !s.hint_text_name.is_empty() {
                            if s.last_time_hovered {
                                self.reset_split_time(&format!("{}HintFadeAnimation", s.name));
                            };
                            if let Ok(id3) = self.get_resource_index("Text", &s.hint_text_name)
                                && let RCR::Text(mut t) =
                                    self.rust_constructor_resource[id3].clone()
                            {
                                if self.timer.total_time
                                    - self
                                        .split_time(&format!("{}HintFadeAnimation", s.name))
                                        .unwrap()[1]
                                    >= self.vertrefresh
                                {
                                    self.reset_split_time(&format!("{}HintFadeAnimation", s.name));
                                    t.color[3] = t.color[3].saturating_sub(10);
                                };
                                self.rust_constructor_resource[id3] = RCR::Text(t);
                            };
                        };
                        im.overlay_color = s.appearance
                            [(s.state * s.animation_count + appearance_count) as usize]
                            .color;
                        if let Ok(id4) = self.get_resource_index(
                            "ImageTexture",
                            &s.appearance
                                [(s.state * s.animation_count + appearance_count) as usize]
                                .texture
                                .clone(),
                        ) && let RCR::ImageTexture(it) =
                            self.rust_constructor_resource[id4].clone()
                        {
                            im.texture = it.texture.clone();
                        };
                        if !s.hint_text_name.is_empty()
                            && let Ok(id3) = self.get_resource_index("Text", &s.hint_text_name)
                            && let RCR::Text(mut t) = self.rust_constructor_resource[id3].clone()
                        {
                            t.background_color[3] = t.color[3];
                            t.content = s.appearance
                                [(s.state * s.animation_count + appearance_count) as usize]
                                .hint_text
                                .clone();
                            self.rust_constructor_resource[id3] = RCR::Text(t);
                        };
                        s.last_time_hovered = hovered;
                        activated[1] = s.state as usize;
                        self.rust_constructor_resource[id] = RCR::Switch(s.clone());
                        self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                        self.image(ui, &s.switch_image_name.clone(), ctx);
                        if self.check_resource_exists("Text", &s.text_name) {
                            if let Ok(id4) = self.get_resource_index("Text", &s.text_name)
                                && let RCR::Text(mut t2) =
                                    self.rust_constructor_resource[id4].clone()
                            {
                                t2.origin_position = [
                                    im.position[0] + s.text_origin_position[0],
                                    im.position[1] + s.text_origin_position[1],
                                ];
                                t2.content = s.appearance
                                    [(s.state * s.animation_count + appearance_count) as usize]
                                    .text
                                    .clone();
                                self.rust_constructor_resource[id4] = RCR::Text(t2);
                            };
                            self.text(ui, &s.text_name, ctx);
                        };
                        if self.check_resource_exists("Text", &s.hint_text_name) {
                            self.text(ui, &s.hint_text_name, ctx);
                        };
                        Ok(activated)
                    } else {
                        // 一般情况下不会触发。
                        Err(RustConstructorError::ImageNotFound {
                            image_name: s.switch_image_name,
                        })
                    }
                } else {
                    self.problem_report(
                        RustConstructorError::ImageNotFound {
                            image_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                    );
                    Err(RustConstructorError::ImageNotFound {
                        image_name: s.switch_image_name,
                    })
                }
            } else {
                // 一般情况下不会触发。
                Err(RustConstructorError::SwitchNotFound {
                    switch_name: name.to_string(),
                })
            }
        } else {
            self.problem_report(
                RustConstructorError::SwitchNotFound {
                    switch_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
            Err(RustConstructorError::SwitchNotFound {
                switch_name: name.to_string(),
            })
        }
    }
}
