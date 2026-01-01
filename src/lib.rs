//! # `Rust Constructor V2`
//!
//!
//! 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式
//!
//! A cross-platform `GUI` framework built on `egui`, the simplest way to develop `GUI` projects in `Rust`
//!
//!
//! 有关`Rust Constructor`的使用方法，请参考[Rust Constructor 指南](https://github.com/ChepleBob30/Rust-Constructor-Guide)。
//!
//! 关于源代码及更多内容，请访问`Rust Constructor`的[GitHub 仓库](https://github.com/ChepleBob30/Rust-Constructor)以获取。
//!
//!
//! About the usage method of `Rust Constructor`, please refer to the [Rust Constructor Guide](https://github.com/ChepleBob30/Rust-Constructor-Guide).
//!
//! About the source code and more content, please visit the `Rust Constructor` [GitHub Repository](https://github.com/ChepleBob30/Rust-Constructor) to get.
//!
//!
//! 如果你对此项目感兴趣，你也可以来看看我们的组织[必达](https://github.com/Binder-organize)的其他项目。
//!
//! If you are interested in this project, You can also come and take a look at other projects of our organization [Binder](https://github.com/Binder-organize).
use eframe::{
    Result,
    emath::Rect,
    epaint::{Stroke, textures::TextureOptions},
};
use egui::{
    Color32, ColorImage, Context, CursorIcon, FontData, FontDefinitions, FontFamily, FontId,
    Galley, Id, ImageSource, Key, OpenUrl, PointerButton, Pos2, Sense, StrokeKind, TextureHandle,
    Ui, Vec2, text::CCursor,
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

    /// 返回活跃状态。
    fn active(&self) -> bool;

    /// 改变活跃状态。
    fn modify_active(&mut self, active: bool);

    /// 用于不可变类型转换。
    fn as_any(&self) -> &dyn Any;

    /// 用于可变类型转换。
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 标记并管理用于显示给用户的基本前端资源。
pub trait BasicFrontResource: RustConstructorResource {
    /// 获取资源尺寸。
    fn display_size(&self) -> [f32; 2];

    /// 获取资源位置。
    fn display_position(&self) -> [f32; 2];

    /// 获取资源偏移量。
    fn display_offset(&self) -> [f32; 2];

    /// 获取资源渲染范围。
    fn display_clip_rect(&self) -> &Option<PositionConfig>;

    /// 获取资源板名称。
    fn display_panel_name(&self) -> &str;

    /// 获取资源板排版。
    fn display_panel_layout(&self) -> &Option<(PanelLocation, PanelLayout)>;

    /// 获取资源对齐方法。
    fn display_center_display(&self) -> &(HorizontalAlign, VerticalAlign);

    /// 获取是否允许资源板滚动。
    fn display_allow_scrolling(&self) -> [bool; 2];

    /// 修改资源尺寸。
    fn modify_size(&mut self, width: f32, height: f32);

    /// 修改资源位置。
    fn modify_position(&mut self, x: f32, y: f32);

    /// 修改资源偏移量。
    fn modify_offset(&mut self, x: f32, y: f32);

    /// 修改资源渲染范围。
    fn modify_clip_rect(&mut self, clip_rect: &Option<PositionConfig>);

    /// 修改资源板名称。
    fn modify_panel_name(&mut self, panel_name: &str);

    /// 修改资源板排版。
    fn modify_panel_layout(&mut self, panel_layout: &Option<(PanelLocation, PanelLayout)>);

    /// 修改资源对齐方法。
    fn modify_center_display(
        &mut self,
        horizontal_align: &HorizontalAlign,
        vertical_align: &VerticalAlign,
    );

    /// 修改是否允许资源板滚动。
    fn modify_allow_scrolling(&mut self, horizontal: bool, vertical: bool);
}

/// 用于配置资源位置的结构体。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PositionConfig {
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式定位。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位。
    pub y_location_grid: [u32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
}

impl Default for PositionConfig {
    fn default() -> Self {
        PositionConfig {
            origin_position: [0_f32, 0_f32],
            origin_size: [100_f32, 100_f32],
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
        }
    }
}

impl PositionConfig {
    pub fn from_image(image: &Image) -> Self {
        Self {
            origin_position: image.origin_position,
            origin_size: image.origin_size,
            x_size_grid: image.x_size_grid,
            y_size_grid: image.y_size_grid,
            x_location_grid: image.x_location_grid,
            y_location_grid: image.y_location_grid,
            center_display: image.center_display,
            offset: image.offset,
        }
    }

    pub fn from_image_config(image_config: &ImageConfig) -> Self {
        Self {
            origin_position: image_config.origin_position,
            origin_size: image_config.origin_size,
            x_size_grid: image_config.x_size_grid,
            y_size_grid: image_config.y_size_grid,
            x_location_grid: image_config.x_location_grid,
            y_location_grid: image_config.y_location_grid,
            center_display: image_config.center_display,
            offset: image_config.offset,
        }
    }

    pub fn from_custom_rect(custom_rect: &CustomRect) -> Self {
        Self {
            origin_position: custom_rect.origin_position,
            origin_size: custom_rect.origin_size,
            x_size_grid: custom_rect.x_size_grid,
            y_size_grid: custom_rect.y_size_grid,
            x_location_grid: custom_rect.x_location_grid,
            y_location_grid: custom_rect.y_location_grid,
            center_display: custom_rect.center_display,
            offset: custom_rect.offset,
        }
    }

    pub fn from_custom_rect_config(custom_rect_config: &CustomRectConfig) -> Self {
        Self {
            origin_position: custom_rect_config.origin_position,
            origin_size: custom_rect_config.origin_size,
            x_size_grid: custom_rect_config.x_size_grid,
            y_size_grid: custom_rect_config.y_size_grid,
            x_location_grid: custom_rect_config.x_location_grid,
            y_location_grid: custom_rect_config.y_location_grid,
            center_display: custom_rect_config.center_display,
            offset: custom_rect_config.offset,
        }
    }

    pub fn from_text(text: &Text) -> Self {
        Self {
            origin_position: text.origin_position,
            origin_size: text.origin_size,
            x_size_grid: text.x_size_grid,
            y_size_grid: text.y_size_grid,
            x_location_grid: text.x_location_grid,
            y_location_grid: text.y_location_grid,
            center_display: text.center_display,
            offset: text.offset,
        }
    }

    pub fn from_text_config(text_config: &TextConfig) -> Self {
        Self {
            origin_position: text_config.origin_position,
            origin_size: text_config.origin_size,
            x_size_grid: text_config.x_size_grid,
            y_size_grid: text_config.y_size_grid,
            x_location_grid: text_config.x_location_grid,
            y_location_grid: text_config.y_location_grid,
            center_display: text_config.center_display,
            offset: text_config.offset,
        }
    }

    pub fn from_mouse_detector(mouse_detector: &MouseDetector) -> Self {
        Self {
            origin_position: mouse_detector.origin_position,
            origin_size: mouse_detector.origin_size,
            x_size_grid: mouse_detector.x_size_grid,
            y_size_grid: mouse_detector.y_size_grid,
            x_location_grid: mouse_detector.x_location_grid,
            y_location_grid: mouse_detector.y_location_grid,
            center_display: mouse_detector.center_display,
            offset: mouse_detector.offset,
        }
    }

    pub fn from_resource_panel(resource_panel: &ResourcePanel) -> Self {
        Self {
            origin_position: resource_panel.origin_position,
            origin_size: resource_panel.origin_size,
            x_size_grid: resource_panel.x_size_grid,
            y_size_grid: resource_panel.y_size_grid,
            x_location_grid: resource_panel.x_location_grid,
            y_location_grid: resource_panel.y_location_grid,
            center_display: resource_panel.center_display,
            offset: resource_panel.offset,
        }
    }

    #[inline]
    pub fn origin_position(mut self, x: f32, y: f32) -> Self {
        self.origin_position = [x, y];
        self
    }

    #[inline]
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
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
    /// 弱警告：会影响程序正常执行，但一般情况下不会有严重后果。
    MildWarning,
    /// 强警告：会影响程序正常执行，且可能导致意外情况。
    SevereWarning,
    /// 错误：会导致程序无法运行。
    Error,
}

/// 用于确认基本前端资源在资源板中的排版方式。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PanelLayout {
    /// 垂直布局。
    Vertical(f32, f32, f32, f32, bool),
    /// 水平布局。
    Horizontal(f32, f32, f32, f32, bool),
    /// 无布局。
    None(f32, f32, f32, f32, bool),
}

/// 用于控制基本前端资源在资源板中的定位方式。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PanelLocation {
    /// 依照此资源到资源板左上角的距离定位。
    Absolute(f32, f32),
    /// 依照网格式定位方法进行定位。
    Relative([[u32; 2]; 2]),
}

/// 用于存储页面数据的RC资源。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PageData {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 是否强制在每帧都刷新页面。
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

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
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
#[derive(Debug, Clone, PartialEq)]
pub struct ImageTexture {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
    /// 图片路径。
    pub cite_path: String,
    /// 翻转图片。
    pub flip: [bool; 2],
    /// 加载资源。
    pub ctx: Context,
}

impl RustConstructorResource for ImageTexture {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
            texture: None,
            cite_path: String::from(""),
            flip: [false, false],
            ctx: Context::default(),
        }
    }
}

impl ImageTexture {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn cite_path(mut self, cite_path: &str) -> Self {
        self.cite_path = cite_path.to_string();
        self
    }

    #[inline]
    pub fn flip(mut self, horizontal_flip: bool, vertical_flip: bool) -> Self {
        self.flip = [horizontal_flip, vertical_flip];
        self
    }

    #[inline]
    pub fn ctx(mut self, ctx: &Context) -> Self {
        self.ctx = ctx.clone();
        self
    }
}

/// 矩形的可配置项。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRectConfig {
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// 圆角。
    pub rounding: f32,
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
    /// 颜色。
    pub color: [u8; 4],
    /// 边框宽度。
    pub border_width: f32,
    /// 边框颜色。
    pub border_color: [u8; 4],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl Default for CustomRectConfig {
    fn default() -> Self {
        Self {
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            rounding: 2_f32,
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            color: [255, 255, 255, 255],
            border_width: 2_f32,
            border_color: [0, 0, 0, 255],
            origin_position: [0_f32, 0_f32],
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl CustomRectConfig {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_custom_rect(custom_rect: &CustomRect) -> Self {
        Self {
            origin_size: custom_rect.origin_size,
            x_size_grid: custom_rect.x_size_grid,
            y_size_grid: custom_rect.y_size_grid,
            rounding: custom_rect.rounding,
            x_location_grid: custom_rect.x_location_grid,
            y_location_grid: custom_rect.y_location_grid,
            center_display: custom_rect.center_display,
            offset: custom_rect.offset,
            color: custom_rect.color,
            border_width: custom_rect.border_width,
            border_color: custom_rect.border_color,
            origin_position: custom_rect.origin_position,
            clip_rect: custom_rect.clip_rect.clone(),
            panel_name: custom_rect.panel_name.clone(),
            panel_layout: custom_rect.panel_layout.clone(),
            allow_scrolling: custom_rect.allow_scrolling,
        }
    }

    #[inline]
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
        self
    }
}

/// RC的矩形资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRect {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 位置。
    pub position: [f32; 2],
    /// 尺寸。
    pub size: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// 圆角。
    pub rounding: f32,
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
    /// 颜色。
    pub color: [u8; 4],
    /// 边框宽度。
    pub border_width: f32,
    /// 边框颜色。
    pub border_color: [u8; 4],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl RustConstructorResource for CustomRect {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl BasicFrontResource for CustomRect {
    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.size
    }

    fn display_offset(&self) -> [f32; 2] {
        self.offset
    }

    fn display_clip_rect(&self) -> &Option<PositionConfig> {
        &self.clip_rect
    }

    fn display_panel_name(&self) -> &str {
        &self.panel_name
    }

    fn display_panel_layout(&self) -> &Option<(PanelLocation, PanelLayout)> {
        &self.panel_layout
    }

    fn display_center_display(&self) -> &(HorizontalAlign, VerticalAlign) {
        &self.center_display
    }

    fn display_allow_scrolling(&self) -> [bool; 2] {
        self.allow_scrolling
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, height: f32) {
        self.origin_size = [width, height];
    }

    fn modify_offset(&mut self, x: f32, y: f32) {
        self.offset = [x, y];
    }

    fn modify_clip_rect(&mut self, clip_rect: &Option<PositionConfig>) {
        self.clip_rect = clip_rect.clone();
    }

    fn modify_panel_name(&mut self, panel_name: &str) {
        self.panel_name = panel_name.to_string();
    }

    fn modify_panel_layout(&mut self, panel_layout: &Option<(PanelLocation, PanelLayout)>) {
        self.panel_layout = panel_layout.clone();
    }

    fn modify_center_display(
        &mut self,
        horizontal_align: &HorizontalAlign,
        vertical_align: &VerticalAlign,
    ) {
        self.center_display = (*horizontal_align, *vertical_align);
    }

    fn modify_allow_scrolling(&mut self, horizontal: bool, vertical: bool) {
        self.allow_scrolling = [horizontal, vertical];
    }
}

impl Default for CustomRect {
    fn default() -> Self {
        Self {
            discern_type: String::from("CustomRect"),
            name: String::from("CustomRect"),
            active: false,
            position: [0_f32, 0_f32],
            size: [100_f32, 100_f32],
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            rounding: 2_f32,
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            color: [255, 255, 255, 255],
            border_width: 2_f32,
            border_color: [0, 0, 0, 255],
            origin_position: [0_f32, 0_f32],
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl CustomRect {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_config(mut self, config: &CustomRectConfig) -> Self {
        self.origin_size = config.origin_size;
        self.x_size_grid = config.x_size_grid;
        self.y_size_grid = config.y_size_grid;
        self.rounding = config.rounding;
        self.x_location_grid = config.x_location_grid;
        self.y_location_grid = config.y_location_grid;
        self.center_display = config.center_display;
        self.offset = config.offset;
        self.color = config.color;
        self.border_width = config.border_width;
        self.border_color = config.border_color;
        self.origin_position = config.origin_position;
        self.clip_rect = config.clip_rect.clone();
        self.panel_name = config.panel_name.clone();
        self.panel_layout = config.panel_layout.clone();
        self.allow_scrolling = config.allow_scrolling;
        self
    }

    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
        self
    }
}

/// 图片的可配置项。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ImageConfig {
    /// 图片原始大小。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
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
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            alpha: 255,
            overlay_color: [255, 255, 255, 255],
            background_color: [0, 0, 0, 0],
            rotate_angle: 0_f32,
            rotate_center: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            cite_texture: String::from("ImageTexture"),
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl ImageConfig {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_image(image: &Image) -> Self {
        Self {
            origin_size: image.origin_size,
            x_size_grid: image.x_size_grid,
            y_size_grid: image.y_size_grid,
            x_location_grid: image.x_location_grid,
            y_location_grid: image.y_location_grid,
            center_display: image.center_display,
            offset: image.offset,
            alpha: image.alpha,
            overlay_color: image.overlay_color,
            background_color: image.background_color,
            rotate_angle: image.rotate_angle,
            rotate_center: image.rotate_center,
            cite_texture: image.cite_texture.clone(),
            origin_position: image.origin_position,
            clip_rect: image.clip_rect.clone(),
            panel_name: image.panel_name.clone(),
            panel_layout: image.panel_layout.clone(),
            allow_scrolling: image.allow_scrolling,
        }
    }

    #[inline]
    pub fn origin_position(mut self, x: f32, y: f32) -> Self {
        self.origin_position = [x, y];
        self
    }

    #[inline]
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = [r, g, b, a];
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

    #[inline]
    pub fn cite_texture(mut self, cite_texture: &str) -> Self {
        self.cite_texture = cite_texture.to_string();
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
        self
    }
}

/// RC的图片资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
    /// 图片位置。
    pub position: [f32; 2],
    /// 图片大小。
    pub size: [f32; 2],
    /// 图片原始大小。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
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
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl RustConstructorResource for Image {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl BasicFrontResource for Image {
    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.size
    }

    fn display_offset(&self) -> [f32; 2] {
        self.offset
    }

    fn display_clip_rect(&self) -> &Option<PositionConfig> {
        &self.clip_rect
    }

    fn display_panel_name(&self) -> &str {
        &self.panel_name
    }

    fn display_panel_layout(&self) -> &Option<(PanelLocation, PanelLayout)> {
        &self.panel_layout
    }

    fn display_center_display(&self) -> &(HorizontalAlign, VerticalAlign) {
        &self.center_display
    }

    fn display_allow_scrolling(&self) -> [bool; 2] {
        self.allow_scrolling
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, height: f32) {
        self.origin_size = [width, height];
    }

    fn modify_offset(&mut self, x: f32, y: f32) {
        self.offset = [x, y];
    }

    fn modify_clip_rect(&mut self, clip_rect: &Option<PositionConfig>) {
        self.clip_rect = clip_rect.clone();
    }

    fn modify_panel_name(&mut self, panel_name: &str) {
        self.panel_name = panel_name.to_string();
    }

    fn modify_panel_layout(&mut self, panel_layout: &Option<(PanelLocation, PanelLayout)>) {
        self.panel_layout = panel_layout.clone();
    }

    fn modify_center_display(
        &mut self,
        horizontal_align: &HorizontalAlign,
        vertical_align: &VerticalAlign,
    ) {
        self.center_display = (*horizontal_align, *vertical_align);
    }

    fn modify_allow_scrolling(&mut self, horizontal: bool, vertical: bool) {
        self.allow_scrolling = [horizontal, vertical];
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            discern_type: String::from("Image"),
            name: String::from("Image"),
            active: false,
            texture: None,
            position: [0_f32, 0_f32],
            size: [100_f32, 100_f32],
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            alpha: 255,
            overlay_color: [255, 255, 255, 255],
            background_color: [0, 0, 0, 0],
            rotate_angle: 0_f32,
            rotate_center: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            cite_texture: String::from("ImageTexture"),
            last_frame_cite_texture: String::from("ImageTexture"),
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl Image {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_config(mut self, config: &ImageConfig) -> Self {
        self.origin_size = config.origin_size;
        self.x_size_grid = config.x_size_grid;
        self.y_size_grid = config.y_size_grid;
        self.x_location_grid = config.x_location_grid;
        self.y_location_grid = config.y_location_grid;
        self.center_display = config.center_display;
        self.offset = config.offset;
        self.alpha = config.alpha;
        self.overlay_color = config.overlay_color;
        self.background_color = config.background_color;
        self.rotate_angle = config.rotate_angle;
        self.rotate_center = config.rotate_center;
        self.origin_position = config.origin_position;
        self.cite_texture = config.cite_texture.clone();
        self.clip_rect = config.clip_rect.clone();
        self.panel_name = config.panel_name.clone();
        self.panel_layout = config.panel_layout.clone();
        self.allow_scrolling = config.allow_scrolling;
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
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = [r, g, b, a];
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

    #[inline]
    pub fn cite_texture(mut self, cite_texture: &str) -> Self {
        self.cite_texture = cite_texture.to_string();
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
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
    /// 偏移量。
    pub offset: [f32; 2],
    /// 背景颜色。
    pub background_color: [u8; 4],
    /// 圆角。
    pub background_rounding: f32,
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 字体。
    pub font: String,
    /// 是否可框选。
    pub selectable: bool,
    /// 超链接文本。
    pub hyperlink_text: Vec<(String, HyperlinkSelectMethod)>,
    /// 是否使用截断文本功能。
    pub truncate: bool,
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            content: String::from("Hello world"),
            font_size: 16_f32,
            origin_size: [0_f32, 0_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            color: [255, 255, 255, 255],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            background_color: [0, 0, 0, 0],
            background_rounding: 2_f32,
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            origin_position: [0_f32, 0_f32],
            font: String::new(),
            selectable: true,
            hyperlink_text: Vec::new(),
            truncate: false,
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl TextConfig {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_text(text: &Text) -> Self {
        Self {
            content: text.content.clone(),
            font_size: text.font_size,
            origin_size: text.origin_size,
            x_size_grid: text.x_size_grid,
            y_size_grid: text.y_size_grid,
            color: text.color,
            center_display: text.center_display,
            offset: text.offset,
            background_color: text.background_color,
            background_rounding: text.background_rounding,
            x_location_grid: text.x_location_grid,
            y_location_grid: text.y_location_grid,
            origin_position: text.origin_position,
            font: text.font.clone(),
            selectable: text.selectable,
            hyperlink_text: text.hyperlink_text.clone(),
            truncate: text.truncate,
            clip_rect: text.clip_rect.clone(),
            panel_name: text.panel_name.clone(),
            panel_layout: text.panel_layout.clone(),
            allow_scrolling: text.allow_scrolling,
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
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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

    #[inline]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
        self
    }
}

/// RC的文本资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 文本内容。
    pub content: String,
    /// 字号。
    pub font_size: f32,
    /// 文本实际尺寸。
    pub actual_size: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// 文本颜色。
    pub color: [u8; 4],
    /// 文本位置。
    pub position: [f32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
    /// 背景颜色。
    pub background_color: [u8; 4],
    /// 圆角。
    pub background_rounding: f32,
    /// x轴的网格式定位：窗口宽 / 第二项 * 第一项 = x轴的原始位置。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位：窗口高 / 第二项 * 第一项 = y轴的原始位置。
    pub y_location_grid: [u32; 2],
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
    /// 文本大小。
    pub size: [f32; 2],
    /// 是否截断文本。
    pub truncate: bool,
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionConfig>,
    /// 资源板名称。
    pub panel_name: String,
    /// 资源排版方式(如果不允许移动资源留空即可)。
    pub panel_layout: Option<(PanelLocation, PanelLayout)>,
    /// 允许在资源板中滚动。
    pub allow_scrolling: [bool; 2],
}

impl RustConstructorResource for Text {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl BasicFrontResource for Text {
    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.actual_size
    }

    fn display_offset(&self) -> [f32; 2] {
        self.offset
    }

    fn display_clip_rect(&self) -> &Option<PositionConfig> {
        &self.clip_rect
    }

    fn display_panel_name(&self) -> &str {
        &self.panel_name
    }

    fn display_panel_layout(&self) -> &Option<(PanelLocation, PanelLayout)> {
        &self.panel_layout
    }

    fn display_center_display(&self) -> &(HorizontalAlign, VerticalAlign) {
        &self.center_display
    }

    fn display_allow_scrolling(&self) -> [bool; 2] {
        self.allow_scrolling
    }

    fn modify_position(&mut self, x: f32, y: f32) {
        self.origin_position = [x, y];
    }

    fn modify_size(&mut self, width: f32, height: f32) {
        self.origin_size = [width, height];
    }

    fn modify_offset(&mut self, x: f32, y: f32) {
        self.offset = [x, y];
    }

    fn modify_clip_rect(&mut self, clip_rect: &Option<PositionConfig>) {
        self.clip_rect = clip_rect.clone();
    }

    fn modify_panel_name(&mut self, panel_name: &str) {
        self.panel_name = panel_name.to_string();
    }

    fn modify_panel_layout(&mut self, panel_layout: &Option<(PanelLocation, PanelLayout)>) {
        self.panel_layout = panel_layout.clone();
    }

    fn modify_center_display(
        &mut self,
        horizontal_align: &HorizontalAlign,
        vertical_align: &VerticalAlign,
    ) {
        self.center_display = (*horizontal_align, *vertical_align);
    }

    fn modify_allow_scrolling(&mut self, horizontal: bool, vertical: bool) {
        self.allow_scrolling = [horizontal, vertical];
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            discern_type: String::from("Text"),
            name: String::from("Text"),
            active: false,
            content: String::from("Hello world"),
            font_size: 16_f32,
            actual_size: [0_f32, 0_f32],
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            color: [255, 255, 255, 255],
            position: [0_f32, 0_f32],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            background_color: [0, 0, 0, 0],
            background_rounding: 2_f32,
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            origin_position: [0_f32, 0_f32],
            font: String::new(),
            selection: None,
            selectable: true,
            hyperlink_text: Vec::new(),
            hyperlink_index: Vec::new(),
            last_frame_content: String::from(""),
            size: [100_f32, 100_f32],
            truncate: false,
            clip_rect: None,
            panel_name: String::new(),
            panel_layout: None,
            allow_scrolling: [false, false],
        }
    }
}

impl Text {
    pub fn from_position_config(mut self, position_config: &PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self
    }

    pub fn from_config(mut self, config: &TextConfig) -> Self {
        self.content = config.content.clone();
        self.font_size = config.font_size;
        self.origin_size = config.origin_size;
        self.x_size_grid = config.x_size_grid;
        self.y_size_grid = config.y_size_grid;
        self.color = config.color;
        self.center_display = config.center_display;
        self.offset = config.offset;
        self.background_color = config.background_color;
        self.background_rounding = config.background_rounding;
        self.x_location_grid = config.x_location_grid;
        self.y_location_grid = config.y_location_grid;
        self.origin_position = config.origin_position;
        self.font = config.font.clone();
        self.selectable = config.selectable;
        self.hyperlink_text = config.hyperlink_text.clone();
        self.clip_rect = config.clip_rect.clone();
        self.truncate = config.truncate;
        self.panel_name = config.panel_name.clone();
        self.panel_layout = config.panel_layout.clone();
        self.allow_scrolling = config.allow_scrolling;
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
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
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
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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

    #[inline]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn panel_name(mut self, panel_name: &str) -> Self {
        self.panel_name = panel_name.to_string();
        self
    }

    #[inline]
    pub fn panel_layout(mut self, panel_layout: Option<(PanelLocation, PanelLayout)>) -> Self {
        self.panel_layout = panel_layout;
        self
    }

    #[inline]
    pub fn allow_scrolling(mut self, horizontal: bool, vertical: bool) -> Self {
        self.allow_scrolling = [horizontal, vertical];
        self
    }
}

/// RC的变量资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable<T> {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 变量的值。
    pub value: Option<T>,
}

impl<T: Debug + 'static> RustConstructorResource for Variable<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> Default for Variable<T> {
    fn default() -> Self {
        Variable {
            discern_type: String::from("Variable"),
            name: String::from("Variable"),
            active: false,
            value: None,
        }
    }
}

impl<T> Variable<T> {
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = value;
        self
    }
}

/// RC的字体资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    pub name: String,
    pub discern_type: String,
    pub active: bool,
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

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
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
    pub active: bool,
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

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
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

/// 开关的外观。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SwitchAppearance {
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

/// 用于开关资源判定的一些字段集合。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwitchData {
    /// 是否点击切换状态。
    pub switched: bool,
    /// 点击的方法。
    pub last_time_clicked_index: usize,
    /// 开关状态。
    pub state: u32,
}

/// RC的开关资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 外观（包括各类资源配置项，数量为开启的内容数量*开关状态总数）。
    pub appearance: Vec<SwitchAppearance>,
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
    /// 是否切换了开关状态。
    pub switched: bool,
}

impl RustConstructorResource for Switch {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
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
            hint_text_name: String::new(),
            text_name: String::new(),
            text_origin_position: [0_f32, 0_f32],
            switched: false,
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
    pub fn appearance(mut self, appearance: Vec<SwitchAppearance>) -> Self {
        self.appearance = appearance;
        self
    }

    #[inline]
    pub fn fill_resource_name(mut self, fill_resource_name: &str) -> Self {
        self.fill_resource_name = fill_resource_name.to_string();
        self
    }

    #[inline]
    pub fn fill_resource_type(mut self, fill_resource_type: &str) -> Self {
        self.fill_resource_type = fill_resource_type.to_string();
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

    #[inline]
    pub fn text_name(mut self, text_name: &str) -> Self {
        self.text_name = text_name.to_string();
        self
    }
}

/// RC的消息框资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageBox {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
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
    /// 关闭按钮的填充资源名称。
    pub close_switch_fill_resource_name: String,
    /// 关闭按钮的填充资源类型。
    pub close_switch_fill_resource_type: String,
}

impl RustConstructorResource for MessageBox {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
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
            active: false,
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
            close_switch_fill_resource_name: String::from("CloseSwitch"),
            close_switch_fill_resource_type: String::from("Image"),
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
    pub fn content_name(mut self, content_name: &str) -> Self {
        self.content_name = content_name.to_string();
        self
    }

    #[inline]
    pub fn title_name(mut self, title_name: &str) -> Self {
        self.title_name = title_name.to_string();
        self
    }

    #[inline]
    pub fn image_name(mut self, image_name: &str) -> Self {
        self.image_name = image_name.to_string();
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

    #[inline]
    pub fn close_switch_fill_resource_name(
        mut self,
        close_switch_fill_resource_name: &str,
    ) -> Self {
        self.close_switch_fill_resource_name = close_switch_fill_resource_name.to_string();
        self
    }

    #[inline]
    pub fn close_switch_fill_resource_type(
        mut self,
        close_switch_fill_resource_type: &str,
    ) -> Self {
        self.close_switch_fill_resource_type = close_switch_fill_resource_type.to_string();
        self
    }
}

/// 鼠标检测器单次检测等级。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseDetectorLevel {
    /// 精简模式，只进行最基础的检测。
    Lite,
    /// 标准模式，检测大部分常用鼠标行为。
    #[default]
    Default,
    /// 完整模式，检测所有鼠标行为。
    Pro,
}

/// 鼠标检测器检测结果。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct MouseDetectResult {
    /// 是否点击。
    pub clicked: bool,
    /// 鼠标是否在检测范围内。
    pub contains_pointer: bool,
    /// 是否点击右键。
    pub secondary_clicked: Option<bool>,
    /// 是否点击中键。
    pub middle_clicked: Option<bool>,
    /// 是否点击扩展按键。
    pub clicked_by_extra_button: Option<[bool; 2]>,
    /// 是否长时间触屏。
    pub long_touched: Option<bool>,
    /// 是否双击。
    pub double_clicked: Option<bool>,
    /// 是否三击。
    pub triple_clicked: Option<bool>,
    /// 双击的方法。
    pub double_clicked_by: Option<[bool; 5]>,
    /// 三击的方法。
    pub triple_clicked_by: Option<[bool; 5]>,
    /// 是否在检测范围外点击。
    pub clicked_elsewhere: Option<bool>,
    /// 是否悬挂在检测范围内。
    pub hovered: Option<bool>,
    /// 是否开始拖动。
    pub drag_started: Option<bool>,
    /// 开始拖动的方法。
    pub drag_started_by: Option<[bool; 5]>,
    /// 是否正在拖动。
    pub dragged: Option<bool>,
    /// 拖动方法。
    pub dragged_by: Option<[bool; 5]>,
    /// 是否结束拖动。
    pub drag_stopped: Option<bool>,
    /// 结束拖动方法。
    pub deag_stopped_by: Option<[bool; 5]>,
    /// 上一帧拖动经过了多少格像素。
    pub drag_delta: Option<[f32; 2]>,
    /// 一次拖动中总共经过了多少格像素。
    pub total_drag_delta: Option<Option<[f32; 2]>>,
    /// 鼠标上一帧拖动了多远。
    pub drag_motion: Option<[f32; 2]>,
    /// 鼠标交互的位置。
    pub interact_pointer_pos: Option<Option<[f32; 2]>>,
    /// 鼠标悬挂的位置。
    pub hover_pos: Option<Option<[f32; 2]>>,
    /// 鼠标是否按下按键。
    pub is_pointer_button_down_on: Option<bool>,
    /// 鼠标滚轮滚动幅度。
    pub raw_scroll_delta: Option<[f32; 2]>,
    /// 平滑鼠标滚轮滚动幅度。
    pub smooth_scroll_delta: Option<[f32; 2]>,
}

/// RC的鼠标检测器资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MouseDetector {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 位置。
    pub position: [f32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 尺寸。
    pub size: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// x轴的网格式定位。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位。
    pub y_location_grid: [u32; 2],
    /// 对齐方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
    /// 鼠标检测结果。
    pub detect_result: MouseDetectResult,
}

impl RustConstructorResource for MouseDetector {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for MouseDetector {
    fn default() -> Self {
        Self {
            discern_type: String::from("MouseDetector"),
            name: String::from("MouseDetector"),
            active: false,
            position: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            size: [100_f32, 100_f32],
            origin_size: [100_f32, 100_f32],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            detect_result: MouseDetectResult::default(),
        }
    }
}

impl MouseDetector {
    pub fn from_position_config(mut self, position_config: PositionConfig) -> Self {
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
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
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
    }
}

/// 滚动区域滚动长度(尺寸)配置。
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum ScrollLengthMethod {
    /// 固定尺寸。
    Fixed(f32),
    /// 自适应尺寸。
    #[default]
    AutoFit,
}

/// 鼠标点击资源板的目的。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClickAim {
    #[default]
    /// 移动资源板。
    Move,
    /// 在上方缩放。
    TopResize,
    /// 在下方缩放。
    BottomResize,
    /// 在左侧缩放。
    LeftResize,
    /// 在右侧缩放。
    RightResize,
    /// 在左上方缩放。
    LeftTopResize,
    /// 在右上方缩放。
    RightTopResize,
    /// 在左下方缩放。
    LeftBottomResize,
    /// 在右下方缩放。
    RightBottomResize,
}

/// RC的资源板。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ResourcePanel {
    pub discern_type: String,
    pub name: String,
    pub active: bool,
    /// 尺寸。
    pub size: [f32; 2],
    /// 位置。
    pub position: [f32; 2],
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式定位。
    pub x_location_grid: [u32; 2],
    /// y轴的网格式定位。
    pub y_location_grid: [u32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [u32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [u32; 2],
    /// 居中显示方法。
    pub center_display: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
    /// 是否可通过拖拽更改尺寸。
    pub resizable: [bool; 4],
    /// 是否在资源底部显示方框。
    pub display_rect: Option<CustomRectConfig>,
    /// 是否按下鼠标与按下后鼠标状态。
    pub last_frame_mouse_status: Option<([f32; 2], ClickAim, [f32; 2])>,
    /// 最小尺寸。
    pub min_size: [f32; 2],
    /// 最大尺寸(可选)。
    pub max_size: Option<[f32; 2]>,
    /// 允许拖动资源板。
    pub movable: [bool; 2],
    /// 滚动长度计算方法(不需要滚动留空即可)。
    pub scroll_length_method: [Option<ScrollLengthMethod>; 2],
    /// 滚动长度。
    pub scroll_length: [f32; 2],
    /// 滚动进度。
    pub scroll_progress: [f32; 2],
    /// 滚动敏感度。
    pub scroll_sensitivity: f32,
    /// 是否使用平滑滚动。
    pub use_smooth_scroll_delta: bool,
    /// 上一帧的滚动进度。
    pub last_frame_scroll_progress: [f32; 2],
}

impl RustConstructorResource for ResourcePanel {
    fn name(&self) -> &str {
        &self.name
    }

    fn expose_type(&self) -> &str {
        &self.discern_type
    }

    fn active(&self) -> bool {
        self.active
    }

    fn modify_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for ResourcePanel {
    fn default() -> Self {
        Self {
            discern_type: String::from("ResourcePanel"),
            name: String::from("ResourcePanel"),
            active: false,
            size: [100_f32, 100_f32],
            position: [0_f32, 0_f32],
            origin_position: [0_f32, 0_f32],
            origin_size: [100_f32, 100_f32],
            x_location_grid: [0, 0],
            y_location_grid: [0, 0],
            x_size_grid: [0, 0],
            y_size_grid: [0, 0],
            center_display: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
            resizable: [true, true, true, true],
            display_rect: None,
            last_frame_mouse_status: None,
            min_size: [10_f32, 10_f32],
            max_size: None,
            movable: [true, true],
            scroll_length_method: [None, None],
            scroll_length: [0_f32, 0_f32],
            scroll_progress: [0_f32, 0_f32],
            scroll_sensitivity: 0_f32,
            use_smooth_scroll_delta: true,
            last_frame_scroll_progress: [0_f32, 0_f32],
        }
    }
}

impl ResourcePanel {
    pub fn from_position_config(mut self, position_config: PositionConfig) -> Self {
        self.center_display = position_config.center_display;
        self.offset = position_config.offset;
        self.origin_position = position_config.origin_position;
        self.origin_size = position_config.origin_size;
        self.x_size_grid = position_config.x_size_grid;
        self.y_size_grid = position_config.y_size_grid;
        self.x_location_grid = position_config.x_location_grid;
        self.y_location_grid = position_config.y_location_grid;
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
    pub fn origin_size(mut self, width: f32, height: f32) -> Self {
        self.origin_size = [width, height];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: u32, total: u32) -> Self {
        self.y_size_grid = [fetch, total];
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
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
    }

    #[inline]
    pub fn resizable(mut self, top: bool, bottom: bool, left: bool, right: bool) -> Self {
        self.resizable = [top, bottom, left, right];
        self
    }

    #[inline]
    pub fn display_rect(mut self, display_rect: Option<CustomRectConfig>) -> Self {
        self.display_rect = display_rect;
        self
    }

    #[inline]
    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.min_size = [width, height];
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: Option<[f32; 2]>) -> Self {
        self.max_size = max_size;
        self
    }

    #[inline]
    pub fn movable(mut self, horizontal: bool, vertical: bool) -> Self {
        self.movable = [horizontal, vertical];
        self
    }

    #[inline]
    pub fn scroll_length_method(
        mut self,
        horizontal: Option<ScrollLengthMethod>,
        vertical: Option<ScrollLengthMethod>,
    ) -> Self {
        self.scroll_length_method = [horizontal, vertical];
        self
    }

    #[inline]
    pub fn scroll_sensitivity(mut self, scroll_sensitivity: f32) -> Self {
        self.scroll_sensitivity = scroll_sensitivity;
        self
    }

    #[inline]
    pub fn use_smooth_scroll_delta(mut self, use_smooth_scroll_delta: bool) -> Self {
        self.use_smooth_scroll_delta = use_smooth_scroll_delta;
        self
    }
}

/// RC资源最基本的错误处理。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustConstructorError {
    /// 资源名重复。
    ResourceNameRepetition {
        resource_name: String,
        resource_type: String,
    },
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
    /// 由于输入类型错误，变量值无法匹配。
    VariableTypeMismatch { variable_name: String },
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
    /// 鼠标检测器未找到。
    MouseDetectorNotFound { mouse_detector_name: String },
    /// 获取字体失败。
    FontGetFailed { font_path: String },
    /// 字体未找到。
    FontNotFound { font_name: String },
    /// 矩形未找到。
    RectNotFound { rect_name: String },
    /// 资源板未找到。
    ResourcePanelNotFound { resource_panel_name: String },
    /// 资源未找到。
    ResourceNotFound {
        resource_name: String,
        resource_type: String,
    },
    /// 资源未命名。
    ResourceUntitled { resource_type: String },
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
#[derive(Debug)]
pub struct App {
    /// 配置项，用于控制出现问题时是否panic。
    pub strict_mode: bool,
    /// 配置项，用于全局控制是否在调用资源时检查资源是否存在。
    pub safe_mode: bool,
    /// RC资源。
    pub rust_constructor_resource: Vec<Box<dyn RustConstructorResource>>,
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
    /// 标记哪些资源属于基本前端资源。
    pub basic_front_resource_list: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            strict_mode: false,
            safe_mode: true,
            rust_constructor_resource: Vec::new(),
            problem_list: Vec::new(),
            tick_interval: 0.05,
            current_page: String::new(),
            timer: Timer::default(),
            frame_times: Vec::new(),
            last_frame_time: None,
            basic_front_resource_list: vec![
                String::from("Image"),
                String::from("Text"),
                String::from("CustomRect"),
            ],
        }
    }
}

impl App {
    #[inline]
    pub fn strict_mode(mut self, strict_mode: bool) -> Self {
        self.strict_mode = strict_mode;
        self
    }

    #[inline]
    pub fn safe_mode(mut self, safe_mode: bool) -> Self {
        self.safe_mode = safe_mode;
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

    /// 打印资源活跃情况。
    pub fn print_resource_active_info(&self, display_complex_info: bool, format: bool) -> String {
        let mut text = String::from("Resource Active Info:\n");
        for rcr in &self.rust_constructor_resource {
            if rcr.active() {
                if display_complex_info {
                    text += &if format {
                        format!(
                            "\nName: {:?}\nType: {:?}\nDetail: {:#?}\n",
                            rcr.name(),
                            rcr.expose_type(),
                            rcr
                        )
                    } else {
                        format!(
                            "\nName: {:?}\nType: {:?}\nDetail: {:?}\n",
                            rcr.name(),
                            rcr.expose_type(),
                            rcr
                        )
                    };
                } else {
                    text += &format!("\nName: {:?}\nType: {:?}\n", rcr.name(), rcr.expose_type());
                };
            };
        }
        text
    }

    /// 添加资源。
    pub fn add_resource<T: RustConstructorResource + 'static>(
        &mut self,
        mut resource: T,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode {
            if self.check_resource_exists(resource.name(), resource.expose_type()) {
                self.problem_report_custom(
                    RustConstructorError::ResourceNameRepetition {
                        resource_name: resource.name().to_string(),
                        resource_type: resource.expose_type().to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::ResourceNameRepetition {
                    resource_name: resource.name().to_string(),
                    resource_type: resource.expose_type().to_string(),
                });
            };
            if resource.name().is_empty() {
                self.problem_report_custom(
                    RustConstructorError::ResourceUntitled {
                        resource_type: resource.expose_type().to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::ResourceUntitled {
                    resource_type: resource.expose_type().to_string(),
                });
            };
        };
        match resource.expose_type() {
            "PageData" => {}
            "CustomRect" => {}
            "Text" => {}
            "Variable" => {}
            "MouseDetector" => {}
            "SplitTime" => {
                let split_time = resource.as_any_mut().downcast_mut::<SplitTime>().unwrap();
                split_time.time = [self.timer.now_time, self.timer.total_time];
            }
            "ResourcePanel" => {
                let resource_panel = resource
                    .as_any_mut()
                    .downcast_mut::<ResourcePanel>()
                    .unwrap();
                if resource_panel.display_rect.is_some() {
                    self.add_resource(
                        CustomRect::default().name(&format!("{}DisplayRect", resource_panel.name)),
                        safe_mode,
                    )
                    .unwrap();
                };
            }
            "ImageTexture" => {
                let image_texture = resource
                    .as_any_mut()
                    .downcast_mut::<ImageTexture>()
                    .unwrap();
                if let Ok(mut file) = File::open(image_texture.cite_path.clone()) {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();
                    let img_bytes = buffer;
                    let img = image::load_from_memory(&img_bytes).unwrap();
                    let color_data = match image_texture.flip {
                        [true, true] => img.fliph().flipv().into_rgba8(),
                        [true, false] => img.fliph().into_rgba8(),
                        [false, true] => img.flipv().into_rgba8(),
                        _ => img.into_rgba8(),
                    };
                    let (w, h) = (color_data.width(), color_data.height());
                    let raw_data: Vec<u8> = color_data.into_raw();

                    let color_image =
                        ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
                    let loaded_image_texture = image_texture.ctx.load_texture(
                        image_texture.name.clone(),
                        color_image,
                        TextureOptions::LINEAR,
                    );
                    image_texture.texture = Some(DebugTextureHandle::new(loaded_image_texture));
                    image_texture.cite_path = image_texture.cite_path.to_string();
                } else {
                    self.problem_report_custom(
                        RustConstructorError::ImageGetFailed {
                            image_path: image_texture.cite_path.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                };
            }
            "Image" => {
                let image = resource.as_any_mut().downcast_mut::<Image>().unwrap();
                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(&image.cite_texture, "ImageTexture")
                {
                    self.problem_report_custom(
                        RustConstructorError::ImageTextureNotFound {
                            image_texture_name: image.cite_texture.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::ImageTextureNotFound {
                        image_texture_name: image.cite_texture.clone(),
                    });
                };
                let image_texture = self
                    .get_resource::<ImageTexture>(&image.cite_texture, "ImageTexture")
                    .unwrap()
                    .unwrap();
                image.texture = image_texture.texture.clone();
                image.last_frame_cite_texture = image_texture.name.clone();
            }
            "Font" => {
                let font = resource.as_any_mut().downcast_mut::<Font>().unwrap();
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
                } else {
                    self.problem_report_custom(
                        RustConstructorError::FontGetFailed {
                            font_path: font.path.to_string(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::FontGetFailed {
                        font_path: font.path.to_string(),
                    });
                }
            }
            "Switch" => {
                let switch = resource.as_any_mut().downcast_mut::<Switch>().unwrap();
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
                if self.check_resource_exists(&switch.text_name, "Text") {
                    let t = self
                        .get_resource_mut::<Text>(&switch.text_name, "Text")
                        .unwrap()
                        .unwrap();
                    switch.text_origin_position = t.origin_position;
                    t.center_display = (HorizontalAlign::Center, VerticalAlign::Center);
                    t.x_location_grid = [0, 0];
                    t.y_location_grid = [0, 0];
                } else if !switch.text_name.is_empty() {
                    switch.text_name = String::new();
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: switch.text_name.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                };
                if self
                    .check_resource_exists(&switch.fill_resource_name, &switch.fill_resource_type)
                {
                    match switch.fill_resource_type.as_str() {
                        "Image" | "CustomRect" => {
                            switch.fill_resource_name = switch.fill_resource_name.clone();
                            switch.fill_resource_type = switch.fill_resource_type.clone();
                        }
                        _ => {
                            self.problem_report_custom(
                                RustConstructorError::SwitchFillResourceMismatch {
                                    switch_name: switch.name.clone(),
                                    fill_resource_name: switch.fill_resource_name.clone(),
                                    fill_resource_type: switch.fill_resource_type.clone(),
                                },
                                SeverityLevel::SevereWarning,
                                self.problem_list.clone(),
                            );
                            return Err(RustConstructorError::SwitchFillResourceMismatch {
                                switch_name: switch.name.clone(),
                                fill_resource_name: switch.fill_resource_name.clone(),
                                fill_resource_type: switch.fill_resource_type.clone(),
                            });
                        }
                    };
                } else {
                    self.problem_report_custom(
                        RustConstructorError::ResourceNotFound {
                            resource_name: switch.fill_resource_name.clone(),
                            resource_type: switch.fill_resource_type.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::ResourceNotFound {
                        resource_name: switch.fill_resource_name.clone(),
                        resource_type: switch.fill_resource_type.clone(),
                    });
                };
                if switch
                    .appearance
                    .iter()
                    .filter(|x| !x.hint_text.is_empty())
                    .count()
                    > 0
                {
                    switch.hint_text_name = format!("{}Hint", switch.name);
                    self.add_resource(
                        Text::default()
                            .name(&format!("{}Hint", switch.name))
                            .content("")
                            .origin_position(0_f32, 0_f32)
                            .font_size(25_f32)
                            .origin_size(300_f32, 0_f32)
                            .background_rounding(10_f32)
                            .color(255, 255, 255, 0)
                            .background_color(0, 0, 0, 255)
                            .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                            .selectable(false),
                        safe_mode,
                    )
                    .unwrap();
                    self.add_resource(
                        SplitTime::default().name(&format!("{}StartHoverTime", switch.name)),
                        safe_mode,
                    )
                    .unwrap();
                    self.add_resource(
                        SplitTime::default().name(&format!("{}HintFadeAnimation", switch.name)),
                        safe_mode,
                    )
                    .unwrap();
                };
                switch.animation_count = count as u32;
            }
            "MessageBox" => {
                let message_box = resource.as_any_mut().downcast_mut::<MessageBox>().unwrap();
                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && self.check_resource_exists(&message_box.name, "MessageBox")
                {
                    self.problem_report_custom(
                        RustConstructorError::MessageBoxAlreadyExists {
                            message_box_name: message_box.name.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::MessageBoxAlreadyExists {
                        message_box_name: message_box.name.clone(),
                    });
                };
                message_box.exist = true;
                message_box.memory_offset = 0_f32;

                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(&message_box.image_name, "Image")
                {
                    self.problem_report_custom(
                        RustConstructorError::ImageNotFound {
                            image_name: message_box.image_name.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::ImageNotFound {
                        image_name: message_box.image_name.clone(),
                    });
                };
                let im = self
                    .get_resource_mut::<Image>(&message_box.image_name, "Image")
                    .unwrap()
                    .unwrap();
                im.origin_size = [message_box.size[1] - 15_f32, message_box.size[1] - 15_f32];
                im.center_display = (HorizontalAlign::Left, VerticalAlign::Center);
                im.x_location_grid = [1, 1];
                im.y_location_grid = [0, 1];
                im.x_size_grid = [0, 0];
                im.y_size_grid = [0, 0];
                im.name = format!("MessageBox{}", im.name);
                message_box.image_name = im.name.to_string();

                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(&message_box.title_name, "Text")
                {
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: message_box.title_name.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::TextNotFound {
                        text_name: message_box.title_name.clone(),
                    });
                };
                let t = self
                    .get_resource_mut::<Text>(&message_box.title_name, "Text")
                    .unwrap()
                    .unwrap();
                t.x_location_grid = [1, 1];
                t.y_location_grid = [0, 1];
                t.x_size_grid = [0, 0];
                t.y_size_grid = [0, 0];
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.origin_size[0] = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
                message_box.title_name = t.name.to_string();

                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(&message_box.content_name, "Text")
                {
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: message_box.content_name.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::TextNotFound {
                        text_name: message_box.content_name.clone(),
                    });
                };
                let t = self
                    .get_resource_mut::<Text>(&message_box.content_name, "Text")
                    .unwrap()
                    .unwrap();
                t.center_display = (HorizontalAlign::Left, VerticalAlign::Top);
                t.x_location_grid = [1, 1];
                t.y_location_grid = [0, 1];
                t.x_size_grid = [0, 0];
                t.y_size_grid = [0, 0];
                t.origin_size[0] = message_box.size[0] - message_box.size[1] + 5_f32;
                t.name = format!("MessageBox{}", t.name);
                message_box.content_name = t.name.to_string();

                if !message_box.keep_existing {
                    self.add_resource(
                        SplitTime::default().name(&format!("MessageBox{}", message_box.name)),
                        safe_mode,
                    )
                    .unwrap();
                };

                self.add_resource(
                    SplitTime::default().name(&format!("MessageBox{}Animation", message_box.name)),
                    safe_mode,
                )
                .unwrap();

                self.add_resource(
                    CustomRect::default()
                        .name(&format!("MessageBox{}", message_box.name))
                        .origin_position(0_f32, 0_f32)
                        .origin_size(message_box.size[0], message_box.size[1])
                        .rounding(20_f32)
                        .x_location_grid(1, 1)
                        .y_location_grid(0, 1)
                        .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                        .color(100, 100, 100, 125)
                        .border_width(0_f32),
                    safe_mode,
                )
                .unwrap();

                if safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode {
                    match message_box.close_switch_fill_resource_type.as_str() {
                        "Image" | "CustomRect" => {}
                        _ => {
                            self.problem_report_custom(
                                RustConstructorError::SwitchFillResourceMismatch {
                                    switch_name: format!("MessageBox{}Close", message_box.name),
                                    fill_resource_name: message_box
                                        .close_switch_fill_resource_name
                                        .clone(),
                                    fill_resource_type: message_box
                                        .close_switch_fill_resource_type
                                        .clone(),
                                },
                                SeverityLevel::SevereWarning,
                                self.problem_list.clone(),
                            );
                            return Err(RustConstructorError::SwitchFillResourceMismatch {
                                switch_name: format!("MessageBox{}Close", message_box.name),
                                fill_resource_name: message_box
                                    .close_switch_fill_resource_name
                                    .clone(),
                                fill_resource_type: message_box
                                    .close_switch_fill_resource_type
                                    .clone(),
                            });
                        }
                    };
                };

                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(
                        &message_box.close_switch_fill_resource_name,
                        &message_box.close_switch_fill_resource_type,
                    )
                {
                    self.problem_report_custom(
                        RustConstructorError::ResourceNotFound {
                            resource_name: message_box.close_switch_fill_resource_name.clone(),
                            resource_type: message_box.close_switch_fill_resource_type.clone(),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    return Err(RustConstructorError::ResourceNotFound {
                        resource_name: message_box.close_switch_fill_resource_name.clone(),
                        resource_type: message_box.close_switch_fill_resource_type.clone(),
                    });
                };

                let (texture, image_config, custom_rect_config, color, border_color) =
                    match message_box.close_switch_fill_resource_type.as_str() {
                        "Image" => {
                            let im = self
                                .get_resource_mut::<Image>(
                                    &message_box.close_switch_fill_resource_name,
                                    "Image",
                                )
                                .unwrap()
                                .unwrap();
                            im.name = format!(
                                "MessageBox{}Close",
                                message_box.close_switch_fill_resource_name
                            );
                            (
                                im.cite_texture.clone(),
                                ImageConfig::from_image(im)
                                    .origin_size(30_f32, 30_f32)
                                    .center_display(HorizontalAlign::Center, VerticalAlign::Center),
                                CustomRectConfig::default(),
                                im.overlay_color,
                                [0, 0, 0, 0],
                            )
                        }
                        "CustomRect" => {
                            let cr = self
                                .get_resource_mut::<CustomRect>(
                                    &message_box.close_switch_fill_resource_name,
                                    "CustomRect",
                                )
                                .unwrap()
                                .unwrap();
                            cr.name = format!(
                                "MessageBox{}Close",
                                message_box.close_switch_fill_resource_name
                            );
                            (
                                String::new(),
                                ImageConfig::default(),
                                CustomRectConfig::from_custom_rect(cr)
                                    .origin_size(30_f32, 30_f32)
                                    .center_display(HorizontalAlign::Center, VerticalAlign::Center),
                                cr.color,
                                cr.border_color,
                            )
                        }
                        _ => {
                            self.problem_report_custom(
                                RustConstructorError::SwitchFillResourceMismatch {
                                    switch_name: format!("MessageBox{}Close", message_box.name),
                                    fill_resource_name: message_box
                                        .close_switch_fill_resource_name
                                        .clone(),
                                    fill_resource_type: message_box
                                        .close_switch_fill_resource_type
                                        .clone(),
                                },
                                SeverityLevel::SevereWarning,
                                self.problem_list.clone(),
                            );
                            return Err(RustConstructorError::SwitchFillResourceMismatch {
                                switch_name: format!("MessageBox{}Close", message_box.name),
                                fill_resource_name: message_box
                                    .close_switch_fill_resource_name
                                    .clone(),
                                fill_resource_type: message_box
                                    .close_switch_fill_resource_type
                                    .clone(),
                            });
                        }
                    };

                self.add_resource(
                    Switch::default()
                        .name(&format!("MessageBox{}Close", message_box.name))
                        .appearance(vec![
                            SwitchAppearance {
                                image_config: image_config
                                    .clone()
                                    .overlay_color(color[0], color[1], color[2], 0),
                                custom_rect_config: custom_rect_config
                                    .clone()
                                    .color(color[0], color[1], color[2], 0)
                                    .border_color(
                                        border_color[0],
                                        border_color[1],
                                        border_color[2],
                                        0,
                                    ),
                                text_config: TextConfig::default(),
                                texture: texture.clone(),
                                hint_text: String::new(),
                            },
                            SwitchAppearance {
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
                            SwitchAppearance {
                                image_config: image_config.clone().overlay_color(0, 0, 0, 0),
                                custom_rect_config: custom_rect_config
                                    .clone()
                                    .color(0, 0, 0, 0)
                                    .border_color(0, 0, 0, 0),
                                text_config: TextConfig::default(),
                                texture: texture.clone(),
                                hint_text: String::new(),
                            },
                            SwitchAppearance {
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
                        }])
                        .fill_resource_name(&format!(
                            "MessageBox{}Close",
                            message_box.close_switch_fill_resource_name
                        ))
                        .fill_resource_type(&message_box.close_switch_fill_resource_type)
                        .text_name(""),
                    safe_mode,
                )
                .unwrap();
            }
            _ => {}
        };
        self.rust_constructor_resource.push(Box::new(resource));
        Ok(())
    }

    /// 整合所有页面需要一次性处理的功能。
    pub fn page_data(
        &mut self,
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        // 更新帧数
        self.update_frame_stats(ctx);
        // 更新资源活跃状态。
        for rcr in &mut self.rust_constructor_resource {
            rcr.modify_active(false);
        }
        // 更新计时器
        self.update_timer();
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(&self.current_page, "PageData")
        {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: self.current_page.clone(),
                },
                SeverityLevel::MildWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::PageNotFound {
                page_name: self.current_page.clone(),
            });
        };
        let page_data = self
            .get_resource_mut::<PageData>(&self.current_page.clone(), "PageData")
            .unwrap()
            .unwrap();
        page_data.modify_active(true);
        if page_data.forced_update {
            ctx.request_repaint();
        };
        Ok(())
    }

    /// 切换页面。
    pub fn switch_page(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "PageData")
        {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            });
        };
        self.current_page = name.to_string();
        let pd = self
            .get_resource_mut::<PageData>(name, "PageData")
            .unwrap()
            .unwrap();
        pd.enter_page_updated = false;
        self.timer.start_time = self.timer.total_time;
        self.update_timer();
        Ok(())
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
    ) -> Result<Option<&T>, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if self.check_resource_exists(name, discern_type) {
            Ok(self
                .rust_constructor_resource
                .iter()
                .find(|resource| resource.name() == name && resource.expose_type() == discern_type)
                .and_then(|resource| resource.as_any().downcast_ref::<T>()))
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
    ) -> Result<Option<&mut T>, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if self.check_resource_exists(name, discern_type) {
            Ok(self
                .rust_constructor_resource
                .iter_mut()
                .find(|resource| resource.name() == name && resource.expose_type() == discern_type)
                .and_then(|resource| resource.as_any_mut().downcast_mut::<T>()))
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

    /// 输出字体资源。
    pub fn font(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<FontDefinitions, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Font")
        {
            self.problem_report_custom(
                RustConstructorError::FontNotFound {
                    font_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::FontNotFound {
                font_name: name.to_string(),
            });
        };
        let f = self
            .get_resource_mut::<Font>(name, "Font")
            .unwrap()
            .unwrap();
        f.modify_active(true);
        Ok(f.font_definitions.clone())
    }

    /// 将所有已添加到RC的字体资源添加到egui中。
    pub fn register_all_fonts(&mut self, ctx: &Context, safe_mode: Option<bool>) {
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
            if let Ok(font_def) = self.font(&font_name, safe_mode) {
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

    /// 处理错误。
    pub fn problem_processor(
        &self,
        problem_type: RustConstructorError,
        severity_level: SeverityLevel,
    ) -> (String, String) {
        let (problem, annotation) = match problem_type.clone() {
            RustConstructorError::ResourceNameRepetition { resource_name, resource_type } => (
                format!(
                    "Resource name repetition({:?}): {}({})",
                    severity_level, resource_name, resource_type
                ),
                "Please check whether the resource name is repeated.".to_string(),
            ),

            RustConstructorError::FontGetFailed { font_path } => (
                format!("Font get failed({:?}): {}", severity_level, font_path,),
                "Please check if the font file exists and the path is correct.".to_string(),
            ),
            RustConstructorError::FontNotFound { font_name } => (
                format!("Font not found({:?}): {}", severity_level, font_name,),
                "Please check whether the font has been added.".to_string(),
            ),
            RustConstructorError::ImageGetFailed { image_path } => (
                format!("Image get failed({:?}): {}", severity_level, image_path,),
                "Please check whether the image path is correct and whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageNotFound { image_name } => (
                format!("Image not found({:?}): {}", severity_level, image_name,),
                "Please check whether the image has been added.".to_string(),
            ),
            RustConstructorError::ImageTextureNotFound { image_texture_name } => (
                format!("Image texture not found({:?}): {}", severity_level, image_texture_name,),
                "Please check whether the image texture has been added.".to_string(),
            ),
            RustConstructorError::TextNotFound { text_name } => (
                format!("Text not found({:?}): {}", severity_level, text_name,),
                "Please check whether the text has been added.".to_string(),
            ),
            RustConstructorError::MessageBoxAlreadyExists { message_box_name } => (
                format!("Message box already exists({:?}): {}", severity_level, message_box_name,),
                "Please check whether the code for generating the message box has been accidentally called multiple times.".to_string(),
            ),
            RustConstructorError::MouseDetectorNotFound { mouse_detector_name } => (
                format!("Mouse detector not found({:?}): {}", severity_level, mouse_detector_name,),
                "Please check whether the mouse detector has been added.".to_string(),
            ),
            RustConstructorError::SplitTimeNotFound { split_time_name } => (
                format!("Split time not found({:?}): {}", severity_level, split_time_name,),
                "Please check whether the split time has been added.".to_string(),
            ),
            RustConstructorError::SwitchAppearanceMismatch {
                switch_name,
                differ,
            } => (
                format!(
                    "Switch appearance list's number of items is large / small {} more({:?}): {}",
                    differ, severity_level, switch_name
                ),
                "Please check whether the number of appearance list items matches the number of enabled animations.".to_string(),
            ),
            RustConstructorError::SwitchNotFound { switch_name } => (
                format!("Switch not found({:?}): {}", severity_level, switch_name,),
                "Please check whether the switch has been added.".to_string(),
            ),
            RustConstructorError::SwitchFillResourceMismatch { switch_name, fill_resource_name, fill_resource_type } => (
                format!("Switch fill resource mismatch({:?}): Resource {} of switch {} is not of type {}", severity_level, fill_resource_name, switch_name, fill_resource_type,),
                "Please check whether the imported fill resource is correctly typed.".to_string(),
            ),
            RustConstructorError::PageNotFound { page_name } => (
                format!("Page not found({:?}): {}", severity_level, page_name,),
                "Please check whether the page has been added.".to_string(),
            ),
            RustConstructorError::VariableNotFound { variable_name } => (
                format!("Variable not found({:?}): {}", severity_level, variable_name,),
                "Please check whether the variable has been added.".to_string(),
            ),
            RustConstructorError::VariableTypeMismatch { variable_name } => (
                format!("Variable type mismatch({:?}): {}", severity_level, variable_name,),
                "Please check whether the generics used for matching are filled correctly.".to_string(),
            ),
            RustConstructorError::RectNotFound { rect_name } => (
                format!("Rect not found({:?}): {}", severity_level, rect_name,),
                "Please check whether the rect has been added.".to_string(),
            ),
            RustConstructorError::ResourcePanelNotFound { resource_panel_name } => (
                format!("Resource panel not found({:?}): {}", severity_level, resource_panel_name,),
                "Please check whether the resource panel has been added.".to_string(),
            ),
            RustConstructorError::ResourceNotFound {
                resource_name,
                resource_type,
            } => (
                format!(
                    "Resource not found({:?}): {}(\"{}\")",
                    severity_level, resource_type, resource_name,
                ),
                "Please check whether the resource has been added.".to_string(),
            ),
            RustConstructorError::ResourceUntitled { resource_type } => (
                format!("Resource untitled({:?}): {}", severity_level, resource_type,),
                "Resources must have names.".to_string(),
            ),
            RustConstructorError::CustomError { error_name, error_message, error_annotation } => (
                format!("Custom error({}, {:?}): {}", error_name, severity_level, error_message),
                error_annotation
            )
        };
        // 如果处于严格模式下，则直接崩溃！
        if self.strict_mode {
            panic!(
                "Rust Constructor Error({:?}): {}\nnote: {}",
                problem_type, problem, annotation
            );
        };
        (problem, annotation)
    }

    /// 发生问题时推送报告。
    pub fn problem_report(
        &mut self,
        problem_type: RustConstructorError,
        severity_level: SeverityLevel,
    ) {
        let (problem, annotation) = self.problem_processor(problem_type.clone(), severity_level);
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
        let (problem, annotation) = self.problem_processor(problem_type.clone(), severity_level);
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

    /// 处理最基本的位置计算。
    pub fn position_size_processor(
        &self,
        position_config: PositionConfig,
        ctx: &Context,
    ) -> [[f32; 2]; 2] {
        let mut position = [0_f32, 0_f32];
        let mut size = [0_f32, 0_f32];
        size[0] = match position_config.x_size_grid[0] {
            0 => position_config.origin_size[0],
            _ => {
                (ctx.available_rect().width() as f64 / position_config.x_size_grid[1] as f64
                    * position_config.x_size_grid[0] as f64) as f32
                    + position_config.origin_size[0]
            }
        };
        size[1] = match position_config.y_size_grid[0] {
            0 => position_config.origin_size[1],
            _ => {
                (ctx.available_rect().height() as f64 / position_config.y_size_grid[1] as f64
                    * position_config.y_size_grid[0] as f64) as f32
                    + position_config.origin_size[1]
            }
        };
        position[0] = match position_config.x_location_grid[1] {
            0 => position_config.origin_position[0],
            _ => {
                (ctx.available_rect().width() as f64 / position_config.x_location_grid[1] as f64
                    * position_config.x_location_grid[0] as f64) as f32
                    + position_config.origin_position[0]
            }
        };
        position[1] = match position_config.y_location_grid[1] {
            0 => position_config.origin_position[1],
            _ => {
                (ctx.available_rect().height() as f64 / position_config.y_location_grid[1] as f64
                    * position_config.y_location_grid[0] as f64) as f32
                    + position_config.origin_position[1]
            }
        };
        match position_config.center_display.0 {
            HorizontalAlign::Left => {}
            HorizontalAlign::Center => position[0] -= size[0] / 2.0,
            HorizontalAlign::Right => position[0] -= size[0],
        };
        match position_config.center_display.1 {
            VerticalAlign::Top => {}
            VerticalAlign::Center => position[1] -= size[1] / 2.0,
            VerticalAlign::Bottom => position[1] -= size[1],
        };
        position[0] += position_config.offset[0];
        position[1] += position_config.offset[1];
        [position, size]
    }

    /// 检查页面是否已完成首次加载。
    pub fn check_updated(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<bool, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "PageData")
        {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            });
        };
        let pd = self
            .get_resource::<PageData>(name, "PageData")
            .unwrap()
            .unwrap()
            .clone();
        if !pd.change_page_updated {
            self.new_page_update(name, safe_mode).unwrap();
        };
        Ok(pd.change_page_updated)
    }

    /// 检查页面是否已完成加载。
    pub fn check_enter_updated(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<bool, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "PageData")
        {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            });
        };
        let pd = self
            .get_resource_mut::<PageData>(name, "PageData")
            .unwrap()
            .unwrap();
        let return_value = pd.enter_page_updated;
        pd.enter_page_updated = true;
        Ok(return_value)
    }

    /// 进入新页面时的更新。
    pub fn new_page_update(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "PageData")
        {
            self.problem_report_custom(
                RustConstructorError::PageNotFound {
                    page_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::PageNotFound {
                page_name: name.to_string(),
            });
        }
        self.timer.start_time = self.timer.total_time;
        self.update_timer();
        let pd = self
            .get_resource_mut::<PageData>(name, "PageData")
            .unwrap()
            .unwrap();
        pd.change_page_updated = true;
        Ok(())
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

    /// 重置分段时间。
    pub fn reset_split_time(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "SplitTime")
        {
            self.problem_report_custom(
                RustConstructorError::SplitTimeNotFound {
                    split_time_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            });
        };
        let new_time = [self.timer.now_time, self.timer.total_time];
        let split_time = self
            .get_resource_mut::<SplitTime>(name, "SplitTime")
            .unwrap()
            .unwrap();
        split_time.time = new_time;
        Ok(())
    }

    /// 输出分段时间。
    pub fn split_time(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<[f32; 2], RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "SplitTime")
        {
            self.problem_report_custom(
                RustConstructorError::SplitTimeNotFound {
                    split_time_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::SplitTimeNotFound {
                split_time_name: name.to_string(),
            });
        };
        let split_time = self
            .get_resource_mut::<SplitTime>(name, "SplitTime")
            .unwrap()
            .unwrap();
        split_time.modify_active(true);
        Ok(split_time.time)
    }

    /// 更新计时器。
    pub fn update_timer(&mut self) {
        let elapsed = self.timer.timer.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();
        self.timer.total_time = seconds as f32 + milliseconds as f32 / 1000.0;
        self.timer.now_time = self.timer.total_time - self.timer.start_time
    }

    /// 显示矩形资源。
    pub fn custom_rect(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "CustomRect")
        {
            self.problem_report_custom(
                RustConstructorError::RectNotFound {
                    rect_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::RectNotFound {
                rect_name: name.to_string(),
            });
        };
        let mut custom_rect = self
            .get_resource_mut::<CustomRect>(name, "CustomRect")
            .unwrap()
            .unwrap()
            .clone();
        custom_rect.modify_active(true);
        [custom_rect.position, custom_rect.size] =
            self.position_size_processor(PositionConfig::from_custom_rect(&custom_rect), ctx);
        if custom_rect.clip_rect.is_some() {
            let [min, size] =
                self.position_size_processor(custom_rect.clip_rect.clone().unwrap(), ctx);
            ui.set_clip_rect(Rect::from_min_size(min.into(), size.into()));
        };
        ui.painter().rect(
            Rect::from_min_max(
                Pos2::new(custom_rect.position[0], custom_rect.position[1]),
                Pos2::new(
                    custom_rect.position[0] + custom_rect.size[0],
                    custom_rect.position[1] + custom_rect.size[1],
                ),
            ),
            custom_rect.rounding,
            Color32::from_rgba_unmultiplied(
                custom_rect.color[0],
                custom_rect.color[1],
                custom_rect.color[2],
                custom_rect.color[3],
            ),
            Stroke {
                width: custom_rect.border_width,
                color: Color32::from_rgba_unmultiplied(
                    custom_rect.border_color[0],
                    custom_rect.border_color[1],
                    custom_rect.border_color[2],
                    custom_rect.border_color[3],
                ),
            },
            StrokeKind::Inside,
        );
        if custom_rect.clip_rect.is_some() {
            ui.set_clip_rect(Rect::from_min_size(
                [0_f32, 0_f32].into(),
                [ctx.available_rect().width(), ctx.available_rect().height()].into(),
            ));
        };
        self.replace_resource(name, "CustomRect", custom_rect)
            .unwrap();
        Ok(())
    }

    /// 显示文本资源。
    pub fn text(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Text")
        {
            self.problem_report_custom(
                RustConstructorError::TextNotFound {
                    text_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::TextNotFound {
                text_name: name.to_string(),
            });
        };
        let mut text = self
            .get_resource::<Text>(name, "Text")
            .unwrap()
            .unwrap()
            .clone();
        text.modify_active(true);
        [text.position, text.size] =
            self.position_size_processor(PositionConfig::from_text(&text), ctx);
        let display_content = if text.truncate {
            let original_galley = ui.fonts_mut(|f| {
                f.layout(
                    text.content.to_string(),
                    FontId::proportional(text.font_size),
                    Color32::default(),
                    text.size[0],
                )
            });

            let mut truncated = text.content.to_string();
            let mut ellipsis = "";
            if original_galley.size().y > text.size[1] {
                // 如果超出，逐步缩短文本直到加上省略号后能放下
                ellipsis = "...";

                while !truncated.is_empty() {
                    let test_text = format!("{}{}", truncated, ellipsis);
                    let test_galley = ui.fonts_mut(|f| {
                        f.layout(
                            test_text.clone(),
                            FontId::proportional(text.font_size),
                            Color32::default(),
                            text.size[0],
                        )
                    });

                    if test_galley.size().y <= text.size[1] {
                        break;
                    }

                    // 移除最后一个字符
                    truncated.pop();
                }
            };
            format!("{}{}", truncated, ellipsis)
        } else {
            text.content.to_string()
        };
        // 计算文本大小
        let galley: Arc<Galley> = ui.fonts_mut(|f| {
            f.layout(
                display_content.to_string(),
                if !text.font.is_empty() {
                    if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                        && !self.check_resource_exists(&text.font.clone(), "Font")
                    {
                        self.problem_report_custom(
                            RustConstructorError::FontNotFound {
                                font_name: text.font.clone(),
                            },
                            SeverityLevel::MildWarning,
                            self.problem_list.clone(),
                        );
                        FontId::new(text.font_size, FontFamily::Name(text.font.clone().into()))
                    } else {
                        FontId::proportional(text.font_size)
                    }
                } else {
                    FontId::proportional(text.font_size)
                },
                Color32::from_rgba_unmultiplied(
                    text.color[0],
                    text.color[1],
                    text.color[2],
                    text.color[3],
                ),
                text.size[0],
            )
        });
        text.actual_size = [galley.size().x, galley.size().y];
        // 使用绝对定位放置文本
        let rect = Rect::from_min_size(text.position.into(), text.actual_size.into());
        // 绘制背景颜色
        ui.painter().rect_filled(
            rect,
            text.background_rounding,
            Color32::from_rgba_unmultiplied(
                text.background_color[0],
                text.background_color[1],
                text.background_color[2],
                text.background_color[3],
            ),
        );

        if text.clip_rect.is_some() {
            let [min, size] = self.position_size_processor(text.clip_rect.clone().unwrap(), ctx);
            ui.set_clip_rect(Rect::from_min_size(min.into(), size.into()));
        };

        // 绘制文本
        ui.painter().galley(
            text.position.into(),
            galley.clone(),
            Color32::from_rgba_unmultiplied(
                text.color[0],
                text.color[1],
                text.color[2],
                text.color[3], // 应用透明度
            ),
        );

        // 查找超链接索引值
        if text.last_frame_content != display_content {
            text.hyperlink_index.clear();

            // 创建字节索引到字符索引的映射
            let byte_to_char_map: std::collections::HashMap<usize, usize> = display_content
                .char_indices()
                .enumerate()
                .map(|(char_idx, (byte_idx, _))| (byte_idx, char_idx))
                .collect();

            for (hyperlink_text, method) in &text.hyperlink_text {
                let matches: Vec<(usize, &str)> =
                    display_content.match_indices(hyperlink_text).collect();
                let text_char_count = hyperlink_text.chars().count();

                if let HyperlinkSelectMethod::All(url) = method {
                    for (byte_index, _) in matches {
                        if let Some(&start_char_index) = byte_to_char_map.get(&byte_index) {
                            text.hyperlink_index.push((
                                start_char_index,
                                start_char_index + text_char_count,
                                url.clone(),
                            ));
                        };
                    }
                } else if let HyperlinkSelectMethod::Segment(list) = method {
                    for (index, url) in list {
                        if *index >= matches.len() {
                            continue;
                        };
                        let (byte_index, _) = matches[*index];
                        if let Some(&start_char_index) = byte_to_char_map.get(&byte_index) {
                            text.hyperlink_index.push((
                                start_char_index,
                                start_char_index + text_char_count,
                                url.clone(),
                            ));
                        };
                    }
                };
            }
        };

        // 绘制超链接
        for (start, end, _) in &text.hyperlink_index {
            // 获取超链接文本的范围
            let start_cursor = galley.pos_from_cursor(CCursor::new(*start));
            let end_cursor = galley.pos_from_cursor(CCursor::new(*end));

            let start_pos = start_cursor.left_top();
            let end_pos = end_cursor.right_top();
            // 绘制超链接下划线
            // 检查超链接是否跨行
            if start_cursor.min.y == end_cursor.min.y {
                // 单行超链接
                let underline_y = text.position[1]
                    + start_pos.y
                    + galley.rows.first().map_or(14.0, |row| row.height())
                    - 2.0;

                // 绘制下划线
                let color = Color32::from_rgba_unmultiplied(
                    text.color[0],
                    text.color[1],
                    text.color[2],
                    text.color[3],
                );

                ui.painter().line_segment(
                    [
                        Pos2::new(text.position[0] + start_pos.x, underline_y),
                        Pos2::new(text.position[0] + end_pos.x, underline_y),
                    ],
                    Stroke::new(text.font_size / 10_f32, color),
                );
            } else {
                // 多行超链接
                let row_height = galley.rows.first().map_or(14.0, |row| row.height()); // 默认行高14.0

                // 计算起始行和结束行的索引
                let start_row = (start_pos.y / row_height).round() as usize;
                let end_row = (end_pos.y / row_height).round() as usize;

                for row in start_row..=end_row {
                    let row_y = text.position[1] + row as f32 * row_height + row_height - 2.0; // 行底部稍微上移一点绘制下划线

                    // 获取当前行的矩形范围
                    if let Some(current_row) = galley.rows.get(row) {
                        let row_rect = current_row.rect();

                        let color = Color32::from_rgba_unmultiplied(
                            text.color[0],
                            text.color[1],
                            text.color[2],
                            text.color[3],
                        );

                        if row == start_row {
                            // 第一行从文本开始位置到行尾
                            ui.painter().line_segment(
                                [
                                    Pos2::new(text.position[0] + start_pos.x, row_y),
                                    Pos2::new(text.position[0] + row_rect.max.x, row_y),
                                ],
                                Stroke::new(text.font_size / 10_f32, color),
                            );
                        } else if row == end_row {
                            // 最后一行从行首到文本结束位置
                            ui.painter().line_segment(
                                [
                                    Pos2::new(text.position[0] + row_rect.min.x, row_y),
                                    Pos2::new(text.position[0] + end_pos.x, row_y),
                                ],
                                Stroke::new(text.font_size / 10_f32, color),
                            );
                        } else {
                            // 中间整行下划线
                            ui.painter().line_segment(
                                [
                                    Pos2::new(text.position[0] + row_rect.min.x, row_y),
                                    Pos2::new(text.position[0] + row_rect.max.x, row_y),
                                ],
                                Stroke::new(text.font_size / 10_f32, color),
                            );
                        };
                    };
                }
            };
        }

        if text.selectable {
            if !self.check_resource_exists(&text.name, "MouseDetector") {
                self.add_resource(
                    MouseDetector::default()
                        .name(&text.name)
                        .from_position_config(PositionConfig::from_text(&text))
                        .offset(-20_f32, -5_f32)
                        .origin_size(text.actual_size[0] + 40_f32, text.actual_size[1] + 10_f32),
                    safe_mode,
                )
                .unwrap();
            } else {
                self.replace_resource(
                    &text.name,
                    "MouseDetector",
                    MouseDetector::default()
                        .name(&text.name)
                        .from_position_config(PositionConfig::from_text(&text))
                        .offset(-20_f32, -5_f32)
                        .origin_size(text.actual_size[0] + 40_f32, text.actual_size[1] + 10_f32),
                )
                .unwrap();
            };

            // 处理选择逻辑
            let cursor_at_pointer = |pointer_pos: Vec2| -> usize {
                let relative_pos = pointer_pos - text.position.into();
                let cursor = galley.cursor_from_pos(relative_pos);
                cursor.index
            };

            self.mouse_detector(&text.name, ui, ctx, MouseDetectorLevel::Default, safe_mode)
                .unwrap();
            let fullscreen_detect_result = ui.input(|i| i.pointer.clone());
            let detect_result = self
                .check_mouse_detect_result(&text.name, safe_mode)
                .unwrap();
            if !detect_result.clicked
                && (fullscreen_detect_result.any_click() || fullscreen_detect_result.any_pressed())
            {
                text.selection = None;
            };

            if (detect_result.clicked || detect_result.drag_started.unwrap())
                && let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos())
            {
                let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                text.selection = Some((cursor, cursor));
            };

            if detect_result.dragged.unwrap()
                && text.selection.is_some()
                && let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos())
            {
                let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                if let Some((start, _)) = text.selection {
                    text.selection = Some((start, cursor));
                };
            };

            if text.selection.is_some()
                && ui.input(|input| input.key_released(Key::A) && input.modifiers.command)
            {
                text.selection = Some((0, display_content.chars().count()));
            };

            // 处理复制操作
            let copy_triggered = ui.input(|input| {
                let c_released = input.key_released(Key::C);
                let cmd_pressed = input.modifiers.command;
                c_released && cmd_pressed
            });
            if copy_triggered && let Some((start, end)) = text.selection {
                let (start, end) = (start.min(end), start.max(end));
                let chars: Vec<char> = display_content.chars().collect();
                if start <= chars.len() && end <= chars.len() && start < end {
                    let selected_text: String = chars[start..end].iter().collect();
                    ui.ctx().copy_text(selected_text);
                };
            };

            // 绘制选择区域背景
            if let Some((start, end)) = text.selection {
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
                        let rows = &galley.rows;
                        let row_height = if !rows.is_empty() {
                            // 获取实际行的高度
                            if let Some(row) = rows.first() {
                                row.height()
                            } else {
                                text.actual_size[1] / display_content.lines().count() as f32
                            }
                        } else {
                            text.actual_size[1] / display_content.lines().count() as f32
                        };

                        let selection_rect = Rect::from_min_max(
                            Pos2::new(
                                text.position[0] + start_pos.x,
                                text.position[1] + start_pos.y,
                            ),
                            Pos2::new(
                                text.position[0] + end_pos.x,
                                text.position[1] + start_pos.y + row_height,
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
                            text.actual_size[1] / display_content.lines().count() as f32
                        };

                        // 计算选择的上下边界
                        let selection_top = text.position[1] + start_pos.y.min(end_pos.y);
                        let selection_bottom = text.position[1] + start_pos.y.max(end_pos.y);

                        // 确定起始行和结束行的索引
                        let start_row_index = (start_pos.y / row_height).floor() as usize;
                        let end_row_index = (end_pos.y / row_height).floor() as usize;
                        let (first_row_index, last_row_index) = if start_row_index <= end_row_index
                        {
                            (start_row_index, end_row_index)
                        } else {
                            (end_row_index, start_row_index)
                        };

                        for (i, row) in rows.iter().enumerate() {
                            let row_y = text.position[1] + row_height * i as f32;
                            let row_bottom = row_y + row_height;
                            // 检查当前行是否与选择区域相交
                            if row_bottom > selection_top && row_y <= selection_bottom {
                                let left = if i == first_row_index {
                                    // 首行 - 从选择开始位置开始
                                    text.position[0] + start_pos.x
                                } else {
                                    // 非首行 - 从行首开始
                                    text.position[0] + row.rect().min.x
                                };

                                let right = if i == last_row_index {
                                    // 尾行 - 到选择结束位置结束
                                    text.position[0] + end_pos.x
                                } else {
                                    // 非尾行 - 到行尾结束
                                    text.position[0] + row.rect().max.x
                                };

                                let selection_rect = Rect::from_min_max(
                                    Pos2::new(left, row_y),
                                    Pos2::new(right, row_bottom),
                                );

                                // 确保矩形有效
                                if selection_rect.width() > 0.0 && selection_rect.height() > 0.0 {
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
        for (start, end, url) in &text.hyperlink_index {
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
                    Pos2::new(
                        text.position[0] + start_pos.x,
                        text.position[1] + start_pos.y,
                    ),
                    Pos2::new(
                        text.position[0] + end_pos.x,
                        text.position[1] + start_pos.y + row_height,
                    ),
                );
                vec![ui.interact(
                    link_rect,
                    egui::Id::new(format!("link_{}_{}_{}", text.name, start, end)),
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
                        let row_y = text.position[1] + row as f32 * row_height;

                        let link_rect = if row == start_row {
                            // 第一行从文本开始位置到行尾
                            Rect::from_min_max(
                                Pos2::new(text.position[0] + start_pos.x, row_y),
                                Pos2::new(text.position[0] + row_rect.max.x, row_y + row_height),
                            )
                        } else if row == end_row {
                            // 最后一行从行首到文本结束位置
                            Rect::from_min_max(
                                Pos2::new(text.position[0] + row_rect.min.x, row_y),
                                Pos2::new(text.position[0] + end_pos.x, row_y + row_height),
                            )
                        } else {
                            // 中间整行
                            Rect::from_min_max(
                                Pos2::new(text.position[0] + row_rect.min.x, row_y),
                                Pos2::new(text.position[0] + row_rect.max.x, row_y + row_height),
                            )
                        };

                        responses.push(ui.interact(
                            link_rect,
                            Id::new(format!("link_{}_{}_{}_row_{}", text.name, start, end, row)),
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
                    text.selection = None;
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                        let relative_pos =
                            pointer_pos - <[f32; 2] as Into<Pos2>>::into(text.position);
                        let cursor = galley.cursor_from_pos(relative_pos);
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
                    let relative_pos = pointer_pos - <[f32; 2] as Into<Pos2>>::into(text.position);
                    let cursor = galley.cursor_from_pos(relative_pos);
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
                        Pos2::new(
                            text.position[0] + start_pos.x,
                            text.position[1] + start_pos.y,
                        ),
                        Pos2::new(
                            text.position[0] + end_pos.x,
                            text.position[1]
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
                                        text.position[0] + start_pos.x,
                                        text.position[1] + row as f32 * row_height,
                                    ),
                                    Pos2::new(
                                        text.position[0] + row_rect.max.x,
                                        text.position[1] + row as f32 * row_height + row_height,
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
                                        text.position[0] + row_rect.min.x,
                                        text.position[1] + row as f32 * row_height,
                                    ),
                                    Pos2::new(
                                        text.position[0] + end_pos.x,
                                        text.position[1] + row as f32 * row_height + row_height,
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
                                        text.position[0] + row_rect.min.x,
                                        text.position[1] + row as f32 * row_height,
                                    ),
                                    Pos2::new(
                                        text.position[0] + row_rect.max.x,
                                        text.position[1] + row as f32 * row_height + row_height,
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
        text.last_frame_content = display_content.clone();
        if text.clip_rect.is_some() {
            ui.set_clip_rect(Rect::from_min_size(
                [0_f32, 0_f32].into(),
                [ctx.available_rect().width(), ctx.available_rect().height()].into(),
            ));
        };
        self.replace_resource(name, "Text", text).unwrap();
        Ok(())
    }

    /// 修改变量资源。
    pub fn modify_var<T: Debug + 'static>(
        &mut self,
        name: &str,
        value: Option<T>,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Variable")
        {
            self.problem_report_custom(
                RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::VariableNotFound {
                variable_name: name.to_string(),
            });
        };
        let v = self
            .get_resource_mut::<Variable<T>>(name, "Variable")
            .unwrap()
            .unwrap();
        v.value = value;
        Ok(())
    }

    /// 取出变量。
    pub fn var<T: Debug + 'static>(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<Option<&T>, RustConstructorError> {
        if safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode {
            if !self.check_resource_exists(name, "Variable") {
                self.problem_report_custom(
                    RustConstructorError::VariableNotFound {
                        variable_name: name.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::VariableNotFound {
                    variable_name: name.to_string(),
                });
            };
            if self
                .get_resource::<Variable<T>>(name, "Variable")
                .unwrap()
                .is_none()
            {
                self.problem_report_custom(
                    RustConstructorError::VariableTypeMismatch {
                        variable_name: name.to_string(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::VariableTypeMismatch {
                    variable_name: name.to_string(),
                });
            };
        };
        let v = self
            .get_resource_mut::<Variable<T>>(name, "Variable")
            .unwrap()
            .unwrap();
        v.modify_active(true);
        Ok(v.value.as_ref())
    }

    /// 输出图片纹理。
    pub fn image_texture(
        &mut self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<Option<DebugTextureHandle>, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "ImageTexture")
        {
            self.problem_report_custom(
                RustConstructorError::ImageNotFound {
                    image_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ImageNotFound {
                image_name: name.to_string(),
            });
        };
        let image_texture = self
            .get_resource_mut::<ImageTexture>(name, "ImageTexture")
            .unwrap()
            .unwrap();
        image_texture.modify_active(true);
        Ok(image_texture.texture.clone())
    }

    /// 重置图片纹理。
    pub fn reset_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "ImageTexture")
        {
            self.problem_report_custom(
                RustConstructorError::ImageTextureNotFound {
                    image_texture_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ImageTextureNotFound {
                image_texture_name: name.to_string(),
            });
        };
        let image_texture = self
            .get_resource_mut::<ImageTexture>(name, "ImageTexture")
            .unwrap()
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
            let texture = ctx.load_texture(
                image_texture.name.clone(),
                color_image,
                TextureOptions::LINEAR,
            );
            image_texture.texture = Some(DebugTextureHandle::new(texture));
            image_texture.cite_path = path.to_string();
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
    }

    /// 显示图片资源。
    pub fn image(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Image")
        {
            self.problem_report_custom(
                RustConstructorError::ImageNotFound {
                    image_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ImageNotFound {
                image_name: name.to_string(),
            });
        };
        let mut image = self
            .get_resource_mut::<Image>(name, "Image")
            .unwrap()
            .unwrap()
            .clone();
        if image.cite_texture != image.last_frame_cite_texture {
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&image.cite_texture, "ImageTexture")
            {
                self.problem_report_custom(
                    RustConstructorError::ImageTextureNotFound {
                        image_texture_name: image.cite_texture.clone(),
                    },
                    SeverityLevel::MildWarning,
                    self.problem_list.clone(),
                );
            } else {
                let it = self
                    .get_resource::<ImageTexture>(&image.cite_texture, "ImageTexture")
                    .unwrap()
                    .unwrap();
                image.texture = it.texture.clone();
            };
        };
        image.modify_active(true);
        [image.position, image.size] =
            self.position_size_processor(PositionConfig::from_image(&image), ctx);
        if image.clip_rect.is_some() {
            let [min, size] = self.position_size_processor(image.clip_rect.clone().unwrap(), ctx);
            ui.set_clip_rect(Rect::from_min_size(min.into(), size.into()));
        };
        if let Some(texture) = &image.texture {
            let rect = Rect::from_min_size(
                Pos2::new(image.position[0], image.position[1]),
                Vec2::new(image.size[0], image.size[1]),
            );

            // 直接绘制图片
            egui::Image::new(ImageSource::Texture((&texture.0).into()))
                .tint(Color32::from_rgba_unmultiplied(
                    image.overlay_color[0],
                    image.overlay_color[1],
                    image.overlay_color[2],
                    // 将图片透明度与覆盖颜色透明度相乘
                    (image.alpha as f32 * image.overlay_color[3] as f32 / 255.0) as u8,
                ))
                .bg_fill(Color32::from_rgba_unmultiplied(
                    image.background_color[0],
                    image.background_color[1],
                    image.background_color[2],
                    image.background_color[3],
                ))
                .rotate(
                    image.rotate_angle,
                    [
                        image.rotate_center[0] / image.size[0],
                        image.rotate_center[1] / image.size[1],
                    ]
                    .into(),
                )
                .paint_at(ui, rect)
        };
        image.last_frame_cite_texture = image.cite_texture.clone();
        if image.clip_rect.is_some() {
            ui.set_clip_rect(Rect::from_min_size(
                [0_f32, 0_f32].into(),
                [ctx.available_rect().width(), ctx.available_rect().height()].into(),
            ));
        };
        self.replace_resource(name, "Image", image).unwrap();
        Ok(())
    }

    /// 处理所有已添加的消息框资源。
    pub fn message_box(&mut self, ctx: &Context, ui: &mut Ui, safe_mode: Option<bool>) {
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
            if safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode {
                if !self.check_resource_exists(&mb.image_name, "Image") {
                    self.problem_report_custom(
                        RustConstructorError::ImageNotFound {
                            image_name: mb.image_name,
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !self.check_resource_exists(&format!("MessageBox{}", mb.name), "CustomRect") {
                    self.problem_report_custom(
                        RustConstructorError::RectNotFound {
                            rect_name: format!("MessageBox{}", mb.name),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !self.check_resource_exists(&mb.title_name, "Text") {
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: mb.title_name,
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !self.check_resource_exists(&mb.content_name, "Text") {
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: mb.content_name,
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !self.check_resource_exists(&format!("MessageBox{}Close", mb.name), "Switch") {
                    self.problem_report_custom(
                        RustConstructorError::SwitchNotFound {
                            switch_name: format!("MessageBox{}Close", mb.name),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !self
                    .check_resource_exists(&format!("MessageBox{}Animation", mb.name), "SplitTime")
                {
                    self.problem_report_custom(
                        RustConstructorError::SplitTimeNotFound {
                            split_time_name: format!("MessageBox{}Animation", mb.name),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
                if !mb.keep_existing
                    && !self.check_resource_exists(&format!("MessageBox{}", mb.name), "SplitTime")
                {
                    self.problem_report_custom(
                        RustConstructorError::SplitTimeNotFound {
                            split_time_name: format!("MessageBox{}", mb.name),
                        },
                        SeverityLevel::SevereWarning,
                        self.problem_list.clone(),
                    );
                    continue;
                };
            };
            let mut im1 = self
                .get_resource::<Image>(&mb.image_name, "Image")
                .unwrap()
                .unwrap()
                .clone();
            let mut cr = self
                .get_resource::<CustomRect>(&format!("MessageBox{}", mb.name), "CustomRect")
                .unwrap()
                .unwrap()
                .clone();
            let mut t1 = self
                .get_resource::<Text>(&mb.title_name, "Text")
                .unwrap()
                .unwrap()
                .clone();
            let mut t2 = self
                .get_resource::<Text>(&mb.content_name, "Text")
                .unwrap()
                .unwrap()
                .clone();
            let mut s = self
                .get_resource::<Switch>(&format!("MessageBox{}Close", mb.name), "Switch")
                .unwrap()
                .unwrap()
                .clone();
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&s.fill_resource_name, &s.fill_resource_type)
            {
                self.problem_report_custom(
                    RustConstructorError::ResourceNotFound {
                        resource_name: s.fill_resource_name,
                        resource_type: s.fill_resource_type,
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                continue;
            };
            let fr: Box<dyn BasicFrontResource> = match s.fill_resource_type.as_str() {
                "Image" => Box::new(
                    self.get_resource::<Image>(&s.fill_resource_name, "Image")
                        .unwrap()
                        .unwrap()
                        .clone(),
                ),
                "CustomRect" => Box::new(
                    self.get_resource::<CustomRect>(&s.fill_resource_name, "CustomRect")
                        .unwrap()
                        .unwrap()
                        .clone(),
                ),
                _ => {
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
            mb.modify_active(true);
            if mb.size[1] < t1.actual_size[1] + t2.actual_size[1] + 10_f32 {
                mb.size[1] = t1.actual_size[1] + t2.actual_size[1] + 10_f32;
                cr.origin_size[1] = mb.size[1];
                im1.origin_size = [mb.size[1] - 15_f32, mb.size[1] - 15_f32];
                t1.origin_size[0] = mb.size[0] - mb.size[1] + 5_f32;
                t2.origin_size[0] = mb.size[0] - mb.size[1] + 5_f32;
            };
            if self.timer.total_time
                - self
                    .split_time(&format!("MessageBox{}Animation", mb.name), safe_mode)
                    .unwrap()[1]
                >= self.tick_interval
            {
                self.reset_split_time(&format!("MessageBox{}Animation", mb.name), safe_mode)
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
                                self.reset_split_time(&format!("MessageBox{}", mb.name), safe_mode)
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
                im1.origin_position[0] + im1.origin_size[0] + 5_f32,
                cr.origin_position[1] + 5_f32,
            ];
            t2.origin_position = [
                im1.origin_position[0] + im1.origin_size[0] + 5_f32,
                t1.origin_position[1] + t1.actual_size[1] + 5_f32,
            ];
            for sd in &mut s.appearance {
                sd.image_config.origin_position = cr.position;
                sd.custom_rect_config.origin_position = cr.position;
            }
            if !mb.keep_existing
                && self.timer.total_time
                    - self
                        .split_time(&format!("MessageBox{}", mb.name), safe_mode)
                        .unwrap()[1]
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
                        x: fr.display_position()[0],
                        y: fr.display_position()[1],
                    },
                    Vec2 {
                        x: cr.origin_size[0] + 15_f32,
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
                _ => {
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
            self.custom_rect(&format!("MessageBox{}", mb.name), ui, ctx, safe_mode)
                .unwrap();
            self.image(&mb.image_name.clone(), ui, ctx, safe_mode)
                .unwrap();
            self.text(&t1.name.clone(), ui, ctx, safe_mode).unwrap();
            self.text(&t2.name.clone(), ui, ctx, safe_mode).unwrap();
            self.switch(
                &format!("MessageBox{}Close", mb.name),
                ui,
                ctx,
                s.state == 0 && mb.exist,
                safe_mode,
            )
            .unwrap();
            let switch_data = self
                .check_switch_data(&format!("MessageBox{}Close", mb.name), safe_mode)
                .unwrap();
            if switch_data.last_time_clicked_index == 0
                && switch_data.state == 1
                && switch_data.switched
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
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "MouseDetector" && x.name() == t1.name)
                        .unwrap(),
                );
                self.rust_constructor_resource.remove(
                    self.rust_constructor_resource
                        .iter()
                        .position(|x| x.expose_type() == "MouseDetector" && x.name() == t2.name)
                        .unwrap(),
                );
            } else {
                offset += mb.size[1] + 15_f32;
            };
        }
    }

    /// 显示开关资源。
    pub fn switch(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        enable: bool,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        let mut appearance_count = 0;
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Switch")
        {
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
        let mut s = self
            .get_resource::<Switch>(name, "Switch")
            .unwrap()
            .unwrap()
            .clone();
        s.switched = false;
        if safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode {
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
                    self.add_resource(
                        SplitTime::default().name(&format!("{}StartHoverTime", s.name)),
                        safe_mode,
                    )
                    .unwrap();
                };
                if !self.check_resource_exists(&format!("{}HintFadeAnimation", s.name), "SplitTime")
                {
                    self.problem_report_custom(
                        RustConstructorError::SplitTimeNotFound {
                            split_time_name: format!("{}HintFadeAnimation", s.name),
                        },
                        SeverityLevel::MildWarning,
                        self.problem_list.clone(),
                    );
                    self.add_resource(
                        SplitTime::default().name(&format!("{}HintFadeAnimation", s.name)),
                        safe_mode,
                    )
                    .unwrap();
                };
                if !self.check_resource_exists(&s.hint_text_name, "Text") {
                    self.problem_report_custom(
                        RustConstructorError::TextNotFound {
                            text_name: s.hint_text_name.clone(),
                        },
                        SeverityLevel::MildWarning,
                        self.problem_list.clone(),
                    );
                    self.add_resource(
                        Text::default()
                            .name(&s.hint_text_name)
                            .content("")
                            .origin_position(0_f32, 0_f32)
                            .font_size(25_f32)
                            .origin_size(300_f32, 0_f32)
                            .background_rounding(10_f32)
                            .color(255, 255, 255, 0)
                            .background_color(0, 0, 0, 255)
                            .center_display(HorizontalAlign::Left, VerticalAlign::Top)
                            .selectable(false),
                        safe_mode,
                    )
                    .unwrap();
                };
            };
        };
        let fr: Box<dyn BasicFrontResource> = match &*s.fill_resource_type {
            "Image" => Box::new(
                self.get_resource::<Image>(&s.fill_resource_name.clone(), &s.fill_resource_type)
                    .unwrap()
                    .unwrap()
                    .clone(),
            ),
            "CustomRect" => Box::new(
                self.get_resource::<CustomRect>(
                    &s.fill_resource_name.clone(),
                    &s.fill_resource_type,
                )
                .unwrap()
                .unwrap()
                .clone(),
            ),
            _ => {
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
        s.modify_active(true);
        let rect = Rect::from_min_size(
            Pos2::new(fr.display_position()[0], fr.display_position()[1]),
            Vec2::new(fr.display_size()[0], fr.display_size()[1]),
        );
        let mut hovered = false;
        if enable {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                // 判断是否在矩形内
                if rect.contains(mouse_pos) {
                    if !s.hint_text_name.is_empty() {
                        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                            && !self.check_resource_exists(&s.hint_text_name, "Text")
                        {
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
                        let mut t = self
                            .get_resource::<Text>(&s.hint_text_name, "Text")
                            .unwrap()
                            .unwrap()
                            .clone();
                        if !s.last_time_hovered {
                            self.reset_split_time(&format!("{}StartHoverTime", s.name), safe_mode)
                                .unwrap();
                        } else if self.timer.total_time
                            - self
                                .split_time(&format!("{}StartHoverTime", s.name), safe_mode)
                                .unwrap()[1]
                            >= 2_f32
                            || t.color[3] != 0
                        {
                            t.color[3] = 255;
                            t.origin_position = [mouse_pos.x, mouse_pos.y];
                        };
                        t.center_display.0 =
                            if mouse_pos.x + t.actual_size[0] <= ctx.available_rect().width() {
                                HorizontalAlign::Left
                            } else {
                                HorizontalAlign::Right
                            };
                        t.center_display.1 =
                            if mouse_pos.y + t.actual_size[1] <= ctx.available_rect().height() {
                                VerticalAlign::Top
                            } else {
                                VerticalAlign::Bottom
                            };
                        self.replace_resource(&s.hint_text_name, "Text", t.clone())
                            .unwrap();
                    };
                    hovered = true;
                    let mut clicked = vec![];
                    for u in 0..s.click_method.len() as u32 {
                        clicked.push(ui.input(|i| {
                            i.pointer
                                .button_down(s.click_method[u as usize].click_method)
                        }));
                        if clicked[u as usize] {
                            s.last_time_clicked_index = u as usize;
                            break;
                        };
                    }
                    if clicked.iter().any(|x| *x) {
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
                            s.switched = true;
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
                self.reset_split_time(&format!("{}HintFadeAnimation", s.name), safe_mode)
                    .unwrap();
            };
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&s.hint_text_name, "Text")
            {
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
            let mut t = self
                .get_resource::<Text>(&s.hint_text_name, "Text")
                .unwrap()
                .unwrap()
                .clone();
            if self.timer.total_time
                - self
                    .split_time(&format!("{}HintFadeAnimation", s.name), safe_mode)
                    .unwrap()[1]
                >= self.tick_interval
            {
                self.reset_split_time(&format!("{}HintFadeAnimation", s.name), safe_mode)
                    .unwrap();
                t.color[3] = t.color[3].saturating_sub(10);
            };
            self.replace_resource(&s.hint_text_name, "Text", t.clone())
                .unwrap();
        };
        let fr: Box<dyn BasicFrontResource> = match &*s.fill_resource_type {
            "Image" => {
                let mut im = Box::new(
                    fr.as_any()
                        .downcast_ref::<Image>()
                        .unwrap()
                        .clone()
                        .from_config(
                            &s.appearance
                                [(s.state * s.animation_count + appearance_count) as usize]
                                .image_config
                                .clone(),
                        ),
                );
                if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                    && !self.check_resource_exists(
                        &s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                            .texture
                            .clone(),
                        "ImageTexture",
                    )
                {
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
                im.cite_texture = s.appearance
                    [(s.state * s.animation_count + appearance_count) as usize]
                    .texture
                    .clone();
                im
            }
            "CustomRect" => Box::new(
                fr.as_any()
                    .downcast_ref::<CustomRect>()
                    .unwrap()
                    .clone()
                    .name(&s.fill_resource_name)
                    .from_config(
                        &s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                            .custom_rect_config
                            .clone(),
                    ),
            ),
            _ => {
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
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&s.hint_text_name, "Text")
            {
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
            let mut t = self
                .get_resource::<Text>(&s.hint_text_name, "Text")
                .unwrap()
                .unwrap()
                .clone();
            t.background_color[3] = t.color[3];
            t.content = s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                .hint_text
                .clone();
            self.replace_resource(&s.hint_text_name, "Text", t.clone())
                .unwrap();
        };
        s.last_time_hovered = hovered;
        self.replace_resource(name, "Switch", s.clone()).unwrap();
        match s.fill_resource_type.as_str() {
            "Image" => {
                let im = fr.as_any().downcast_ref::<Image>().unwrap().clone();
                self.replace_resource(&s.fill_resource_name, &s.fill_resource_type, im)
                    .unwrap();
                self.image(&s.fill_resource_name.clone(), ui, ctx, safe_mode)
                    .unwrap();
            }
            "CustomRect" => {
                let cr = fr.as_any().downcast_ref::<CustomRect>().unwrap().clone();
                self.replace_resource(&s.fill_resource_name, &s.fill_resource_type, cr)
                    .unwrap();
                self.custom_rect(&s.fill_resource_name.clone(), ui, ctx, safe_mode)
                    .unwrap();
            }
            _ => {}
        };
        s.text_origin_position = s.appearance
            [(s.state * s.animation_count + appearance_count) as usize]
            .text_config
            .origin_position;
        if !s.text_name.is_empty() {
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&s.text_name, "Text")
            {
                self.problem_report_custom(
                    RustConstructorError::TextNotFound {
                        text_name: s.text_name.clone(),
                    },
                    SeverityLevel::SevereWarning,
                    self.problem_list.clone(),
                );
                return Err(RustConstructorError::TextNotFound {
                    text_name: s.text_name,
                });
            };
            let mut t = self
                .get_resource::<Text>(&s.text_name, "Text")
                .unwrap()
                .unwrap()
                .clone();
            t.origin_position = [
                fr.display_position()[0] + s.text_origin_position[0],
                fr.display_position()[1] + s.text_origin_position[1],
            ];
            t = t.from_config(
                &s.appearance[(s.state * s.animation_count + appearance_count) as usize]
                    .text_config
                    .clone(),
            );
            self.replace_resource(&s.text_name, "Text", t.clone())
                .unwrap();
            self.text(&s.text_name, ui, ctx, safe_mode).unwrap();
        };
        if !s.hint_text_name.is_empty() {
            if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
                && !self.check_resource_exists(&s.hint_text_name, "Text")
            {
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
            self.text(&s.hint_text_name, ui, ctx, safe_mode).unwrap();
        };
        Ok(())
    }

    /// 查找指定开关的常用判定字段集合。
    pub fn check_switch_data(
        &self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<SwitchData, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "Switch")
        {
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
        let s = self
            .get_resource::<Switch>(name, "Switch")
            .unwrap()
            .unwrap();
        Ok(SwitchData {
            switched: s.switched,
            last_time_clicked_index: s.last_time_clicked_index,
            state: s.state,
        })
    }

    pub fn mouse_detector(
        &mut self,
        name: &str,
        ui: &Ui,
        ctx: &Context,
        mouse_detector_level: MouseDetectorLevel,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "MouseDetector")
        {
            self.problem_report_custom(
                RustConstructorError::MouseDetectorNotFound {
                    mouse_detector_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::MouseDetectorNotFound {
                mouse_detector_name: name.to_string(),
            });
        };
        let mut mouse_detector = self
            .get_resource_mut::<MouseDetector>(name, "MouseDetector")
            .unwrap()
            .unwrap()
            .clone();
        mouse_detector.modify_active(true);
        [mouse_detector.position, mouse_detector.size] =
            self.position_size_processor(PositionConfig::from_mouse_detector(&mouse_detector), ctx);
        let rect = Rect::from_min_size(mouse_detector.position.into(), mouse_detector.size.into());
        let response = ui.interact(rect, Id::new(name), Sense::click_and_drag());
        mouse_detector.detect_result = match mouse_detector_level {
            MouseDetectorLevel::Lite => MouseDetectResult {
                clicked: response.clicked(),
                contains_pointer: response.contains_pointer(),
                secondary_clicked: None,
                middle_clicked: None,
                clicked_by_extra_button: None,
                long_touched: None,
                double_clicked: None,
                triple_clicked: None,
                double_clicked_by: None,
                triple_clicked_by: None,
                clicked_elsewhere: None,
                hovered: None,
                drag_started: None,
                drag_started_by: None,
                dragged: None,
                dragged_by: None,
                drag_stopped: None,
                deag_stopped_by: None,
                drag_delta: None,
                total_drag_delta: None,
                drag_motion: None,
                interact_pointer_pos: None,
                hover_pos: None,
                is_pointer_button_down_on: None,
                raw_scroll_delta: None,
                smooth_scroll_delta: None,
            },
            MouseDetectorLevel::Default => {
                let interact_hover_pos = response.interact_pointer_pos();
                let hover_pos = response.hover_pos();
                MouseDetectResult {
                    clicked: response.clicked(),
                    contains_pointer: response.contains_pointer(),
                    secondary_clicked: Some(response.secondary_clicked()),
                    middle_clicked: Some(response.middle_clicked()),
                    clicked_by_extra_button: None,
                    long_touched: None,
                    double_clicked: Some(response.double_clicked()),
                    triple_clicked: Some(response.triple_clicked()),
                    double_clicked_by: None,
                    triple_clicked_by: None,
                    clicked_elsewhere: Some(response.clicked_elsewhere()),
                    hovered: Some(response.hovered()),
                    drag_started: Some(response.drag_started()),
                    drag_started_by: None,
                    dragged: Some(response.dragged()),
                    dragged_by: None,
                    drag_stopped: Some(response.drag_stopped()),
                    deag_stopped_by: None,
                    drag_delta: None,
                    total_drag_delta: None,
                    drag_motion: None,
                    interact_pointer_pos: if let Some(interact_hover_pos) = interact_hover_pos {
                        Some(Some([interact_hover_pos.x, interact_hover_pos.y]))
                    } else {
                        Some(None)
                    },
                    hover_pos: if let Some(hover_pos) = hover_pos {
                        Some(Some([hover_pos.x, hover_pos.y]))
                    } else {
                        Some(None)
                    },
                    is_pointer_button_down_on: Some(response.is_pointer_button_down_on()),
                    raw_scroll_delta: Some(ui.input(|i| i.raw_scroll_delta).into()),
                    smooth_scroll_delta: Some(ui.input(|i| i.smooth_scroll_delta).into()),
                }
            }
            MouseDetectorLevel::Pro => {
                let interact_hover_pos = response.interact_pointer_pos();
                let hover_pos = response.hover_pos();
                let total_drag_delta = response.total_drag_delta();
                MouseDetectResult {
                    clicked: response.clicked(),
                    contains_pointer: response.contains_pointer(),
                    secondary_clicked: Some(response.secondary_clicked()),
                    middle_clicked: Some(response.middle_clicked()),
                    clicked_by_extra_button: Some([
                        response.clicked_by(PointerButton::Extra1),
                        response.clicked_by(PointerButton::Extra2),
                    ]),
                    long_touched: Some(response.long_touched()),
                    double_clicked: Some(response.double_clicked()),
                    triple_clicked: Some(response.triple_clicked()),
                    double_clicked_by: Some([
                        response.double_clicked_by(PointerButton::Primary),
                        response.double_clicked_by(PointerButton::Secondary),
                        response.double_clicked_by(PointerButton::Middle),
                        response.double_clicked_by(PointerButton::Extra1),
                        response.double_clicked_by(PointerButton::Extra2),
                    ]),
                    triple_clicked_by: Some([
                        response.triple_clicked_by(PointerButton::Primary),
                        response.triple_clicked_by(PointerButton::Secondary),
                        response.triple_clicked_by(PointerButton::Middle),
                        response.triple_clicked_by(PointerButton::Extra1),
                        response.triple_clicked_by(PointerButton::Extra2),
                    ]),
                    clicked_elsewhere: Some(response.clicked_elsewhere()),
                    hovered: Some(response.hovered()),
                    drag_started: Some(response.drag_started()),
                    drag_started_by: Some([
                        response.drag_started_by(PointerButton::Primary),
                        response.drag_started_by(PointerButton::Secondary),
                        response.drag_started_by(PointerButton::Middle),
                        response.drag_started_by(PointerButton::Extra1),
                        response.drag_started_by(PointerButton::Extra2),
                    ]),
                    dragged: Some(response.dragged()),
                    dragged_by: Some([
                        response.dragged_by(PointerButton::Primary),
                        response.dragged_by(PointerButton::Secondary),
                        response.dragged_by(PointerButton::Middle),
                        response.dragged_by(PointerButton::Extra1),
                        response.dragged_by(PointerButton::Extra2),
                    ]),
                    drag_stopped: Some(response.drag_stopped()),
                    deag_stopped_by: Some([
                        response.drag_stopped_by(PointerButton::Primary),
                        response.drag_stopped_by(PointerButton::Secondary),
                        response.drag_stopped_by(PointerButton::Middle),
                        response.drag_stopped_by(PointerButton::Extra1),
                        response.drag_stopped_by(PointerButton::Extra2),
                    ]),
                    drag_delta: Some([response.drag_delta().x, response.drag_delta().y]),
                    total_drag_delta: if let Some(total_drag_delta) = total_drag_delta {
                        Some(Some([total_drag_delta.x, total_drag_delta.y]))
                    } else {
                        Some(None)
                    },
                    drag_motion: Some([response.drag_motion().x, response.drag_motion().y]),
                    interact_pointer_pos: if let Some(interact_hover_pos) = interact_hover_pos {
                        Some(Some([interact_hover_pos.x, interact_hover_pos.y]))
                    } else {
                        Some(None)
                    },
                    hover_pos: if let Some(hover_pos) = hover_pos {
                        Some(Some([hover_pos.x, hover_pos.y]))
                    } else {
                        Some(None)
                    },
                    is_pointer_button_down_on: Some(response.is_pointer_button_down_on()),
                    raw_scroll_delta: Some(ui.input(|i| i.raw_scroll_delta).into()),
                    smooth_scroll_delta: Some(ui.input(|i| i.smooth_scroll_delta).into()),
                }
            }
        };
        self.replace_resource(name, "MouseDetector", mouse_detector)
            .unwrap();
        Ok(())
    }

    pub fn check_mouse_detect_result(
        &self,
        name: &str,
        safe_mode: Option<bool>,
    ) -> Result<MouseDetectResult, RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "MouseDetector")
        {
            self.problem_report_custom(
                RustConstructorError::MouseDetectorNotFound {
                    mouse_detector_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::MouseDetectorNotFound {
                mouse_detector_name: name.to_string(),
            });
        };
        let md = self
            .get_resource::<MouseDetector>(name, "MouseDetector")
            .unwrap()
            .unwrap();
        Ok(md.detect_result.clone())
    }

    pub fn resource_panel(
        &mut self,
        name: &str,
        ui: &mut Ui,
        ctx: &Context,
        safe_mode: Option<bool>,
    ) -> Result<(), RustConstructorError> {
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && !self.check_resource_exists(name, "ResourcePanel")
        {
            self.problem_report_custom(
                RustConstructorError::ResourcePanelNotFound {
                    resource_panel_name: name.to_string(),
                },
                SeverityLevel::SevereWarning,
                self.problem_list.clone(),
            );
            return Err(RustConstructorError::ResourcePanelNotFound {
                resource_panel_name: name.to_string(),
            });
        };
        let mut resource_panel = self
            .get_resource_mut::<ResourcePanel>(name, "ResourcePanel")
            .unwrap()
            .unwrap()
            .clone();
        if (safe_mode.is_some() && safe_mode.unwrap() || self.safe_mode)
            && resource_panel.display_rect.is_some()
            && !self
                .check_resource_exists(&format!("{}DisplayRect", resource_panel.name), "CustomRect")
        {
            self.problem_report_custom(
                RustConstructorError::RectNotFound {
                    rect_name: format!("{}DisplayRect", resource_panel.name),
                },
                SeverityLevel::MildWarning,
                self.problem_list.clone(),
            );
            self.add_resource(
                CustomRect::default().name(&format!("{}DisplayRect", resource_panel.name)),
                safe_mode,
            )
            .unwrap();
        };
        resource_panel.modify_active(true);
        let rect = Rect::from_min_size(resource_panel.position.into(), resource_panel.size.into());
        if resource_panel.resizable.contains(&true) {
            resource_panel.x_location_grid = [0, 0];
            resource_panel.y_location_grid = [0, 0];
            resource_panel.x_size_grid = [0, 0];
            resource_panel.y_size_grid = [0, 0];
        };
        if resource_panel.min_size[0] < 10_f32 {
            resource_panel.min_size[0] = 10_f32;
        };
        if resource_panel.min_size[1] < 10_f32 {
            resource_panel.min_size[1] = 10_f32;
        };
        if resource_panel.origin_size[0] < resource_panel.min_size[0] {
            resource_panel.origin_size[0] = resource_panel.min_size[0];
        };
        if resource_panel.origin_size[1] < resource_panel.min_size[1] {
            resource_panel.origin_size[1] = resource_panel.min_size[1];
        };
        [resource_panel.position, resource_panel.size] =
            self.position_size_processor(PositionConfig::from_resource_panel(&resource_panel), ctx);
        if let Some(custom_rect_config) = &mut resource_panel.display_rect.clone() {
            *custom_rect_config = custom_rect_config
                .clone()
                .from_position_config(&PositionConfig::from_resource_panel(&resource_panel));
        };
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            let top_rect = Rect::from_min_size(
                [
                    resource_panel.position[0] - 3_f32,
                    resource_panel.position[1] - 3_f32,
                ]
                .into(),
                [resource_panel.size[0] + 6_f32, 6_f32].into(),
            );
            let bottom_rect = Rect::from_min_size(
                [
                    resource_panel.position[0] - 3_f32,
                    resource_panel.position[1] + resource_panel.size[1] - 3_f32,
                ]
                .into(),
                [resource_panel.size[0] + 6_f32, 6_f32].into(),
            );
            let left_rect = Rect::from_min_size(
                [
                    resource_panel.position[0] - 3_f32,
                    resource_panel.position[1] - 3_f32,
                ]
                .into(),
                [6_f32, resource_panel.size[1] + 6_f32].into(),
            );
            let right_rect = Rect::from_min_size(
                [
                    resource_panel.position[0] + resource_panel.size[0] - 3_f32,
                    resource_panel.position[1] - 3_f32,
                ]
                .into(),
                [6_f32, resource_panel.size[1] + 6_f32].into(),
            );
            match [
                top_rect.contains(mouse_pos),
                bottom_rect.contains(mouse_pos),
                left_rect.contains(mouse_pos),
                right_rect.contains(mouse_pos),
            ] {
                [true, false, false, false] => {
                    if resource_panel.resizable[0] {
                        if resource_panel.last_frame_mouse_status.is_none()
                            && ui.input(|i| i.pointer.primary_pressed())
                        {
                            resource_panel.last_frame_mouse_status = Some((
                                mouse_pos.into(),
                                ClickAim::TopResize,
                                [
                                    mouse_pos.x - resource_panel.position[0],
                                    mouse_pos.y - resource_panel.position[1],
                                ],
                            ))
                        };
                        if resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                        } else if resource_panel.max_size.is_some()
                            && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                        } else {
                            ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                        };
                    };
                }
                [false, true, false, false] => {
                    if resource_panel.resizable[1] {
                        if resource_panel.last_frame_mouse_status.is_none()
                            && ui.input(|i| i.pointer.primary_pressed())
                        {
                            resource_panel.last_frame_mouse_status = Some((
                                mouse_pos.into(),
                                ClickAim::BottomResize,
                                [
                                    mouse_pos.x - resource_panel.position[0],
                                    mouse_pos.y - resource_panel.position[1],
                                ],
                            ))
                        };
                        if resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                        } else if resource_panel.max_size.is_some()
                            && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                        } else {
                            ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                        };
                    };
                }
                [false, false, true, false] => {
                    if resource_panel.resizable[2] {
                        if resource_panel.last_frame_mouse_status.is_none()
                            && ui.input(|i| i.pointer.primary_pressed())
                        {
                            resource_panel.last_frame_mouse_status = Some((
                                mouse_pos.into(),
                                ClickAim::LeftResize,
                                [
                                    mouse_pos.x - resource_panel.position[0],
                                    mouse_pos.y - resource_panel.position[1],
                                ],
                            ))
                        };
                        if resource_panel.size[0] > resource_panel.min_size[0]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                        } else if resource_panel.max_size.is_some()
                            && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeEast);
                        } else {
                            ctx.set_cursor_icon(CursorIcon::ResizeWest);
                        };
                    };
                }
                [false, false, false, true] => {
                    if resource_panel.resizable[3] {
                        if resource_panel.last_frame_mouse_status.is_none()
                            && ui.input(|i| i.pointer.primary_pressed())
                        {
                            resource_panel.last_frame_mouse_status = Some((
                                mouse_pos.into(),
                                ClickAim::RightResize,
                                [
                                    mouse_pos.x - resource_panel.position[0],
                                    mouse_pos.y - resource_panel.position[1],
                                ],
                            ))
                        };
                        if resource_panel.size[0] > resource_panel.min_size[0]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                        } else if resource_panel.max_size.is_some()
                            && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        {
                            ctx.set_cursor_icon(CursorIcon::ResizeWest);
                        } else {
                            ctx.set_cursor_icon(CursorIcon::ResizeEast);
                        };
                    };
                }
                [true, false, true, false] => {
                    match [resource_panel.resizable[0], resource_panel.resizable[2]] {
                        [true, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::LeftTopResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                                || resource_panel.size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || resource_panel.size[1]
                                            < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouthEast);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorthWest)
                            };
                        }
                        [false, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::LeftResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeEast);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeWest);
                            };
                        }
                        [true, false] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::TopResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[1] > resource_panel.min_size[1]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                            };
                        }
                        [false, false] => {}
                    }
                }
                [false, true, false, true] => {
                    match [resource_panel.resizable[1], resource_panel.resizable[3]] {
                        [true, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::RightBottomResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                                || resource_panel.size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || resource_panel.size[1]
                                            < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorthWest);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouthEast)
                            };
                        }
                        [false, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::RightResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeWest);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeEast);
                            };
                        }
                        [true, false] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::BottomResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[1] > resource_panel.min_size[1]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                            };
                        }
                        [false, false] => {}
                    }
                }
                [true, false, false, true] => {
                    match [resource_panel.resizable[0], resource_panel.resizable[3]] {
                        [true, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::RightTopResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                                || resource_panel.size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || resource_panel.size[1]
                                            < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouthWest);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorthEast)
                            };
                        }
                        [false, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::RightResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeWest);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeEast);
                            };
                        }
                        [true, false] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::TopResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[1] > resource_panel.min_size[1]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                            };
                        }
                        [false, false] => {}
                    }
                }
                [false, true, true, false] => {
                    match [resource_panel.resizable[1], resource_panel.resizable[2]] {
                        [true, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::LeftBottomResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                                || resource_panel.size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || resource_panel.size[1]
                                            < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorthEast);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouthWest)
                            };
                        }
                        [false, true] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::LeftResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[0] > resource_panel.min_size[0]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeEast);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeWest);
                            };
                        }
                        [true, false] => {
                            if resource_panel.last_frame_mouse_status.is_none()
                                && ui.input(|i| i.pointer.primary_pressed())
                            {
                                resource_panel.last_frame_mouse_status = Some((
                                    mouse_pos.into(),
                                    ClickAim::BottomResize,
                                    [
                                        mouse_pos.x - resource_panel.position[0],
                                        mouse_pos.y - resource_panel.position[1],
                                    ],
                                ))
                            };
                            if resource_panel.size[1] > resource_panel.min_size[1]
                                && (resource_panel.max_size.is_none()
                                    || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            } else if resource_panel.max_size.is_some()
                                && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                            {
                                ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                            } else {
                                ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                            };
                        }
                        [false, false] => {}
                    }
                }
                _ => {}
            };
            resource_panel.last_frame_mouse_status =
                if resource_panel.last_frame_mouse_status.is_none()
                    && rect.contains(mouse_pos)
                    && ui.input(|i| i.pointer.primary_pressed())
                {
                    Some((
                        [mouse_pos.x, mouse_pos.y],
                        ClickAim::Move,
                        [
                            mouse_pos.x - resource_panel.position[0],
                            mouse_pos.y - resource_panel.position[1],
                        ],
                    ))
                } else if resource_panel.last_frame_mouse_status.is_some()
                    && !ui.input(|i| i.pointer.primary_released())
                {
                    Some((
                        [mouse_pos.x, mouse_pos.y],
                        resource_panel.last_frame_mouse_status.unwrap().1,
                        resource_panel.last_frame_mouse_status.unwrap().2,
                    ))
                } else {
                    None
                };
            let [x_scroll_delta, y_scroll_delta] = if resource_panel.use_smooth_scroll_delta {
                ui.input(|i| i.smooth_scroll_delta).into()
            } else {
                ui.input(|i| i.raw_scroll_delta).into()
            };
            if let Some(_) = resource_panel.scroll_length_method[0]
                && x_scroll_delta != 0_f32
            {
                resource_panel.scroll_progress[0] = if resource_panel.scroll_progress[0]
                    + x_scroll_delta * resource_panel.scroll_sensitivity
                    > resource_panel.scroll_length[0]
                {
                    resource_panel.scroll_length[0]
                } else if resource_panel.scroll_progress[0]
                    + x_scroll_delta * resource_panel.scroll_sensitivity
                    > 0_f32
                {
                    resource_panel.scroll_progress[0]
                        + x_scroll_delta * resource_panel.scroll_sensitivity
                } else {
                    0_f32
                };
            };
            if let Some(_) = resource_panel.scroll_length_method[1]
                && y_scroll_delta != 0_f32
            {
                resource_panel.scroll_progress[1] = if resource_panel.scroll_progress[1]
                    + y_scroll_delta * resource_panel.scroll_sensitivity
                    > resource_panel.scroll_length[1]
                {
                    resource_panel.scroll_length[1]
                } else if resource_panel.scroll_progress[1]
                    + y_scroll_delta * resource_panel.scroll_sensitivity
                    > 0_f32
                {
                    resource_panel.scroll_progress[1]
                        + y_scroll_delta * resource_panel.scroll_sensitivity
                } else {
                    0_f32
                };
            };
        };
        if let Some((mouse_pos, click_aim, offset)) = resource_panel.last_frame_mouse_status {
            match click_aim {
                ClickAim::LeftTopResize => {
                    if resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                        > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] += resource_panel.position[0] - mouse_pos[0];
                        resource_panel.origin_position[0] = mouse_pos[0];
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_position[0] -=
                            resource_panel.max_size.unwrap()[0] - resource_panel.origin_size[0];
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                    } else {
                        resource_panel.origin_position[0] +=
                            resource_panel.origin_size[0] - resource_panel.min_size[0];
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                    };
                    if resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                        > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] += resource_panel.position[1] - mouse_pos[1];
                        resource_panel.origin_position[1] = mouse_pos[1];
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_position[1] -=
                            resource_panel.max_size.unwrap()[1] - resource_panel.origin_size[1];
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                    } else {
                        resource_panel.origin_position[1] +=
                            resource_panel.origin_size[1] - resource_panel.min_size[1];
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                    };
                    if resource_panel.size[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        || resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthEast);
                    } else {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthWest)
                    };
                }
                ClickAim::RightBottomResize => {
                    if mouse_pos[0] - resource_panel.position[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[0] - resource_panel.position[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] = mouse_pos[0] - resource_panel.position[0];
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[0] - resource_panel.position[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                    } else {
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                    };
                    if mouse_pos[1] - resource_panel.position[1] > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[1] - resource_panel.position[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] = mouse_pos[1] - resource_panel.position[1];
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[1] - resource_panel.position[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                    } else {
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                    };
                    if resource_panel.size[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        || resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthWest);
                    } else {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthEast)
                    };
                }
                ClickAim::RightTopResize => {
                    if mouse_pos[0] - resource_panel.position[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[0] - resource_panel.position[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] = mouse_pos[0] - resource_panel.position[0];
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[0] - resource_panel.position[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                    } else {
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                    };
                    if resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                        > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] += resource_panel.position[1] - mouse_pos[1];
                        resource_panel.origin_position[1] = mouse_pos[1];
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_position[1] -=
                            resource_panel.max_size.unwrap()[1] - resource_panel.origin_size[1];
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                    } else {
                        resource_panel.origin_position[1] +=
                            resource_panel.origin_size[1] - resource_panel.min_size[1];
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                    };
                    if resource_panel.size[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        || resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthWest);
                    } else {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthEast)
                    };
                }
                ClickAim::LeftBottomResize => {
                    if resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                        > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] += resource_panel.position[0] - mouse_pos[0];
                        resource_panel.origin_position[0] = mouse_pos[0];
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_position[0] -=
                            resource_panel.max_size.unwrap()[0] - resource_panel.origin_size[0];
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                    } else {
                        resource_panel.origin_position[0] +=
                            resource_panel.origin_size[0] - resource_panel.min_size[0];
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                    };
                    if mouse_pos[1] - resource_panel.position[1] > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[1] - resource_panel.position[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] = mouse_pos[1] - resource_panel.position[1];
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[1] - resource_panel.position[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                    } else {
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                    };
                    if resource_panel.size[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.size[0] < resource_panel.max_size.unwrap()[0])
                        || resource_panel.size[1] > resource_panel.min_size[1]
                            && (resource_panel.max_size.is_none()
                                || resource_panel.size[1] < resource_panel.max_size.unwrap()[1])
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.size[0] >= resource_panel.max_size.unwrap()[0]
                        && resource_panel.size[1] >= resource_panel.max_size.unwrap()[1]
                    {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthEast);
                    } else {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthWest)
                    };
                }
                ClickAim::TopResize => {
                    if resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                        > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] += resource_panel.position[1] - mouse_pos[1];
                        resource_panel.origin_position[1] = mouse_pos[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[1] - mouse_pos[1] + resource_panel.size[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_position[1] -=
                            resource_panel.max_size.unwrap()[1] - resource_panel.origin_size[1];
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                    } else {
                        resource_panel.origin_position[1] +=
                            resource_panel.origin_size[1] - resource_panel.min_size[1];
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                    };
                }
                ClickAim::BottomResize => {
                    if mouse_pos[1] - resource_panel.position[1] > resource_panel.min_size[1]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[1] - resource_panel.position[1]
                                < resource_panel.max_size.unwrap()[1])
                    {
                        resource_panel.origin_size[1] = mouse_pos[1] - resource_panel.position[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[1] - resource_panel.position[1]
                            >= resource_panel.max_size.unwrap()[1]
                    {
                        resource_panel.origin_size[1] = resource_panel.max_size.unwrap()[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                    } else {
                        resource_panel.origin_size[1] = resource_panel.min_size[1];
                        ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                    };
                }
                ClickAim::LeftResize => {
                    if resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                        > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] += resource_panel.position[0] - mouse_pos[0];
                        resource_panel.origin_position[0] = mouse_pos[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                    } else if resource_panel.max_size.is_some()
                        && resource_panel.position[0] - mouse_pos[0] + resource_panel.size[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_position[0] -=
                            resource_panel.max_size.unwrap()[0] - resource_panel.origin_size[0];
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeEast);
                    } else {
                        resource_panel.origin_position[0] +=
                            resource_panel.origin_size[0] - resource_panel.min_size[0];
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeWest);
                    };
                }
                ClickAim::RightResize => {
                    if mouse_pos[0] - resource_panel.position[0] > resource_panel.min_size[0]
                        && (resource_panel.max_size.is_none()
                            || mouse_pos[0] - resource_panel.position[0]
                                < resource_panel.max_size.unwrap()[0])
                    {
                        resource_panel.origin_size[0] = mouse_pos[0] - resource_panel.position[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                    } else if resource_panel.max_size.is_some()
                        && mouse_pos[0] - resource_panel.position[0]
                            >= resource_panel.max_size.unwrap()[0]
                    {
                        resource_panel.origin_size[0] = resource_panel.max_size.unwrap()[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeWest);
                    } else {
                        resource_panel.origin_size[0] = resource_panel.min_size[0];
                        ctx.set_cursor_icon(CursorIcon::ResizeEast);
                    };
                }
                ClickAim::Move => {
                    if resource_panel.movable[0] {
                        resource_panel.origin_position[0] = mouse_pos[0] - offset[0];
                    };
                    if resource_panel.movable[1] {
                        resource_panel.origin_position[1] = mouse_pos[1] - offset[1];
                    };
                }
            };
        };
        if let Some(config) = &mut resource_panel.display_rect.clone() {
            *config = config
                .clone()
                .from_position_config(&PositionConfig::from_resource_panel(&resource_panel));
            let custom_rect = self
                .get_resource_mut::<CustomRect>(
                    &format!("{}DisplayRect", resource_panel.name),
                    "CustomRect",
                )
                .unwrap()
                .unwrap();
            *custom_rect = CustomRect::default()
                .name(&format!("{}DisplayRect", resource_panel.name))
                .from_config(config)
                .from_position_config(&PositionConfig::from_resource_panel(&resource_panel));
            self.custom_rect(
                &format!("{}DisplayRect", resource_panel.name),
                ui,
                ctx,
                safe_mode,
            )
            .unwrap();
        };
        let mut resource_point_list: Vec<([f32; 2], [f32; 2], [bool; 2])> = Vec::new();
        for rcr in &mut self.rust_constructor_resource {
            if self
                .basic_front_resource_list
                .contains(&rcr.expose_type().to_string())
            {
                let mut basic_front_resource: Box<dyn BasicFrontResource> = match rcr.expose_type()
                {
                    "Image" => {
                        let mut image =
                            Box::new(rcr.as_any().downcast_ref::<Image>().unwrap().clone());
                        image.x_location_grid = [0, 0];
                        image.y_location_grid = [0, 0];
                        image.x_size_grid = [0, 0];
                        image.y_size_grid = [0, 0];
                        image
                    }
                    "Text" => {
                        let mut text =
                            Box::new(rcr.as_any().downcast_ref::<Text>().unwrap().clone());
                        text.x_location_grid = [0, 0];
                        text.y_location_grid = [0, 0];
                        text.x_size_grid = [0, 0];
                        text.y_size_grid = [0, 0];
                        text
                    }
                    "CustomRect" => {
                        let mut custom_rect =
                            Box::new(rcr.as_any().downcast_ref::<CustomRect>().unwrap().clone());
                        custom_rect.x_location_grid = [0, 0];
                        custom_rect.y_location_grid = [0, 0];
                        custom_rect.x_size_grid = [0, 0];
                        custom_rect.y_size_grid = [0, 0];
                        custom_rect
                    }
                    _ => {
                        unreachable!()
                    }
                };
                if basic_front_resource.display_panel_name() == resource_panel.name {
                    basic_front_resource.modify_clip_rect(&Some(
                        PositionConfig::from_resource_panel(&resource_panel),
                    ));
                    basic_front_resource.modify_offset(
                        if basic_front_resource.display_allow_scrolling()[0] {
                            basic_front_resource.display_offset()[0]
                                + (resource_panel.scroll_progress[0]
                                    - resource_panel.last_frame_scroll_progress[0])
                        } else {
                            basic_front_resource.display_offset()[0]
                        },
                        if basic_front_resource.display_allow_scrolling()[1] {
                            basic_front_resource.display_offset()[1]
                                + (resource_panel.scroll_progress[1]
                                    - resource_panel.last_frame_scroll_progress[1])
                        } else {
                            basic_front_resource.display_offset()[1]
                        },
                    );
                    if let Some(layout) = &mut basic_front_resource.display_panel_layout().clone() {
                        if basic_front_resource
                            .display_allow_scrolling()
                            .contains(&false)
                        {
                            layout.1 = match layout.1 {
                                PanelLayout::Horizontal(top, bottom, left, right, _) => {
                                    PanelLayout::None(top, bottom, left, right, false)
                                }
                                PanelLayout::Vertical(top, bottom, left, right, _) => {
                                    PanelLayout::None(top, bottom, left, right, false)
                                }
                                PanelLayout::None(_, _, _, _, _) => layout.1,
                            };
                        };
                        match layout.1 {
                            PanelLayout::Vertical(top, bottom, left, right, move_to_bottom) => {
                                let mut modify_y = 0_f32;
                                let [default_x_position, default_y_position] = match layout.0 {
                                    PanelLocation::Absolute(x, y) => [
                                        resource_panel.position[0] + x,
                                        resource_panel.position[1] + y,
                                    ],
                                    PanelLocation::Relative([x, y]) => [
                                        resource_panel.position[0]
                                            + (resource_panel.size[0] / x[1] as f32 * x[0] as f32),
                                        resource_panel.position[1]
                                            + (resource_panel.size[1] / y[1] as f32 * y[0] as f32),
                                    ],
                                };
                                let default_x_position =
                                    match basic_front_resource.display_center_display().0 {
                                        HorizontalAlign::Left => default_x_position,
                                        HorizontalAlign::Center => {
                                            default_x_position
                                                - basic_front_resource.display_size()[0] / 2.0
                                        }
                                        HorizontalAlign::Right => {
                                            default_x_position
                                                - basic_front_resource.display_size()[0]
                                        }
                                    };
                                let default_y_position =
                                    match basic_front_resource.display_center_display().1 {
                                        VerticalAlign::Top => default_y_position,
                                        VerticalAlign::Center => {
                                            default_y_position
                                                - basic_front_resource.display_size()[1] / 2.0
                                        }
                                        VerticalAlign::Bottom => {
                                            default_y_position
                                                - basic_front_resource.display_size()[1]
                                        }
                                    };
                                for point in &resource_point_list {
                                    if default_x_position - left < point.1[0]
                                        && default_y_position - top < point.1[1]
                                        && default_x_position
                                            + basic_front_resource.display_size()[0]
                                            + right
                                            > point.0[0]
                                        && default_y_position
                                            + basic_front_resource.display_size()[1]
                                            + bottom
                                            > point.0[1]
                                    {
                                        if move_to_bottom
                                            && point.1[1] - default_y_position + top > modify_y
                                        {
                                            modify_y = point.1[1] - default_y_position + top;
                                        } else if !move_to_bottom
                                            && point.0[1]
                                                - default_y_position
                                                - basic_front_resource.display_size()[1]
                                                - bottom
                                                < modify_y
                                        {
                                            modify_y = point.0[1]
                                                - default_y_position
                                                - basic_front_resource.display_size()[1];
                                        };
                                    };
                                }
                                let real_x_position =
                                    match basic_front_resource.display_center_display().0 {
                                        HorizontalAlign::Left => default_x_position,
                                        HorizontalAlign::Center => {
                                            default_x_position
                                                + basic_front_resource.display_size()[0] / 2.0
                                        }
                                        HorizontalAlign::Right => {
                                            default_x_position
                                                + basic_front_resource.display_size()[0]
                                        }
                                    };
                                let real_y_position =
                                    match basic_front_resource.display_center_display().1 {
                                        VerticalAlign::Top => default_y_position + modify_y,
                                        VerticalAlign::Center => {
                                            default_y_position
                                                + modify_y
                                                + basic_front_resource.display_size()[1] / 2.0
                                        }
                                        VerticalAlign::Bottom => {
                                            default_y_position
                                                + modify_y
                                                + basic_front_resource.display_size()[1]
                                        }
                                    };
                                basic_front_resource
                                    .modify_position(real_x_position, real_y_position);
                                resource_point_list.push((
                                    [real_x_position - left, real_y_position - top],
                                    [
                                        real_x_position
                                            + basic_front_resource.display_size()[0]
                                            + right,
                                        real_y_position
                                            + basic_front_resource.display_size()[1]
                                            + bottom,
                                    ],
                                    basic_front_resource.display_allow_scrolling(),
                                ));
                            }
                            PanelLayout::Horizontal(top, bottom, left, right, move_to_right) => {
                                let mut modify_x = 0_f32;
                                let [default_x_position, default_y_position] = match layout.0 {
                                    PanelLocation::Absolute(x, y) => [
                                        resource_panel.position[0] + x,
                                        resource_panel.position[1] + y,
                                    ],
                                    PanelLocation::Relative([x, y]) => [
                                        resource_panel.position[0]
                                            + (resource_panel.size[0] / x[1] as f32 * x[0] as f32),
                                        resource_panel.position[1]
                                            + (resource_panel.size[1] / y[1] as f32 * y[0] as f32),
                                    ],
                                };
                                let default_x_position =
                                    match basic_front_resource.display_center_display().0 {
                                        HorizontalAlign::Left => default_x_position,
                                        HorizontalAlign::Center => {
                                            default_x_position
                                                - basic_front_resource.display_size()[0] / 2.0
                                        }
                                        HorizontalAlign::Right => {
                                            default_x_position
                                                - basic_front_resource.display_size()[0]
                                        }
                                    };
                                let default_y_position =
                                    match basic_front_resource.display_center_display().1 {
                                        VerticalAlign::Top => default_y_position,
                                        VerticalAlign::Center => {
                                            default_y_position
                                                - basic_front_resource.display_size()[1] / 2.0
                                        }
                                        VerticalAlign::Bottom => {
                                            default_y_position
                                                - basic_front_resource.display_size()[1]
                                        }
                                    };
                                for point in &resource_point_list {
                                    if default_x_position - left < point.1[0]
                                        && default_y_position - top < point.1[1]
                                        && default_x_position
                                            + basic_front_resource.display_size()[0]
                                            + right
                                            > point.0[0]
                                        && default_y_position
                                            + basic_front_resource.display_size()[1]
                                            + bottom
                                            > point.0[1]
                                    {
                                        if move_to_right
                                            && point.1[0] - default_x_position + left > modify_x
                                        {
                                            modify_x = point.1[0] - default_x_position + left;
                                        } else if !move_to_right
                                            && point.0[0]
                                                - default_x_position
                                                - basic_front_resource.display_size()[0]
                                                - right
                                                < modify_x
                                        {
                                            modify_x = point.0[0]
                                                - default_x_position
                                                - basic_front_resource.display_size()[0]
                                                - right;
                                        };
                                    };
                                }
                                let real_x_position =
                                    match basic_front_resource.display_center_display().0 {
                                        HorizontalAlign::Left => default_x_position + modify_x,
                                        HorizontalAlign::Center => {
                                            default_x_position
                                                + modify_x
                                                + basic_front_resource.display_size()[0] / 2.0
                                        }
                                        HorizontalAlign::Right => {
                                            default_x_position
                                                + modify_x
                                                + basic_front_resource.display_size()[0]
                                        }
                                    };
                                let real_y_position =
                                    match basic_front_resource.display_center_display().1 {
                                        VerticalAlign::Top => default_y_position,
                                        VerticalAlign::Center => {
                                            default_y_position
                                                + basic_front_resource.display_size()[1] / 2.0
                                        }
                                        VerticalAlign::Bottom => {
                                            default_y_position
                                                + basic_front_resource.display_size()[1]
                                        }
                                    };
                                basic_front_resource
                                    .modify_position(real_x_position, real_y_position);
                                resource_point_list.push((
                                    [real_x_position - left, real_y_position - top],
                                    [
                                        real_x_position
                                            + basic_front_resource.display_size()[0]
                                            + right,
                                        real_y_position
                                            + basic_front_resource.display_size()[1]
                                            + bottom,
                                    ],
                                    basic_front_resource.display_allow_scrolling(),
                                ));
                            }
                            PanelLayout::None(top, bottom, left, right, influence_layout) => {
                                let [default_x_position, default_y_position] = match layout.0 {
                                    PanelLocation::Absolute(x, y) => [
                                        resource_panel.position[0] + x,
                                        resource_panel.position[1] + y,
                                    ],
                                    PanelLocation::Relative([x, y]) => [
                                        resource_panel.position[0]
                                            + (resource_panel.size[0] / x[1] as f32 * x[0] as f32),
                                        resource_panel.position[1]
                                            + (resource_panel.size[1] / y[1] as f32 * y[0] as f32),
                                    ],
                                };
                                basic_front_resource
                                    .modify_position(default_x_position, default_y_position);
                                if influence_layout {
                                    resource_point_list.push((
                                        [default_x_position - left, default_y_position - top],
                                        [
                                            default_x_position
                                                + basic_front_resource.display_size()[0]
                                                + right,
                                            default_y_position
                                                + basic_front_resource.display_size()[1]
                                                + bottom,
                                        ],
                                        basic_front_resource.display_allow_scrolling(),
                                    ));
                                };
                            }
                        };
                    };
                    *rcr = basic_front_resource;
                };
            };
        }
        let mut resource_length = [[None, None], [None, None]];
        for point in resource_point_list {
            resource_length = [
                [
                    if resource_length[0][0].is_none()
                        || resource_length[0][0].is_some()
                            && point.0[0] < resource_length[0][0].unwrap()
                            && point.2[0]
                    {
                        Some(point.0[0])
                    } else {
                        resource_length[0][0]
                    },
                    if resource_length[0][1].is_none()
                        || resource_length[0][1].is_some()
                            && point.0[1] < resource_length[0][1].unwrap()
                            && point.2[1]
                    {
                        Some(point.0[1])
                    } else {
                        resource_length[0][1]
                    },
                ],
                [
                    if resource_length[1][0].is_none()
                        || resource_length[1][0].is_some()
                            && point.1[0] < resource_length[1][0].unwrap()
                            && point.2[0]
                    {
                        Some(point.1[0])
                    } else {
                        resource_length[1][0]
                    },
                    if resource_length[1][1].is_none()
                        || resource_length[1][1].is_some()
                            && point.1[1] < resource_length[1][1].unwrap()
                            && point.2[1]
                    {
                        Some(point.1[1])
                    } else {
                        resource_length[1][1]
                    },
                ],
            ]
        }
        if let Some(horizontal_scroll_length_method) = resource_panel.scroll_length_method[0] {
            resource_panel.scroll_length[0] = match horizontal_scroll_length_method {
                ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                ScrollLengthMethod::AutoFit => {
                    if let [Some(min), Some(max)] = [resource_length[0][0], resource_length[1][0]] {
                        let width = max - min;
                        if width - resource_panel.size[0] > 0_f32 {
                            width - resource_panel.size[0]
                        } else {
                            0_f32
                        }
                    } else {
                        0_f32
                    }
                }
            };
            resource_panel.scroll_progress[0] =
                if resource_panel.scroll_progress[0] > resource_panel.scroll_length[0] {
                    resource_panel.scroll_length[0]
                } else {
                    resource_panel.scroll_progress[0]
                };
        };
        if let Some(vertical_scroll_length_method) = resource_panel.scroll_length_method[1] {
            resource_panel.scroll_length[1] = match vertical_scroll_length_method {
                ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                ScrollLengthMethod::AutoFit => {
                    if let [Some(min), Some(max)] = [resource_length[0][1], resource_length[1][1]] {
                        let height = max - min;
                        if height - resource_panel.size[1] > 0_f32 {
                            height - resource_panel.size[1]
                        } else {
                            0_f32
                        }
                    } else {
                        0_f32
                    }
                }
            };
            resource_panel.scroll_progress[1] =
                if resource_panel.scroll_progress[1] > resource_panel.scroll_length[1] {
                    resource_panel.scroll_length[1]
                } else {
                    resource_panel.scroll_progress[1]
                };
        };
        resource_panel.last_frame_scroll_progress = resource_panel.scroll_progress;
        self.replace_resource(
            &resource_panel.name,
            "ResourcePanel",
            resource_panel.clone(),
        )
        .unwrap();
        Ok(())
    }
}
