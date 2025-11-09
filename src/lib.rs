//! Rust Constructor V2
//! 基于egui构建的跨平台GUI框架, 用Rust开发GUI项目最简单的方式。
use eframe::{emath::Rect, epaint::Stroke, epaint::textures::TextureOptions};
use egui::{
    Color32, ColorImage, Context, FontData, FontDefinitions, FontFamily, FontId, Galley, Id,
    ImageSource, Key, OpenUrl, PointerButton, Pos2, Sense, StrokeKind, TextureHandle, Ui, Vec2,
    text::CCursor,
};
use std::{
    any::Any,
    error::Error,
    fmt::{Debug, Display, Formatter},
    fs::{File, read},
    io::Read,
    sync::Arc,
    time::Instant,
    vec::Vec,
};

/// 核心特征，用于统一管理Rust Constructor资源。
pub trait RustConstructorResource: Debug {
    /// 返回资源名称。
    fn name(&self) -> &str;

    /// 返回资源类型。
    fn expose_type(&self) -> &str;

    /// 注册渲染的资源。
    fn reg_render_resource(&self, render_list: &mut Vec<RenderResource>) {
        render_list.push(RenderResource {
            discern_type: self.expose_type().to_string(),
            name: self.name().to_string(),
        });
    }

    /// 用于不可变类型转换。
    fn as_any(&self) -> &dyn Any;

    /// 用于可变类型转换。
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 标记并管理用于显示给用户的资源。
pub trait FrontResource: RustConstructorResource {
    fn size(&self) -> [f32; 2];

    fn position(&self) -> [f32; 2];

    fn modify_size(&mut self, width: f32, height: f32);

    fn modify_position(&mut self, x: f32, y: f32);

    fn modify_add_position(&mut self, add_x: f32, add_y: f32);
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

impl RustConstructorResource for PageData {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start_time: 0_f32,
            total_time: 0_f32,
            timer: Instant::now(),
            now_time: 0_f32,
        }
    }
}

/// 为图片纹理支持派生Debug特征。
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DebugTextureHandle(TextureHandle);

impl Debug for DebugTextureHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 只输出类型信息，不输出具体纹理数据
        f.debug_struct("DebugTextureHandle").finish()
    }
}

impl DebugTextureHandle {
    pub fn new(texture_handle: TextureHandle) -> Self {
        Self(texture_handle)
    }
}

/// 用于存储图片纹理的RC资源。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageTexture {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
    /// 图片路径。
    pub cite_path: String,
}

impl RustConstructorResource for ImageTexture {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

/// 矩形的可配置项。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRectConfig {
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

impl Default for CustomRectConfig {
    fn default() -> Self {
        Self {
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

impl CustomRectConfig {
    pub fn from_custom_rect(custom_rect: CustomRect) -> Self {
        Self {
            size: custom_rect.size,
            rounding: custom_rect.rounding,
            x_grid: custom_rect.x_grid,
            y_grid: custom_rect.y_grid,
            center_display: custom_rect.center_display,
            color: custom_rect.color,
            border_width: custom_rect.border_width,
            border_color: custom_rect.border_color,
            origin_position: custom_rect.origin_position,
        }
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

impl RustConstructorResource for CustomRect {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl FrontResource for CustomRect {
    fn position(&self) -> [f32; 2] {
        self.position
    }

    fn size(&self) -> [f32; 2] {
        self.size
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, height: f32) {
        self.size = [width, height];
    }

    fn modify_add_position(&mut self, add_x: f32, add_y: f32) {
        self.origin_position = [self.position[0] + add_x, self.position[1] + add_y];
    }
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
    pub fn from_config(mut self, config: CustomRectConfig) -> Self {
        self.size = config.size;
        self.rounding = config.rounding;
        self.x_grid = config.x_grid;
        self.y_grid = config.y_grid;
        self.center_display = config.center_display;
        self.color = config.color;
        self.border_width = config.border_width;
        self.border_color = config.border_color;
        self.origin_position = config.origin_position;
        self
    }

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

/// 图片的可配置项。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ImageConfig {
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
    /// 背景颜色。
    pub background_color: [u8; 4],
    /// 旋转角度(只能顺时针，建议搭配std::f32::PI使用)。
    pub rotate_angle: f32,
    /// 旋转中心。
    pub rotate_center: [f32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 引用纹理名。
    pub cite_texture: String,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            size: [100_f32, 100_f32],
            x_grid: [0, 0],
            y_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            alpha: 255,
            overlay_color: [255, 255, 255, 255],
            background_color: [0, 0, 0, 0],
            rotate_angle: 0_f32,
            rotate_center: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            cite_texture: String::from("ImageTexture"),
        }
    }
}

impl ImageConfig {
    pub fn from_image(image: Image) -> Self {
        Self {
            size: image.size,
            x_grid: image.x_grid,
            y_grid: image.y_grid,
            center_display: image.center_display,
            alpha: image.alpha,
            overlay_color: image.overlay_color,
            background_color: image.background_color,
            rotate_angle: image.rotate_angle,
            rotate_center: image.rotate_center,
            cite_texture: image.cite_texture,
            origin_position: image.origin_position,
        }
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
    pub fn background_color(mut self, background_color: [u8; 4]) -> Self {
        self.background_color = background_color;
        self
    }

    #[inline]
    pub fn rotate_angle(mut self, rotate_angle: f32) -> Self {
        self.rotate_angle = rotate_angle;
        self
    }

    #[inline]
    pub fn rotate_center(mut self, x: f32, y: f32) -> Self {
        self.rotate_center = [x, y];
        self
    }
}

/// RC的图片资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub discern_type: String,
    pub name: String,
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
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
    /// 背景颜色。
    pub background_color: [u8; 4],
    /// 旋转角度(只能顺时针，建议搭配std::f32::consts::PI使用)。
    pub rotate_angle: f32,
    /// 旋转中心。
    pub rotate_center: [f32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 引用纹理名。
    pub cite_texture: String,
    /// 上一帧引用纹理名。
    pub last_frame_cite_texture: String,
}

impl RustConstructorResource for Image {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl FrontResource for Image {
    fn position(&self) -> [f32; 2] {
        self.position
    }

    fn size(&self) -> [f32; 2] {
        self.size
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, height: f32) {
        self.size = [width, height];
    }

    fn modify_add_position(&mut self, add_x: f32, add_y: f32) {
        self.origin_position = [self.position[0] + add_x, self.position[1] + add_y];
    }
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
            background_color: [0, 0, 0, 0],
            rotate_angle: 0_f32,
            rotate_center: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            cite_texture: String::from("ImageTexture"),
            last_frame_cite_texture: String::from("ImageTexture"),
        }
    }
}

impl Image {
    pub fn from_config(mut self, config: ImageConfig) -> Self {
        self.size = config.size;
        self.x_grid = config.x_grid;
        self.y_grid = config.y_grid;
        self.center_display = config.center_display;
        self.alpha = config.alpha;
        self.overlay_color = config.overlay_color;
        self.background_color = config.background_color;
        self.rotate_angle = config.rotate_angle;
        self.rotate_center = config.rotate_center;
        self.origin_position = config.origin_position;
        self.cite_texture = config.cite_texture;
        self
    }

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
    pub fn background_color(mut self, background_color: [u8; 4]) -> Self {
        self.background_color = background_color;
        self
    }

    #[inline]
    pub fn rotate_angle(mut self, rotate_angle: f32) -> Self {
        self.rotate_angle = rotate_angle;
        self
    }

    #[inline]
    pub fn rotate_center(mut self, x: f32, y: f32) -> Self {
        self.rotate_center = [x, y];
        self
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

/// 文本的可配置项。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TextConfig {
    /// 文本内容。
    pub content: String,
    /// 字号。
    pub font_size: f32,
    /// 文本颜色。
    pub color: [u8; 4],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 单行宽度。
    pub wrap_width: f32,
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
    /// 是否可框选。
    pub selectable: bool,
    /// 超链接文本。
    pub hyperlink_text: Vec<(String, HyperlinkSelectMethod)>,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            content: String::from("Hello world"),
            font_size: 16_f32,
            color: [255, 255, 255, 255],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            wrap_width: 200_f32,
            background_color: [0, 0, 0, 0],
            background_rounding: 2_f32,
            x_grid: [0, 0],
            y_grid: [0, 0],
            origin_position: [0_f32, 0_f32],
            font: String::new(),
            selectable: true,
            hyperlink_text: Vec::new(),
        }
    }
}

impl TextConfig {
    pub fn from_text(text: Text) -> Self {
        Self {
            content: text.content,
            font_size: text.font_size,
            color: text.color,
            center_display: text.center_display,
            wrap_width: text.wrap_width,
            background_color: text.background_color,
            background_rounding: text.background_rounding,
            x_grid: text.x_grid,
            y_grid: text.y_grid,
            origin_position: text.origin_position,
            font: text.font,
            selectable: text.selectable,
            hyperlink_text: text.hyperlink_text,
        }
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
    pub fn background_color(mut self, background_color: [u8; 4]) -> Self {
        self.background_color = background_color;
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

impl RustConstructorResource for Text {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl FrontResource for Text {
    fn position(&self) -> [f32; 2] {
        self.position
    }

    fn size(&self) -> [f32; 2] {
        [self.wrap_width, -1_f32]
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, _height: f32) {
        self.wrap_width = width;
    }

    fn modify_add_position(&mut self, add_x: f32, add_y: f32) {
        self.origin_position = [self.position[0] + add_x, self.position[1] + add_y];
    }
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
            background_color: [0, 0, 0, 0],
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
    pub fn from_config(mut self, config: TextConfig) -> Self {
        self.content = config.content;
        self.font_size = config.font_size;
        self.color = config.color;
        self.center_display = config.center_display;
        self.wrap_width = config.wrap_width;
        self.background_color = config.background_color;
        self.background_rounding = config.background_rounding;
        self.x_grid = config.x_grid;
        self.y_grid = config.y_grid;
        self.origin_position = config.origin_position;
        self.font = config.font;
        self.selectable = config.selectable;
        self.hyperlink_text = config.hyperlink_text;
        self
    }

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
    pub fn background_color(mut self, background_color: [u8; 4]) -> Self {
        self.background_color = background_color;
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

/// RC的变量资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable {
    pub discern_type: String,
    pub name: String,
    /// 变量的值。
    pub value: Value,
}

impl RustConstructorResource for Variable {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn value<T: Into<Value>>(mut self, value: T) -> Self {
        self.value = value.into();
        self
    }

    pub fn new<T: Into<Value>>(name: &str, value: T) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: value.into(),
        }
    }

    pub fn from_bool(name: &str, value: bool) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::Bool(value),
        }
    }

    pub fn from_int(name: &str, value: i32) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::Int(value),
        }
    }

    pub fn from_uint(name: &str, value: u32) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::UInt(value),
        }
    }

    pub fn from_float(name: &str, value: f32) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::Float(value),
        }
    }

    pub fn from_vec(name: &str, value: Vec<Value>) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::Vec(value),
        }
    }

    pub fn from_string<T: Into<String>>(name: &str, value: T) -> Self {
        Self {
            discern_type: String::from("Variable"),
            name: String::from(name),
            value: Value::String(value.into()),
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

impl RustConstructorResource for Font {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

/// RC的时间分段资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SplitTime {
    pub discern_type: String,
    pub name: String,
    /// 时间点（第一个值为页面运行时间，第二个值为总运行时间）。
    pub time: [f32; 2],
}

impl RustConstructorResource for SplitTime {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

/// RC的开关资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    pub discern_type: String,
    pub name: String,
    /// 外观（包括各类资源配置项，数量为开启的内容数量*开关状态总数）。
    pub appearance: Vec<SwitchData>,
    /// 开关使用的填充资源名称。
    pub fill_resource_name: String,
    /// 开关使用的填充资源类型。
    pub fill_resource_type: String,
    /// 是否启用鼠标悬浮和点击时的显示内容。
    pub enable_hover_click_fill_resource: [bool; 2],
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
}

impl RustConstructorResource for Switch {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self {
            discern_type: String::from("Switch"),
            name: String::from("Switch"),
            appearance: vec![],
            fill_resource_name: String::from("FillResource"),
            fill_resource_type: String::from("Image"),
            enable_hover_click_fill_resource: [false, false],
            state: 0,
            click_method: vec![],
            last_time_hovered: false,
            last_time_clicked: false,
            last_time_clicked_index: 5,
            animation_count: 0,
            hint_text_name: String::from("HintText"),
            text_name: String::from("Text"),
            text_origin_position: [0_f32, 0_f32],
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
    pub fn enable_hover_click_fill_resource(
        mut self,
        enable_hover_fill_resource: bool,
        enable_click_fill_resource: bool,
    ) -> Self {
        self.enable_hover_click_fill_resource =
            [enable_hover_fill_resource, enable_click_fill_resource];
        self
    }

    #[inline]
    pub fn click_method(mut self, click_method: Vec<SwitchClickAction>) -> Self {
        self.click_method = click_method;
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
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SwitchData {
    /// 当填充资源为图片时的配置项。
    pub image_config: ImageConfig,
    /// 当填充资源为矩形时的配置项。
    pub custom_rect_config: CustomRectConfig,
    /// 当启用文本时，文本的配置项。
    pub text_config: TextConfig,
    /// 当填充资源为图片时，开关的纹理。
    pub texture: String,
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

impl RustConstructorResource for MessageBox {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

/// RC资源最基本的错误处理。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustConstructorError {
    /// 图片获取失败。
    ImageGetFailed { image_path: String },
    /// 图片未找到。
    ImageNotFound { image_name: String },
    /// 图片纹理未找到。
    ImageTextureNotFound { image_texture_name: String },
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
    /// 开关填充资源类型不匹配。
    SwitchFillResourceMismatch {
        switch_name: String,
        fill_resource_name: String,
        fill_resource_type: String,
    },
    /// 开关未找到。
    SwitchNotFound { switch_name: String },
    /// 消息框已存在。
    MessageBoxAlreadyExists { message_box_name: String },
    /// 获取字体失败。
    FontGetFailed { font_path: String },
    /// 字体未找到。
    FontNotFound { font_name: String },
    /// 矩形未找到。
    RectNotFound { rect_name: String },
    /// 资源未找到。
    ResourceNotFound {
        resource_name: String,
        resource_type: String,
    },
    /// 页面未找到。
    PageNotFound { page_name: String },
    /// 自定义错误。
    CustomError {
        error_name: String,
        error_message: String,
        error_annotation: String,
    },
}

impl Display for RustConstructorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for RustConstructorError {}

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
#[derive(Debug, Default)]
pub struct App {
    /// 配置项。
    pub strict_mode: bool,
    /// RC资源。
    pub rust_constructor_resource: Vec<Box<dyn RustConstructorResource>>,
    /// 渲染资源列表。
    pub render_resource_list: Vec<RenderResource>,
    /// 问题列表。
    pub problem_list: Vec<Problem>,
    /// RC资源刷新率。
    pub tick_interval: f32,
    /// 当前页面。
    pub current_page: String,
    /// 计时器。
    pub timer: Timer,
    /// 帧时间。
    pub frame_times: Vec<f32>,
    /// 上一帧时间。
    pub last_frame_time: Option<f64>,
}

impl App {
    #[inline]
    pub fn strict_mode(mut self, strict_mode: bool) -> Self {
        self.strict_mode = strict_mode;
        self
    }

    #[inline]
    pub fn tick_interval(mut self, tick_interval: f32) -> Self {
        self.tick_interval = tick_interval;
        self
    }

    #[inline]
    pub fn current_page(mut self, current_page: &str) -> Self {
        self.current_page = current_page.to_string();
        self
    }

    /// 整合所有页面需要一次性处理的功能。
    pub fn page_handler(&mut self, ctx: &Context) {
        // 更新帧数
        self.update_frame_stats(ctx);
        // 更新渲染资源列表
        self.render_resource_list = Vec::new();
        // 更新计时器
        self.update_timer();
        if let Ok(pd) = self.get_resource::<PageData>(&self.current_page.clone(), "PageData")
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
        self.rust_constructor_resource.push(Box::new(page_data));
    }

    /// 切换页面。
    pub fn switch_page(&mut self, page: &str) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(page, "PageData") {
            self.current_page = page.to_string();
            let pd = self.get_resource_mut::<PageData>(page, "PageData").unwrap();
            pd.enter_page_updated = false;
            self.timer.start_time = self.timer.total_time;
            self.update_timer();
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: page.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::PageNotFound {
                page_name: page.to_string(),
            })
        }
    }

    /// 从指定列表中替换资源。
    pub fn replace_resource_custom<T>(
        &self,
        name: &str,
        discern_type: &str,
        resource: T,
        mut target: Vec<Box<dyn RustConstructorResource>>,
    ) -> Result<(), RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if let Some(index) = self
            .rust_constructor_resource
            .iter()
            .position(|x| x.name() == name && x.expose_type() == discern_type)
        {
            target[index] = Box::new(resource);
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: name.to_string(),
                    resource_type: discern_type.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ResourceNotFound {
                resource_name: name.to_string(),
                resource_type: discern_type.to_string(),
            })
        }
    }

    /// 从列表中替换资源。
    pub fn replace_resource<T>(
        &mut self,
        name: &str,
        discern_type: &str,
        resource: T,
    ) -> Result<(), RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if let Some(index) = self
            .rust_constructor_resource
            .iter()
            .position(|x| x.name() == name && x.expose_type() == discern_type)
        {
            self.rust_constructor_resource[index] = Box::new(resource);
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: name.to_string(),
                    resource_type: discern_type.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ResourceNotFound {
                resource_name: name.to_string(),
                resource_type: discern_type.to_string(),
            })
        }
    }

    /// 从列表中获取不可变资源。
    pub fn get_resource<T>(
        &self,
        name: &str,
        discern_type: &str,
    ) -> Result<&T, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if self.check_resource_exists(name, discern_type) {
            Ok(self
                .rust_constructor_resource
                .iter()
                .find(|resource| resource.name() == name && resource.expose_type() == discern_type)
                .and_then(|resource| resource.as_any().downcast_ref::<T>())
                .unwrap())
        } else {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: name.to_string(),
                    resource_type: discern_type.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ResourceNotFound {
                resource_name: name.to_string(),
                resource_type: discern_type.to_string(),
            })
        }
    }

    /// 从列表中获取可变资源。
    pub fn get_resource_mut<T>(
        &mut self,
        name: &str,
        discern_type: &str,
    ) -> Result<&mut T, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if self.check_resource_exists(name, discern_type) {
            Ok(self
                .rust_constructor_resource
                .iter_mut()
                .find(|resource| resource.name() == name && resource.expose_type() == discern_type)
                .and_then(|resource| resource.as_any_mut().downcast_mut::<T>())
                .unwrap())
        } else {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: name.to_string(),
                    resource_type: discern_type.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ResourceNotFound {
                resource_name: name.to_string(),
                resource_type: discern_type.to_string(),
            })
        }
    }

    /// 检查是否存在特定资源。
    pub fn check_resource_exists(&self, name: &str, discern_type: &str) -> bool {
        self.rust_constructor_resource
            .iter()
            .any(|x| x.name() == name && x.expose_type() == discern_type)
    }

    /// 添加字体资源。
    pub fn add_fonts(&mut self, mut font: Font) {
        let mut fonts = FontDefinitions::default();
        if let Ok(font_read_data) = read(font.path.clone()) {
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
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, font.name.to_owned());

            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, font.name.to_owned());

            font.font_definitions = fonts;
            self.rust_constructor_resource.push(Box::new(font));
        } else {
            self.problem_report_custom(
                RustConstructorError::FontGetFailed {
                    font_path: font.path.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
        };
    }

    /// 输出字体资源。
    pub fn font(&mut self, name: &str) -> Result<FontDefinitions, RustConstructorError> {
        if let Ok(f) = self.get_resource::<Font>(name, "Font") {
            return Ok(f.font_definitions.clone());
        }
        self.problem_report_custom(
            RustConstructorError::FontNotFound {
                font_name: name.to_string(),
            },
            SeverityLevel::SevereWarning,
            self.problem_list.clone(),
        );
        Err(RustConstructorError::FontNotFound {
            font_name: name.to_string(),
        })
    }

    /// 将所有已添加到RC的字体资源添加到egui中。
    pub fn register_all_fonts(&mut self, ctx: &Context) {
        let mut font_definitions = FontDefinitions::default();
        let mut font_resources = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let Some(f) = self.rust_constructor_resource[i]
                .as_any()
                .downcast_ref::<Font>()
            {
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
                        .entry(FontFamily::Name(font_name.clone().into()))
                        .or_default()
                        .push(font_name.clone());
                };

                // 将字体添加到字体列表中
                font_definitions
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.to_owned());

                font_definitions
                    .families
                    .entry(FontFamily::Monospace)
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
                "Please check if the font file exists and the path is correct.".to_string(),
            ),
            RustConstructorError::FontNotFound { font_name } => (
                format!("Font not found: {}", font_name,),
                "Please check whether the font has been added.".to_string(),
            ),
            RustConstructorError::ImageGetFailed { image_path } => (
                format!("Image get failed: {}", image_path,),
                "Please check whether the image path is correct and whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageNotFound { image_name } => (
                format!("Image not found: {}", image_name,),
                "Please check whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageTextureNotFound { image_texture_name } => (
                format!("Image texture not found: {}", image_texture_name,),
                "Please check whether the image texture has been added.".to_string(),
            ),
            RustConstructorError::TextNotFound { text_name } => (
                format!("Text not found: {}", text_name,),
                "Please check whether the text has been added.".to_string(),
            ),
            RustConstructorError::MessageBoxAlreadyExists { message_box_name } => (
                format!("Message box already exists: {}", message_box_name,),
                "Please check whether the code for generating the message box has been accidentally called multiple times.".to_string(),
            ),
            RustConstructorError::SplitTimeNotFound { split_time_name } => (
                format!("Split time not found: {}", split_time_name,),
                "Please check whether the split time has been added.".to_string(),
            ),
            RustConstructorError::SwitchAppearanceMismatch {
                switch_name,
                differ,
            } => (
                format!(
                    "Switch appearance list's number of items is large / small {} more: {}",
                    differ, switch_name
                ),
                "Please check whether the number of appearance list items matches the number of enabled animations.".to_string(),
            ),
            RustConstructorError::SwitchNotFound { switch_name } => (
                format!("Switch not found: {}", switch_name,),
                "Please check whether the switch has been added.".to_string(),
            ),
            RustConstructorError::SwitchFillResourceMismatch { switch_name, fill_resource_name, fill_resource_type } => (
                format!("Switch fill resource mismatch: Resource {} of switch {} is not of type {}", fill_resource_name, switch_name, fill_resource_type,),
                "Please check whether the imported fill resource is correctly typed.".to_string(),
            ),
            RustConstructorError::PageNotFound { page_name } => (
                format!("Page not found: {}", page_name,),
                "Please check whether the page has been added.".to_string(),
            ),
            RustConstructorError::VariableNotFound { variable_name } => (
                format!("Variable not found: {}", variable_name,),
                "Please check whether the variable has been added.".to_string(),
            ),
            RustConstructorError::VariableNotBool { variable_name } => (
                format!("Variable is not bool: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotFloat { variable_name } => (
                format!("Variable is not f32: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotInt { variable_name } => (
                format!("Variable is not int: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotString { variable_name } => (
                format!("Variable is not string: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotUInt { variable_name } => (
                format!("Variable is not uint: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotVec { variable_name } => (
                format!("Variable is not vec: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::RectNotFound { rect_name } => (
                format!("Rect not found: {}", rect_name,),
                "Please check whether the rect has been added.".to_string(),
            ),
            RustConstructorError::ResourceNotFound {
                resource_name,
                resource_type,
            } => (
                format!(
                    "Resource not found: {}(\"{}\")",
                    resource_type, resource_name,
                ),
                "Please check whether the resource has been added.".to_string(),
            ),
            RustConstructorError::CustomError { error_name, error_message, error_annotation } => (
                format!("Custom error({}): {}", error_name, error_message),
                error_annotation
            )
        };
        // 如果处于严格模式下，则直接崩溃！
        if self.strict_mode {
            panic!("{}", problem);
        };
        self.problem_list.push(Problem {
            severity_level,
            problem,
            annotation,
            report_state: ReportState {
                current_page: self.current_page.clone(),
                current_total_runtime: self.timer.total_time,
                current_page_runtime: self.timer.now_time,
            },
            problem_type: problem_type.clone(),
        });
    }

    /// 发生问题时向指定列表推送报告。
    pub fn problem_report_custom(
        &self,
        problem_type: RustConstructorError,
        severity_level: SeverityLevel,
        mut problem_storage: Vec<Problem>,
    ) {
        let (problem, annotation) = match problem_type.clone() {
            RustConstructorError::FontGetFailed { font_path } => (
                format!("Font get failed: {}", font_path,),
                "Please check if the font file exists and the path is correct.".to_string(),
            ),
            RustConstructorError::FontNotFound { font_name } => (
                format!("Font not found: {}", font_name,),
                "Please check whether the font has been added.".to_string(),
            ),
            RustConstructorError::ImageGetFailed { image_path } => (
                format!("Image get failed: {}", image_path,),
                "Please check whether the image path is correct and whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageNotFound { image_name } => (
                format!("Image not found: {}", image_name,),
                "Please check whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageTextureNotFound { image_texture_name } => (
                format!("Image texture not found: {}", image_texture_name,),
                "Please check whether the image texture has been added.".to_string(),
            ),
            RustConstructorError::TextNotFound { text_name } => (
                format!("Text not found: {}", text_name,),
                "Please check whether the text has been added.".to_string(),
            ),
            RustConstructorError::MessageBoxAlreadyExists { message_box_name } => (
                format!("Message box already exists: {}", message_box_name,),
                "Please check whether the code for generating the message box has been accidentally called multiple times.".to_string(),
            ),
            RustConstructorError::SplitTimeNotFound { split_time_name } => (
                format!("Split time not found: {}", split_time_name,),
                "Please check whether the split time has been added.".to_string(),
            ),
            RustConstructorError::SwitchAppearanceMismatch {
                switch_name,
                differ,
            } => (
                format!(
                    "Switch appearance list's number of items is large / small {} more: {}",
                    differ, switch_name
                ),
                "Please check whether the number of appearance list items matches the number of enabled animations.".to_string(),
            ),
            RustConstructorError::SwitchNotFound { switch_name } => (
                format!("Switch not found: {}", switch_name,),
                "Please check whether the switch has been added.".to_string(),
            ),
            RustConstructorError::SwitchFillResourceMismatch { switch_name, fill_resource_name, fill_resource_type } => (
                format!("Switch fill resource mismatch: Resource {} of switch {} is not of type {}", fill_resource_name, switch_name, fill_resource_type,),
                "Please check whether the imported fill resource is correctly typed.".to_string(),
            ),
            RustConstructorError::PageNotFound { page_name } => (
                format!("Page not found: {}", page_name,),
                "Please check whether the page has been added.".to_string(),
            ),
            RustConstructorError::VariableNotFound { variable_name } => (
                format!("Variable not found: {}", variable_name,),
                "Please check whether the variable has been added.".to_string(),
            ),
            RustConstructorError::VariableNotBool { variable_name } => (
                format!("Variable is not bool: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotFloat { variable_name } => (
                format!("Variable is not f32: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotInt { variable_name } => (
                format!("Variable is not int: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotString { variable_name } => (
                format!("Variable is not string: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotUInt { variable_name } => (
                format!("Variable is not uint: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::VariableNotVec { variable_name } => (
                format!("Variable is not vec: {}", variable_name,),
                "Please check whether the variable names and types are correct and whether there are duplicate items.".to_string(),
            ),
            RustConstructorError::RectNotFound { rect_name } => (
                format!("Rect not found: {}", rect_name,),
                "Please check whether the rect has been added.".to_string(),
            ),
            RustConstructorError::ResourceNotFound {
                resource_name,
                resource_type,
            } => (
                format!(
                    "Resource not found: {}(\"{}\")",
                    resource_type, resource_name,
                ),
                "Please check whether the resource has been added.".to_string(),
            ),
            RustConstructorError::CustomError { error_name, error_message, error_annotation } => (
                format!("Custom error({}): {}", error_name, error_message),
                error_annotation
            )
        };
        // 如果处于严格模式下，则直接崩溃！
        if self.strict_mode {
            panic!("{}", problem);
        };
        problem_storage.push(Problem {
            severity_level,
            problem,
            annotation,
            report_state: ReportState {
                current_page: self.current_page.clone(),
                current_total_runtime: self.timer.total_time,
                current_page_runtime: self.timer.now_time,
            },
            problem_type: problem_type.clone(),
        });
    }

    /// 检查页面是否已完成首次加载。
    pub fn check_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        if self.check_resource_exists(name, "PageData") {
            let pd = self
                .get_resource::<PageData>(name, "PageData")
                .unwrap()
                .clone();
            if !pd.change_page_updated {
                self.new_page_update(name).unwrap();
            };
            Ok(pd.change_page_updated)
        } else {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            })
        }
    }

    /// 检查页面是否已完成加载。
    pub fn check_enter_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        if self.check_resource_exists(name, "PageData") {
            let pd = self.get_resource_mut::<PageData>(name, "PageData").unwrap();
            let return_value = pd.enter_page_updated;
            pd.enter_page_updated = true;
            Ok(return_value)
        } else {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            })
        }
    }

    /// 进入新页面时的更新。
    pub fn new_page_update(&mut self, name: &str) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "PageData") {
            self.timer.start_time = self.timer.total_time;
            self.update_timer();
            let pd = self.get_resource_mut::<PageData>(name, "PageData").unwrap();
            pd.change_page_updated = true;
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            })
        }
    }

    /// 更新帧数。
    pub fn update_frame_stats(&mut self, ctx: &Context) {
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
        self.rust_constructor_resource.push(Box::new(split_time));
    }

    /// 重置分段时间。
    pub fn reset_split_time(&mut self, name: &str) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "SplitTime") {
            let new_time = [self.timer.now_time, self.timer.total_time];
            let st = self
                .get_resource_mut::<SplitTime>(name, "SplitTime")
                .unwrap();
            st.time = new_time;
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::SplitTimeNotFound {
                    split_time_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            })
        }
    }

    /// 输出分段时间。
    pub fn split_time(&self, name: &str) -> Result<[f32; 2], RustConstructorError> {
        if self.check_resource_exists(name, "SplitTime") {
            let st = self.get_resource::<SplitTime>(name, "SplitTime").unwrap();
            Ok(st.time)
        } else {
            self.problem_report_custom(
                RustConstructorError::SplitTimeNotFound {
                    split_time_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            })
        }
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
        self.rust_constructor_resource.push(Box::new(custom_rect));
    }

    /// 显示矩形资源。
    pub fn custom_rect(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "CustomRect") {
            let render_resource_list = &mut self.render_resource_list.clone();
            let cr = self
                .get_resource_mut::<CustomRect>(name, "CustomRect")
                .unwrap();
            cr.reg_render_resource(render_resource_list);
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
            match cr.center_display.0 {
                HorizontalAlign::Left => {}
                HorizontalAlign::Center => cr.position[0] -= cr.size[0] / 2.0,
                HorizontalAlign::Right => cr.position[0] -= cr.size[0],
            };
            match cr.center_display.1 {
                VerticalAlign::Top => {}
                VerticalAlign::Center => cr.position[1] -= cr.size[1] / 2.0,
                VerticalAlign::Bottom => cr.position[1] -= cr.size[1],
            };
            ui.painter().rect(
                Rect::from_min_max(
                    Pos2::new(cr.position[0], cr.position[1]),
                    Pos2::new(cr.position[0] + cr.size[0], cr.position[1] + cr.size[1]),
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
                StrokeKind::Inside,
            );
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::RectNotFound {
                    rect_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::RectNotFound {
                rect_name: name.to_string(),
            })
        }
    }

    /// 添加文本资源。
    pub fn add_text(&mut self, text: Text) {
        self.rust_constructor_resource.push(Box::new(text));
    }

    /// 显示文本资源。
    pub fn text(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "Text") {
            let mut t = self.get_resource::<Text>(name, "Text").unwrap().clone();
            t.reg_render_resource(&mut self.render_resource_list);
            // 计算文本大小
            let galley: Arc<Galley> = ui.fonts_mut(|f| {
                f.layout(
                    t.content.to_string(),
                    if self.check_resource_exists(&t.font.clone(), "Font") {
                        FontId::new(t.font_size, FontFamily::Name(t.font.clone().into()))
                    } else {
                        if !t.font.is_empty() {
                            self.problem_report_custom(
                                RustConstructorError::FontNotFound {
                                    font_name: t.font.clone(),
                                },
                                SeverityLevel::MildWarning,
                                self.problem_list.clone(),
                            );
                        };
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
                    Id::new(format!("text_{}_click_and_drag", t.name)),
                    Sense::click_and_drag(),
                );

                let response2 = ui.interact(
                    rect2,
                    Id::new(format!("text_{}_total", t.name)),
                    Sense::click(),
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
                        let c_released = input.key_released(Key::C);
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
                                Id::new(format!("link_{}_{}_{}_row_{}", t.name, start, end, row)),
                                Sense::click(),
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
                        ui.ctx().open_url(OpenUrl::new_tab(url));
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
            self.replace_resource(name, "Text", t).unwrap();
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::TextNotFound {
                    text_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::TextNotFound {
                text_name: name.to_string(),
            })
        }
    }

    /// 获取文本大小。
    pub fn get_text_size(&self, name: &str, ui: &mut Ui) -> Result<[f32; 2], RustConstructorError> {
        if self.check_resource_exists(name, "Text") {
            let t = self.get_resource::<Text>(name, "Text").unwrap();
            let galley = ui.fonts_mut(|f| {
                f.layout(
                    t.content.to_string(),
                    FontId::proportional(t.font_size),
                    Color32::from_rgba_unmultiplied(t.color[0], t.color[1], t.color[2], t.color[3]),
                    t.wrap_width,
                )
            });
            Ok([galley.size().x, galley.size().y])
        } else {
            self.problem_report_custom(
                RustConstructorError::TextNotFound {
                    text_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::TextNotFound {
                text_name: name.to_string(),
            })
        }
    }

    /// 添加变量资源。
    pub fn add_var(&mut self, variable: Variable) {
        self.rust_constructor_resource.push(Box::new(variable));
    }

    /// 修改变量资源。
    pub fn modify_var<T: Into<Value>>(
        &mut self,
        name: &str,
        value: T,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource_mut::<Variable>(name, "Variable").unwrap();
            v.value = value.into();
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出Value变量。
    pub fn var(&self, name: &str) -> Result<Value, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            Ok(v.value.clone())
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出i32变量。
    pub fn var_i(&self, name: &str) -> Result<i32, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::Int(i) => Ok(*i),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotInt {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotInt {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出u32资源。
    pub fn var_u(&self, name: &str) -> Result<u32, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::UInt(u) => Ok(*u),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotUInt {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotUInt {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出f32资源。
    pub fn var_f(&self, name: &str) -> Result<f32, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::Float(f) => Ok(*f),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotFloat {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotFloat {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出布尔值资源。
    pub fn var_b(&self, name: &str) -> Result<bool, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::Bool(b) => Ok(*b),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotBool {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotBool {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出包含Value的Vec资源。
    pub fn var_v(&self, name: &str) -> Result<Vec<Value>, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::Vec(v) => Ok(v.clone()),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotVec {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotVec {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 取出字符串资源。
    pub fn var_s(&self, name: &str) -> Result<String, RustConstructorError> {
        if self.check_resource_exists(name, "Variable") {
            let v = self.get_resource::<Variable>(name, "Variable").unwrap();
            match &v.value {
                // 直接访问 value 字段
                Value::String(s) => Ok(s.clone()),
                _ => {
                    self.problem_report_custom(
                        RustConstructorError::VariableNotString {
                            variable_name: name.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    Err(RustConstructorError::VariableNotString {
                        variable_name: name.to_string(),
                    })
                }
            }
        } else {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            })
        }
    }

    /// 尝试将Value转换成布尔值。
    pub fn var_decode_b(&self, target: Value) -> Result<bool, RustConstructorError> {
        match target {
            Value::Bool(b) => {
                // 处理布尔值
                Ok(b)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotBool {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                Err(RustConstructorError::VariableNotBool {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成i32。
    pub fn var_decode_i(&self, target: Value) -> Result<i32, RustConstructorError> {
        match target {
            Value::Int(i) => {
                // 处理i32整型
                Ok(i)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotInt {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                Err(RustConstructorError::VariableNotInt {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成u32。
    pub fn var_decode_u(&self, target: Value) -> Result<u32, RustConstructorError> {
        match target {
            Value::UInt(u) => {
                // 处理u32无符号整型
                Ok(u)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotUInt {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                Err(RustConstructorError::VariableNotUInt {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成f32。
    pub fn var_decode_f(&self, target: Value) -> Result<f32, RustConstructorError> {
        match target {
            Value::Float(f) => {
                // 处理浮点数
                Ok(f)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotFloat {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                Err(RustConstructorError::VariableNotFloat {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成字符串。
    pub fn var_decode_s(&self, target: Value) -> Result<String, RustConstructorError> {
        match target {
            Value::String(s) => {
                // 处理字符串
                Ok(s)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotString {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                Err(RustConstructorError::VariableNotString {
                    variable_name: format!("{:?}", target),
                })
            }
        }
    }

    /// 尝试将Value转换成Vec。
    pub fn var_decode_v(&self, target: Value) -> Result<Vec<Value>, RustConstructorError> {
        match target {
            Value::Vec(v) => {
                // 处理字符串
                Ok(v)
            }
            _ => {
                self.problem_report_custom(
                    RustConstructorError::VariableNotVec {
                        variable_name: format!("{:?}", target),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
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
        ctx: &Context,
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
                ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
            let loaded_image_texture = ctx.load_texture(
                image_texture.name.clone(),
                color_image,
                TextureOptions::LINEAR,
            );
            image_texture.texture = Some(DebugTextureHandle::new(loaded_image_texture));
            image_texture.cite_path = path.to_string();
            self.rust_constructor_resource.push(Box::new(image_texture));
        } else {
            self.problem_report_custom(
                RustConstructorError::ImageGetFailed {
                    image_path: path.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
        };
    }

    /// 输出图片纹理。
    pub fn image_texture(
        &self,
        name: &str,
    ) -> Result<Option<DebugTextureHandle>, RustConstructorError> {
        if self.check_resource_exists(name, "ImageTexture") {
            let it = self
                .get_resource::<ImageTexture>(name, "ImageTexture")
                .unwrap();
            Ok(it.texture.clone())
        } else {
            self.problem_report_custom(
                RustConstructorError::ImageNotFound {
                    image_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ImageNotFound {
                image_name: name.to_string(),
            })
        }
    }

    /// 重置图片纹理。
    pub fn reset_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "ImageTexture") {
            let it = self
                .get_resource_mut::<ImageTexture>(name, "ImageTexture")
                .unwrap();
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
                    ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
                let image_texture =
                    ctx.load_texture(it.name.clone(), color_image, TextureOptions::LINEAR);
                it.texture = Some(DebugTextureHandle::new(image_texture));
                it.cite_path = path.to_string();
            } else {
                self.problem_report_custom(
                    RustConstructorError::ImageGetFailed {
                        image_path: path.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
            };
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::ImageTextureNotFound {
                    image_texture_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ImageTextureNotFound {
                image_texture_name: name.to_string(),
            })
        }
    }

    /// 添加图片资源。
    pub fn add_image(
        &mut self,
        mut image: Image,
        image_texture_name: &str,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(image_texture_name, "ImageTexture") {
            let it = self
                .get_resource::<ImageTexture>(image_texture_name, "ImageTexture")
                .unwrap();
            image.texture = it.texture.clone();
            image.cite_texture = it.name.clone();
            image.last_frame_cite_texture = it.name.clone();
            self.rust_constructor_resource.push(Box::new(image));
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::ImageTextureNotFound {
                    image_texture_name: image_texture_name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ImageTextureNotFound {
                image_texture_name: image_texture_name.to_string(),
            })
        }
    }

    /// 显示图片资源。
    pub fn image(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, "Image") {
            let mut im = self
                .get_resource_mut::<Image>(name, "Image")
                .unwrap()
                .clone();
            if im.cite_texture != im.last_frame_cite_texture {
                if self.check_resource_exists(&im.cite_texture, "ImageTexture") {
                    let it = self
                        .get_resource::<ImageTexture>(&im.cite_texture, "ImageTexture")
                        .unwrap();
                    im.texture = it.texture.clone();
                } else {
                    self.problem_report_custom(
                        RustConstructorError::ImageTextureNotFound {
                            image_texture_name: im.cite_texture.clone(),
                        },
                        SeverityLevel::MildWarning,
                        self.problem_list.clone(),
                    );
                };
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

                // 直接绘制图片
                egui::Image::new(ImageSource::Texture((&texture.0).into()))
                    .tint(Color32::from_rgba_unmultiplied(
                        im.overlay_color[0],
                        im.overlay_color[1],
                        im.overlay_color[2],
                        // 将图片透明度与覆盖颜色透明度相乘
                        (im.alpha as f32 * im.overlay_color[3] as f32 / 255.0) as u8,
                    ))
                    .bg_fill(Color32::from_rgba_unmultiplied(
                        im.background_color[0],
                        im.background_color[1],
                        im.background_color[2],
                        im.background_color[3],
                    ))
                    .rotate(
                        im.rotate_angle,
                        [
                            im.rotate_center[0] / im.size[0],
                            im.rotate_center[1] / im.size[1],
                        ]
                        .into(),
                    )
                    .paint_at(ui, rect)
            };
            im.last_frame_cite_texture = im.cite_texture.clone();
            self.replace_resource(name, "Image", im).unwrap();
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::ImageNotFound {
                    image_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::ImageNotFound {
                image_name: name.to_string(),
            })
        }
    }

    /// 添加消息框资源。
    pub fn add_message_box(
        &mut self,
        mut message_box: MessageBox,
        title_name: &str,
        content_name: &str,
        image_name: &str,
        close_switch_fill_resource_name: &str,
        close_switch_fill_resource_type: &str,
    ) -> Result<(), RustConstructorError> {
        if !self.check_resource_exists(&message_box.name, "MessageBox") {
            message_box.exist = true;
            message_box.memory_offset = 0_f32;

            if self.check_resource_exists(image_name, "Image") {
                let im = self.get_resource_mut::<Image>(image_name, "Image").unwrap();
                im.size = [message_box.size[1] - 15_f32, message_box.size[1] - 15_f32];
                im.center_display = (HorizontalAlign::Left, VerticalAlign::Center);
                im.x_grid = [1, 1];
                im.y_grid = [0, 1];
                im.name = format!("MessageBox{}", im.name);
                message_box.image_name = im.name.to_string();
            } else {
                self.problem_report_custom(
                    RustConstructorError::ImageNotFound {
                        image_name: image_name.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::ImageNotFound {
                    image_name: image_name.to_string(),
                });
            }

            if self.check_resource_exists(title_name, "Text") {
                let t = self.get_resource_mut::<Text>(title_name, "Text").unwrap();
                t.x_grid = [1, 1];
                t.y_grid = [0, 1];
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.wrap_width = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
                message_box.title_name = t.name.to_string();
            } else {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: title_name.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::TextNotFound {
                    text_name: title_name.to_string(),
                });
            }

            if self.check_resource_exists(content_name, "Text") {
                let t = self.get_resource_mut::<Text>(content_name, "Text").unwrap();
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.x_grid = [1, 1];
                t.y_grid = [0, 1];
                t.wrap_width = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
                message_box.content_name = t.name.to_string();
            } else {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: content_name.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::TextNotFound {
                    text_name: content_name.to_string(),
                });
            }

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

            match close_switch_fill_resource_type {
                "Image" | "CustomRect" => {}
                &_ => {
                    self.problem_report_custom(
                        RustConstructorError::SwitchFillResourceMismatch {
                            switch_name: format!("MessageBox{}Close", message_box.name),
                            fill_resource_name: close_switch_fill_resource_name.to_string(),
                            fill_resource_type: close_switch_fill_resource_type.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::SwitchFillResourceMismatch {
                        switch_name: format!("MessageBox{}Close", message_box.name),
                        fill_resource_name: close_switch_fill_resource_name.to_string(),
                        fill_resource_type: close_switch_fill_resource_type.to_string(),
                    });
                }
            };

            if !self.check_resource_exists(
                close_switch_fill_resource_name,
                close_switch_fill_resource_type,
            ) {
                self.problem_report_custom(
                    RustConstructorError::ResourceNotFound {
                        resource_name: close_switch_fill_resource_name.to_string(),
                        resource_type: close_switch_fill_resource_type.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::ResourceNotFound {
                    resource_name: close_switch_fill_resource_name.to_string(),
                    resource_type: close_switch_fill_resource_type.to_string(),
                });
            };

            let (texture, image_config, custom_rect_config, color, border_color) =
                match close_switch_fill_resource_type {
                    "Image" => {
                        let im = self
                            .get_resource_mut::<Image>(close_switch_fill_resource_name, "Image")
                            .unwrap();
                        im.name = format!("MessageBox{}Close", close_switch_fill_resource_name);
                        (
                            im.cite_texture.clone(),
                            ImageConfig::from_image(im.clone())
                                .size(30_f32, 30_f32)
                                .center_display(HorizontalAlign::Center, VerticalAlign::Center),
                            CustomRectConfig::default(),
                            im.overlay_color,
                            [0, 0, 0, 0],
                        )
                    }
                    "CustomRect" => {
                        let cr = self
                            .get_resource_mut::<CustomRect>(
                                close_switch_fill_resource_name,
                                "CustomRect",
                            )
                            .unwrap();
                        cr.name = format!("MessageBox{}Close", close_switch_fill_resource_name);
                        (
                            String::new(),
                            ImageConfig::default(),
                            CustomRectConfig::from_custom_rect(cr.clone())
                                .size(30_f32, 30_f32)
                                .center_display(HorizontalAlign::Center, VerticalAlign::Center),
                            cr.color,
                            cr.border_color,
                        )
                    }
                    &_ => {
                        unreachable!()
                    }
                };

            self.add_switch(
                Switch::default()
                    .name(&format!("MessageBox{}Close", message_box.name))
                    .appearance(vec![
                        SwitchData {
                            image_config: image_config
                                .clone()
                                .overlay_color(color[0], color[1], color[2], 0),
                            custom_rect_config: custom_rect_config
                                .clone()
                                .color(color[0], color[1], color[2], 0)
                                .border_color(border_color[0], border_color[1], border_color[2], 0),
                            text_config: TextConfig::default(),
                            texture: texture.clone(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            image_config: image_config.clone().overlay_color(
                                (color[0] as u32 * 180 / 255) as u8,
                                (color[1] as u32 * 180 / 255) as u8,
                                (color[2] as u32 * 180 / 255) as u8,
                                255,
                            ),
                            custom_rect_config: custom_rect_config
                                .clone()
                                .color(
                                    (color[0] as u32 * 180 / 255) as u8,
                                    (color[1] as u32 * 180 / 255) as u8,
                                    (color[2] as u32 * 180 / 255) as u8,
                                    255,
                                )
                                .border_color(
                                    (border_color[0] as u32 * 180 / 255) as u8,
                                    (border_color[1] as u32 * 180 / 255) as u8,
                                    (border_color[2] as u32 * 180 / 255) as u8,
                                    255,
                                ),
                            text_config: TextConfig::default(),
                            texture: texture.clone(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            image_config: image_config.clone().overlay_color(0, 0, 0, 0),
                            custom_rect_config: custom_rect_config
                                .clone()
                                .color(0, 0, 0, 0)
                                .border_color(0, 0, 0, 0),
                            text_config: TextConfig::default(),
                            texture: texture.clone(),
                            hint_text: String::new(),
                        },
                        SwitchData {
                            image_config: image_config.overlay_color(0, 0, 0, 0),
                            custom_rect_config: custom_rect_config
                                .color(0, 0, 0, 0)
                                .border_color(0, 0, 0, 0),
                            text_config: TextConfig::default(),
                            texture,
                            hint_text: String::new(),
                        },
                    ])
                    .enable_hover_click_fill_resource(false, true)
                    .click_method(vec![SwitchClickAction {
                        click_method: PointerButton::Primary,
                        action: true,
                    }]),
                &format!("MessageBox{}Close", close_switch_fill_resource_name),
                close_switch_fill_resource_type,
                "",
            )
            .unwrap();
            self.rust_constructor_resource
                .push(Box::new(message_box.clone()));
            Ok(())
        } else {
            self.problem_report_custom(
                RustConstructorError::MessageBoxAlreadyExists {
                    message_box_name: message_box.name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::MessageBoxAlreadyExists {
                message_box_name: message_box.name.to_string(),
            })
        }
    }

    /// 处理所有已添加的消息框资源。
    pub fn message_box_display(&mut self, ctx: &Context, ui: &mut Ui) {
        let mut offset = 0_f32;
        let mut delete_count = 0;
        let mut index_list = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if self.rust_constructor_resource[i]
                .as_any()
                .downcast_ref::<MessageBox>()
                .is_some()
            {
                index_list.push(i);
            };
        }
        for u in 0..index_list.len() {
            let mut deleted = false;
            let i = u - delete_count;
            let mut mb = self.rust_constructor_resource[index_list[i]]
                .as_any()
                .downcast_ref::<MessageBox>()
                .unwrap()
                .clone();
            let mut im1 = self
                .get_resource::<Image>(&mb.image_name, "Image")
                .unwrap()
                .clone();
            let mut cr = self
                .get_resource::<CustomRect>(&format!("MessageBox{}", mb.name), "CustomRect")
                .unwrap()
                .clone();
            let mut t1 = self
                .get_resource::<Text>(&mb.title_name, "Text")
                .unwrap()
                .clone();
            let mut t2 = self
                .get_resource::<Text>(&mb.content_name, "Text")
                .unwrap()
                .clone();
            let mut s = self
                .get_resource::<Switch>(&format!("MessageBox{}Close", mb.name), "Switch")
                .unwrap()
                .clone();
            let fr: Box<dyn FrontResource> = match s.fill_resource_type.as_str() {
                "Image" => Box::new(
                    self.get_resource::<Image>(&s.fill_resource_name, "Image")
                        .unwrap()
                        .clone(),
                ),
                "CustomRect" => Box::new(
                    self.get_resource::<CustomRect>(&s.fill_resource_name, "CustomRect")
                        .unwrap()
                        .clone(),
                ),
                &_ => {
                    self.problem_report_custom(
                        RustConstructorError::SwitchFillResourceMismatch {
                            switch_name: s.name,
                            fill_resource_name: s.fill_resource_name,
                            fill_resource_type: s.fill_resource_type,
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                }
            };
            mb.reg_render_resource(&mut self.render_resource_list.clone());
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
                >= self.tick_interval
            {
                self.reset_split_time(&format!("MessageBox{}Animation", mb.name))
                    .unwrap();
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
                                &format!("MessageBox{}", mb.name),
                                "SplitTime",
                            ) {
                                self.reset_split_time(&format!("MessageBox{}", mb.name))
                                    .unwrap();
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
                t1.origin_position[1] + self.get_text_size(&mb.title_name.clone(), ui).unwrap()[1],
            ];
            for sd in &mut s.appearance {
                sd.image_config.origin_position = cr.position;
                sd.custom_rect_config.origin_position = cr.position;
            }
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
                let rect = Rect::from_min_size(
                    Pos2 {
                        x: fr.position()[0],
                        y: fr.position()[1],
                    },
                    Vec2 {
                        x: cr.size[0] + 15_f32,
                        y: cr.size[1] + 15_f32,
                    },
                );
                if rect.contains(mouse_pos) {
                    s.appearance[0].image_config.overlay_color[3] = 255;
                    s.appearance[0].custom_rect_config.color[3] = 255;
                    s.appearance[0].custom_rect_config.border_color[3] = 255;
                } else {
                    s.appearance[0].image_config.overlay_color[3] = 0;
                    s.appearance[0].custom_rect_config.color[3] = 0;
                    s.appearance[0].custom_rect_config.border_color[3] = 0;
                };
            };
            self.rust_constructor_resource[index_list[i]] = Box::new(mb.clone());
            self.replace_resource(&mb.image_name, "Image", im1.clone())
                .unwrap();
            self.replace_resource(&format!("MessageBox{}", mb.name), "CustomRect", cr.clone())
                .unwrap();
            self.replace_resource(&mb.title_name, "Text", t1.clone())
                .unwrap();
            self.replace_resource(&mb.content_name, "Text", t2.clone())
                .unwrap();
            self.replace_resource(&format!("MessageBox{}Close", mb.name), "Switch", s.clone())
                .unwrap();
            match s.fill_resource_type.as_str() {
                "Image" => {
                    self.replace_resource(
                        &s.fill_resource_name,
                        &s.fill_resource_type,
                        fr.as_any().downcast_ref::<Image>().unwrap().clone(),
                    )
                    .unwrap();
                }
                "CustomRect" => {
                    self.replace_resource(
                        &s.fill_resource_name,
                        &s.fill_resource_type,
                        fr.as_any().downcast_ref::<CustomRect>().unwrap().clone(),
                    )
                    .unwrap();
                }
                &_ => {
                    unreachable!()
                }
            };
            self.custom_rect(&format!("MessageBox{}", mb.name), ui, ctx)
                .unwrap();
            self.image(&mb.image_name.clone(), ui, ctx).unwrap();
            self.text(&t1.name.clone(), ui, ctx).unwrap();
            self.text(&t2.name.clone(), ui, ctx).unwrap();
            self.switch(
                &format!("MessageBox{}Close", mb.name),
                ui,
                ctx,
                s.state == 0 && mb.exist,
            )
            .unwrap();
            if self
                .check_switch_click_index(&format!("MessageBox{}Close", mb.name))
                .unwrap()
                == 0
                && self
                    .check_switch_state(&format!("MessageBox{}Close", mb.name))
                    .unwrap()
                    == 1
            {
                mb.exist = false;
                if cr.origin_position[0] + mb.speed >= 15_f32 {
                    cr.origin_position[0] = 15_f32;
                } else {
                    cr.origin_position[0] += mb.speed;
                };
                self.rust_constructor_resource[index_list[i]] = Box::new(mb.clone());
                self.replace_resource(&format!("MessageBox{}", mb.name), "CustomRect", cr.clone())
                    .unwrap();
            };
            if deleted {
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "Image" && x.name() == mb.image_name)
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| {
                            x.expose_type() == "CustomRect"
                                && x.name() == format!("MessageBox{}", mb.name)
                        })
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "Text" && x.name() == mb.title_name)
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "Text" && x.name() == mb.content_name)
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| {
                            x.expose_type() == "Switch"
                                && x.name() == format!("MessageBox{}Close", mb.name)
                        })
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| {
                            x.expose_type() == s.fill_resource_type
                                && x.name() == s.fill_resource_name
                        })
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| {
                            x.expose_type() == "SplitTime"
                                && x.name() == format!("MessageBox{}Animation", mb.name)
                        })
                        .unwrap(),
                );
                if !mb.keep_existing {
                    self.rust_constructor_resource.remove(
                        self.rust_constructor_resource
                            .iter()
                            .position(|x| {
                                x.expose_type() == "SplitTime"
                                    && x.name() == format!("MessageBox{}", mb.name)
                            })
                            .unwrap(),
                    );
                };
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "MessageBox" && x.name() == mb.name)
                        .unwrap(),
                );
            } else {
                offset += mb.size[1] + 15_f32;
            };
        }
    }

    /// 添加开关资源。
    pub fn add_switch(
        &mut self,
        mut switch: Switch,
        fill_resource_name: &str,
        fill_resource_type: &str,
        text_name: &str,
    ) -> Result<(), RustConstructorError> {
        let mut count = 1;
        if switch.enable_hover_click_fill_resource[0] {
            count += 1;
        };
        if switch.enable_hover_click_fill_resource[1] {
            count += 1;
        };
        if switch.appearance.len() < count {
            self.problem_report_custom(
                RustConstructorError::SwitchAppearanceMismatch {
                    switch_name: switch.name.clone(),
                    differ: count as u32 - switch.appearance.len() as u32,
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::SwitchAppearanceMismatch {
                switch_name: switch.name.clone(),
                differ: count as u32 - switch.appearance.len() as u32,
            });
        };
        for _ in 0..switch.appearance.len() % count {
            switch.appearance.pop();
        }
        if self.check_resource_exists(text_name, "Text") {
            let t = self.get_resource_mut::<Text>(text_name, "Text").unwrap();
            switch.text_name = text_name.to_string();
            switch.text_origin_position = t.origin_position;
            t.center_display = (HorizontalAlign::Center, VerticalAlign::Center);
            t.x_grid = [0, 0];
            t.y_grid = [0, 0];
        } else if !text_name.is_empty() {
            self.problem_report_custom(
                RustConstructorError::TextNotFound {
                    text_name: text_name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
        };
        if self.check_resource_exists(fill_resource_name, fill_resource_type) {
            match fill_resource_type {
                "Image" | "CustomRect" => {
                    switch.fill_resource_name = fill_resource_name.to_string();
                    switch.fill_resource_type = fill_resource_type.to_string();
                }
                &_ => {
                    self.problem_report_custom(
                        RustConstructorError::SwitchFillResourceMismatch {
                            switch_name: switch.name.clone(),
                            fill_resource_name: fill_resource_name.to_string(),
                            fill_resource_type: fill_resource_type.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::SwitchFillResourceMismatch {
                        switch_name: switch.name.clone(),
                        fill_resource_name: fill_resource_name.to_string(),
                        fill_resource_type: fill_resource_type.to_string(),
                    });
                }
            };
        } else {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: fill_resource_name.to_string(),
                    resource_type: fill_resource_type.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ResourceNotFound {
                resource_name: fill_resource_name.to_string(),
                resource_type: fill_resource_type.to_string(),
            });
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
                    .background_color([0, 0, 0, 255])
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
        self.rust_constructor_resource.push(Box::new(switch));
        Ok(())
    }

    /// 显示开关资源并返回点击方法和开关状态。
    pub fn switch(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        enable: bool,
    ) -> Result<(), RustConstructorError> {
        let mut appearance_count = 0;
        if !self.check_resource_exists(name, "Switch") {
            self.problem_report_custom(
                RustConstructorError::SwitchNotFound {
                    switch_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::SwitchNotFound {
                switch_name: name.to_string(),
            });
        };
        let mut s = self.get_resource::<Switch>(name, "Switch").unwrap().clone();
        if !self.check_resource_exists(&s.fill_resource_name.clone(), &s.fill_resource_type) {
            self.problem_report_custom(
                RustConstructorError::ResourceNotFound {
                    resource_name: s.fill_resource_name.clone(),
                    resource_type: s.fill_resource_type.clone(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ResourceNotFound {
                resource_name: s.fill_resource_name,
                resource_type: s.fill_resource_type,
            });
        };
        if !s.hint_text_name.is_empty() {
            if !self.check_resource_exists(&format!("{}StartHoverTime", s.name), "SplitTime") {
                self.problem_report_custom(
                    RustConstructorError::SplitTimeNotFound {
                        split_time_name: format!("{}StartHoverTime", s.name),
                    },
                    SeverityLevel::MildWarning,
                    self.problem_list.clone(),
                );
                self.add_split_time(
                    SplitTime::default().name(&format!("{}StartHoverTime", s.name)),
                );
            };
            if !self.check_resource_exists(&format!("{}HintFadeAnimation", s.name), "SplitTime") {
                self.problem_report_custom(
                    RustConstructorError::SplitTimeNotFound {
                        split_time_name: format!("{}HintFadeAnimation", s.name),
                    },
                    SeverityLevel::MildWarning,
                    self.problem_list.clone(),
                );
                self.add_split_time(
                    SplitTime::default().name(&format!("{}HintFadeAnimation", s.name)),
                );
            };
            if !self.check_resource_exists(&s.hint_text_name, "Text") {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: s.hint_text_name.clone(),
                    },
                    SeverityLevel::MildWarning,
                    self.problem_list.clone(),
                );
                self.add_text(
                    Text::default()
                        .name(&s.hint_text_name)
                        .content("")
                        .origin_position(0_f32, 0_f32)
                        .font_size(25_f32)
                        .wrap_width(300_f32)
                        .background_rounding(10_f32)
                        .color(255, 255, 255, 0)
                        .background_color([0, 0, 0, 255])
                        .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                        .selectable(false),
                );
            };
        };
        let fr: Box<dyn FrontResource> = match &*s.fill_resource_type {
            "Image" => Box::new(
                self.get_resource::<Image>(&s.fill_resource_name.clone(), &s.fill_resource_type)
                    .unwrap()
                    .clone(),
            ),
            "CustomRect" => Box::new(
                self.get_resource::<CustomRect>(
                    &s.fill_resource_name.clone(),
                    &s.fill_resource_type,
                )
                .unwrap()
                .clone(),
            ),
            &_ => {
                self.problem_report_custom(
                    RustConstructorError::SwitchFillResourceMismatch {
                        switch_name: name.to_string(),
                        fill_resource_name: s.fill_resource_name.clone(),
                        fill_resource_type: s.fill_resource_type.clone(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::SwitchFillResourceMismatch {
                    switch_name: name.to_string(),
                    fill_resource_name: s.fill_resource_name,
                    fill_resource_type: s.fill_resource_type,
                });
            }
        };
        s.reg_render_resource(&mut self.render_resource_list);
        let rect = Rect::from_min_size(
            Pos2::new(fr.position()[0], fr.position()[1]),
            Vec2::new(fr.size()[0], fr.size()[1]),
        );
        let mut hovered = false;
        if enable {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                // 判断是否在矩形内
                if rect.contains(mouse_pos) {
                    if !s.hint_text_name.is_empty() {
                        if self.check_resource_exists(&s.hint_text_name, "Text") {
                            let mut t = self
                                .get_resource::<Text>(&s.hint_text_name, "Text")
                                .unwrap()
                                .clone();
                            if !s.last_time_hovered {
                                self.reset_split_time(&format!("{}StartHoverTime", s.name))
                                    .unwrap();
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
                            self.replace_resource(&s.hint_text_name, "Text", t.clone())
                                .unwrap();
                        } else {
                            self.problem_report_custom(
                                RustConstructorError::TextNotFound {
                                    text_name: s.hint_text_name.clone(),
                                },
                                SeverityLevel::SevereWarning,
                                self.problem_list.clone(),
                            );
                            return Err(RustConstructorError::TextNotFound {
                                text_name: s.hint_text_name.clone(),
                            });
                        };
                    };
                    hovered = true;
                    let mut clicked = vec![];
                    let mut active = false;
                    for u in 0..s.click_method.len() as u32 {
                        clicked.push(ui.input(|i| {
                            i.pointer
                                .button_down(s.click_method[u as usize].click_method)
                        }));
                        if clicked[u as usize] {
                            active = true;
                            s.last_time_clicked_index = u as usize;
                            break;
                        };
                    }
                    if active {
                        s.last_time_clicked = true;
                        if s.enable_hover_click_fill_resource[1] {
                            if s.enable_hover_click_fill_resource[0] {
                                appearance_count = 2;
                            } else {
                                appearance_count = 1;
                            };
                        } else if !s.enable_hover_click_fill_resource[0] {
                            appearance_count = 0;
                        };
                    } else {
                        if s.last_time_clicked {
                            if s.click_method[s.last_time_clicked_index].action {
                                if s.state
                                    < (s.appearance.len() / s.animation_count as usize - 1) as u32
                                {
                                    s.state += 1;
                                } else {
                                    s.state = 0;
                                };
                            };
                            s.last_time_clicked = false;
                        };
                        if s.enable_hover_click_fill_resource[0] {
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
                self.reset_split_time(&format!("{}HintFadeAnimation", s.name))
                    .unwrap();
            };
            if self.check_resource_exists(&s.hint_text_name, "Text") {
                let mut t = self
                    .get_resource::<Text>(&s.hint_text_name, "Text")
                    .unwrap()
                    .clone();
                if self.timer.total_time
                    - self
                        .split_time(&format!("{}HintFadeAnimation", s.name))
                        .unwrap()[1]
                    >= self.tick_interval
                {
                    self.reset_split_time(&format!("{}HintFadeAnimation", s.name))
                        .unwrap();
                    t.color[3] = t.color[3].saturating_sub(10);
                };
                self.replace_resource(&s.hint_text_name, "Text", t.clone())
                    .unwrap();
            } else {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: s.hint_text_name.clone(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::TextNotFound {
                    text_name: s.hint_text_name.clone(),
                });
            };
        };
        let fr: Box<dyn FrontResource> = match &*s.fill_resource_type {
            "Image" => {
                let mut im = Box::new(
                    fr.as_any()
                        .downcast_ref::<Image>()
                        .unwrap()
                        .clone()
                        .from_config(
                            s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                                .image_config
                                .clone(),
                        ),
                );
                if self.check_resource_exists(
                    &s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                        .texture
                        .clone(),
                    "ImageTexture",
                ) {
                    im.cite_texture = s.appearance
                        [(s.state * s.animation_count + appearance_count) as usize]
                        .texture
                        .clone();
                } else {
                    self.problem_report_custom(
                        RustConstructorError::ImageTextureNotFound {
                            image_texture_name: s.appearance
                                [(s.state * s.animation_count + appearance_count) as usize]
                                .texture
                                .clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::ImageTextureNotFound {
                        image_texture_name: s.appearance
                            [(s.state * s.animation_count + appearance_count) as usize]
                            .texture
                            .clone(),
                    });
                };
                im
            }
            "CustomRect" => Box::new(
                fr.as_any()
                    .downcast_ref::<CustomRect>()
                    .unwrap()
                    .clone()
                    .name(&s.fill_resource_name)
                    .from_config(
                        s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                            .custom_rect_config
                            .clone(),
                    ),
            ),
            &_ => {
                self.problem_report_custom(
                    RustConstructorError::SwitchFillResourceMismatch {
                        switch_name: name.to_string(),
                        fill_resource_name: s.fill_resource_name.clone(),
                        fill_resource_type: s.fill_resource_type.clone(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::SwitchFillResourceMismatch {
                    switch_name: name.to_string(),
                    fill_resource_name: s.fill_resource_name,
                    fill_resource_type: s.fill_resource_type,
                });
            }
        };
        if !s.hint_text_name.is_empty() {
            if self.check_resource_exists(&s.hint_text_name, "Text") {
                let mut t = self
                    .get_resource::<Text>(&s.hint_text_name, "Text")
                    .unwrap()
                    .clone();
                t.background_color[3] = t.color[3];
                t.content = s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                    .hint_text
                    .clone();
                self.replace_resource(&s.hint_text_name, "Text", t.clone())
                    .unwrap();
            } else {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: s.hint_text_name.clone(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::TextNotFound {
                    text_name: s.hint_text_name,
                });
            };
        };
        s.last_time_hovered = hovered;
        self.replace_resource(name, "Switch", s.clone()).unwrap();
        match s.fill_resource_type.as_str() {
            "Image" => {
                let im = fr.as_any().downcast_ref::<Image>().unwrap().clone();
                self.replace_resource(&s.fill_resource_name, &s.fill_resource_type, im)
                    .unwrap();
                self.image(&s.fill_resource_name.clone(), ui, ctx).unwrap();
            }
            "CustomRect" => {
                let cr = fr.as_any().downcast_ref::<CustomRect>().unwrap().clone();
                self.replace_resource(&s.fill_resource_name, &s.fill_resource_type, cr)
                    .unwrap();
                self.custom_rect(&s.fill_resource_name.clone(), ui, ctx)
                    .unwrap();
            }
            &_ => {}
        }
        if self.check_resource_exists(&s.text_name, "Text") {
            let mut t = self
                .get_resource::<Text>(&s.text_name, "Text")
                .unwrap()
                .clone();
            s.text_origin_position = s.appearance
                [(s.state * s.animation_count + appearance_count) as usize]
                .text_config
                .origin_position;
            t.origin_position = [
                fr.position()[0] + s.text_origin_position[0],
                fr.position()[1] + s.text_origin_position[1],
            ];
            t = t.from_config(
                s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                    .text_config
                    .clone(),
            );
            self.replace_resource(&s.text_name, "Text", t.clone())
                .unwrap();
            self.text(&s.text_name, ui, ctx).unwrap();
        };
        if self.check_resource_exists(&s.hint_text_name, "Text") {
            self.text(&s.hint_text_name, ui, ctx).unwrap();
        };
        Ok(())
    }

    /// 查找指定开关的点击方法(取决于开关的click_method元素，未点击则返回5)。
    pub fn check_switch_click_index(&self, name: &str) -> Result<usize, RustConstructorError> {
        if self.check_resource_exists(name, "Switch") {
            let s = self.get_resource::<Switch>(name, "Switch").unwrap();
            Ok(s.last_time_clicked_index)
        } else {
            self.problem_report_custom(
                RustConstructorError::SwitchNotFound {
                    switch_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::SwitchNotFound {
                switch_name: name.to_string(),
            })
        }
    }

    /// 查找特定开关的状态。
    pub fn check_switch_state(&self, name: &str) -> Result<u32, RustConstructorError> {
        if self.check_resource_exists(name, "Switch") {
            let s = self.get_resource::<Switch>(name, "Switch").unwrap();
            Ok(s.state)
        } else {
            self.problem_report_custom(
                RustConstructorError::SwitchNotFound {
                    switch_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            Err(RustConstructorError::SwitchNotFound {
                switch_name: name.to_string(),
            })
        }
    }
}
