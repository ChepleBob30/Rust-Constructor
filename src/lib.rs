//! Rust Constructor v2.0.3
//! 开发者: Cheple_Bob
//! 一个强大的跨平台GUI框架，在Rust中开发GUI项目的最简单方法。
use anyhow::Context;
use eframe::{emath::Rect, epaint::Stroke, epaint::textures::TextureOptions};
use egui::{
    Color32, FontData, FontDefinitions, FontId, Frame, PointerButton, Pos2, Ui, Vec2, text::CCursor,
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
    let rgba = image.into_raw();
    Ok(Icon::from_rgba(rgba, width, height)?)
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
        } else if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().contains(prefix) {
                matches.push(path);
            }
        }
    }

    Ok(matches)
}

/// 配置文件。
#[derive(Debug, Clone)]
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
    pub fn from_json_value(value: &JsonValue) -> Option<Config> {
        Some(Config {
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

/// 存储特定值的枚举。
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct ReportState {
    /// 问题发生时所在页面。
    pub current_page: String,
    /// 问题发生时程序总运行时间。
    pub current_total_runtime: f32,
    /// 问题发生时页面运行时间。
    pub current_page_runtime: f32,
}

/// 出现问题时用于存储问题内容、状态及注释的结构体。
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

/// 用于存储运行时间的计时器。
#[derive(Clone, Debug)]
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
#[derive(Clone)]
pub struct ImageTexture {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub texture: Option<egui::TextureHandle>,
    /// 图片路径。
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

/// RC的矩形资源。
#[derive(Clone, Debug)]
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
    pub center_display: [bool; 4],
    /// 颜色。
    pub color: [u8; 4],
    /// 边框宽度。
    pub border_width: f32,
    /// 边框颜色。
    pub border_color: [u8; 4],
    /// 原始位置。
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

/// RC的图片资源。
#[derive(Clone)]
pub struct Image {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub image_texture: Option<egui::TextureHandle>,
    /// 图片位置。
    pub image_position: [f32; 2],
    /// 图片大小。
    pub image_size: [f32; 2],
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: [bool; 4],
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

/// RC的文本资源。
#[derive(Clone, Debug)]
pub struct Text {
    pub discern_type: String,
    pub name: String,
    /// 文本内容。
    pub text_content: String,
    /// 字号。
    pub font_size: f32,
    /// 文本颜色。
    pub rgba: [u8; 4],
    /// 文本位置。
    pub position: [f32; 2],
    /// 对齐方法。
    pub center_display: [bool; 4],
    /// 单行宽度。
    pub wrap_width: f32,
    /// 是否有背景。
    pub write_background: bool,
    /// 背景颜色。
    pub background_rgb: [u8; 4],
    /// 圆角。
    pub rounding: f32,
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
    pub hyperlink_text: Vec<(usize, usize, String)>,
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

/// RC的滚动背景资源。
#[derive(Clone, Debug)]
pub struct ScrollBackground {
    pub discern_type: String,
    pub name: String,
    /// 所有图片名称。
    pub image_name: Vec<String>,
    /// true：横向滚动；false：纵向滚动。
    pub horizontal_or_vertical: bool,
    /// 横向true：往左；横向false：往右。
    /// 纵向true：往上；纵向false：往下。
    pub left_and_top_or_right_and_bottom: bool,
    /// 滚动速度。
    pub scroll_speed: u32,
    /// 边界（到达此处会复位）。
    pub boundary: f32,
    /// 恢复点（复位时会回到此处）。
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

/// RC的变量资源。
#[derive(Clone, Debug)]
pub struct Variable {
    pub discern_type: String,
    pub name: String,
    /// 变量的值。
    pub value: Value,
}

/// RC的字体资源。
#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub discern_type: String,
    /// 字体定义。
    pub font_definitions: FontDefinitions,
    /// 字体路径。
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

/// RC的时间分段资源。
#[derive(Clone, Debug)]
pub struct SplitTime {
    pub discern_type: String,
    pub name: String,
    /// 时间点（第一个值为页面运行时间，第二个值为总运行时间）。
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

/// RC的开关资源。
#[derive(Clone, Debug)]
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

/// 渲染的RC资源。
#[derive(Clone, Debug)]
pub struct RenderResource {
    pub discern_type: String,
    pub name: String,
}

/// 开关的外观。
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SwitchClickAction {
    /// 开关的点击方法。
    pub click_method: PointerButton,
    /// 点击后是否改变开关状态。
    pub action: bool,
}

/// RC的消息框资源。
#[derive(Clone, Debug)]
pub struct MessageBox {
    pub discern_type: String,
    pub name: String,
    /// 消息框大小。
    pub box_size: [f32; 2],
    /// 框内内容资源名。
    pub box_content_name: String,
    /// 框内标题资源名。
    pub box_title_name: String,
    /// 框内图片资源名。
    pub box_image_name: String,
    /// 消息框是否持续存在。
    pub box_keep_existing: bool,
    /// 如果不持续存在，消息框的持续时间。
    pub box_existing_time: f32,
    /// 消息框是否存在（不等于是否显示）。
    pub box_exist: bool,
    /// 消息框移动速度。
    pub box_speed: f32,
    /// 消息框补位速度。
    pub box_restore_speed: f32,
    /// 消息框上一次渲染时的y轴偏移量（用于实现补位动画）。
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

/// 用于将RC资源存储进vec的枚举。
#[derive(Clone)]
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

/// RC资源最基本的错误处理。
#[derive(Clone, Debug)]
pub enum RustConstructorError {
    /// 图片获取失败。
    ImageGetFailed { image_path: String },
    /// 图片未找到（需在外部自行调用问题报告）。
    ImageNotFound { image_name: String },
    /// 文本未找到（需在外部自行调用问题报告）。
    TextNotFound { text_name: String },
    /// 变量未找到（需在外部自行调用问题报告）。
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
    /// 分段时间未找到（需在外部自行调用问题报告）。
    SplitTimeNotFound { split_time_name: String },
    /// 开关外观数量不匹配。
    SwitchAppearanceMismatch { switch_name: String, differ: u32 },
    /// 开关未找到（需在外部自行调用问题报告）。
    SwitchNotFound { switch_name: String },
    /// 消息框已存在。
    MessageBoxAlreadyExists { message_box_name: String },
    /// 获取字体失败。
    FontGetFailed { font_path: String },
    /// 字体未找到（需在外部自行调用问题报告）。
    FontNotFound { font_name: String },
    /// 资源未找到。
    ResourceNotFound {
        resource_name: String,
        resource_type: String,
    },
    /// 页面未找到（需在外部自行调用问题报告）。
    PageNotFound { page_name: String },
}

/// 程序主体。
#[derive(Clone)]
pub struct App {
    /// 配置项。
    pub config: Config,
    /// 文本。
    pub game_text: GameText,
    /// RC资源。
    pub rust_constructor_resource: Vec<RCR>,
    /// 渲染资源列表。
    pub render_resource_list: Vec<RenderResource>,
    /// 问题列表。
    pub problem_list: Vec<Problem>,
    /// 窗口样式。
    pub frame: Frame,
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
    pub fn new(config_path: &str, game_text_path: &str) -> Self {
        let mut config = Config {
            language: 0,
            amount_languages: 0,
            rc_strict_mode: false,
            problem_report_sound: String::new(),
        };
        let mut game_text = GameText {
            game_text: HashMap::new(),
        };
        if let Ok(json_value) = read_from_json(config_path) {
            if let Some(read_config) = Config::from_json_value(&json_value) {
                config = read_config;
            }
        }
        if let Ok(json_value) = read_from_json(game_text_path) {
            if let Some(read_game_text) = GameText::from_json_value(&json_value) {
                game_text = read_game_text;
            }
        }
        Self {
            config,
            game_text,
            rust_constructor_resource: vec![],
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

    /// 运行时添加新页面。
    pub fn add_page(&mut self, name: &str, forced_update: bool) {
        self.rust_constructor_resource.push(RCR::PageData(PageData {
            discern_type: "PageData".to_string(),
            name: name.to_string(),
            forced_update,
            change_page_updated: false,
            enter_page_updated: false,
        }));
    }

    /// 切换页面。
    pub fn switch_page(&mut self, page: &str) {
        if let Ok(id) = self.get_resource_index("PageData", page) {
            self.page = page.to_string();
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                pd.change_page_updated = false;
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
                RCR::ScrollBackground(sb) => {
                    if sb.match_resource(resource_name, resource_type) {
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
                RCR::ScrollBackground(sb) => {
                    if sb.match_resource(resource_name, resource_type) {
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
        } else {
            self.problem_report(
                RustConstructorError::FontGetFailed {
                    font_path: font_path.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
        };
        // 应用字体定义
        // ctx.set_fonts(fonts);
    }

    /// 输出字体资源。
    pub fn font(&mut self, name: &str) -> Result<FontDefinitions, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Font", name) {
            if let RCR::Font(f) = &mut self.rust_constructor_resource[id] {
                return Ok(f.font_definitions.clone());
            }
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
            eprintln!("something goes wrong.");
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
        if let Ok(id) = self.get_resource_index("PageData", name) {
            if let RCR::PageData(pd) = self.rust_constructor_resource[id].clone() {
                if !pd.change_page_updated {
                    self.new_page_update(name);
                };
                return Ok(pd.change_page_updated);
            };
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
        if let Ok(id) = self.get_resource_index("PageData", name) {
            if let RCR::PageData(pd) = &mut self.rust_constructor_resource[id] {
                let return_value = pd.enter_page_updated;
                pd.enter_page_updated = true;
                return Ok(return_value);
            };
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
    pub fn add_split_time(&mut self, name: &str, reset: bool) {
        if reset {
            if let Ok(id) = self.get_resource_index("SplitTime", name) {
                if let RCR::SplitTime(st) = &mut self.rust_constructor_resource[id] {
                    st.time = [self.timer.now_time, self.timer.total_time];
                };
            };
        } else {
            self.rust_constructor_resource
                .push(RCR::SplitTime(SplitTime {
                    discern_type: "SplitTime".to_string(),
                    name: name.to_string(),
                    time: [self.timer.now_time, self.timer.total_time],
                }));
        };
    }

    /// 输出分段时间。
    pub fn split_time(&mut self, name: &str) -> Result<[f32; 2], RustConstructorError> {
        if let Ok(id) = self.get_resource_index("SplitTime", name) {
            if let RCR::SplitTime(st) = self.rust_constructor_resource[id].clone() {
                return Ok(st.time);
            };
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
    pub fn add_rect(
        &mut self,
        name: &str,
        position_size_and_rounding: [f32; 5],
        grid: [u32; 4],
        center_display: [bool; 4],
        color: [u8; 8],
        border_width: f32,
    ) {
        self.rust_constructor_resource
            .push(RCR::CustomRect(CustomRect {
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

    /// 显示矩形资源。
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
                        Pos2::new(pos_x + cr.size[0], pos_y + cr.size[1]),
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

    /// 添加文本资源。
    pub fn add_text(
        &mut self,
        name_content_and_font: [&str; 3],
        position_font_size_wrap_width_rounding: [f32; 5],
        color: [u8; 8],
        center_display_write_background_and_enable_copy: [bool; 6],
        grid: [u32; 4],
        hyperlink_text: Vec<(usize, usize, &str)>,
    ) {
        self.rust_constructor_resource.push(RCR::Text(Text {
            discern_type: "Text".to_string(),
            name: name_content_and_font[0].to_string(),
            text_content: name_content_and_font[1].to_string(),
            font_size: position_font_size_wrap_width_rounding[2],
            rgba: [color[0], color[1], color[2], color[3]],
            position: [
                position_font_size_wrap_width_rounding[0],
                position_font_size_wrap_width_rounding[1],
            ],
            center_display: [
                center_display_write_background_and_enable_copy[0],
                center_display_write_background_and_enable_copy[1],
                center_display_write_background_and_enable_copy[2],
                center_display_write_background_and_enable_copy[3],
            ],
            wrap_width: position_font_size_wrap_width_rounding[3],
            write_background: center_display_write_background_and_enable_copy[4],
            background_rgb: [color[4], color[5], color[6], color[7]],
            rounding: position_font_size_wrap_width_rounding[4],
            x_grid: [grid[0], grid[1]],
            y_grid: [grid[2], grid[3]],
            origin_position: [
                position_font_size_wrap_width_rounding[0],
                position_font_size_wrap_width_rounding[1],
            ],
            font: name_content_and_font[2].to_string(),
            selection: None,
            selectable: center_display_write_background_and_enable_copy[5],
            hyperlink_text: hyperlink_text
                .into_iter()
                .map(|(a, b, c)| {
                    (
                        a,
                        if b > name_content_and_font[1].len() - 1 {
                            name_content_and_font[1].len() - 1
                        } else {
                            b
                        },
                        c.to_string(),
                    )
                })
                .collect(),
        }));
    }

    /// 显示文本资源。
    pub fn text(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Text", name) {
            if let RCR::Text(mut t) = self.rust_constructor_resource[id].clone() {
                t.reg_render_resource(&mut self.render_resource_list);
                // 计算文本大小
                let galley = ui.fonts(|f| {
                    f.layout(
                        t.text_content.to_string(),
                        if self.check_resource_exists("Font", &t.font.clone()) {
                            FontId::new(t.font_size, egui::FontFamily::Name(t.font.clone().into()))
                        } else {
                            FontId::proportional(t.font_size)
                        },
                        Color32::from_rgba_unmultiplied(t.rgba[0], t.rgba[1], t.rgba[2], t.rgba[3]),
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
                    galley.clone(),
                    Color32::from_rgba_unmultiplied(
                        t.rgba[0], t.rgba[1], t.rgba[2], t.rgba[3], // 应用透明度
                    ),
                );

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

                    if response.dragged() && t.selection.is_some() {
                        if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                            if let Some((start, _)) = t.selection {
                                t.selection = Some((start, cursor));
                            };
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
                        if copy_triggered {
                            if let Some((start, end)) = t.selection {
                                let (start, end) = (start.min(end), start.max(end));
                                let chars: Vec<char> = t.text_content.chars().collect();
                                if start <= chars.len() && end <= chars.len() && start < end {
                                    let selected_text: String = chars[start..end].iter().collect();
                                    ui.ctx().copy_text(selected_text);
                                };
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
                                        text_size.y / t.text_content.lines().count() as f32
                                    }
                                } else {
                                    text_size.y / t.text_content.lines().count() as f32
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
                                    text_size.y / t.text_content.lines().count() as f32
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
                                        if selection_rect.width() > 0.0
                                            && selection_rect.height() > 0.0
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

                // 绘制超链接
                for (start, end, url) in &t.hyperlink_text {
                    // 获取超链接文本的范围
                    let start_cursor = galley.pos_from_cursor(CCursor::new(*start));
                    let end_cursor = galley.pos_from_cursor(CCursor::new(*end));

                    let start_pos = start_cursor.left_top();
                    let end_pos = end_cursor.right_top();

                    // 检查鼠标是否在超链接上
                    let mut is_hovering_link = false;
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let relative_pos = pointer_pos - position.to_vec2();
                        let cursor = galley.cursor_from_pos(relative_pos.to_vec2());
                        if cursor.index >= *start && cursor.index <= *end {
                            is_hovering_link = true;
                        };
                    };

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
                        if link_response.is_pointer_button_down_on()
                            && !link_response.drag_started()
                        {
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
                        if link_response.clicked() {
                            if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                                let relative_pos = pointer_pos - position.to_vec2();
                                let cursor = galley.cursor_from_pos(relative_pos.to_vec2());
                                if cursor.index >= *start && cursor.index <= *end {
                                    clicked_on_link = true;
                                    break;
                                };
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

                    // 绘制超链接下划线
                    // 检查超链接是否跨行
                    if start_cursor.min.y == end_cursor.min.y {
                        // 单行超链接
                        let underline_y = position.y
                            + start_pos.y
                            + galley.rows.first().map_or(14.0, |row| row.height())
                            - 2.0;

                        // 绘制下划线
                        let color = if is_hovering_link {
                            Color32::from_rgba_unmultiplied(
                                t.rgba[0].saturating_add(50),
                                t.rgba[1],
                                t.rgba[2],
                                t.rgba[3],
                            )
                        } else {
                            Color32::from_rgba_unmultiplied(
                                t.rgba[0], t.rgba[1], t.rgba[2], t.rgba[3],
                            )
                        };

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
                                    t.rgba[0], t.rgba[1], t.rgba[2], t.rgba[3],
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
                self.rust_constructor_resource[id] = RCR::Text(t);
            };
        };
    }

    /// 获取文本大小。
    pub fn get_text_size(
        &mut self,
        resource_name: &str,
        ui: &mut Ui,
    ) -> Result<[f32; 2], RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Text", resource_name) {
            if let RCR::Text(t) = self.rust_constructor_resource[id].clone() {
                let galley = ui.fonts(|f| {
                    f.layout(
                        t.text_content.to_string(),
                        FontId::proportional(t.font_size),
                        Color32::from_rgba_unmultiplied(t.rgba[0], t.rgba[1], t.rgba[2], t.rgba[3]),
                        t.wrap_width,
                    )
                });
                return Ok([galley.size().x, galley.size().y]);
            };
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
    pub fn add_var<T: Into<Value>>(&mut self, name: &str, value: T) {
        self.rust_constructor_resource.push(RCR::Variable(Variable {
            discern_type: "Variable".to_string(),
            name: name.to_string(),
            value: value.into(),
        }));
    }

    /// 修改变量资源。
    pub fn modify_var<T: Into<Value>>(&mut self, name: &str, value: T) {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = &mut self.rust_constructor_resource[id] {
                v.value = value.into();
            };
        };
    }

    /// 取出Value变量。
    pub fn var(&mut self, name: &str) -> Result<Value, RustConstructorError> {
        if let Ok(id) = self.get_resource_index("Variable", name) {
            if let RCR::Variable(v) = self.rust_constructor_resource[id].clone() {
                return Ok(v.clone().value);
            };
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

    /// 添加滚动背景资源。
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
            }
        }
        for (count, _) in image_id.clone().into_iter().enumerate() {
            if let RCR::Image(im) = &mut self.rust_constructor_resource[image_id[count]] {
                im.x_grid = [0, 0];
                im.y_grid = [0, 0];
                im.center_display = [true, true, false, false];
                im.image_size = [size_position_boundary[0], size_position_boundary[1]];
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
                    im.origin_position = [temp_position, size_position_boundary[3]];
                } else {
                    for _ in 0..count {
                        if left_and_top_or_right_and_bottom {
                            temp_position += size_position_boundary[1];
                        } else {
                            temp_position -= size_position_boundary[1];
                        };
                    }
                    im.origin_position = [size_position_boundary[2], temp_position];
                };
            };
        }
        if let RCR::Image(im) = self.rust_constructor_resource[image_id[image_id.len() - 1]].clone()
        {
            let resume_point = if horizontal_or_vertical {
                im.origin_position[0]
            } else {
                im.origin_position[1]
            };
            self.rust_constructor_resource
                .push(RCR::ScrollBackground(ScrollBackground {
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

    /// 显示滚动背景。
    pub fn scroll_background(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("ScrollBackground", name) {
            if let RCR::ScrollBackground(sb) = self.rust_constructor_resource[id].clone() {
                sb.reg_render_resource(&mut self.render_resource_list);
                if self.get_resource_index("SplitTime", name).is_err() {
                    self.add_split_time(name, false);
                };
                for i in 0..sb.image_name.len() {
                    self.image(ui, &sb.image_name[i].clone(), ctx);
                }
                if self.timer.now_time - self.split_time(name).unwrap()[0] >= self.vertrefresh {
                    self.add_split_time(name, true);
                    for i in 0..sb.image_name.len() {
                        if let Ok(id2) = self.get_resource_index("Image", &sb.image_name[i].clone())
                        {
                            if let RCR::Image(mut im) = self.rust_constructor_resource[id2].clone()
                            {
                                if sb.horizontal_or_vertical {
                                    if sb.left_and_top_or_right_and_bottom {
                                        for _ in 0..sb.scroll_speed {
                                            im.origin_position[0] -= 1_f32;
                                            if im.origin_position[0] <= sb.boundary {
                                                im.origin_position[0] = sb.resume_point;
                                            };
                                        }
                                        self.rust_constructor_resource[id2] =
                                            RCR::Image(im.clone());
                                    } else {
                                        for _ in 0..sb.scroll_speed {
                                            im.origin_position[0] += 1_f32;
                                            if im.origin_position[0] >= sb.boundary {
                                                im.origin_position[0] = sb.resume_point;
                                            };
                                        }
                                        self.rust_constructor_resource[id2] =
                                            RCR::Image(im.clone());
                                    };
                                } else if sb.left_and_top_or_right_and_bottom {
                                    for _ in 0..sb.scroll_speed {
                                        im.origin_position[1] -= 1_f32;
                                        if im.origin_position[1] <= sb.boundary {
                                            im.origin_position[1] = sb.resume_point;
                                        };
                                    }
                                    self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                } else {
                                    for _ in 0..sb.scroll_speed {
                                        im.origin_position[1] += 1_f32;
                                        if im.origin_position[1] >= sb.boundary {
                                            im.origin_position[1] = sb.resume_point;
                                        };
                                    }
                                    self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                                };
                            };
                        };
                    }
                };
            };
        };
    }

    /// 添加图片纹理资源。
    pub fn add_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        create_new_resource: bool,
        ctx: &egui::Context,
    ) {
        if let Ok(mut file) = File::open(path) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let img_bytes = buffer;
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
            if create_new_resource {
                self.rust_constructor_resource
                    .push(RCR::ImageTexture(ImageTexture {
                        discern_type: "ImageTexture".to_string(),
                        name: name.to_string(),
                        texture: image_texture,
                        cite_path: path.to_string(),
                    }));
            } else if let Ok(id) = self.get_resource_index("ImageTexture", name) {
                if let RCR::ImageTexture(it) = &mut self.rust_constructor_resource[id] {
                    if !create_new_resource {
                        it.texture = image_texture;
                        it.cite_path = path.to_string();
                    };
                };
            } else {
                self.rust_constructor_resource
                    .push(RCR::ImageTexture(ImageTexture {
                        discern_type: "ImageTexture".to_string(),
                        name: name.to_string(),
                        texture: image_texture,
                        cite_path: path.to_string(),
                    }));
            };
        } else {
            self.problem_report(
                RustConstructorError::ImageGetFailed {
                    image_path: path.to_string(),
                },
                SeverityLevel::SevereWarning,
            );
        };
    }

    /// 添加图片资源。
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
                    cite_texture: image_texture_name.to_string(),
                    last_frame_cite_texture: image_texture_name.to_string(),
                }));
            };
        };
    }

    /// 显示图片资源。
    pub fn image(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        if let Ok(id) = self.get_resource_index("Image", name) {
            if let RCR::Image(mut im) = self.rust_constructor_resource[id].clone() {
                if im.cite_texture != im.last_frame_cite_texture {
                    if let Ok(id2) = self.get_resource_index("ImageTexture", &im.cite_texture) {
                        if let RCR::ImageTexture(it) = self.rust_constructor_resource[id2].clone() {
                            im.image_texture = it.texture;
                        };
                    };
                };
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
                    im.image_position[0] -= im.image_size[0] / 2.0;
                } else if !im.center_display[0] {
                    im.image_position[0] -= im.image_size[0];
                };
                if im.center_display[3] {
                    im.image_position[1] -= im.image_size[1] / 2.0;
                } else if !im.center_display[1] {
                    im.image_position[1] -= im.image_size[1];
                };
                if let Some(texture) = &im.image_texture {
                    let rect = Rect::from_min_size(
                        Pos2::new(im.image_position[0], im.image_position[1]),
                        Vec2::new(im.image_size[0], im.image_size[1]),
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
        };
    }

    /// 添加消息框资源(重要事项：你需要添加一个CloseMessageBox图片纹理资源才可正常使用！)。
    pub fn add_message_box(
        &mut self,
        box_itself_title_content_image_name_and_sound_path: [&str; 5],
        box_size: [f32; 2],
        box_keep_existing: bool,
        box_existing_time: f32,
        box_normal_and_restore_speed: [f32; 2],
    ) {
        if !self.check_resource_exists(
            "MessageBox",
            box_itself_title_content_image_name_and_sound_path[0],
        ) {
            if let Ok(id) = self.get_resource_index(
                "Image",
                box_itself_title_content_image_name_and_sound_path[3],
            ) {
                if let RCR::Image(im) = &mut self.rust_constructor_resource[id] {
                    im.image_size = [box_size[1] - 15_f32, box_size[1] - 15_f32];
                    im.center_display = [true, false, false, true];
                    im.x_grid = [1, 1];
                    im.y_grid = [0, 1];
                    im.name = format!("MessageBox{}", im.name);
                };
            };
            if let Ok(id) = self.get_resource_index(
                "Text",
                box_itself_title_content_image_name_and_sound_path[1],
            ) {
                if let RCR::Text(t) = &mut self.rust_constructor_resource[id] {
                    t.x_grid = [1, 1];
                    t.y_grid = [0, 1];
                    t.center_display = [true, true, false, false];
                    t.wrap_width = box_size[0] - box_size[1] + 5_f32;
                    t.name = format!("MessageBox{}", t.name);
                };
            };
            if let Ok(id) = self.get_resource_index(
                "Text",
                box_itself_title_content_image_name_and_sound_path[2],
            ) {
                if let RCR::Text(t) = &mut self.rust_constructor_resource[id] {
                    t.center_display = [true, true, false, false];
                    t.x_grid = [1, 1];
                    t.y_grid = [0, 1];
                    t.wrap_width = box_size[0] - box_size[1] + 5_f32;
                    t.name = format!("MessageBox{}", t.name);
                };
            };
            self.rust_constructor_resource
                .push(RCR::MessageBox(MessageBox {
                    discern_type: "MessageBox".to_string(),
                    name: box_itself_title_content_image_name_and_sound_path[0].to_string(),
                    box_size,
                    box_title_name: format!(
                        "MessageBox{}",
                        box_itself_title_content_image_name_and_sound_path[1]
                    ),
                    box_content_name: format!(
                        "MessageBox{}",
                        box_itself_title_content_image_name_and_sound_path[2]
                    ),
                    box_image_name: format!(
                        "MessageBox{}",
                        box_itself_title_content_image_name_and_sound_path[3]
                    ),
                    box_keep_existing,
                    box_existing_time,
                    box_exist: true,
                    box_speed: box_normal_and_restore_speed[0],
                    box_restore_speed: box_normal_and_restore_speed[1],
                    box_memory_offset: 0_f32,
                }));
            if !box_keep_existing {
                self.add_split_time(
                    &format!(
                        "MessageBox{}",
                        box_itself_title_content_image_name_and_sound_path[0]
                    ),
                    false,
                );
            };
            self.add_split_time(
                &format!(
                    "MessageBox{}Animation",
                    box_itself_title_content_image_name_and_sound_path[0]
                ),
                false,
            );
            self.add_rect(
                &format!(
                    "MessageBox{}",
                    box_itself_title_content_image_name_and_sound_path[0]
                ),
                [0_f32, 0_f32, box_size[0], box_size[1], 20_f32],
                [1, 1, 0, 1],
                [true, true, false, false],
                [100, 100, 100, 125, 240, 255, 255, 255],
                0.0,
            );
            self.add_image(
                &format!(
                    "MessageBox{}Close",
                    box_itself_title_content_image_name_and_sound_path[0]
                ),
                [0_f32, 0_f32, 30_f32, 30_f32],
                [0, 0, 0, 0],
                [false, false, true, true, false],
                [255, 0, 0, 0, 0],
                "CloseMessageBox",
            );
            self.add_switch(
                [
                    &format!(
                        "MessageBox{}Close",
                        box_itself_title_content_image_name_and_sound_path[0]
                    ),
                    &format!(
                        "MessageBox{}Close",
                        box_itself_title_content_image_name_and_sound_path[0]
                    ),
                    "",
                    box_itself_title_content_image_name_and_sound_path[4],
                ],
                vec![
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
                ],
                [false, true, true],
                2,
                vec![SwitchClickAction {
                    click_method: PointerButton::Primary,
                    action: true,
                }],
            );
        } else {
            self.problem_report(
                RustConstructorError::MessageBoxAlreadyExists {
                    message_box_name: box_itself_title_content_image_name_and_sound_path[0]
                        .to_string(),
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
            if let RCR::MessageBox(mut mb) = self.rust_constructor_resource[index_list[i]].clone() {
                if let Ok(id1) = self.get_resource_index("Image", &mb.box_image_name) {
                    if let RCR::Image(mut im1) = self.rust_constructor_resource[id1].clone() {
                        if let Ok(id2) =
                            self.get_resource_index("CustomRect", &format!("MessageBox{}", mb.name))
                        {
                            if let RCR::CustomRect(mut cr) =
                                self.rust_constructor_resource[id2].clone()
                            {
                                if let Ok(id3) = self.get_resource_index("Text", &mb.box_title_name)
                                {
                                    if let RCR::Text(mut t1) =
                                        self.rust_constructor_resource[id3].clone()
                                    {
                                        if let Ok(id4) =
                                            self.get_resource_index("Text", &mb.box_content_name)
                                        {
                                            if let RCR::Text(mut t2) =
                                                self.rust_constructor_resource[id4].clone()
                                            {
                                                if let Ok(id5) = self.get_resource_index(
                                                    "Switch",
                                                    &format!("MessageBox{}Close", mb.name),
                                                ) {
                                                    if let RCR::Switch(mut s) =
                                                        self.rust_constructor_resource[id5].clone()
                                                    {
                                                        if let Ok(id6) = self.get_resource_index(
                                                            "Image",
                                                            &format!("MessageBox{}Close", mb.name),
                                                        ) {
                                                            if let RCR::Image(mut im2) = self
                                                                .rust_constructor_resource[id6]
                                                                .clone()
                                                            {
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
                                                                    - self
                                                                        .split_time(&format!(
                                                                            "MessageBox{}Animation",
                                                                            mb.name
                                                                        ))
                                                                        .unwrap()[1]
                                                                    >= self.vertrefresh
                                                                {
                                                                    self.add_split_time(
                                                                        &format!(
                                                                            "MessageBox{}Animation",
                                                                            mb.name
                                                                        ),
                                                                        true,
                                                                    );
                                                                    if offset
                                                                        != mb.box_memory_offset
                                                                    {
                                                                        if mb.box_memory_offset
                                                                            < offset
                                                                        {
                                                                            if mb.box_memory_offset
                                                                                + mb.box_restore_speed
                                                                                >= offset
                                                                            {
                                                                                mb.box_memory_offset = offset;
                                                                            } else {
                                                                                mb.box_memory_offset +=
                                                                                    mb.box_restore_speed;
                                                                            };
                                                                        } else if mb
                                                                            .box_memory_offset
                                                                            - mb.box_restore_speed
                                                                            <= offset
                                                                        {
                                                                            mb.box_memory_offset =
                                                                                offset;
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
                                                                                <= -mb.box_size[0]
                                                                                    - 5_f32
                                                                            {
                                                                                cr.origin_position[0] =
                                                                                    -mb.box_size[0] - 5_f32;
                                                                                if self.check_resource_exists("SplitTime", &format!("MessageBox{}", mb.name)) {
                                                                                    self.add_split_time(
                                                                                        &format!("MessageBox{}", mb.name),
                                                                                        true,
                                                                                    );
                                                                                };
                                                                            } else {
                                                                                cr.origin_position[0] -=
                                                                                    mb.box_speed;
                                                                            };
                                                                        } else if cr.origin_position
                                                                            [0]
                                                                            + mb.box_speed
                                                                            >= 15_f32
                                                                        {
                                                                            cr.origin_position[0] =
                                                                                15_f32;
                                                                            delete_count += 1;
                                                                            deleted = true;
                                                                        } else {
                                                                            cr.origin_position
                                                                                [0] += mb.box_speed;
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
                                                                        + self
                                                                            .get_text_size(
                                                                                &mb.box_title_name
                                                                                    .clone(),
                                                                                ui,
                                                                            )
                                                                            .unwrap()[1],
                                                                ];
                                                                im2.origin_position = cr.position;
                                                                if !mb.box_keep_existing
                                                                    && self.timer.total_time
                                                                        - self
                                                                            .split_time(&format!(
                                                                                "MessageBox{}",
                                                                                mb.name
                                                                            ))
                                                                            .unwrap()[1]
                                                                        >= mb.box_existing_time
                                                                    && cr.origin_position[0]
                                                                        == -mb.box_size[0] - 5_f32
                                                                {
                                                                    mb.box_exist = false;
                                                                    if cr.origin_position[0]
                                                                        + mb.box_speed
                                                                        >= 15_f32
                                                                    {
                                                                        cr.origin_position[0] =
                                                                            15_f32;
                                                                    } else {
                                                                        cr.origin_position[0] +=
                                                                            mb.box_speed;
                                                                    };
                                                                };
                                                                if let Some(mouse_pos) =
                                                                    ui.input(|i| {
                                                                        i.pointer.hover_pos()
                                                                    })
                                                                {
                                                                    let rect =
                                                                        egui::Rect::from_min_size(
                                                                            Pos2 {
                                                                                x: im2
                                                                                    .image_position
                                                                                    [0],
                                                                                y: im2
                                                                                    .image_position
                                                                                    [1],
                                                                            },
                                                                            Vec2 {
                                                                                x: cr.size[0]
                                                                                    + 25_f32,
                                                                                y: cr.size[1]
                                                                                    + 25_f32,
                                                                            },
                                                                        );
                                                                    if rect.contains(mouse_pos) {
                                                                        s.appearance[0].color[3] =
                                                                            200;
                                                                    } else {
                                                                        s.appearance[0].color[3] =
                                                                            0;
                                                                    };
                                                                };
                                                                self.rust_constructor_resource
                                                                    [index_list[i]] =
                                                                    RCR::MessageBox(mb.clone());
                                                                self.rust_constructor_resource
                                                                    [id1] = RCR::Image(im1.clone());
                                                                self.rust_constructor_resource
                                                                    [id2] =
                                                                    RCR::CustomRect(cr.clone());
                                                                self.rust_constructor_resource
                                                                    [id3] = RCR::Text(t1.clone());
                                                                self.rust_constructor_resource
                                                                    [id4] = RCR::Text(t2.clone());
                                                                self.rust_constructor_resource
                                                                    [id5] = RCR::Switch(s.clone());
                                                                self.rust_constructor_resource
                                                                    [id6] = RCR::Image(im2.clone());
                                                                self.rect(
                                                                    ui,
                                                                    &format!(
                                                                        "MessageBox{}",
                                                                        mb.name
                                                                    ),
                                                                    ctx,
                                                                );
                                                                self.image(
                                                                    ui,
                                                                    &mb.box_image_name.clone(),
                                                                    ctx,
                                                                );
                                                                self.text(
                                                                    ui,
                                                                    &t1.name.clone(),
                                                                    ctx,
                                                                );
                                                                self.text(
                                                                    ui,
                                                                    &t2.name.clone(),
                                                                    ctx,
                                                                );
                                                                if self
                                                                    .switch(
                                                                        &format!(
                                                                            "MessageBox{}Close",
                                                                            mb.name
                                                                        ),
                                                                        ui,
                                                                        ctx,
                                                                        s.state == 0
                                                                            && mb.box_exist,
                                                                        true,
                                                                    )
                                                                    .unwrap()[0]
                                                                    == 0
                                                                {
                                                                    mb.box_exist = false;
                                                                    if cr.origin_position[0]
                                                                        + mb.box_speed
                                                                        >= 15_f32
                                                                    {
                                                                        cr.origin_position[0] =
                                                                            15_f32;
                                                                    } else {
                                                                        cr.origin_position[0] +=
                                                                            mb.box_speed;
                                                                    };
                                                                    self.rust_constructor_resource[id2] = RCR::CustomRect(cr.clone());
                                                                    self.rust_constructor_resource[index_list[i]] = RCR::MessageBox(mb.clone());
                                                                };
                                                                if deleted {
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "Image",
                                                                            &mb.box_image_name,
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "CustomRect",
                                                                            &format!(
                                                                                "MessageBox{}",
                                                                                mb.name
                                                                            ),
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "Text",
                                                                            &mb.box_title_name,
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "Text",
                                                                            &mb.box_content_name,
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "Switch",
                                                                            &format!(
                                                                                "MessageBox{}Close",
                                                                                mb.name
                                                                            ),
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "Image",
                                                                            &format!(
                                                                                "MessageBox{}Close",
                                                                                mb.name
                                                                            ),
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if let Ok(id) = self.get_resource_index("SplitTime", &format!("MessageBox{}Animation", mb.name)) {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                    if !mb.box_keep_existing {
                                                                        if let Ok(id) = self
                                                                            .get_resource_index(
                                                                                "SplitTime",
                                                                                &format!(
                                                                                    "MessageBox{}",
                                                                                    mb.name
                                                                                ),
                                                                            )
                                                                        {
                                                                            self.rust_constructor_resource.remove(id);
                                                                        };
                                                                    };
                                                                    if let Ok(id) = self
                                                                        .get_resource_index(
                                                                            "MessageBox",
                                                                            &mb.name,
                                                                        )
                                                                    {
                                                                        self.rust_constructor_resource.remove(id);
                                                                    };
                                                                } else {
                                                                    offset +=
                                                                        mb.box_size[1] + 15_f32;
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
    }

    /// 添加开关资源。
    pub fn add_switch(
        &mut self,
        name_switch_image_name_text_name_and_sound_path: [&str; 4],
        mut appearance: Vec<SwitchData>,
        enable_hover_click_image_and_use_overlay: [bool; 3],
        switch_amounts_state: u32,
        click_method: Vec<SwitchClickAction>,
    ) {
        let mut count = 1;
        if enable_hover_click_image_and_use_overlay[0] {
            count += 1;
        };
        if enable_hover_click_image_and_use_overlay[1] {
            count += 1;
        };
        if appearance.len() as u32 != count * switch_amounts_state {
            self.problem_report(
                RustConstructorError::SwitchAppearanceMismatch {
                    switch_name: name_switch_image_name_text_name_and_sound_path[0].to_string(),
                    differ: (count as i32 * switch_amounts_state as i32 - appearance.len() as i32)
                        .unsigned_abs(),
                },
                SeverityLevel::SevereWarning,
            );
            for _ in
                0..(count as i32 * switch_amounts_state as i32 - appearance.len() as i32) as usize
            {
                appearance.push(SwitchData {
                    texture: "Error".to_string(),
                    color: [255, 255, 255, 255],
                    text: String::new(),
                    hint_text: String::new(),
                });
            }
        };
        let mut text_origin_position = [0_f32, 0_f32];
        if let Ok(id) =
            self.get_resource_index("Image", name_switch_image_name_text_name_and_sound_path[1])
        {
            if let RCR::Image(mut im) = self.rust_constructor_resource[id].clone() {
                im.use_overlay_color = true;
                if self.check_resource_exists(
                    "Text",
                    name_switch_image_name_text_name_and_sound_path[2],
                ) {
                    if let Ok(id2) = self.get_resource_index(
                        "Text",
                        name_switch_image_name_text_name_and_sound_path[2],
                    ) {
                        if let RCR::Text(t) = &mut self.rust_constructor_resource[id2] {
                            t.center_display = [false, false, true, true];
                            t.x_grid = [0, 0];
                            t.y_grid = [0, 0];
                            text_origin_position = t.origin_position;
                        };
                    };
                };
                self.rust_constructor_resource[id] = RCR::Image(im);
            };
        };
        if !appearance.iter().any(|x| x.hint_text.is_empty())
        {
            self.add_text(
                [
                    &format!("{}Hint", name_switch_image_name_text_name_and_sound_path[0]),
                    "",
                    "Content",
                ],
                [0_f32, 0_f32, 25_f32, 300_f32, 10_f32],
                [255, 255, 255, 0, 0, 0, 0, 0],
                [true, true, false, false, true, false],
                [0, 0, 0, 0],
                vec![],
            );
            self.add_split_time(
                &format!(
                    "{}StartHoverTime",
                    name_switch_image_name_text_name_and_sound_path[0]
                ),
                false,
            );
            self.add_split_time(
                &format!(
                    "{}HintFadeAnimation",
                    name_switch_image_name_text_name_and_sound_path[0]
                ),
                false,
            );
        };
        self.rust_constructor_resource.push(RCR::Switch(Switch {
            discern_type: "Switch".to_string(),
            name: name_switch_image_name_text_name_and_sound_path[0].to_string(),
            appearance: appearance.clone(),
            switch_image_name: name_switch_image_name_text_name_and_sound_path[1].to_string(),
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
            hint_text_name: if !appearance
                .iter()
                .any(|x| x.hint_text.is_empty())
            {
                format!("{}Hint", name_switch_image_name_text_name_and_sound_path[0])
            } else {
                "".to_string()
            },
            text_name: name_switch_image_name_text_name_and_sound_path[2].to_string(),
            text_origin_position,
            sound_path: name_switch_image_name_text_name_and_sound_path[3].to_string(),
        }));
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
                            Pos2::new(im.image_position[0], im.image_position[1]),
                            Vec2::new(im.image_size[0], im.image_size[1]),
                        );
                        let mut hovered = false;
                        if enable {
                            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                // 判断是否在矩形内
                                if rect.contains(mouse_pos) {
                                    if !s.hint_text_name.is_empty() {
                                        if let Ok(id3) =
                                            self.get_resource_index("Text", &s.hint_text_name)
                                        {
                                            if let RCR::Text(mut t) =
                                                self.rust_constructor_resource[id3].clone()
                                            {
                                                if !s.last_time_hovered {
                                                    self.add_split_time(
                                                        &format!("{}StartHoverTime", s.name),
                                                        true,
                                                    );
                                                } else if self.timer.total_time
                                                    - self
                                                        .split_time(&format!(
                                                            "{}StartHoverTime",
                                                            s.name
                                                        ))
                                                        .unwrap()[1]
                                                    >= 2_f32
                                                    || t.rgba[3] != 0
                                                {
                                                    t.rgba[3] = 255;
                                                    t.origin_position = [mouse_pos.x, mouse_pos.y];
                                                };
                                                t.center_display[0] = mouse_pos.x + self
                                                        .get_text_size(&s.hint_text_name, ui)
                                                        .unwrap()[0]
                                                    <= ctx.available_rect().width();
                                                t.center_display[1] = mouse_pos.y
                                                    + self
                                                        .get_text_size(&s.hint_text_name, ui)
                                                        .unwrap()[1]
                                                    <= ctx.available_rect().height();
                                                self.rust_constructor_resource[id3] = RCR::Text(t);
                                            };
                                        };
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
                                self.add_split_time(&format!("{}HintFadeAnimation", s.name), true);
                            };
                            if let Ok(id3) = self.get_resource_index("Text", &s.hint_text_name) {
                                if let RCR::Text(mut t) =
                                    self.rust_constructor_resource[id3].clone()
                                {
                                    if self.timer.total_time
                                        - self
                                            .split_time(&format!("{}HintFadeAnimation", s.name))
                                            .unwrap()[1]
                                        >= self.vertrefresh
                                    {
                                        t.rgba[3] = t.rgba[3].saturating_sub(1);
                                    };
                                    self.rust_constructor_resource[id3] = RCR::Text(t);
                                };
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
                        ) {
                            if let RCR::ImageTexture(it) =
                                self.rust_constructor_resource[id4].clone()
                            {
                                im.image_texture = it.texture.clone();
                            };
                        };
                        if !s.hint_text_name.is_empty() {
                            if let Ok(id3) = self.get_resource_index("Text", &s.hint_text_name) {
                                if let RCR::Text(mut t) =
                                    self.rust_constructor_resource[id3].clone()
                                {
                                    t.background_rgb[3] = t.rgba[3];
                                    t.text_content = s.appearance
                                        [(s.state * s.animation_count + appearance_count) as usize]
                                        .hint_text
                                        .clone();
                                    self.rust_constructor_resource[id3] = RCR::Text(t);
                                };
                            };
                        };
                        s.last_time_hovered = hovered;
                        activated[1] = s.state as usize;
                        self.rust_constructor_resource[id] = RCR::Switch(s.clone());
                        self.rust_constructor_resource[id2] = RCR::Image(im.clone());
                        self.image(ui, &s.switch_image_name.clone(), ctx);
                        if self.check_resource_exists("Text", &s.text_name) {
                            if let Ok(id4) = self.get_resource_index("Text", &s.text_name) {
                                if let RCR::Text(mut t2) =
                                    self.rust_constructor_resource[id4].clone()
                                {
                                    t2.origin_position = [
                                        im.image_position[0] + s.text_origin_position[0],
                                        im.image_position[1] + s.text_origin_position[1],
                                    ];
                                    t2.text_content = s.appearance
                                        [(s.state * s.animation_count + appearance_count) as usize]
                                        .text
                                        .clone();
                                    self.rust_constructor_resource[id4] = RCR::Text(t2);
                                };
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
