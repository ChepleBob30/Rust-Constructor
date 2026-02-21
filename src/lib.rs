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
    Color32, ColorImage, Context, CornerRadius, CursorIcon, FontData, FontDefinitions, FontFamily,
    FontId, Galley, Id, ImageSource, Key, OpenUrl, PointerButton, Pos2, Sense, StrokeKind,
    TextureHandle, Ui, Vec2, text::CCursor,
};
use std::{
    any::{Any, type_name_of_val},
    char,
    cmp::Ordering,
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
    /// 用于不可变类型转换。
    fn as_any(&self) -> &dyn Any;

    /// 用于可变类型转换。
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// 显示显示信息。
    fn display_display_info(&self) -> Option<DisplayInfo>;

    /// 修改显示信息。
    fn modify_display_info(&mut self, display_info: DisplayInfo);

    /// 显示已有的标签。
    fn display_tags(&self) -> Vec<[String; 2]>;

    /// 修改已有的标签。
    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool);
}

/// 标记并管理用于显示给用户的基本前端资源。
pub trait BasicFrontResource: RustConstructorResource {
    /// 获取基本前端资源配置。
    fn display_basic_front_resource_config(&self) -> BasicFrontResourceConfig;

    /// 获取位置尺寸配置。
    fn display_position_size_config(&self) -> PositionSizeConfig;

    /// 获取资源渲染范围。
    fn display_clip_rect(&self) -> Option<PositionSizeConfig>;

    /// 获取资源显示位置。
    fn display_position(&self) -> [f32; 2];

    /// 获取资源尺寸。
    fn display_size(&self) -> [f32; 2];

    /// 修改基本前端资源配置。
    fn modify_basic_front_resource_config(
        &mut self,
        basic_front_resource_config: BasicFrontResourceConfig,
    );

    /// 修改位置尺寸配置。
    fn modify_position_size_config(&mut self, position_size_config: PositionSizeConfig);

    /// 修改资源渲染范围。
    fn modify_clip_rect(&mut self, clip_rect: Option<PositionSizeConfig>);
}

/// 标记RCR的名称与类型。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RustConstructorId {
    /// 资源名称。
    pub name: String,
    /// 资源类型。
    pub discern_type: String,
}

/// RCR的基本结构。
#[derive(Debug)]
pub struct RustConstructorResourceBox {
    /// 资源ID。
    pub id: RustConstructorId,
    /// 资源内容。
    pub content: Box<dyn RustConstructorResource>,
}

impl RustConstructorResourceBox {
    pub fn new(name: &str, discern_type: &str, content: Box<dyn RustConstructorResource>) -> Self {
        Self {
            id: RustConstructorId {
                name: name.to_string(),
                discern_type: discern_type.to_string(),
            },
            content,
        }
    }
}

/// 基本前端资源配置。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct BasicFrontResourceConfig {
    /// 资源位置和尺寸配置。
    pub position_size_config: PositionSizeConfig,
    /// 允许渲染的范围。
    pub clip_rect: Option<PositionSizeConfig>,
}

impl BasicFrontResourceConfig {
    #[inline]
    pub fn position_size_config(mut self, position_size_config: PositionSizeConfig) -> Self {
        self.position_size_config = position_size_config;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<PositionSizeConfig>) -> Self {
        self.clip_rect = clip_rect;
        self
    }
}

/// 用于配置资源位置和尺寸的结构体。
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositionSizeConfig {
    /// 原始位置。
    pub origin_position: [f32; 2],
    /// 原始尺寸。
    pub origin_size: [f32; 2],
    /// x轴的网格式定位。
    pub x_location_grid: [f32; 2],
    /// y轴的网格式定位。
    pub y_location_grid: [f32; 2],
    /// x轴的网格式缩放。
    pub x_size_grid: [f32; 2],
    /// y轴的网格式缩放。
    pub y_size_grid: [f32; 2],
    /// 对齐方法。
    pub display_method: (HorizontalAlign, VerticalAlign),
    /// 偏移量。
    pub offset: [f32; 2],
}

impl Default for PositionSizeConfig {
    fn default() -> Self {
        PositionSizeConfig {
            origin_position: [0_f32, 0_f32],
            origin_size: [0_f32, 0_f32],
            x_location_grid: [0_f32, 0_f32],
            y_location_grid: [0_f32, 0_f32],
            x_size_grid: [0_f32, 0_f32],
            y_size_grid: [0_f32, 0_f32],
            display_method: (HorizontalAlign::default(), VerticalAlign::default()),
            offset: [0_f32, 0_f32],
        }
    }
}

impl PositionSizeConfig {
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
    pub fn x_size_grid(mut self, fetch: f32, total: f32) -> Self {
        self.x_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_size_grid(mut self, fetch: f32, total: f32) -> Self {
        self.y_size_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn x_location_grid(mut self, fetch: f32, total: f32) -> Self {
        self.x_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn y_location_grid(mut self, fetch: f32, total: f32) -> Self {
        self.y_location_grid = [fetch, total];
        self
    }

    #[inline]
    pub fn display_method(
        mut self,
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        self.display_method = (horizontal_align, vertical_align);
        self
    }

    #[inline]
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
    }
}

/// 事件发生时的状态。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EventState {
    /// 所在页面。
    pub current_page: String,
    /// 程序总运行时间。
    pub current_total_runtime: f32,
    /// 页面运行时间。
    pub current_page_runtime: f32,
}

/// 用于存储页面数据的RC资源。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PageData {
    /// 是否强制在每帧都刷新页面。
    pub forced_update: bool,
    /// 是否已经加载完首次进入此页面所需内容。
    pub change_page_updated: bool,
    /// 是否已经加载完进入此页面所需内容。
    pub enter_page_updated: bool,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for PageData {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Default for PageData {
    fn default() -> Self {
        PageData {
            forced_update: true,
            change_page_updated: false,
            enter_page_updated: false,
            tags: Vec::new(),
        }
    }
}

impl PageData {
    #[inline]
    pub fn forced_update(mut self, forced_update: bool) -> Self {
        self.forced_update = forced_update;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // 只输出类型信息，不输出具体纹理数据
        f.debug_struct("DebugTextureHandle").finish()
    }
}

impl DebugTextureHandle {
    pub fn new(texture_handle: &TextureHandle) -> Self {
        Self(texture_handle.clone())
    }
}

/// 用于存储图片纹理的RC资源。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageTexture {
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
    /// 图片路径。
    pub cite_path: String,
    /// 翻转图片。
    pub flip: [bool; 2],
    /// 加载资源。
    pub context: Context,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for ImageTexture {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl ImageTexture {
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
    pub fn ctx(mut self, context: &Context) -> Self {
        self.context = context.clone();
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// 矩形边框的类型。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BorderKind {
    /// 内部。
    #[default]
    Inside,
    /// 居中。
    Middle,
    /// 外部。
    Outside,
}

/// 矩形的可配置项。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct CustomRectConfig {
    /// 位置尺寸配置。
    pub position_size_config: Option<PositionSizeConfig>,
    /// 允许渲染的范围。
    pub clip_rect: Option<Option<PositionSizeConfig>>,
    /// 隐藏。
    pub hidden: Option<bool>,
    /// 忽略渲染层级。
    pub ignore_render_layer: Option<bool>,
    /// 圆角。
    pub rounding: Option<f32>,
    /// 颜色。
    pub color: Option<[u8; 3]>,
    /// 不透明度。
    pub alpha: Option<u8>,
    /// 边框宽度。
    pub border_width: Option<f32>,
    /// 边框颜色。
    pub border_color: Option<[u8; 3]>,
    /// 边框透明度。
    pub border_alpha: Option<u8>,
    /// 边框类型。
    pub border_kind: Option<BorderKind>,
    /// 标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl CustomRectConfig {
    pub fn from_custom_rect(custom_rect: &CustomRect) -> Self {
        Self {
            position_size_config: Some(
                custom_rect.basic_front_resource_config.position_size_config,
            ),
            clip_rect: Some(custom_rect.basic_front_resource_config.clip_rect),
            hidden: Some(custom_rect.display_info.hidden),
            ignore_render_layer: Some(custom_rect.display_info.ignore_render_layer),
            rounding: Some(custom_rect.rounding),
            color: Some(custom_rect.color),
            alpha: Some(custom_rect.alpha),
            border_width: Some(custom_rect.border_width),
            border_color: Some(custom_rect.border_color),
            border_alpha: Some(custom_rect.border_alpha),
            border_kind: Some(custom_rect.border_kind),
            tags: Some(custom_rect.tags.clone()),
        }
    }

    #[inline]
    pub fn position_size_config(
        mut self,
        position_size_config: Option<PositionSizeConfig>,
    ) -> Self {
        self.position_size_config = position_size_config;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<Option<PositionSizeConfig>>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: Option<bool>) -> Self {
        self.ignore_render_layer = ignore_render_layer;
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: Option<f32>) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub fn color(mut self, color: Option<[u8; 3]>) -> Self {
        self.color = color;
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: Option<u8>) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn border_width(mut self, border_width: Option<f32>) -> Self {
        self.border_width = border_width;
        self
    }

    #[inline]
    pub fn border_color(mut self, border_color: Option<[u8; 3]>) -> Self {
        self.border_color = border_color;
        self
    }

    #[inline]
    pub fn border_alpha(mut self, border_alpha: Option<u8>) -> Self {
        self.border_alpha = border_alpha;
        self
    }

    #[inline]
    pub fn border_kind(mut self, border_kind: Option<BorderKind>) -> Self {
        self.border_kind = border_kind;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// RC的矩形资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRect {
    /// 基本配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,
    /// 资源位置。
    pub position: [f32; 2],
    /// 资源尺寸。
    pub size: [f32; 2],
    /// 显示信息。
    pub display_info: DisplayInfo,
    /// 圆角。
    pub rounding: f32,
    /// 颜色。
    pub color: [u8; 3],
    /// 不透明度。
    pub alpha: u8,
    /// 边框宽度。
    pub border_width: f32,
    /// 边框颜色。
    pub border_color: [u8; 3],
    /// 边框不透明度。
    pub border_alpha: u8,
    /// 边框类型。
    pub border_kind: BorderKind,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for CustomRect {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        Some(self.display_info)
    }

    fn modify_display_info(&mut self, display_info: DisplayInfo) {
        self.display_info = display_info;
    }

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl BasicFrontResource for CustomRect {
    fn display_basic_front_resource_config(&self) -> BasicFrontResourceConfig {
        self.basic_front_resource_config.clone()
    }

    fn display_position_size_config(&self) -> PositionSizeConfig {
        self.basic_front_resource_config.position_size_config
    }

    fn display_clip_rect(&self) -> Option<PositionSizeConfig> {
        self.basic_front_resource_config.clip_rect
    }

    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.size
    }

    fn modify_basic_front_resource_config(
        &mut self,
        basic_front_resource_config: BasicFrontResourceConfig,
    ) {
        self.basic_front_resource_config = basic_front_resource_config;
    }

    fn modify_position_size_config(&mut self, position_size_config: PositionSizeConfig) {
        self.basic_front_resource_config.position_size_config = position_size_config;
    }

    fn modify_clip_rect(&mut self, clip_rect: Option<PositionSizeConfig>) {
        self.basic_front_resource_config.clip_rect = clip_rect;
    }
}

impl Default for CustomRect {
    fn default() -> Self {
        Self {
            basic_front_resource_config: BasicFrontResourceConfig::default(),
            position: [0_f32, 0_f32],
            size: [0_f32, 0_f32],
            display_info: DisplayInfo::default(),
            rounding: 2_f32,
            color: [255, 255, 255],
            alpha: 255,
            border_width: 2_f32,
            border_color: [0, 0, 0],
            border_alpha: 255,
            border_kind: BorderKind::default(),
            tags: Vec::new(),
        }
    }
}

impl CustomRect {
    pub fn from_config(mut self, config: &CustomRectConfig) -> Self {
        if let Some(position_size_config) = config.position_size_config {
            self.basic_front_resource_config.position_size_config = position_size_config;
        };
        if let Some(clip_rect) = config.clip_rect {
            self.basic_front_resource_config.clip_rect = clip_rect;
        };
        if let Some(hidden) = config.hidden {
            self.display_info.hidden = hidden;
        };
        if let Some(ignore_render_layer) = config.ignore_render_layer {
            self.display_info.ignore_render_layer = ignore_render_layer;
        };
        if let Some(rounding) = config.rounding {
            self.rounding = rounding;
        };
        if let Some(color) = config.color {
            self.color = color;
        };
        if let Some(alpha) = config.alpha {
            self.alpha = alpha;
        };
        if let Some(border_width) = config.border_width {
            self.border_width = border_width;
        };
        if let Some(border_color) = config.border_color {
            self.border_color = border_color;
        };
        if let Some(border_alpha) = config.border_alpha {
            self.border_alpha = border_alpha;
        };
        if let Some(border_kind) = config.border_kind {
            self.border_kind = border_kind;
        };
        if let Some(tags) = config.tags.clone() {
            self.tags = tags;
        };
        self
    }

    #[inline]
    pub fn basic_front_resource_config(
        mut self,
        basic_front_resource_config: &BasicFrontResourceConfig,
    ) -> Self {
        self.basic_front_resource_config = basic_front_resource_config.clone();
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.display_info.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: bool) -> Self {
        self.display_info.ignore_render_layer = ignore_render_layer;
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = [r, g, b];
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width;
        self
    }

    #[inline]
    pub fn border_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.border_color = [r, g, b];
        self
    }

    #[inline]
    pub fn border_alpha(mut self, border_alpha: u8) -> Self {
        self.border_alpha = border_alpha;
        self
    }

    #[inline]
    pub fn border_kind(mut self, border_kind: BorderKind) -> Self {
        self.border_kind = border_kind;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// 图片的可配置项。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct ImageConfig {
    /// 位置尺寸配置。
    pub position_size_config: Option<PositionSizeConfig>,
    /// 允许渲染的范围。
    pub clip_rect: Option<Option<PositionSizeConfig>>,
    /// 隐藏。
    pub hidden: Option<bool>,
    /// 忽略渲染层级。
    pub ignore_render_layer: Option<bool>,
    /// 不透明度。
    pub alpha: Option<u8>,
    /// 叠加颜色。
    pub overlay_color: Option<[u8; 3]>,
    /// 叠加透明度。
    pub overlay_alpha: Option<u8>,
    /// 背景颜色。
    pub background_color: Option<[u8; 3]>,
    /// 背景透明度。
    pub background_alpha: Option<u8>,
    /// 旋转角度(只能顺时针，建议搭配std::f32::PI使用)。
    pub rotate_angle: Option<f32>,
    /// 旋转中心。
    pub rotate_center: Option<[f32; 2]>,
    /// 引用纹理名。
    pub cite_texture: Option<String>,
    /// 标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl ImageConfig {
    pub fn from_image(image: &Image) -> Self {
        Self {
            position_size_config: Some(image.basic_front_resource_config.position_size_config),
            clip_rect: Some(image.basic_front_resource_config.clip_rect),
            hidden: Some(image.display_info.hidden),
            ignore_render_layer: Some(image.display_info.ignore_render_layer),
            alpha: Some(image.alpha),
            overlay_color: Some(image.overlay_color),
            overlay_alpha: Some(image.overlay_alpha),
            background_color: Some(image.background_color),
            background_alpha: Some(image.background_alpha),
            rotate_angle: Some(image.rotate_angle),
            rotate_center: Some(image.rotate_center),
            cite_texture: Some(image.cite_texture.clone()),
            tags: Some(image.tags.clone()),
        }
    }

    #[inline]
    pub fn position_size_config(
        mut self,
        position_size_config: Option<PositionSizeConfig>,
    ) -> Self {
        self.position_size_config = position_size_config;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<Option<PositionSizeConfig>>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: Option<bool>) -> Self {
        self.ignore_render_layer = ignore_render_layer;
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: Option<u8>) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn overlay_color(mut self, overlay_color: Option<[u8; 3]>) -> Self {
        self.overlay_color = overlay_color;
        self
    }

    #[inline]
    pub fn overlay_alpha(mut self, overlay_alpha: Option<u8>) -> Self {
        self.overlay_alpha = overlay_alpha;
        self
    }

    #[inline]
    pub fn background_color(mut self, background_color: Option<[u8; 3]>) -> Self {
        self.background_color = background_color;
        self
    }

    #[inline]
    pub fn background_alpha(mut self, background_alpha: Option<u8>) -> Self {
        self.background_alpha = background_alpha;
        self
    }

    #[inline]
    pub fn rotate_angle(mut self, rotate_angle: Option<f32>) -> Self {
        self.rotate_angle = rotate_angle;
        self
    }

    #[inline]
    pub fn rotate_center(mut self, rotate_center: Option<[f32; 2]>) -> Self {
        self.rotate_center = rotate_center;
        self
    }

    #[inline]
    pub fn cite_texture(mut self, cite_texture: Option<String>) -> Self {
        self.cite_texture = cite_texture;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// RC的图片资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// 基本配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,
    /// 资源位置。
    pub position: [f32; 2],
    /// 资源尺寸。
    pub size: [f32; 2],
    /// 显示信息。
    pub display_info: DisplayInfo,
    /// 图片纹理。
    pub texture: Option<DebugTextureHandle>,
    /// 不透明度。
    pub alpha: u8,
    /// 叠加颜色。
    pub overlay_color: [u8; 3],
    /// 叠加透明度。
    pub overlay_alpha: u8,
    /// 背景颜色。
    pub background_color: [u8; 3],
    /// 背景透明度。
    pub background_alpha: u8,
    /// 旋转角度(只能顺时针，建议搭配std::f32::consts::PI使用)。
    pub rotate_angle: f32,
    /// 旋转中心。
    pub rotate_center: [f32; 2],
    /// 引用纹理名。
    pub cite_texture: String,
    /// 上一帧引用纹理名。
    pub last_frame_cite_texture: String,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Image {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        Some(self.display_info)
    }

    fn modify_display_info(&mut self, display_info: DisplayInfo) {
        self.display_info = display_info;
    }

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl BasicFrontResource for Image {
    fn display_basic_front_resource_config(&self) -> BasicFrontResourceConfig {
        self.basic_front_resource_config.clone()
    }

    fn display_position_size_config(&self) -> PositionSizeConfig {
        self.basic_front_resource_config.position_size_config
    }

    fn display_clip_rect(&self) -> Option<PositionSizeConfig> {
        self.basic_front_resource_config.clip_rect
    }

    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.size
    }

    fn modify_basic_front_resource_config(
        &mut self,
        basic_front_resource_config: BasicFrontResourceConfig,
    ) {
        self.basic_front_resource_config = basic_front_resource_config;
    }

    fn modify_position_size_config(&mut self, position_size_config: PositionSizeConfig) {
        self.basic_front_resource_config.position_size_config = position_size_config;
    }

    fn modify_clip_rect(&mut self, clip_rect: Option<PositionSizeConfig>) {
        self.basic_front_resource_config.clip_rect = clip_rect;
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            basic_front_resource_config: BasicFrontResourceConfig::default(),
            position: [0_f32, 0_f32],
            size: [0_f32, 0_f32],
            display_info: DisplayInfo::default(),
            texture: None,
            alpha: 255,
            overlay_color: [255, 255, 255],
            overlay_alpha: 255,
            background_color: [0, 0, 0],
            background_alpha: 0,
            rotate_angle: 0_f32,
            rotate_center: [0_f32, 0_f32],
            cite_texture: String::from("rust_constructor::ImageTexture"),
            last_frame_cite_texture: String::from("rust_constructor::ImageTexture"),
            tags: Vec::new(),
        }
    }
}

impl Image {
    pub fn from_config(mut self, config: &ImageConfig) -> Self {
        if let Some(position_size_config) = config.position_size_config {
            self.basic_front_resource_config.position_size_config = position_size_config;
        };
        if let Some(clip_rect) = config.clip_rect {
            self.basic_front_resource_config.clip_rect = clip_rect;
        };
        if let Some(hidden) = config.hidden {
            self.display_info.hidden = hidden;
        };
        if let Some(ignore_render_layer) = config.ignore_render_layer {
            self.display_info.ignore_render_layer = ignore_render_layer;
        };
        if let Some(alpha) = config.alpha {
            self.alpha = alpha;
        };
        if let Some(overlay_color) = config.overlay_color {
            self.overlay_color = overlay_color;
        };
        if let Some(overlay_alpha) = config.overlay_alpha {
            self.overlay_alpha = overlay_alpha;
        };
        if let Some(background_color) = config.background_color {
            self.background_color = background_color;
        };
        if let Some(background_alpha) = config.background_alpha {
            self.background_alpha = background_alpha;
        };
        if let Some(rotate_angle) = config.rotate_angle {
            self.rotate_angle = rotate_angle;
        };
        if let Some(rotate_center) = config.rotate_center {
            self.rotate_center = rotate_center;
        };
        if let Some(cite_texture) = config.cite_texture.clone() {
            self.cite_texture = cite_texture;
        };
        if let Some(tags) = config.tags.clone() {
            self.tags = tags;
        };
        self
    }

    #[inline]
    pub fn basic_front_resource_config(
        mut self,
        basic_front_resource_config: &BasicFrontResourceConfig,
    ) -> Self {
        self.basic_front_resource_config = basic_front_resource_config.clone();
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.display_info.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: bool) -> Self {
        self.display_info.ignore_render_layer = ignore_render_layer;
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn overlay_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.overlay_color = [r, g, b];
        self
    }

    #[inline]
    pub fn overlay_alpha(mut self, overlay_alpha: u8) -> Self {
        self.overlay_alpha = overlay_alpha;
        self
    }

    #[inline]
    pub fn background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = [r, g, b];
        self
    }

    #[inline]
    pub fn background_alpha(mut self, background_alpha: u8) -> Self {
        self.background_alpha = background_alpha;
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
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
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
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct TextConfig {
    /// 位置尺寸配置。
    pub position_size_config: Option<PositionSizeConfig>,
    /// 允许渲染的范围。
    pub clip_rect: Option<Option<PositionSizeConfig>>,
    /// 隐藏。
    pub hidden: Option<bool>,
    /// 忽略渲染层。
    pub ignore_render_layer: Option<bool>,
    /// 文本内容。
    pub content: Option<String>,
    /// 字号。
    pub font_size: Option<f32>,
    /// 文本颜色。
    pub color: Option<[u8; 3]>,
    /// 文本透明度。
    pub alpha: Option<u8>,
    /// 背景颜色。
    pub background_color: Option<[u8; 3]>,
    /// 背景透明度。
    pub background_alpha: Option<u8>,
    /// 圆角。
    pub background_rounding: Option<f32>,
    /// 字体。
    pub font: Option<String>,
    /// 是否可框选。
    pub selectable: Option<bool>,
    /// 超链接文本。
    pub hyperlink_text: Option<Vec<(String, HyperlinkSelectMethod)>>,
    /// 是否让渲染层大小自动匹配实际大小。
    pub auto_fit: Option<[bool; 2]>,
    /// 标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl TextConfig {
    pub fn from_text(text: &Text) -> Self {
        Self {
            position_size_config: Some(text.basic_front_resource_config.position_size_config),
            clip_rect: Some(text.basic_front_resource_config.clip_rect),
            hidden: Some(text.display_info.hidden),
            ignore_render_layer: Some(text.display_info.ignore_render_layer),
            content: Some(text.content.clone()),
            font_size: Some(text.font_size),
            color: Some(text.color),
            alpha: Some(text.alpha),
            background_color: Some(text.background_color),
            background_alpha: Some(text.background_alpha),
            background_rounding: Some(text.background_rounding),
            font: Some(text.font.clone()),
            selectable: Some(text.selectable),
            hyperlink_text: Some(text.hyperlink_text.clone()),
            auto_fit: Some(text.auto_fit),
            tags: Some(text.tags.clone()),
        }
    }

    #[inline]
    pub fn position_size_config(
        mut self,
        position_size_config: Option<PositionSizeConfig>,
    ) -> Self {
        self.position_size_config = position_size_config;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, clip_rect: Option<Option<PositionSizeConfig>>) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: Option<bool>) -> Self {
        self.ignore_render_layer = ignore_render_layer;
        self
    }

    #[inline]
    pub fn content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }

    #[inline]
    pub fn font_size(mut self, font_size: Option<f32>) -> Self {
        self.font_size = font_size;
        self
    }

    #[inline]
    pub fn color(mut self, color: Option<[u8; 3]>) -> Self {
        self.color = color;
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: Option<u8>) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn background_color(mut self, background_color: Option<[u8; 3]>) -> Self {
        self.background_color = background_color;
        self
    }

    #[inline]
    pub fn background_alpha(mut self, background_alpha: Option<u8>) -> Self {
        self.background_alpha = background_alpha;
        self
    }

    #[inline]
    pub fn background_rounding(mut self, background_rounding: Option<f32>) -> Self {
        self.background_rounding = background_rounding;
        self
    }

    #[inline]
    pub fn font(mut self, font: Option<String>) -> Self {
        self.font = font;
        self
    }

    #[inline]
    pub fn selectable(mut self, selectable: Option<bool>) -> Self {
        self.selectable = selectable;
        self
    }

    #[inline]
    pub fn hyperlink_text(
        mut self,
        hyperlink_text: Option<Vec<(String, HyperlinkSelectMethod)>>,
    ) -> Self {
        self.hyperlink_text = hyperlink_text;
        self
    }

    #[inline]
    pub fn auto_fit(mut self, auto_fit: Option<[bool; 2]>) -> Self {
        self.auto_fit = auto_fit;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// RC的文本资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    /// 基本配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,
    /// 资源位置。
    pub position: [f32; 2],
    /// 资源尺寸。
    pub size: [f32; 2],
    /// 显示信息。
    pub display_info: DisplayInfo,
    /// 文本内容。
    pub content: String,
    /// 字号。
    pub font_size: f32,
    /// 文本颜色。
    pub color: [u8; 3],
    /// 文本透明度。
    pub alpha: u8,
    /// 背景颜色。
    pub background_color: [u8; 3],
    /// 背景透明度。
    pub background_alpha: u8,
    /// 圆角。
    pub background_rounding: f32,
    /// 字体。
    pub font: String,
    /// 是否可框选。
    pub selectable: bool,
    /// 超链接文本。
    pub hyperlink_text: Vec<(String, HyperlinkSelectMethod)>,
    /// 超链接选取索引值与链接。
    pub hyperlink_index: Vec<(usize, usize, String)>,
    /// 是否让渲染层大小自动匹配实际大小。
    pub auto_fit: [bool; 2],
    /// 上一帧的文本内容。
    pub last_frame_content: String,
    /// 框选选中的文本。
    pub selection: Option<(usize, usize)>,
    /// 文本截断尺寸。
    pub truncate_size: [f32; 2],
    /// 文本实际尺寸。
    pub actual_size: [f32; 2],
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        Some(self.display_info)
    }

    fn modify_display_info(&mut self, display_info: DisplayInfo) {
        self.display_info = display_info;
    }

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl BasicFrontResource for Text {
    fn display_basic_front_resource_config(&self) -> BasicFrontResourceConfig {
        self.basic_front_resource_config.clone()
    }

    fn display_position_size_config(&self) -> PositionSizeConfig {
        self.basic_front_resource_config.position_size_config
    }

    fn display_clip_rect(&self) -> Option<PositionSizeConfig> {
        self.basic_front_resource_config.clip_rect
    }

    fn display_position(&self) -> [f32; 2] {
        self.position
    }

    fn display_size(&self) -> [f32; 2] {
        self.size
    }

    fn modify_basic_front_resource_config(
        &mut self,
        basic_front_resource_config: BasicFrontResourceConfig,
    ) {
        self.basic_front_resource_config = basic_front_resource_config;
    }

    fn modify_position_size_config(&mut self, position_size_config: PositionSizeConfig) {
        self.basic_front_resource_config.position_size_config = position_size_config;
    }

    fn modify_clip_rect(&mut self, clip_rect: Option<PositionSizeConfig>) {
        self.basic_front_resource_config.clip_rect = clip_rect;
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            basic_front_resource_config: BasicFrontResourceConfig::default(),
            position: [0_f32, 0_f32],
            size: [0_f32, 0_f32],
            display_info: DisplayInfo::default(),
            content: String::from("Hello world"),
            font_size: 16_f32,
            color: [255, 255, 255],
            alpha: 255,
            background_color: [0, 0, 0],
            background_alpha: 0,
            background_rounding: 2_f32,
            font: String::new(),
            selectable: true,
            auto_fit: [true, true],
            hyperlink_text: Vec::new(),
            hyperlink_index: Vec::new(),
            last_frame_content: String::from(""),
            selection: None,
            truncate_size: [0_f32, 0_f32],
            actual_size: [0_f32, 0_f32],
            tags: Vec::new(),
        }
    }
}

impl Text {
    pub fn from_config(mut self, config: &TextConfig) -> Self {
        if let Some(position_size_config) = config.position_size_config {
            self.basic_front_resource_config.position_size_config = position_size_config;
        };
        if let Some(clip_rect) = config.clip_rect {
            self.basic_front_resource_config.clip_rect = clip_rect;
        };
        if let Some(hidden) = config.hidden {
            self.display_info.hidden = hidden;
        };
        if let Some(ignore_render_layer) = config.ignore_render_layer {
            self.display_info.ignore_render_layer = ignore_render_layer;
        };
        if let Some(content) = config.content.clone() {
            self.content = content;
        };
        if let Some(font_size) = config.font_size {
            self.font_size = font_size;
        };
        if let Some(color) = config.color {
            self.color = color;
        };
        if let Some(alpha) = config.alpha {
            self.alpha = alpha;
        };
        if let Some(background_color) = config.background_color {
            self.background_color = background_color;
        };
        if let Some(background_alpha) = config.background_alpha {
            self.background_alpha = background_alpha;
        };
        if let Some(background_rounding) = config.background_rounding {
            self.background_rounding = background_rounding;
        };
        if let Some(font) = config.font.clone() {
            self.font = font;
        };
        if let Some(selectable) = config.selectable {
            self.selectable = selectable;
        };
        if let Some(hyperlink_text) = config.hyperlink_text.clone() {
            self.hyperlink_text = hyperlink_text;
        };
        if let Some(auto_fit) = config.auto_fit {
            self.auto_fit = auto_fit;
        };
        if let Some(tags) = config.tags.clone() {
            self.tags = tags;
        };
        self
    }

    #[inline]
    pub fn basic_front_resource_config(
        mut self,
        basic_front_resource_config: &BasicFrontResourceConfig,
    ) -> Self {
        self.basic_front_resource_config = basic_front_resource_config.clone();
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.display_info.hidden = hidden;
        self
    }

    #[inline]
    pub fn ignore_render_layer(mut self, ignore_render_layer: bool) -> Self {
        self.display_info.ignore_render_layer = ignore_render_layer;
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
    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = [r, g, b];
        self
    }

    #[inline]
    pub fn alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    #[inline]
    pub fn background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = [r, g, b];
        self
    }

    #[inline]
    pub fn background_alpha(mut self, alpha: u8) -> Self {
        self.background_alpha = alpha;
        self
    }

    #[inline]
    pub fn background_rounding(mut self, background_rounding: f32) -> Self {
        self.background_rounding = background_rounding;
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
    pub fn push_hyperlink_text(
        mut self,
        target_text: &str,
        select_method: HyperlinkSelectMethod,
    ) -> Self {
        self.hyperlink_text
            .push((target_text.to_string(), select_method));
        self
    }

    #[inline]
    pub fn hyperlink_text(mut self, hyperlink_text: Vec<(String, HyperlinkSelectMethod)>) -> Self {
        self.hyperlink_text = hyperlink_text;
        self
    }

    #[inline]
    pub fn auto_fit(mut self, x: bool, y: bool) -> Self {
        self.auto_fit = [x, y];
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// RC的变量资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable<T> {
    /// 变量的值。
    pub value: Option<T>,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl<T: Debug + 'static> RustConstructorResource for Variable<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl<T> Default for Variable<T> {
    fn default() -> Self {
        Variable {
            value: None,
            tags: Vec::new(),
        }
    }
}

impl<T> Variable<T> {
    #[inline]
    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = value;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// RC的字体资源。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Font {
    /// 字体定义。
    pub font_definitions: FontDefinitions,
    /// 字体路径。
    pub path: String,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Font {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Font {
    #[inline]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// RC的时间分段资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SplitTime {
    /// 时间点（第一个值为页面运行时间，第二个值为总运行时间）。
    pub time: [f32; 2],
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for SplitTime {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Default for SplitTime {
    fn default() -> Self {
        Self {
            time: [0_f32, 0_f32],
            tags: Vec::new(),
        }
    }
}

impl SplitTime {
    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// 控制Background选择的基础前端资源类型。
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum BackgroundType {
    /// 选择Image为底。
    Image(ImageConfig),
    /// 选择CustomRect为底。
    CustomRect(CustomRectConfig),
}

impl Default for BackgroundType {
    fn default() -> Self {
        BackgroundType::CustomRect(CustomRectConfig::default())
    }
}

/// 复合结构体，包含一个Image或一个CustomRect，可以用作UI的背景。
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Background {
    /// 选择填充类型。
    pub background_type: BackgroundType,
    /// 是否让Background自动更新配置。
    pub auto_update: bool,
    /// 是否使用Background的标签。
    pub use_background_tags: bool,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Background {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Background {
    #[inline]
    pub fn background_type(mut self, background_type: &BackgroundType) -> Self {
        self.background_type = background_type.clone();
        self
    }

    #[inline]
    pub fn auto_update(mut self, auto_update: bool) -> Self {
        self.auto_update = auto_update;
        self
    }

    #[inline]
    pub fn use_background_tags(mut self, use_background_tags: bool) -> Self {
        self.use_background_tags = use_background_tags;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// 滚动区域滚动长度(尺寸)配置。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ScrollLengthMethod {
    /// 固定尺寸。
    Fixed(f32),
    /// 自适应尺寸。
    AutoFit(f32),
}

/// 鼠标点击资源板的目的。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClickAim {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// 滚动条显示方法。
pub enum ScrollBarDisplayMethod {
    /// 持续显示。
    Always(BackgroundType, [f32; 2], f32),
    /// 滚动时显示。
    OnlyScroll(BackgroundType, [f32; 2], f32),
    /// 隐藏。
    Hidden,
}

/// 用于确认资源在资源板中的外边距。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PanelMargin {
    /// 垂直布局。
    Vertical([f32; 4], bool),
    /// 水平布局。
    Horizontal([f32; 4], bool),
    /// 无布局。
    None([f32; 4], bool),
}

/// 用于确认资源排版方式。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PanelLayout {
    pub panel_margin: PanelMargin,
    pub panel_location: PanelLocation,
}

/// 用于控制基本前端资源在资源板中的定位方式。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PanelLocation {
    /// 依照此资源到资源板左上角的距离定位。
    Absolute([f32; 2]),
    /// 依照网格式定位方法进行定位。
    Relative([[u32; 2]; 2]),
}

/// 用于存储资源数据的结构体。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct PanelStorage {
    /// 资源Id。
    pub id: RustConstructorId,
    /// 存储资源是否忽略渲染层。
    pub ignore_render_layer: bool,
    /// 存储资源是否隐藏。
    pub hidden: bool,
    /// 存储资源原始尺寸。
    pub origin_size: [f32; 2],
}

/// RC的资源板。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ResourcePanel {
    /// 是否可通过拖拽更改ResourcePanel尺寸。
    pub resizable: [bool; 4],
    /// 在资源底部显示方框。
    pub background: BackgroundType,
    /// 最小尺寸。
    pub min_size: [f32; 2],
    /// 最大尺寸(可选)。
    pub max_size: Option<[f32; 2]>,
    /// 允许拖动ResourcePanel。
    pub movable: [bool; 2],
    /// 滚动长度计算方法(不需要滚动留空即可)。
    pub scroll_length_method: [Option<ScrollLengthMethod>; 2],
    /// 滚动敏感度。
    pub scroll_sensitivity: f32,
    /// 是否使用平滑滚动。
    pub use_smooth_scroll_delta: bool,
    /// 滚动条显示方法。
    pub scroll_bar_display_method: ScrollBarDisplayMethod,
    /// 控制资源的排版方式。
    pub layout: PanelLayout,
    /// 是否隐藏ResourcePanel。
    pub hidden: bool,
    /// 反转滚动方向。
    pub reverse_scroll_direction: [bool; 2],
    /// 自动缩放资源。
    pub auto_shrink: [bool; 2],
    /// 滚动长度。
    pub scroll_length: [f32; 2],
    /// 滚动进度。
    pub scroll_progress: [f32; 2],
    /// 是否按下鼠标与按下后鼠标状态。
    pub last_frame_mouse_status: Option<([f32; 2], ClickAim, [f32; 2])>,
    /// 本帧是否滚动。
    pub scrolled: [bool; 2],
    /// 滚动条透明度。
    pub scroll_bar_alpha: [u8; 2],
    /// 资源原始信息储存列表。
    pub resource_storage: Vec<PanelStorage>,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for ResourcePanel {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Default for ResourcePanel {
    fn default() -> Self {
        Self {
            resizable: [true, true, true, true],
            background: BackgroundType::default(),
            min_size: [10_f32, 10_f32],
            max_size: None,
            movable: [true, true],
            scroll_length_method: [None, None],
            scroll_sensitivity: 0_f32,
            use_smooth_scroll_delta: true,
            scroll_bar_display_method: ScrollBarDisplayMethod::OnlyScroll(
                BackgroundType::default(),
                [4_f32, 2_f32],
                4_f32,
            ),
            layout: (PanelLayout {
                panel_margin: PanelMargin::Vertical([0_f32, 0_f32, 0_f32, 0_f32], false),
                panel_location: PanelLocation::Absolute([0_f32, 0_f32]),
            }),
            hidden: false,
            reverse_scroll_direction: [false, false],
            auto_shrink: [true, false],
            scroll_length: [0_f32, 0_f32],
            scroll_progress: [0_f32, 0_f32],
            last_frame_mouse_status: None,
            scrolled: [false, false],
            scroll_bar_alpha: [0, 0],
            resource_storage: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl ResourcePanel {
    #[inline]
    pub fn resizable(mut self, top: bool, bottom: bool, left: bool, right: bool) -> Self {
        self.resizable = [top, bottom, left, right];
        self
    }

    #[inline]
    pub fn background(mut self, background: &BackgroundType) -> Self {
        self.background = background.clone();
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

    #[inline]
    pub fn scroll_bar_display_method(
        mut self,
        scroll_bar_display_method: ScrollBarDisplayMethod,
    ) -> Self {
        self.scroll_bar_display_method = scroll_bar_display_method;
        self
    }

    #[inline]
    pub fn layout(mut self, layout: PanelLayout) -> Self {
        self.layout = layout;
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    #[inline]
    pub fn reverse_scroll_direction(mut self, horizontal: bool, vertical: bool) -> Self {
        self.reverse_scroll_direction = [horizontal, vertical];
        self
    }

    #[inline]
    pub fn auto_shrink(mut self, horizontal: bool, vertical: bool) -> Self {
        self.auto_shrink = [horizontal, vertical];
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// Switch在不同状态下的的外观配置。
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwitchAppearanceConfig {
    /// Background的配置项。
    pub background_config: BackgroundType,
    /// Text的配置项。
    pub text_config: TextConfig,
    /// 提示Text的配置项。
    pub hint_text_config: TextConfig,
}

/// Switch的可点击方法配置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchClickConfig {
    /// 点击方法。
    pub click_method: PointerButton,
    /// 点击后是否改变Switch状态。
    pub action: bool,
}

/// 用于Switch资源判定的一些字段集合。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwitchData {
    /// 是否点击切换状态。
    pub switched: bool,
    /// 点击的方法。
    pub last_frame_clicked: Option<usize>,
    /// Switch状态。
    pub state: u32,
}

/// RC的开关资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    /// 外观（包括各类资源配置项，数量为开启的内容数量*Switch状态总数）。
    pub appearance: Vec<SwitchAppearanceConfig>,
    /// Background显示内容类型。
    pub background_type: BackgroundType,
    /// Text显示配置。
    pub text_config: TextConfig,
    /// 提示Text显示配置。
    pub hint_text_config: TextConfig,
    /// 是否启用鼠标悬浮和点击时的显示内容。
    pub enable_animation: [bool; 2],
    /// Switch状态总数。
    pub state_amount: u32,
    /// 可以用于点击Switch的方法。
    pub click_method: Vec<SwitchClickConfig>,
    /// 是否启用Switch(不启用会显示出填充资源，但无法交互)。
    pub enable: bool,
    /// Switch当前状态。
    pub state: u32,
    /// 上一帧是否有鼠标悬停。
    pub last_frame_hovered: bool,
    /// 上一帧是否被鼠标点击。
    pub last_frame_clicked: Option<usize>,
    /// 是否切换了Switch状态。
    pub switched: bool,
    /// 是否让创建的资源使用Switch的标签。
    pub use_switch_tags: bool,
    /// 标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Switch {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display_display_info(&self) -> Option<DisplayInfo> {
        None
    }

    fn modify_display_info(&mut self, _display_info: DisplayInfo) {}

    fn display_tags(&self) -> Vec<[String; 2]> {
        self.tags.clone()
    }

    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool) {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self {
            appearance: vec![],
            background_type: BackgroundType::default(),
            text_config: TextConfig::default(),
            hint_text_config: TextConfig::default(),
            enable_animation: [false, false],
            state_amount: 0,
            click_method: vec![],
            enable: true,
            state: 0,
            last_frame_hovered: false,
            last_frame_clicked: None,
            switched: false,
            use_switch_tags: false,
            tags: Vec::new(),
        }
    }
}

impl Switch {
    #[inline]
    pub fn appearance(mut self, appearance: &[SwitchAppearanceConfig]) -> Self {
        self.appearance = appearance.to_owned();
        self
    }

    #[inline]
    pub fn background_type(mut self, background_type: &BackgroundType) -> Self {
        self.background_type = background_type.clone();
        self
    }

    #[inline]
    pub fn text_config(mut self, text_config: &TextConfig) -> Self {
        self.text_config = text_config.clone();
        self
    }

    #[inline]
    pub fn hint_text_config(mut self, hint_text_config: &TextConfig) -> Self {
        self.hint_text_config = hint_text_config.clone();
        self
    }

    #[inline]
    pub fn enable_animation(mut self, enable_hover: bool, enable_click: bool) -> Self {
        self.enable_animation = [enable_hover, enable_click];
        self
    }

    #[inline]
    pub fn state_amount(mut self, state_amount: u32) -> Self {
        self.state_amount = state_amount;
        self
    }

    #[inline]
    pub fn click_method(mut self, click_method: Vec<SwitchClickConfig>) -> Self {
        self.click_method = click_method;
        self
    }

    #[inline]
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }

    #[inline]
    pub fn use_switch_tags(mut self, use_switch_tags: bool) -> Self {
        self.use_switch_tags = use_switch_tags;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: &[[String; 2]], replace: bool) -> Self {
        if replace {
            self.tags = tags.to_owned();
        } else {
            for tag in tags {
                if let Some(index) = self.tags.iter().position(|x| x[0] == tag[0]) {
                    self.tags.remove(index);
                };
            }
            self.tags.extend(tags.iter().cloned());
        };
        self
    }
}

/// RC资源最基本的错误处理。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RustConstructorError {
    /// 错误类型。
    pub error_id: String,
    /// 对此错误的描述。
    pub description: String,
}

impl Display for RustConstructorError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for RustConstructorError {}

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

/// 渲染配置。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RenderConfig {
    Line(f32, [u8; 4]),
    Rect([u8; 4], [u8; 4], [u8; 4], f32, BorderKind),
}

/// 显示信息。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisplayInfo {
    /// 启用资源。
    pub enable: bool,
    /// 隐藏资源。
    pub hidden: bool,
    /// 忽略渲染层级。
    pub ignore_render_layer: bool,
}

/// 定位请求跳过渲染队列的资源的方法。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestMethod {
    /// 使用资源的唯一标识符。
    Id(RustConstructorId),
    /// 使用资源的引用者。
    Citer(RustConstructorId),
}

/// 请求跳过渲染队列的类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestType {
    /// 直接置于顶层。
    Top,
    /// 上移指定层级。
    Up(u32),
}

/// 控制显示活跃资源列表的方法。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActiveListInfoMethod {
    /// 详细显示，包含资源本身与其id(可以选择是否格式化显示)。
    Detailed(bool),
    /// 简单显示，仅包含资源id。
    #[default]
    Simple,
}

/// 程序主体。
#[derive(Debug)]
pub struct App {
    /// RC资源。
    pub rust_constructor_resource: Vec<RustConstructorResourceBox>,
    /// RC资源刷新率。
    pub tick_interval: f32,
    /// 当前页面。
    pub current_page: String,
    /// 计时器。
    pub timer: Timer,
    /// 帧时间。
    pub frame_times: Vec<f32>,
    /// 上一帧时间。
    pub last_frame_time: Option<f32>,
    /// 标记哪些资源属于基本前端资源，此列表不应以任何形式进行修改。
    pub basic_front_resource_list: Vec<String>,
    /// 标记渲染物件的层级和位置。
    pub render_layer: Vec<(RustConstructorId, [[f32; 2]; 2], bool)>,
    /// 活跃资源列表。
    pub active_list: Vec<RustConstructorId>,
    /// 渲染队列。
    pub render_list: Vec<RustConstructorId>,
}

impl Default for App {
    fn default() -> Self {
        App {
            rust_constructor_resource: Vec::new(),
            tick_interval: 0.05,
            current_page: String::new(),
            timer: Timer::default(),
            frame_times: Vec::new(),
            last_frame_time: None,
            basic_front_resource_list: vec![
                String::from("rust_constructor::Image"),
                String::from("rust_constructor::Text"),
                String::from("rust_constructor::CustomRect"),
            ],
            render_layer: Vec::new(),
            active_list: Vec::new(),
            render_list: Vec::new(),
        }
    }
}

impl App {
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

    /// 从指定列表中获取标签。
    pub fn get_tag(&self, tag_name: &str, target: &[[String; 2]]) -> Option<(usize, String)> {
        target
            .iter()
            .position(|x| x[0] == tag_name)
            .map(|index| (index, target[index][1].clone()))
    }

    /// 一次性绘制所有资源，会丢弃所有返回值，不建议使用。
    pub fn draw_resources(&mut self, ui: &mut Ui, ctx: &Context) {
        for i in 0..self.render_list.len() {
            #[allow(warnings)]
            self.draw_resource_by_index(ui, ctx, i);
        }
    }

    /// 根据索引值绘制资源，建议使用for循环搭配。
    pub fn draw_resource_by_index(
        &mut self,
        ui: &mut Ui,
        ctx: &Context,
        index: usize,
    ) -> Result<(), RustConstructorError> {
        if let Some(render_resource) = self.render_list.clone().get(index) {
            match &*render_resource.discern_type {
                "rust_constructor::Image" => {
                    let image = self
                        .get_resource::<Image>(&render_resource.name, "rust_constructor::Image")?;
                    if image.display_info.enable {
                        let mut image = image.clone();
                        if image.cite_texture != image.last_frame_cite_texture {
                            let image_texture = self.get_resource::<ImageTexture>(
                                &image.cite_texture,
                                "rust_constructor::ImageTexture",
                            )?;
                            image.texture = image_texture.texture.clone();
                        };
                        [image.position, image.size] = self.position_size_processor(
                            image.basic_front_resource_config.position_size_config,
                            ctx,
                        );
                        if !image.display_info.hidden {
                            if let Some(clip_rect) = image.basic_front_resource_config.clip_rect {
                                let [min, size] = self.position_size_processor(clip_rect, ctx);
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
                                        (image.alpha as f32 * image.overlay_alpha as f32 / 255_f32)
                                            as u8,
                                    ))
                                    .bg_fill(Color32::from_rgba_unmultiplied(
                                        image.background_color[0],
                                        image.background_color[1],
                                        image.background_color[2],
                                        (image.alpha as f32 * image.background_alpha as f32
                                            / 255_f32)
                                            as u8,
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
                            if image.basic_front_resource_config.clip_rect.is_some() {
                                ui.set_clip_rect(Rect::from_min_size(
                                    [0_f32, 0_f32].into(),
                                    [ctx.available_rect().width(), ctx.available_rect().height()]
                                        .into(),
                                ));
                            };
                        };
                        image.last_frame_cite_texture = image.cite_texture.clone();
                        self.replace_resource(&render_resource.name, image)?;
                    };
                }
                "rust_constructor::Text" => {
                    let text =
                        self.get_resource::<Text>(&render_resource.name, "rust_constructor::Text")?;
                    if text.display_info.enable {
                        let mut text = text.clone();
                        [_, text.truncate_size] = self.position_size_processor(
                            text.basic_front_resource_config.position_size_config,
                            ctx,
                        );
                        let display_content = if text.content.is_empty()
                            || text
                                .basic_front_resource_config
                                .position_size_config
                                .origin_size
                                .contains(&0_f32)
                        {
                            "".to_string()
                        } else {
                            let original_galley = ui.fonts_mut(|f| {
                                f.layout(
                                    text.content.to_string(),
                                    FontId::proportional(text.font_size),
                                    Color32::default(),
                                    text.truncate_size[0],
                                )
                            });

                            let mut truncated = text.content.to_string();
                            let mut ellipsis = "";
                            if original_galley.size().y > text.truncate_size[1] {
                                // 如果超出，逐步缩短文本直到加上省略号后能放下
                                ellipsis = "...";

                                while !truncated.is_empty() {
                                    let test_text = format!("{}{}", truncated, ellipsis);
                                    let test_galley = ui.fonts_mut(|f| {
                                        f.layout(
                                            test_text,
                                            FontId::proportional(text.font_size),
                                            Color32::default(),
                                            text.truncate_size[0],
                                        )
                                    });

                                    if test_galley.size().y <= text.truncate_size[1] {
                                        break;
                                    }

                                    // 移除最后一个字符
                                    truncated.pop();
                                }
                            };
                            format!("{}{}", truncated, ellipsis)
                        };
                        // 计算文本大小
                        let galley: Arc<Galley> = ui.fonts_mut(|f| {
                            f.layout(
                                display_content.to_string(),
                                if !text.font.is_empty() {
                                    if self
                                        .check_resource_exists(&text.font, "rust_constructor::Font")
                                        .is_none()
                                    {
                                        FontId::proportional(text.font_size)
                                    } else {
                                        FontId::new(
                                            text.font_size,
                                            FontFamily::Name(text.font.clone().into()),
                                        )
                                    }
                                } else {
                                    FontId::proportional(text.font_size)
                                },
                                Color32::from_rgba_unmultiplied(
                                    text.color[0],
                                    text.color[1],
                                    text.color[2],
                                    text.alpha,
                                ),
                                text.truncate_size[0],
                            )
                        });
                        text.size = [
                            if text.auto_fit[0] {
                                galley.size().x
                            } else {
                                text.truncate_size[0]
                            },
                            if text.auto_fit[1] {
                                galley.size().y
                            } else {
                                text.truncate_size[1]
                            },
                        ];
                        text.actual_size = [galley.size().x, galley.size().y];
                        [text.position, _] = self.position_size_processor(
                            text.basic_front_resource_config
                                .position_size_config
                                .x_size_grid(0_f32, 0_f32)
                                .y_size_grid(0_f32, 0_f32)
                                .origin_size(text.size[0], text.size[1]),
                            ctx,
                        );
                        // 查找超链接索引值
                        if text.last_frame_content != display_content {
                            text.hyperlink_index.clear();

                            // 创建字节索引到字符索引的映射
                            let byte_to_char_map: std::collections::HashMap<usize, usize> =
                                display_content
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
                                        if let Some(&start_char_index) =
                                            byte_to_char_map.get(&byte_index)
                                        {
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
                                        if let Some(&start_char_index) =
                                            byte_to_char_map.get(&byte_index)
                                        {
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
                        if !text.display_info.hidden {
                            // 使用绝对定位放置文本
                            let rect =
                                Rect::from_min_size(text.position.into(), text.actual_size.into());
                            // 绘制背景颜色
                            ui.painter().rect_filled(
                                rect,
                                text.background_rounding,
                                Color32::from_rgba_unmultiplied(
                                    text.background_color[0],
                                    text.background_color[1],
                                    text.background_color[2],
                                    text.background_alpha,
                                ),
                            );

                            if let Some(clip_rect) = text.basic_front_resource_config.clip_rect {
                                let [min, size] = self.position_size_processor(clip_rect, ctx);
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
                                    text.alpha,
                                ),
                            );

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
                                        text.alpha,
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
                                    let row_height =
                                        galley.rows.first().map_or(14.0, |row| row.height()); // 默认行高14.0

                                    // 计算起始行和结束行的索引
                                    let start_row = (start_pos.y / row_height).round() as usize;
                                    let end_row = (end_pos.y / row_height).round() as usize;

                                    for row in start_row..=end_row {
                                        let row_y =
                                            text.position[1] + row as f32 * row_height + row_height
                                                - 2.0; // 行底部稍微上移一点绘制下划线

                                        // 获取当前行的矩形范围
                                        if let Some(current_row) = galley.rows.get(row) {
                                            let row_rect = current_row.rect();

                                            let color = Color32::from_rgba_unmultiplied(
                                                text.color[0],
                                                text.color[1],
                                                text.color[2],
                                                text.alpha,
                                            );

                                            if row == start_row {
                                                // 第一行从文本开始位置到行尾
                                                ui.painter().line_segment(
                                                    [
                                                        Pos2::new(
                                                            text.position[0] + start_pos.x,
                                                            row_y,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + row_rect.max.x,
                                                            row_y,
                                                        ),
                                                    ],
                                                    Stroke::new(text.font_size / 10_f32, color),
                                                );
                                            } else if row == end_row {
                                                // 最后一行从行首到文本结束位置
                                                ui.painter().line_segment(
                                                    [
                                                        Pos2::new(
                                                            text.position[0] + row_rect.min.x,
                                                            row_y,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + end_pos.x,
                                                            row_y,
                                                        ),
                                                    ],
                                                    Stroke::new(text.font_size / 10_f32, color),
                                                );
                                            } else {
                                                // 中间整行下划线
                                                ui.painter().line_segment(
                                                    [
                                                        Pos2::new(
                                                            text.position[0] + row_rect.min.x,
                                                            row_y,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + row_rect.max.x,
                                                            row_y,
                                                        ),
                                                    ],
                                                    Stroke::new(text.font_size / 10_f32, color),
                                                );
                                            };
                                        };
                                    }
                                };
                            }

                            if text.selectable {
                                // 处理选择逻辑
                                let cursor_at_pointer = |pointer_pos: Vec2| -> usize {
                                    let relative_pos = pointer_pos - text.position.into();
                                    let cursor = galley.cursor_from_pos(relative_pos);
                                    cursor.index
                                };

                                let fullscreen_detect_result = ui.input(|i| i.pointer.clone());
                                let rect = Rect::from_min_size(
                                    text.position.into(),
                                    text.actual_size.into(),
                                );
                                let detect_result = ui.interact(
                                    rect,
                                    Id::new(&render_resource.name),
                                    Sense::click_and_drag(),
                                );

                                if !detect_result.clicked()
                                    && (fullscreen_detect_result.any_click()
                                        || fullscreen_detect_result.any_pressed())
                                {
                                    text.selection = None;
                                };

                                if let Some(index) = self.get_render_layer_resource(
                                    &render_resource.name,
                                    "rust_constructor::Text",
                                ) && let Some(mouse_pos) =
                                    fullscreen_detect_result.interact_pos()
                                    && self.resource_get_focus(index, mouse_pos.into())
                                    && (detect_result.clicked() || detect_result.drag_started())
                                {
                                    let cursor = cursor_at_pointer(mouse_pos.to_vec2());
                                    text.selection = Some((cursor, cursor));
                                };

                                if detect_result.dragged()
                                    && text.selection.is_some()
                                    && let Some(pointer_pos) =
                                        ui.input(|i| i.pointer.interact_pos())
                                {
                                    let cursor = cursor_at_pointer(pointer_pos.to_vec2());
                                    if let Some((start, _)) = text.selection {
                                        text.selection = Some((start, cursor));
                                    };
                                };

                                if text.selection.is_some()
                                    && ui.input(|input| {
                                        input.key_released(Key::A) && input.modifiers.command
                                    })
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
                                        let selected_text: String =
                                            chars[start..end].iter().collect();
                                        ui.ctx().copy_text(selected_text);
                                    };
                                };

                                // 绘制选择区域背景
                                if let Some((start, end)) = text.selection {
                                    let (start, end) = (start.min(end), start.max(end));
                                    if start != end {
                                        // 获取选择区域的范围
                                        let start_cursor =
                                            galley.pos_from_cursor(CCursor::new(start));
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
                                                    text.actual_size[1]
                                                        / display_content.lines().count() as f32
                                                }
                                            } else {
                                                text.actual_size[1]
                                                    / display_content.lines().count() as f32
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
                                                text.actual_size[1]
                                                    / display_content.lines().count() as f32
                                            };

                                            // 计算选择的上下边界
                                            let selection_top =
                                                text.position[1] + start_pos.y.min(end_pos.y);
                                            let selection_bottom =
                                                text.position[1] + start_pos.y.max(end_pos.y);

                                            // 确定起始行和结束行的索引
                                            let start_row_index =
                                                (start_pos.y / row_height).floor() as usize;
                                            let end_row_index =
                                                (end_pos.y / row_height).floor() as usize;
                                            let (first_row_index, last_row_index) =
                                                if start_row_index <= end_row_index {
                                                    (start_row_index, end_row_index)
                                                } else {
                                                    (end_row_index, start_row_index)
                                                };

                                            for (i, row) in rows.iter().enumerate() {
                                                let row_y =
                                                    text.position[1] + row_height * i as f32;
                                                let row_bottom = row_y + row_height;
                                                // 检查当前行是否与选择区域相交
                                                if row_bottom > selection_top
                                                    && row_y <= selection_bottom
                                                {
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
                                                    if selection_rect.width() > 0.0
                                                        && selection_rect.height() > 0.0
                                                    {
                                                        ui.painter().rect_filled(
                                                            selection_rect,
                                                            0.0,
                                                            Color32::from_rgba_unmultiplied(
                                                                0, 120, 255, 100,
                                                            ),
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

                                let row_height =
                                    galley.rows.first().map_or(14.0, |row| row.height());

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
                                        egui::Id::new(format!(
                                            "link_{}_{}_{}",
                                            render_resource.name, start, end
                                        )),
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
                                                    Pos2::new(
                                                        text.position[0] + start_pos.x,
                                                        row_y,
                                                    ),
                                                    Pos2::new(
                                                        text.position[0] + row_rect.max.x,
                                                        row_y + row_height,
                                                    ),
                                                )
                                            } else if row == end_row {
                                                // 最后一行从行首到文本结束位置
                                                Rect::from_min_max(
                                                    Pos2::new(
                                                        text.position[0] + row_rect.min.x,
                                                        row_y,
                                                    ),
                                                    Pos2::new(
                                                        text.position[0] + end_pos.x,
                                                        row_y + row_height,
                                                    ),
                                                )
                                            } else {
                                                // 中间整行
                                                Rect::from_min_max(
                                                    Pos2::new(
                                                        text.position[0] + row_rect.min.x,
                                                        row_y,
                                                    ),
                                                    Pos2::new(
                                                        text.position[0] + row_rect.max.x,
                                                        row_y + row_height,
                                                    ),
                                                )
                                            };

                                            responses.push(ui.interact(
                                                link_rect,
                                                Id::new(format!(
                                                    "link_{}_{}_{}_row_{}",
                                                    render_resource.name, start, end, row
                                                )),
                                                Sense::click(),
                                            ));
                                        };
                                    }
                                    responses
                                };

                                // 检查是否正在点击这个超链接
                                let mut is_pressing_link = false;
                                for link_response in &link_responses {
                                    if let Some(index) = self.get_render_layer_resource(
                                        &render_resource.name,
                                        "rust_constructor::Text",
                                    ) && let Some(mouse_pos) =
                                        ui.input(|i| i.pointer.interact_pos())
                                        && self.resource_get_focus(index, mouse_pos.into())
                                    {
                                        if link_response.is_pointer_button_down_on()
                                            && !link_response.drag_started()
                                        {
                                            text.selection = None;
                                            if let Some(pointer_pos) =
                                                ui.input(|i| i.pointer.interact_pos())
                                            {
                                                let relative_pos = pointer_pos
                                                    - <[f32; 2] as Into<Pos2>>::into(text.position);
                                                let cursor = galley.cursor_from_pos(relative_pos);
                                                if cursor.index >= *start && cursor.index <= *end {
                                                    is_pressing_link = true;
                                                    break;
                                                };
                                            };
                                        };
                                        // 检查是否释放了鼠标（点击完成）
                                        let mut clicked_on_link = false;
                                        for link_response in &link_responses {
                                            if link_response.clicked()
                                                && let Some(pointer_pos) =
                                                    ui.input(|i| i.pointer.interact_pos())
                                            {
                                                let relative_pos = pointer_pos
                                                    - <[f32; 2] as Into<Pos2>>::into(text.position);
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
                                    };
                                }

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
                                                    + galley
                                                        .rows
                                                        .first()
                                                        .map_or(14.0, |row| row.height()),
                                            ),
                                        );
                                        ui.painter().rect_filled(
                                            selection_rect,
                                            0.0,
                                            Color32::from_rgba_unmultiplied(0, 120, 255, 100),
                                        );
                                    } else {
                                        // 多行超链接高亮
                                        let row_height =
                                            galley.rows.first().map_or(14.0, |row| row.height());
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
                                                            text.position[1]
                                                                + row as f32 * row_height,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + row_rect.max.x,
                                                            text.position[1]
                                                                + row as f32 * row_height
                                                                + row_height,
                                                        ),
                                                    );
                                                    ui.painter().rect_filled(
                                                        selection_rect,
                                                        0.0,
                                                        Color32::from_rgba_unmultiplied(
                                                            0, 120, 255, 100,
                                                        ),
                                                    );
                                                } else if row == end_row {
                                                    // 最后一行从行首到文本结束位置
                                                    let selection_rect = Rect::from_min_max(
                                                        Pos2::new(
                                                            text.position[0] + row_rect.min.x,
                                                            text.position[1]
                                                                + row as f32 * row_height,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + end_pos.x,
                                                            text.position[1]
                                                                + row as f32 * row_height
                                                                + row_height,
                                                        ),
                                                    );
                                                    ui.painter().rect_filled(
                                                        selection_rect,
                                                        0.0,
                                                        Color32::from_rgba_unmultiplied(
                                                            0, 120, 255, 100,
                                                        ),
                                                    );
                                                } else {
                                                    // 中间整行高亮
                                                    let selection_rect = Rect::from_min_max(
                                                        Pos2::new(
                                                            text.position[0] + row_rect.min.x,
                                                            text.position[1]
                                                                + row as f32 * row_height,
                                                        ),
                                                        Pos2::new(
                                                            text.position[0] + row_rect.max.x,
                                                            text.position[1]
                                                                + row as f32 * row_height
                                                                + row_height,
                                                        ),
                                                    );
                                                    ui.painter().rect_filled(
                                                        selection_rect,
                                                        0.0,
                                                        Color32::from_rgba_unmultiplied(
                                                            0, 120, 255, 100,
                                                        ),
                                                    );
                                                };
                                            };
                                        }
                                    };
                                };
                            }
                            if text.basic_front_resource_config.clip_rect.is_some() {
                                ui.set_clip_rect(Rect::from_min_size(
                                    [0_f32, 0_f32].into(),
                                    [ctx.available_rect().width(), ctx.available_rect().height()]
                                        .into(),
                                ));
                            };
                        } else {
                            text.selection = None;
                        };
                        text.last_frame_content = display_content;
                        self.replace_resource(&render_resource.name, text)?;
                    };
                }
                "rust_constructor::CustomRect" => {
                    let custom_rect = self.get_resource::<CustomRect>(
                        &render_resource.name,
                        "rust_constructor::CustomRect",
                    )?;
                    if custom_rect.display_info.enable {
                        let mut custom_rect = custom_rect.clone();
                        [custom_rect.position, custom_rect.size] = self.position_size_processor(
                            custom_rect.basic_front_resource_config.position_size_config,
                            ctx,
                        );
                        if !custom_rect.display_info.hidden {
                            if let Some(clip_rect) =
                                custom_rect.basic_front_resource_config.clip_rect
                            {
                                let [min, size] = self.position_size_processor(clip_rect, ctx);
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
                                    custom_rect.alpha,
                                ),
                                Stroke {
                                    width: custom_rect.border_width,
                                    color: Color32::from_rgba_unmultiplied(
                                        custom_rect.border_color[0],
                                        custom_rect.border_color[1],
                                        custom_rect.border_color[2],
                                        custom_rect.alpha,
                                    ),
                                },
                                match custom_rect.border_kind {
                                    BorderKind::Inside => StrokeKind::Inside,
                                    BorderKind::Middle => StrokeKind::Middle,
                                    BorderKind::Outside => StrokeKind::Outside,
                                },
                            );
                            if custom_rect.basic_front_resource_config.clip_rect.is_some() {
                                ui.set_clip_rect(Rect::from_min_size(
                                    [0_f32, 0_f32].into(),
                                    [ctx.available_rect().width(), ctx.available_rect().height()]
                                        .into(),
                                ));
                            };
                        };
                        self.replace_resource(&render_resource.name, custom_rect)?;
                    };
                }
                _ => {
                    unreachable!()
                }
            }
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "IndexOutOfRange".to_string(),
                description: format!(
                    "The maximum index of the target list is {}, but the index is {index}.",
                    self.render_list.len() - 1
                ),
            })
        }
    }

    /// 打印资源活跃情况。
    pub fn active_list_info(&self, method: ActiveListInfoMethod) -> String {
        let mut text = String::from("Resource Active Info:\n");
        for info in &self.active_list {
            if let ActiveListInfoMethod::Detailed(format) = method {
                if let Some(index) = self.check_resource_exists(&info.name, &info.discern_type) {
                    text += &if format {
                        format!(
                            "\nName: {}\nType: {}\nDetail: {:#?}\n",
                            info.name, info.discern_type, self.rust_constructor_resource[index],
                        )
                    } else {
                        format!(
                            "\nName: {}\nType: {}\nDetail: {:?}\n",
                            info.name, info.discern_type, self.rust_constructor_resource[index],
                        )
                    };
                };
            } else {
                text += &format!("\nName: {}\nType: {}\n", info.name, info.discern_type);
            };
        }
        text
    }

    /// 打印渲染层级列表。
    pub fn render_layer_info(&self) -> String {
        let mut text = String::from("Render Layer Info:\n");
        for (
            RustConstructorId { name, discern_type },
            [min_position, max_position],
            ignore_render_layer,
        ) in &self.render_layer
        {
            text += &format!(
                "\nName: {:?}\nType: {:?}\nMin Position: {:?}\nMax Position: {:?}\nIgnore Render Layer: {}\n",
                name, discern_type, min_position, max_position, ignore_render_layer
            );
        }
        text
    }

    /// 打印渲染队列。
    pub fn render_list_info(&self) -> String {
        let mut text = String::from("Render List Info:\n");
        for RustConstructorId { name, discern_type } in &self.render_list {
            text += &format!("\nName: {:?}\nType: {:?}\n", name, discern_type);
        }
        text
    }

    /// 更新渲染队列。
    pub fn update_render_list(&mut self) {
        if self.render_list.is_empty() {
            for info in &self.active_list {
                if self.basic_front_resource_list.contains(&info.discern_type) {
                    self.render_list.push(RustConstructorId {
                        name: info.name.clone(),
                        discern_type: info.discern_type.clone(),
                    });
                };
            }
        } else {
            let mut count = 0;
            for render_resource in &self.render_list.clone() {
                if !self.active_list.contains(render_resource) {
                    self.render_list.remove(count);
                } else {
                    count += 1;
                };
            }
            let mut insert_index = 0;
            for info in &self.active_list {
                if self.basic_front_resource_list.contains(&info.discern_type) {
                    if !self.render_list.contains(info) {
                        self.render_list.insert(
                            insert_index,
                            RustConstructorId {
                                name: info.name.clone(),
                                discern_type: info.discern_type.clone(),
                            },
                        );
                        insert_index += 1;
                    } else if self.render_list[insert_index].cmp(info) == Ordering::Equal {
                        insert_index += 1;
                    };
                };
            }
        };
    }

    /// 请求在渲染队列中插队，且无视申请跳过队列的资源是否存在。
    pub fn try_request_jump_render_list(
        &mut self,
        requester: RequestMethod,
        request_type: RequestType,
    ) {
        #[allow(warnings)]
        self.request_jump_render_list(requester, request_type);
    }

    /// 请求在渲染队列中插队。
    pub fn request_jump_render_list(
        &mut self,
        requester: RequestMethod,
        request_type: RequestType,
    ) -> Result<(), RustConstructorError> {
        match requester {
            RequestMethod::Id(RustConstructorId { name, discern_type }) => {
                if let Some(index) = self
                    .render_list
                    .iter()
                    .position(|x| x.name == name && x.discern_type == discern_type)
                {
                    self.jump_render_list_processor(index, request_type)?;
                    Ok(())
                } else {
                    Err(RustConstructorError {
                        error_id: "RenderResourceNotFound".to_string(),
                        description: format!(
                            "Render resource \"{name}({discern_type})\" not found.",
                        ),
                    })
                }
            }
            RequestMethod::Citer(RustConstructorId { name, discern_type }) => {
                for (i, render_resource) in self.render_list.iter().enumerate() {
                    let [resource_name, resource_type] = [
                        render_resource.name.clone(),
                        render_resource.discern_type.clone(),
                    ];
                    let tags = self
                        .get_box_resource(&resource_name, &resource_type)?
                        .display_tags();
                    if let [Some(tag_name), Some(tag_type)] = [
                        self.get_tag("citer_name", &tags),
                        self.get_tag("citer_type", &tags),
                    ] && tag_name.1 == name
                        && tag_type.1 == discern_type
                    {
                        self.jump_render_list_processor(i, request_type)?;
                        return Ok(());
                    };
                }
                Err(RustConstructorError {
                    error_id: "RenderResourceNotFound".to_string(),
                    description: format!("Render resource \"{name}({discern_type})\" not found.",),
                })
            }
        }
    }

    /// 执行跳过渲染队列操作。
    pub fn jump_render_list_processor(
        &mut self,
        requester_index: usize,
        request_type: RequestType,
    ) -> Result<(), RustConstructorError> {
        if requester_index < self.render_list.len() {
            let requester = self.render_list.remove(requester_index);
            let new_index = match request_type {
                RequestType::Top => self.render_list.len(),
                RequestType::Up(up) => {
                    if requester_index + up as usize <= self.render_list.len() {
                        requester_index + up as usize
                    } else {
                        self.render_list.len()
                    }
                }
            };
            self.render_list.insert(new_index, requester);
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "IndexOutOfRange".to_string(),
                description: format!(
                    "The maximum index of the target list is {}, but the index is {requester_index}.",
                    self.render_list.len() - 1
                ),
            })
        }
    }

    /// 根据活跃资源更新渲染层级列表。
    pub fn update_render_layer(&mut self) {
        self.render_layer.clear();
        for info in &self.render_list {
            if let Some(index) = self.check_resource_exists(&info.name, &info.discern_type) {
                let basic_front_resource: Box<dyn BasicFrontResource> = match &*info.discern_type {
                    "rust_constructor::Image" => Box::new(
                        self.rust_constructor_resource[index]
                            .content
                            .as_any()
                            .downcast_ref::<Image>()
                            .unwrap()
                            .clone(),
                    ),
                    "rust_constructor::Text" => Box::new(
                        self.rust_constructor_resource[index]
                            .content
                            .as_any()
                            .downcast_ref::<Text>()
                            .unwrap()
                            .clone(),
                    ),
                    "rust_constructor::CustomRect" => Box::new(
                        self.rust_constructor_resource[index]
                            .content
                            .as_any()
                            .downcast_ref::<CustomRect>()
                            .unwrap()
                            .clone(),
                    ),
                    _ => {
                        unreachable!()
                    }
                };
                if let Some(display_info) = basic_front_resource.display_display_info() {
                    self.render_layer.push((
                        info.clone(),
                        [
                            basic_front_resource.display_position(),
                            [
                                basic_front_resource.display_position()[0]
                                    + basic_front_resource.display_size()[0],
                                basic_front_resource.display_position()[1]
                                    + basic_front_resource.display_size()[1],
                            ],
                        ],
                        display_info.ignore_render_layer,
                    ));
                };
            };
        }
    }

    /// 绘制渲染层。
    pub fn display_render_layer(
        &self,
        ui: &mut Ui,
        render_config: &RenderConfig,
        ignore_render_config: &RenderConfig,
    ) {
        for (_, point, ignore_render_layer) in &self.render_layer {
            match if *ignore_render_layer {
                ignore_render_config
            } else {
                render_config
            } {
                RenderConfig::Rect(
                    corner_radius,
                    fill_color,
                    border_color,
                    border_width,
                    border_kind,
                ) => {
                    let rect = Rect::from_min_max(point[0].into(), point[1].into());
                    ui.painter().rect(
                        rect,
                        CornerRadius {
                            nw: corner_radius[0],
                            ne: corner_radius[1],
                            sw: corner_radius[2],
                            se: corner_radius[3],
                        },
                        Color32::from_rgba_unmultiplied(
                            fill_color[0],
                            fill_color[1],
                            fill_color[2],
                            fill_color[3],
                        ),
                        Stroke::new(
                            *border_width,
                            Color32::from_rgba_unmultiplied(
                                border_color[0],
                                border_color[1],
                                border_color[2],
                                border_color[3],
                            ),
                        ),
                        match *border_kind {
                            BorderKind::Inside => StrokeKind::Inside,
                            BorderKind::Middle => StrokeKind::Middle,
                            BorderKind::Outside => StrokeKind::Outside,
                        },
                    );
                }
                RenderConfig::Line(width, color) => {
                    ui.painter().line_segment(
                        [point[0].into(), point[1].into()],
                        Stroke::new(
                            *width,
                            Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]),
                        ),
                    );
                }
            };
        }
    }

    /// 通过名称和类型在渲染列表中查找资源。
    pub fn get_render_layer_resource(&self, name: &str, discern_type: &str) -> Option<usize> {
        self.render_layer
            .iter()
            .position(|x| x.0.name == name && x.0.discern_type == discern_type)
    }

    /// 检查资源是否获取鼠标焦点。
    pub fn resource_get_focus(&self, index: usize, mouse_pos: [f32; 2]) -> bool {
        for i in index + 1..self.render_layer.len() {
            let point = self.render_layer[i].1;
            if mouse_pos[0] > point[0][0]
                && mouse_pos[1] > point[0][1]
                && mouse_pos[0] < point[1][0]
                && mouse_pos[1] < point[1][1]
                && !self.render_layer[i].2
            {
                return false;
            };
        }
        true
    }

    /// 标记活跃资源。
    pub fn add_active_resource(
        &mut self,
        name: &str,
        discern_type: &str,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, discern_type).is_some() {
            self.active_list.push(RustConstructorId {
                name: name.to_string(),
                discern_type: discern_type.to_string(),
            });
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
            })
        }
    }

    /// 添加资源。
    pub fn add_resource<T: RustConstructorResource + 'static>(
        &mut self,
        name: &str,
        mut resource: T,
    ) -> Result<(), RustConstructorError> {
        let discern_type = if let Some(list) = type_name_of_val(&resource).split_once("<") {
            list.0
        } else {
            type_name_of_val(&resource)
        };
        if self.check_resource_exists(name, discern_type).is_some() {
            return Err(RustConstructorError {
                error_id: "ResourceNameRepetition".to_string(),
                description: format!("Resource \"{name}({discern_type})\" has already existed."),
            });
        };
        if name.is_empty() {
            return Err(RustConstructorError {
                error_id: "ResourceUntitled".to_string(),
                description: "All resources must have a valid name.".to_string(),
            });
        };
        match discern_type {
            "rust_constructor::SplitTime" => {
                if let Some(split_time) = resource.as_any_mut().downcast_mut::<SplitTime>() {
                    split_time.time = [self.timer.now_time, self.timer.total_time];
                };
            }
            "rust_constructor::ImageTexture" => {
                if let Some(image_texture) = resource.as_any_mut().downcast_mut::<ImageTexture>() {
                    if let Ok(mut file) = File::open(&image_texture.cite_path) {
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
                        let loaded_image_texture = image_texture.context.load_texture(
                            name,
                            color_image,
                            TextureOptions::LINEAR,
                        );
                        image_texture.texture =
                            Some(DebugTextureHandle::new(&loaded_image_texture));
                        image_texture.cite_path = image_texture.cite_path.to_string();
                    } else {
                        return Err(RustConstructorError {
                            error_id: "ImageLoadFailed".to_string(),
                            description: format!(
                                "Failed to load an image from the path \"{}\".",
                                image_texture.cite_path
                            ),
                        });
                    };
                };
            }
            "rust_constructor::Image" => {
                if let Some(image) = resource.as_any_mut().downcast_mut::<Image>() {
                    let image_texture = self.get_resource::<ImageTexture>(
                        &image.cite_texture,
                        "rust_constructor::ImageTexture",
                    )?;
                    image.texture = image_texture.texture.clone();
                    image.last_frame_cite_texture = image.cite_texture.clone();
                };
            }
            "rust_constructor::Font" => {
                if let Some(font) = resource.as_any_mut().downcast_mut::<Font>() {
                    let mut fonts = FontDefinitions::default();
                    if let Ok(font_read_data) = read(&font.path) {
                        let font_data: Arc<Vec<u8>> = Arc::new(font_read_data);
                        fonts.font_data.insert(
                            name.to_owned(),
                            Arc::new(FontData::from_owned(
                                Arc::try_unwrap(font_data).ok().unwrap(),
                            )),
                        );

                        // 将字体添加到字体列表中
                        fonts
                            .families
                            .entry(FontFamily::Proportional)
                            .or_default()
                            .insert(0, name.to_owned());

                        fonts
                            .families
                            .entry(FontFamily::Monospace)
                            .or_default()
                            .insert(0, name.to_owned());

                        font.font_definitions = fonts;
                    } else {
                        return Err(RustConstructorError {
                            error_id: "FontLoadFailed".to_string(),
                            description: format!(
                                "Failed to load a font from the path \"{}\".",
                                font.path
                            ),
                        });
                    }
                };
            }
            "rust_constructor::Background" => {
                if let Some(background) = resource.as_any_mut().downcast_mut::<Background>() {
                    match &background.background_type {
                        BackgroundType::CustomRect(config) => {
                            let mut custom_rect = CustomRect::default().from_config(config);
                            if background.use_background_tags {
                                custom_rect.modify_tags(&background.tags, false);
                            };
                            self.add_resource(name, custom_rect)
                        }
                        BackgroundType::Image(config) => {
                            let mut image = Image::default().from_config(config);
                            if background.use_background_tags {
                                image.modify_tags(&background.tags, false);
                            };
                            self.add_resource(name, image)
                        }
                    }?;
                };
            }
            "rust_constructor::Switch" => {
                if let Some(switch) = resource.as_any_mut().downcast_mut::<Switch>() {
                    let count = 1 + switch.enable_animation.iter().filter(|x| **x).count();
                    if switch.appearance.len() != count * switch.state_amount as usize {
                        return Err(RustConstructorError {
                            error_id: "SwitchAppearanceConfigMismatch".to_string(),
                            description: format!(
                                "Expected {} elements, found {}.",
                                count * switch.state_amount as usize,
                                switch.appearance.len()
                            ),
                        });
                    };
                    self.add_resource(
                        &format!("{name}Background"),
                        Background::default()
                            .background_type(&switch.background_type)
                            .auto_update(true)
                            .use_background_tags(true)
                            .tags(
                                if switch.use_switch_tags {
                                    &switch.tags
                                } else {
                                    &[]
                                },
                                false,
                            )
                            .tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                    )?;
                    self.add_resource(
                        &format!("{name}Text"),
                        Text::default()
                            .from_config(&switch.text_config)
                            .tags(
                                if switch.use_switch_tags {
                                    &switch.tags
                                } else {
                                    &[]
                                },
                                false,
                            )
                            .tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                    )?;
                    self.add_resource(
                        &format!("{name}HintText"),
                        Text::default()
                            .from_config(&switch.hint_text_config)
                            .ignore_render_layer(true)
                            .hidden(true)
                            .alpha(0)
                            .tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                    ["disable_x_scrolling".to_string(), "".to_string()],
                                    ["disable_y_scrolling".to_string(), "".to_string()],
                                ],
                                false,
                            ),
                    )?;
                    self.add_resource(
                        &format!("{name}StartHoverTime"),
                        SplitTime::default().tags(
                            &[
                                ["citer_name".to_string(), name.to_string()],
                                ["citer_type".to_string(), discern_type.to_string()],
                            ],
                            false,
                        ),
                    )?;
                    self.add_resource(
                        &format!("{name}HintFadeAnimation"),
                        SplitTime::default().tags(
                            &[
                                ["citer_name".to_string(), name.to_string()],
                                ["citer_type".to_string(), discern_type.to_string()],
                            ],
                            false,
                        ),
                    )?;
                };
            }
            "rust_constructor::ResourcePanel" => {
                if let Some(resource_panel) = resource.as_any_mut().downcast_mut::<ResourcePanel>()
                {
                    self.add_resource(
                        &format!("{name}Background"),
                        Background::default()
                            .background_type(&resource_panel.background)
                            .auto_update(true)
                            .use_background_tags(true)
                            .tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                    )?;
                    if let ScrollBarDisplayMethod::Always(_, _, _) =
                        &resource_panel.scroll_bar_display_method
                    {
                        self.add_resource(
                            &format!("{name}XScroll"),
                            Background::default()
                                .auto_update(true)
                                .use_background_tags(true)
                                .tags(
                                    &[
                                        ["citer_name".to_string(), name.to_string()],
                                        ["citer_type".to_string(), discern_type.to_string()],
                                    ],
                                    false,
                                ),
                        )?;
                        self.add_resource(
                            &format!("{name}YScroll"),
                            Background::default()
                                .auto_update(true)
                                .use_background_tags(true)
                                .tags(
                                    &[
                                        ["citer_name".to_string(), name.to_string()],
                                        ["citer_type".to_string(), discern_type.to_string()],
                                    ],
                                    false,
                                ),
                        )?;
                    };
                    if let ScrollBarDisplayMethod::OnlyScroll(_, _, _) =
                        &resource_panel.scroll_bar_display_method
                    {
                        self.add_resource(
                            &format!("{name}XScroll"),
                            Background::default()
                                .auto_update(true)
                                .use_background_tags(true)
                                .tags(
                                    &[
                                        ["citer_name".to_string(), name.to_string()],
                                        ["citer_type".to_string(), discern_type.to_string()],
                                    ],
                                    false,
                                ),
                        )?;
                        self.add_resource(
                            &format!("{name}YScroll"),
                            Background::default()
                                .auto_update(true)
                                .use_background_tags(true)
                                .tags(
                                    &[
                                        ["citer_name".to_string(), name.to_string()],
                                        ["citer_type".to_string(), discern_type.to_string()],
                                    ],
                                    false,
                                ),
                        )?;
                        self.add_resource(
                            &format!("{name}ScrollBarXAlpha"),
                            SplitTime::default().tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                        )?;
                        self.add_resource(
                            &format!("{name}ScrollBarXAlphaStart"),
                            SplitTime::default().tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                        )?;
                        self.add_resource(
                            &format!("{name}ScrollBarYAlpha"),
                            SplitTime::default().tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                        )?;
                        self.add_resource(
                            &format!("{name}ScrollBarYAlphaStart"),
                            SplitTime::default().tags(
                                &[
                                    ["citer_name".to_string(), name.to_string()],
                                    ["citer_type".to_string(), discern_type.to_string()],
                                ],
                                false,
                            ),
                        )?;
                    };
                };
            }
            _ => {}
        };
        self.rust_constructor_resource
            .push(RustConstructorResourceBox::new(
                name,
                discern_type,
                Box::new(resource),
            ));
        Ok(())
    }

    /// 移除资源。此方法非常危险！务必确保资源一定不再使用后删除。
    pub fn drop_resource(
        &mut self,
        name: &str,
        discern_type: &str,
    ) -> Result<(), RustConstructorError> {
        if let Some(index) = self.check_resource_exists(name, discern_type) {
            self.rust_constructor_resource.remove(index);
            if let Some(index) = self
                .active_list
                .iter()
                .position(|x| x.name == name && x.discern_type == discern_type)
            {
                self.active_list.remove(index);
            };
            if let Some(index) = self
                .render_layer
                .iter()
                .position(|x| x.0.name == name && x.0.discern_type == discern_type)
            {
                self.render_layer.remove(index);
            };
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
            })
        }
    }

    /// 从列表中替换资源。
    pub fn replace_resource<T>(
        &mut self,
        name: &str,
        resource: T,
    ) -> Result<(), RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        let discern_type = if let Some(list) = type_name_of_val(&resource).split_once("<") {
            list.0
        } else {
            type_name_of_val(&resource)
        };
        if let Some(index) = self.check_resource_exists(name, discern_type) {
            self.rust_constructor_resource[index] =
                RustConstructorResourceBox::new(name, discern_type, Box::new(resource));
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
            })
        }
    }

    /// 从列表中获取封装的不可变资源。
    pub fn get_box_resource(
        &self,
        name: &str,
        discern_type: &str,
    ) -> Result<&dyn RustConstructorResource, RustConstructorError> {
        if let Some(index) = self.check_resource_exists(name, discern_type) {
            Ok(&*self.rust_constructor_resource[index].content)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
            })
        }
    }

    /// 从列表中获取封装的可变资源。
    pub fn get_box_resource_mut(
        &mut self,
        name: &str,
        discern_type: &str,
    ) -> Result<&mut dyn RustConstructorResource, RustConstructorError> {
        if let Some(index) = self.check_resource_exists(name, discern_type) {
            Ok(&mut *self.rust_constructor_resource[index].content)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
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
        if let Some(resource) = self
            .get_box_resource(name, discern_type)?
            .as_any()
            .downcast_ref::<T>()
        {
            Ok(resource)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource \"{name}({discern_type})\" is mismatched."
                ),
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
        if let Some(resource) = self
            .get_box_resource_mut(name, discern_type)?
            .as_any_mut()
            .downcast_mut::<T>()
        {
            Ok(resource)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource \"{name}({discern_type})\" is mismatched."
                ),
            })
        }
    }

    /// 检查是否存在特定资源。
    pub fn check_resource_exists(&self, name: &str, discern_type: &str) -> Option<usize> {
        self.rust_constructor_resource
            .iter()
            .position(|x| x.id.name == name && x.id.discern_type == discern_type)
    }

    /// 快速放置。
    pub fn quick_place<T: RustConstructorResource + 'static>(
        &mut self,
        name: &str,
        resource: T,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        let discern_type = if let Some(list) = type_name_of_val(&resource).split_once("<") {
            list.0
        } else {
            type_name_of_val(&resource)
        };
        if self.check_resource_exists(name, discern_type).is_none() {
            self.add_resource(name, resource)
        } else {
            self.use_resource(name, discern_type, ui, ctx)
        }
    }

    /// 调用资源。
    pub fn use_resource(
        &mut self,
        name: &str,
        discern_type: &str,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(name, discern_type).is_some() {
            match discern_type {
                "rust_constructor::CustomRect"
                | "rust_constructor::Text"
                | "rust_constructor::Image" => {
                    self.add_active_resource(name, discern_type)?;
                }
                "rust_constructor::PageData" => {
                    // 更新帧数
                    self.update_frame_stats();
                    // 更新渲染队列。
                    self.update_render_list();
                    // 绘制渲染队列中的资源。
                    for i in 0..self.render_list.len() {
                        self.draw_resource_by_index(ui, ctx, i)?;
                    }
                    // 更新渲染列表。
                    self.update_render_layer();
                    // 更新资源活跃状态。
                    self.active_list.clear();
                    // 更新资源启用状态。
                    for rcr in &mut self.rust_constructor_resource {
                        if let Some(display_info) = &mut rcr.content.display_display_info() {
                            rcr.content.modify_display_info(DisplayInfo {
                                enable: true,
                                hidden: display_info.hidden,
                                ignore_render_layer: display_info.ignore_render_layer,
                            });
                        };
                    }
                    // 更新计时器
                    self.update_timer();
                    let current_page = &self.current_page.clone();
                    let page_data =
                        self.get_resource::<PageData>(current_page, "rust_constructor::PageData")?;
                    if page_data.forced_update {
                        ctx.request_repaint();
                    };
                }
                "rust_constructor::Background" => {
                    let background = self.get_resource::<Background>(name, discern_type)?.clone();
                    if background.auto_update {
                        match &background.background_type {
                            BackgroundType::CustomRect(config) => {
                                let mut custom_rect = self
                                    .get_resource::<CustomRect>(
                                        name,
                                        "rust_constructor::CustomRect",
                                    )?
                                    .clone()
                                    .from_config(config);
                                if background.use_background_tags {
                                    custom_rect.modify_tags(&background.tags, false);
                                };
                                self.replace_resource(name, custom_rect)?;
                            }
                            BackgroundType::Image(config) => {
                                let mut image = self
                                    .get_resource::<Image>(name, "rust_constructor::Image")?
                                    .clone()
                                    .from_config(config);
                                if background.use_background_tags {
                                    image.modify_tags(&background.tags, false);
                                };
                                self.replace_resource(name, image)?;
                            }
                        };
                    };
                    match background.background_type {
                        BackgroundType::CustomRect(_) => {
                            self.use_resource(name, "rust_constructor::CustomRect", ui, ctx)
                        }
                        BackgroundType::Image(_) => {
                            self.use_resource(name, "rust_constructor::Image", ui, ctx)
                        }
                    }?;
                }
                "rust_constructor::Switch" => {
                    let mut switch = self
                        .get_resource::<Switch>(name, "rust_constructor::Switch")?
                        .clone();
                    let mut background = self
                        .get_resource::<Background>(
                            &format!("{name}Background"),
                            "rust_constructor::Background",
                        )?
                        .clone();
                    let background_resource_type = match switch.background_type {
                        BackgroundType::CustomRect(_) => "rust_constructor::CustomRect",
                        BackgroundType::Image(_) => "rust_constructor::Image",
                    };
                    let background_resource: Box<dyn BasicFrontResource> =
                        match background_resource_type {
                            "rust_constructor::CustomRect" => Box::new(
                                self.get_resource::<CustomRect>(
                                    &format!("{name}Background"),
                                    background_resource_type,
                                )?
                                .clone(),
                            ),
                            "rust_constructor::Image" => Box::new(
                                self.get_resource::<Image>(
                                    &format!("{name}Background"),
                                    background_resource_type,
                                )?
                                .clone(),
                            ),
                            _ => {
                                unreachable!()
                            }
                        };
                    let mut text = self
                        .get_resource::<Text>(&format!("{name}Text"), "rust_constructor::Text")?
                        .clone();
                    let mut hint_text = self
                        .get_resource::<Text>(&format!("{name}HintText"), "rust_constructor::Text")?
                        .clone();
                    let rect = Rect::from_min_size(
                        background_resource.display_position().into(),
                        background_resource.display_size().into(),
                    );
                    switch.switched = false;
                    let animation_count =
                        1 + switch.enable_animation.iter().filter(|x| **x).count();
                    let mut clicked = None;
                    let mut hovered = false;
                    let mut appearance_count = 0;
                    // 处理点击事件
                    if let Some(index) = self.get_render_layer_resource(
                        &format!("{name}Background"),
                        background_resource_type,
                    ) && switch.enable
                        && let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
                        && self.resource_get_focus(index, mouse_pos.into())
                        && let Some(display_info) = background_resource.display_display_info()
                        && !display_info.hidden
                    {
                        // 判断是否在矩形内
                        if rect.contains(mouse_pos) {
                            if !switch.last_frame_hovered {
                                self.reset_split_time(&format!("{name}StartHoverTime"))?;
                            } else if self.timer.total_time
                                - self.get_split_time(&format!("{name}StartHoverTime"))?[1]
                                >= 2_f32
                                || hint_text.alpha != 0
                            {
                                hint_text.alpha = 255;
                                hint_text
                                    .basic_front_resource_config
                                    .position_size_config
                                    .origin_position = [mouse_pos.x, mouse_pos.y];
                            };
                            hint_text
                                .basic_front_resource_config
                                .position_size_config
                                .display_method
                                .0 = if mouse_pos.x + hint_text.actual_size[0]
                                <= ctx.available_rect().width()
                            {
                                HorizontalAlign::Left
                            } else {
                                HorizontalAlign::Right
                            };
                            hint_text
                                .basic_front_resource_config
                                .position_size_config
                                .display_method
                                .1 = if mouse_pos.y + hint_text.actual_size[1]
                                <= ctx.available_rect().height()
                            {
                                VerticalAlign::Top
                            } else {
                                VerticalAlign::Bottom
                            };
                            hovered = true;
                            for (count, click_method) in switch.click_method.iter().enumerate() {
                                if ui.input(|i| {
                                    switch.last_frame_clicked.is_none()
                                        && i.pointer.button_pressed(click_method.click_method)
                                        || switch.last_frame_clicked.is_some()
                                            && i.pointer.button_down(click_method.click_method)
                                }) {
                                    clicked = Some(count);
                                    break;
                                };
                            }
                            if let Some(clicked_index) = switch.last_frame_clicked
                                && clicked.is_none()
                            {
                                switch.switched = true;
                                if switch.click_method[clicked_index].action {
                                    if switch.state
                                        < (switch.appearance.len() / animation_count - 1) as u32
                                    {
                                        switch.state += 1;
                                    } else {
                                        switch.state = 0;
                                    };
                                };
                            };
                            appearance_count = if clicked.is_some() {
                                match switch.enable_animation {
                                    [true, true] => 2,
                                    [true, false] | [false, true] => 1,
                                    [false, false] => 0,
                                }
                            } else if switch.enable_animation[0] {
                                1
                            } else {
                                0
                            };
                        };
                    };

                    // 若鼠标未悬挂在开关上，逐渐隐藏提示文本
                    if !hovered {
                        if switch.last_frame_hovered {
                            self.reset_split_time(&format!("{name}HintFadeAnimation"))?;
                        };
                        if self.timer.total_time
                            - self.get_split_time(&format!("{name}HintFadeAnimation"))?[1]
                            >= self.tick_interval
                        {
                            self.reset_split_time(&format!("{name}HintFadeAnimation"))?;
                            hint_text.alpha = hint_text.alpha.saturating_sub(10);
                        };
                    };

                    hint_text.display_info.hidden = hint_text.alpha == 0;

                    // 更新Background样式。
                    background.background_type = switch.appearance
                        [(switch.state * animation_count as u32 + appearance_count) as usize]
                        .background_config
                        .clone();

                    background.modify_tags(
                        if switch.use_switch_tags {
                            &switch.tags
                        } else {
                            &[]
                        },
                        false,
                    );

                    // 刷新提示Text。
                    let alpha = hint_text.alpha;
                    hint_text = hint_text
                        .from_config(
                            &switch.appearance[(switch.state * animation_count as u32
                                + appearance_count)
                                as usize]
                                .hint_text_config,
                        )
                        .ignore_render_layer(true);
                    hint_text.background_alpha = alpha;
                    hint_text.alpha = alpha;
                    hint_text.display_info.hidden = if let Some(display_info) =
                        background_resource.display_display_info()
                        && display_info.hidden
                    {
                        true
                    } else {
                        hint_text.display_info.hidden
                    };

                    // 更新Text样式。
                    text = text
                        .tags(
                            if switch.use_switch_tags {
                                &switch.tags
                            } else {
                                &[]
                            },
                            false,
                        )
                        .from_config(
                            &switch.appearance[(switch.state * animation_count as u32
                                + appearance_count)
                                as usize]
                                .text_config,
                        );

                    switch.last_frame_hovered = hovered;
                    switch.last_frame_clicked = clicked;

                    self.replace_resource(&format!("{name}Text"), text)?;
                    self.replace_resource(&format!("{name}HintText"), hint_text)?;
                    self.replace_resource(name, switch)?;
                    self.replace_resource(&format!("{name}Background"), background)?;

                    self.use_resource(
                        &format!("{name}Background"),
                        "rust_constructor::Background",
                        ui,
                        ctx,
                    )?;
                    self.use_resource(&format!("{name}Text"), "rust_constructor::Text", ui, ctx)?;
                    if alpha != 0 {
                        self.use_resource(
                            &format!("{name}HintText"),
                            "rust_constructor::Text",
                            ui,
                            ctx,
                        )?;
                    };
                }
                "rust_constructor::ResourcePanel" => {
                    let mut resource_panel = self
                        .get_resource::<ResourcePanel>(name, "rust_constructor::ResourcePanel")?
                        .clone();
                    let background = self
                        .get_resource::<Background>(
                            &format!("{name}Background"),
                            "rust_constructor::Background",
                        )?
                        .clone();
                    let background_resource: Box<dyn BasicFrontResource> =
                        match background.background_type.clone() {
                            BackgroundType::CustomRect(_) => Box::new(
                                self.get_resource::<CustomRect>(
                                    &format!("{name}Background"),
                                    "rust_constructor::CustomRect",
                                )?
                                .clone(),
                            ),
                            BackgroundType::Image(_) => Box::new(
                                self.get_resource::<Image>(
                                    &format!("{name}Background"),
                                    "rust_constructor::Image",
                                )?
                                .clone(),
                            ),
                        };
                    let (mut position_size_config, mut position, mut size) = (
                        background_resource
                            .display_basic_front_resource_config()
                            .position_size_config,
                        background_resource.display_position(),
                        background_resource.display_size(),
                    );
                    let rect = Rect::from_min_size(position.into(), size.into());
                    resource_panel.scrolled = [false, false];
                    if resource_panel.resizable.contains(&true)
                        || resource_panel.movable.contains(&true)
                    {
                        position_size_config.x_location_grid = [0_f32, 0_f32];
                        position_size_config.y_location_grid = [0_f32, 0_f32];
                        position_size_config.x_size_grid = [0_f32, 0_f32];
                        position_size_config.y_size_grid = [0_f32, 0_f32];
                    };
                    if resource_panel.min_size[0] < 10_f32 {
                        resource_panel.min_size[0] = 10_f32;
                    };
                    if resource_panel.min_size[1] < 10_f32 {
                        resource_panel.min_size[1] = 10_f32;
                    };
                    if position_size_config.origin_size[0] < resource_panel.min_size[0] {
                        position_size_config.origin_size[0] = resource_panel.min_size[0];
                    };
                    if position_size_config.origin_size[1] < resource_panel.min_size[1] {
                        position_size_config.origin_size[1] = resource_panel.min_size[1];
                    };
                    [position, size] = self.position_size_processor(position_size_config, ctx);
                    let scroll_delta: [f32; 2] = if resource_panel.use_smooth_scroll_delta {
                        ui.input(|i| i.smooth_scroll_delta).into()
                    } else {
                        ui.input(|i| i.raw_scroll_delta).into()
                    };
                    let [x_scroll_delta, y_scroll_delta] =
                        if scroll_delta[0].abs() >= scroll_delta[1].abs() {
                            [
                                if resource_panel.reverse_scroll_direction[0] {
                                    -scroll_delta[0]
                                } else {
                                    scroll_delta[0]
                                },
                                0_f32,
                            ]
                        } else {
                            [
                                0_f32,
                                if resource_panel.reverse_scroll_direction[1] {
                                    -scroll_delta[1]
                                } else {
                                    scroll_delta[1]
                                },
                            ]
                        };
                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
                        && !resource_panel.hidden
                    {
                        if let Some(index) = self.get_render_layer_resource(
                            &format!("{name}Background"),
                            match background.background_type {
                                BackgroundType::CustomRect(_) => "rust_constructor::CustomRect",
                                BackgroundType::Image(_) => "rust_constructor::Image",
                            },
                        ) && self.resource_get_focus(index, mouse_pos.into())
                        {
                            if ui.input(|i| i.pointer.primary_pressed())
                                && Rect::from_min_size(position.into(), size.into())
                                    .contains(mouse_pos)
                            {
                                self.request_jump_render_list(
                                    RequestMethod::Id(RustConstructorId {
                                        name: format!("{name}Background"),
                                        discern_type: match background.background_type {
                                            BackgroundType::CustomRect(_) => {
                                                "rust_constructor::CustomRect"
                                            }
                                            BackgroundType::Image(_) => "rust_constructor::Image",
                                        }
                                        .to_string(),
                                    }),
                                    RequestType::Top,
                                )
                                .unwrap();
                                let mut update_list = Vec::new();
                                for rcr in &self.rust_constructor_resource {
                                    if self
                                        .basic_front_resource_list
                                        .contains(&rcr.id.discern_type)
                                        && let Some(panel_name) =
                                            self.get_tag("panel_name", &rcr.content.display_tags())
                                        && panel_name.1 == name
                                    {
                                        update_list.push(rcr.id.clone());
                                    };
                                }
                                for id in update_list {
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(id),
                                        RequestType::Top,
                                    );
                                }
                                if let ScrollBarDisplayMethod::Always(ref background_type, _, _) =
                                    resource_panel.scroll_bar_display_method
                                {
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{name}XScroll"),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => {
                                                    "rust_constructor::CustomRect"
                                                }
                                                BackgroundType::Image(_) => {
                                                    "rust_constructor::Image"
                                                }
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{name}YScroll"),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => {
                                                    "rust_constructor::CustomRect"
                                                }
                                                BackgroundType::Image(_) => {
                                                    "rust_constructor::Image"
                                                }
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                };
                                if let ScrollBarDisplayMethod::OnlyScroll(
                                    ref background_type,
                                    _,
                                    _,
                                ) = resource_panel.scroll_bar_display_method
                                {
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{name}XScroll"),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => {
                                                    "rust_constructor::CustomRect"
                                                }
                                                BackgroundType::Image(_) => {
                                                    "rust_constructor::Image"
                                                }
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{name}YScroll"),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => {
                                                    "rust_constructor::CustomRect"
                                                }
                                                BackgroundType::Image(_) => {
                                                    "rust_constructor::Image"
                                                }
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                };
                            };
                            let top_rect = Rect::from_min_size(
                                [position[0], position[1]].into(),
                                [size[0], 3_f32].into(),
                            );
                            let bottom_rect = Rect::from_min_size(
                                [position[0], position[1] + size[1] - 3_f32].into(),
                                [size[0], 3_f32].into(),
                            );
                            let left_rect = Rect::from_min_size(
                                [position[0], position[1]].into(),
                                [3_f32, size[1]].into(),
                            );
                            let right_rect = Rect::from_min_size(
                                [position[0] + size[0] - 3_f32, position[1]].into(),
                                [3_f32, size[1]].into(),
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
                                                    mouse_pos.x - position[0],
                                                    mouse_pos.y - position[1],
                                                ],
                                            ))
                                        };
                                        if size[1] > resource_panel.min_size[1]
                                            && (resource_panel.max_size.is_none()
                                                || size[1] < resource_panel.max_size.unwrap()[1])
                                        {
                                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                        } else if resource_panel.max_size.is_some()
                                            && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                    mouse_pos.x - position[0],
                                                    mouse_pos.y - position[1],
                                                ],
                                            ))
                                        };
                                        if size[1] > resource_panel.min_size[1]
                                            && (resource_panel.max_size.is_none()
                                                || size[1] < resource_panel.max_size.unwrap()[1])
                                        {
                                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                        } else if resource_panel.max_size.is_some()
                                            && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                    mouse_pos.x - position[0],
                                                    mouse_pos.y - position[1],
                                                ],
                                            ))
                                        };
                                        if size[0] > resource_panel.min_size[0]
                                            && (resource_panel.max_size.is_none()
                                                || size[0] < resource_panel.max_size.unwrap()[0])
                                        {
                                            ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                        } else if resource_panel.max_size.is_some()
                                            && size[0] >= resource_panel.max_size.unwrap()[0]
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
                                                    mouse_pos.x - position[0],
                                                    mouse_pos.y - position[1],
                                                ],
                                            ))
                                        };
                                        if size[0] > resource_panel.min_size[0]
                                            && (resource_panel.max_size.is_none()
                                                || size[0] < resource_panel.max_size.unwrap()[0])
                                        {
                                            ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                        } else if resource_panel.max_size.is_some()
                                            && size[0] >= resource_panel.max_size.unwrap()[0]
                                        {
                                            ctx.set_cursor_icon(CursorIcon::ResizeWest);
                                        } else {
                                            ctx.set_cursor_icon(CursorIcon::ResizeEast);
                                        };
                                    };
                                }
                                [true, false, true, false] => {
                                    match [resource_panel.resizable[0], resource_panel.resizable[2]]
                                    {
                                        [true, true] => {
                                            if resource_panel.last_frame_mouse_status.is_none()
                                                && ui.input(|i| i.pointer.primary_pressed())
                                            {
                                                resource_panel.last_frame_mouse_status = Some((
                                                    mouse_pos.into(),
                                                    ClickAim::LeftTopResize,
                                                    [
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                                || size[1] > resource_panel.min_size[1]
                                                    && (resource_panel.max_size.is_none()
                                                        || size[1]
                                                            < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[1] > resource_panel.min_size[1]
                                                && (resource_panel.max_size.is_none()
                                                    || size[1]
                                                        < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                            } else if resource_panel.max_size.is_some()
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                    match [resource_panel.resizable[1], resource_panel.resizable[3]]
                                    {
                                        [true, true] => {
                                            if resource_panel.last_frame_mouse_status.is_none()
                                                && ui.input(|i| i.pointer.primary_pressed())
                                            {
                                                resource_panel.last_frame_mouse_status = Some((
                                                    mouse_pos.into(),
                                                    ClickAim::RightBottomResize,
                                                    [
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                                || size[1] > resource_panel.min_size[1]
                                                    && (resource_panel.max_size.is_none()
                                                        || size[1]
                                                            < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[1] > resource_panel.min_size[1]
                                                && (resource_panel.max_size.is_none()
                                                    || size[1]
                                                        < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                            } else if resource_panel.max_size.is_some()
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                    match [resource_panel.resizable[0], resource_panel.resizable[3]]
                                    {
                                        [true, true] => {
                                            if resource_panel.last_frame_mouse_status.is_none()
                                                && ui.input(|i| i.pointer.primary_pressed())
                                            {
                                                resource_panel.last_frame_mouse_status = Some((
                                                    mouse_pos.into(),
                                                    ClickAim::RightTopResize,
                                                    [
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                                || size[1] > resource_panel.min_size[1]
                                                    && (resource_panel.max_size.is_none()
                                                        || size[1]
                                                            < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[1] > resource_panel.min_size[1]
                                                && (resource_panel.max_size.is_none()
                                                    || size[1]
                                                        < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                            } else if resource_panel.max_size.is_some()
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                    match [resource_panel.resizable[1], resource_panel.resizable[2]]
                                    {
                                        [true, true] => {
                                            if resource_panel.last_frame_mouse_status.is_none()
                                                && ui.input(|i| i.pointer.primary_pressed())
                                            {
                                                resource_panel.last_frame_mouse_status = Some((
                                                    mouse_pos.into(),
                                                    ClickAim::LeftBottomResize,
                                                    [
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                                || size[1] > resource_panel.min_size[1]
                                                    && (resource_panel.max_size.is_none()
                                                        || size[1]
                                                            < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[0] > resource_panel.min_size[0]
                                                && (resource_panel.max_size.is_none()
                                                    || size[0]
                                                        < resource_panel.max_size.unwrap()[0])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                            } else if resource_panel.max_size.is_some()
                                                && size[0] >= resource_panel.max_size.unwrap()[0]
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
                                                        mouse_pos.x - position[0],
                                                        mouse_pos.y - position[1],
                                                    ],
                                                ))
                                            };
                                            if size[1] > resource_panel.min_size[1]
                                                && (resource_panel.max_size.is_none()
                                                    || size[1]
                                                        < resource_panel.max_size.unwrap()[1])
                                            {
                                                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                            } else if resource_panel.max_size.is_some()
                                                && size[1] >= resource_panel.max_size.unwrap()[1]
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
                                        [mouse_pos.x - position[0], mouse_pos.y - position[1]],
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
                            if resource_panel.scroll_length_method[0].is_some()
                                && x_scroll_delta != 0_f32
                                && rect.contains(mouse_pos)
                            {
                                resource_panel.scrolled[0] = true;
                                resource_panel.scroll_progress[0] = if resource_panel
                                    .scroll_progress[0]
                                    + -x_scroll_delta * resource_panel.scroll_sensitivity
                                    > resource_panel.scroll_length[0]
                                {
                                    resource_panel.scroll_length[0]
                                } else if resource_panel.scroll_progress[0]
                                    + -x_scroll_delta * resource_panel.scroll_sensitivity
                                    > 0_f32
                                {
                                    resource_panel.scroll_progress[0]
                                        + -x_scroll_delta * resource_panel.scroll_sensitivity
                                } else {
                                    0_f32
                                };
                            };
                            if resource_panel.scroll_length_method[1].is_some()
                                && y_scroll_delta != 0_f32
                                && rect.contains(mouse_pos)
                            {
                                resource_panel.scrolled[1] = true;
                                resource_panel.scroll_progress[1] = if resource_panel
                                    .scroll_progress[1]
                                    + -y_scroll_delta * resource_panel.scroll_sensitivity
                                    > resource_panel.scroll_length[1]
                                {
                                    resource_panel.scroll_length[1]
                                } else if resource_panel.scroll_progress[1]
                                    + -y_scroll_delta * resource_panel.scroll_sensitivity
                                    > 0_f32
                                {
                                    resource_panel.scroll_progress[1]
                                        + -y_scroll_delta * resource_panel.scroll_sensitivity
                                } else {
                                    0_f32
                                };
                            };
                        } else if ui.input(|i| i.pointer.primary_released()) {
                            resource_panel.last_frame_mouse_status = None;
                        };
                    };
                    if let Some((mouse_pos, click_aim, offset)) =
                        resource_panel.last_frame_mouse_status
                    {
                        match click_aim {
                            ClickAim::LeftTopResize => {
                                if position[0] - mouse_pos[0] + size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || position[0] - mouse_pos[0] + size[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] +=
                                        position[0] - mouse_pos[0];
                                    position_size_config.origin_position[0] = mouse_pos[0];
                                } else if resource_panel.max_size.is_some()
                                    && position[0] - mouse_pos[0] + size[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_position[0] -=
                                        resource_panel.max_size.unwrap()[0]
                                            - position_size_config.origin_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                } else {
                                    position_size_config.origin_position[0] += position_size_config
                                        .origin_size[0]
                                        - resource_panel.min_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                };
                                if position[1] - mouse_pos[1] + size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || position[1] - mouse_pos[1] + size[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] +=
                                        position[1] - mouse_pos[1];
                                    position_size_config.origin_position[1] = mouse_pos[1];
                                } else if resource_panel.max_size.is_some()
                                    && position[1] - mouse_pos[1] + size[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_position[1] -=
                                        resource_panel.max_size.unwrap()[1]
                                            - position_size_config.origin_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                } else {
                                    position_size_config.origin_position[1] += position_size_config
                                        .origin_size[1]
                                        - resource_panel.min_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                };
                                if size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || size[0] < resource_panel.max_size.unwrap()[0])
                                    || size[1] > resource_panel.min_size[1]
                                        && (resource_panel.max_size.is_none()
                                            || size[1] < resource_panel.max_size.unwrap()[1])
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                                } else if resource_panel.max_size.is_some()
                                    && size[0] >= resource_panel.max_size.unwrap()[0]
                                    && size[1] >= resource_panel.max_size.unwrap()[1]
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouthEast);
                                } else {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorthWest)
                                };
                            }
                            ClickAim::RightBottomResize => {
                                if mouse_pos[0] - position[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[0] - position[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] =
                                        mouse_pos[0] - position[0];
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[0] - position[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                } else {
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                };
                                if mouse_pos[1] - position[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[1] - position[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] =
                                        mouse_pos[1] - position[1];
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[1] - position[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                } else {
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                };
                                if size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || size[0] < resource_panel.max_size.unwrap()[0])
                                    || size[1] > resource_panel.min_size[1]
                                        && (resource_panel.max_size.is_none()
                                            || size[1] < resource_panel.max_size.unwrap()[1])
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNwSe);
                                } else if resource_panel.max_size.is_some()
                                    && size[0] >= resource_panel.max_size.unwrap()[0]
                                    && size[1] >= resource_panel.max_size.unwrap()[1]
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorthWest);
                                } else {
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouthEast)
                                };
                            }
                            ClickAim::RightTopResize => {
                                if mouse_pos[0] - position[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[0] - position[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] =
                                        mouse_pos[0] - position[0];
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[0] - position[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                } else {
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                };
                                if position[1] - mouse_pos[1] + size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || position[1] - mouse_pos[1] + size[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] +=
                                        position[1] - mouse_pos[1];
                                    position_size_config.origin_position[1] = mouse_pos[1];
                                } else if resource_panel.max_size.is_some()
                                    && position[1] - mouse_pos[1] + size[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_position[1] -=
                                        resource_panel.max_size.unwrap()[1]
                                            - position_size_config.origin_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                } else {
                                    position_size_config.origin_position[1] += position_size_config
                                        .origin_size[1]
                                        - resource_panel.min_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                };
                                if size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || size[0] < resource_panel.max_size.unwrap()[0])
                                    || size[1] > resource_panel.min_size[1]
                                        && (resource_panel.max_size.is_none()
                                            || size[1] < resource_panel.max_size.unwrap()[1])
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                                } else if resource_panel.max_size.is_some()
                                    && size[0] >= resource_panel.max_size.unwrap()[0]
                                    && size[1] >= resource_panel.max_size.unwrap()[1]
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouthWest);
                                } else {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorthEast)
                                };
                            }
                            ClickAim::LeftBottomResize => {
                                if position[0] - mouse_pos[0] + size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || position[0] - mouse_pos[0] + size[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] +=
                                        position[0] - mouse_pos[0];
                                    position_size_config.origin_position[0] = mouse_pos[0];
                                } else if resource_panel.max_size.is_some()
                                    && position[0] - mouse_pos[0] + size[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_position[0] -=
                                        resource_panel.max_size.unwrap()[0]
                                            - position_size_config.origin_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                } else {
                                    position_size_config.origin_position[0] += position_size_config
                                        .origin_size[0]
                                        - resource_panel.min_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                };
                                if mouse_pos[1] - position[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[1] - position[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] =
                                        mouse_pos[1] - position[1];
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[1] - position[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                } else {
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                };
                                if size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || size[0] < resource_panel.max_size.unwrap()[0])
                                    || size[1] > resource_panel.min_size[1]
                                        && (resource_panel.max_size.is_none()
                                            || size[1] < resource_panel.max_size.unwrap()[1])
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNeSw);
                                } else if resource_panel.max_size.is_some()
                                    && size[0] >= resource_panel.max_size.unwrap()[0]
                                    && size[1] >= resource_panel.max_size.unwrap()[1]
                                {
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorthEast);
                                } else {
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouthWest)
                                };
                            }
                            ClickAim::TopResize => {
                                if position[1] - mouse_pos[1] + size[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || position[1] - mouse_pos[1] + size[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] +=
                                        position[1] - mouse_pos[1];
                                    position_size_config.origin_position[1] = mouse_pos[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                } else if resource_panel.max_size.is_some()
                                    && position[1] - mouse_pos[1] + size[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_position[1] -=
                                        resource_panel.max_size.unwrap()[1]
                                            - position_size_config.origin_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                                } else {
                                    position_size_config.origin_position[1] += position_size_config
                                        .origin_size[1]
                                        - resource_panel.min_size[1];
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                                };
                            }
                            ClickAim::BottomResize => {
                                if mouse_pos[1] - position[1] > resource_panel.min_size[1]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[1] - position[1]
                                            < resource_panel.max_size.unwrap()[1])
                                {
                                    position_size_config.origin_size[1] =
                                        mouse_pos[1] - position[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[1] - position[1]
                                        >= resource_panel.max_size.unwrap()[1]
                                {
                                    position_size_config.origin_size[1] =
                                        resource_panel.max_size.unwrap()[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                                } else {
                                    position_size_config.origin_size[1] =
                                        resource_panel.min_size[1];
                                    ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                                };
                            }
                            ClickAim::LeftResize => {
                                if position[0] - mouse_pos[0] + size[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || position[0] - mouse_pos[0] + size[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] +=
                                        position[0] - mouse_pos[0];
                                    position_size_config.origin_position[0] = mouse_pos[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                } else if resource_panel.max_size.is_some()
                                    && position[0] - mouse_pos[0] + size[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_position[0] -=
                                        resource_panel.max_size.unwrap()[0]
                                            - position_size_config.origin_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeEast);
                                } else {
                                    position_size_config.origin_position[0] += position_size_config
                                        .origin_size[0]
                                        - resource_panel.min_size[0];
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeWest);
                                };
                            }
                            ClickAim::RightResize => {
                                if mouse_pos[0] - position[0] > resource_panel.min_size[0]
                                    && (resource_panel.max_size.is_none()
                                        || mouse_pos[0] - position[0]
                                            < resource_panel.max_size.unwrap()[0])
                                {
                                    position_size_config.origin_size[0] =
                                        mouse_pos[0] - position[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                                } else if resource_panel.max_size.is_some()
                                    && mouse_pos[0] - position[0]
                                        >= resource_panel.max_size.unwrap()[0]
                                {
                                    position_size_config.origin_size[0] =
                                        resource_panel.max_size.unwrap()[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeWest);
                                } else {
                                    position_size_config.origin_size[0] =
                                        resource_panel.min_size[0];
                                    ctx.set_cursor_icon(CursorIcon::ResizeEast);
                                };
                            }
                            ClickAim::Move => {
                                if resource_panel.movable[0] {
                                    position_size_config.origin_position[0] =
                                        mouse_pos[0] - offset[0];
                                };
                                if resource_panel.movable[1] {
                                    position_size_config.origin_position[1] =
                                        mouse_pos[1] - offset[1];
                                };
                            }
                        };
                    };
                    [position, size] = self.position_size_processor(position_size_config, ctx);
                    let background_type = match background.background_type.clone() {
                        BackgroundType::CustomRect(config) => BackgroundType::CustomRect(
                            config
                                .position_size_config(Some(position_size_config))
                                .hidden(Some(resource_panel.hidden)),
                        ),
                        BackgroundType::Image(config) => BackgroundType::Image(
                            config
                                .position_size_config(Some(position_size_config))
                                .hidden(Some(resource_panel.hidden)),
                        ),
                    };
                    self.replace_resource(
                        &format!("{name}Background"),
                        background.clone().background_type(&background_type).clone(),
                    )?;
                    self.use_resource(
                        &format!("{name}Background"),
                        "rust_constructor::Background",
                        ui,
                        ctx,
                    )?;
                    let mut resource_point_list: Vec<([f32; 2], [f32; 2], [bool; 2])> = Vec::new();
                    let mut use_resource_list = Vec::new();
                    let mut replace_resource_list = Vec::new();
                    for rcr in &self.rust_constructor_resource {
                        if self
                            .basic_front_resource_list
                            .contains(&rcr.id.discern_type)
                            && let Some(panel_name) =
                                self.get_tag("panel_name", &rcr.content.display_tags())
                            && panel_name.1 == name
                        {
                            if let [Some(citer_name), Some(citer_type)] = [
                                self.get_tag("citer_name", &rcr.content.display_tags()),
                                self.get_tag("citer_type", &rcr.content.display_tags()),
                            ] {
                                if !use_resource_list
                                    .iter()
                                    .any(|x| x == &[citer_name.1.clone(), citer_type.1.clone()])
                                {
                                    use_resource_list.push([citer_name.1, citer_type.1]);
                                };
                            } else if !use_resource_list
                                .iter()
                                .any(|x| x == &[rcr.id.name.clone(), rcr.id.discern_type.clone()])
                            {
                                use_resource_list
                                    .push([rcr.id.name.clone(), rcr.id.discern_type.clone()]);
                            };
                            let mut basic_front_resource: Box<dyn BasicFrontResource> = match &*rcr
                                .id
                                .discern_type
                            {
                                "rust_constructor::Image" => Box::new(
                                    rcr.content
                                        .as_any()
                                        .downcast_ref::<Image>()
                                        .unwrap()
                                        .clone(),
                                ),
                                "rust_constructor::Text" => Box::new(
                                    rcr.content.as_any().downcast_ref::<Text>().unwrap().clone(),
                                ),
                                "rust_constructor::CustomRect" => Box::new(
                                    rcr.content
                                        .as_any()
                                        .downcast_ref::<CustomRect>()
                                        .unwrap()
                                        .clone(),
                                ),
                                _ => {
                                    unreachable!()
                                }
                            };
                            if !resource_panel
                                .resource_storage
                                .iter()
                                .any(|x| x.id == rcr.id)
                            {
                                resource_panel.resource_storage.push(PanelStorage {
                                    id: rcr.id.clone(),
                                    ignore_render_layer: if let Some(display_info) =
                                        basic_front_resource.display_display_info()
                                    {
                                        display_info.ignore_render_layer
                                    } else {
                                        false
                                    },
                                    hidden: if let Some(display_info) =
                                        basic_front_resource.display_display_info()
                                    {
                                        display_info.hidden
                                    } else {
                                        false
                                    },
                                    origin_size: basic_front_resource
                                        .display_position_size_config()
                                        .origin_size,
                                });
                            };
                            let enable_scrolling = [
                                self.get_tag("disable_x_scrolling", &rcr.content.display_tags())
                                    .is_none(),
                                self.get_tag("disable_y_scrolling", &rcr.content.display_tags())
                                    .is_none(),
                            ];
                            let offset = basic_front_resource.display_position_size_config().offset;
                            basic_front_resource.modify_position_size_config(
                                basic_front_resource
                                    .display_position_size_config()
                                    .x_location_grid(0_f32, 0_f32)
                                    .y_location_grid(0_f32, 0_f32)
                                    .x_size_grid(0_f32, 0_f32)
                                    .y_size_grid(0_f32, 0_f32)
                                    .offset(
                                        if enable_scrolling[0] {
                                            -resource_panel.scroll_progress[0]
                                        } else {
                                            offset[0]
                                        },
                                        if enable_scrolling[1] {
                                            -resource_panel.scroll_progress[1]
                                        } else {
                                            offset[1]
                                        },
                                    ),
                            );
                            match resource_panel.layout.panel_margin {
                                PanelMargin::Vertical(
                                    [top, bottom, left, right],
                                    move_to_bottom,
                                ) => {
                                    let mut modify_y = 0_f32;
                                    let [default_x_position, default_y_position] =
                                        match resource_panel.layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0] + (size[0] / x[1] as f32 * x[0] as f32),
                                                position[1] + (size[1] / y[1] as f32 * y[0] as f32),
                                            ],
                                        };
                                    let default_x_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .0
                                    {
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
                                    let default_y_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .1
                                    {
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
                                            && default_y_position - top + modify_y < point.1[1]
                                            && default_x_position
                                                + basic_front_resource.display_size()[0]
                                                + right
                                                > point.0[0]
                                            && default_y_position
                                                + basic_front_resource.display_size()[1]
                                                + bottom
                                                + modify_y
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
                                                    - basic_front_resource.display_size()[1]
                                                    - bottom;
                                            };
                                        };
                                    }
                                    let real_x_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .0
                                    {
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
                                    let real_y_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .1
                                    {
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
                                    let default_storage = if let Some(resource_storage) =
                                        resource_panel
                                            .resource_storage
                                            .iter()
                                            .find(|x| x.id == rcr.id)
                                    {
                                        (true, resource_storage.origin_size)
                                    } else {
                                        (false, [0_f32, 0_f32])
                                    };
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_size(
                                                if basic_front_resource.display_position()[0]
                                                    < position[0] + size[0]
                                                {
                                                    if resource_panel.auto_shrink[0]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                    {
                                                        position_size_config.origin_size[0]
                                                            - (basic_front_resource
                                                                .display_position()[0]
                                                                - position[0])
                                                            - right
                                                    } else if default_storage.0 {
                                                        if default_storage.1[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        {
                                                            position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        } else {
                                                            default_storage.1[0]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                                if basic_front_resource.display_position()[1]
                                                    < position[1] + size[1]
                                                {
                                                    if resource_panel.auto_shrink[1]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                    {
                                                        position_size_config.origin_size[1]
                                                            - (basic_front_resource
                                                                .display_position()[1]
                                                                - position[1])
                                                            - bottom
                                                    } else if default_storage.0 {
                                                        if default_storage.1[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        {
                                                            position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        } else {
                                                            default_storage.1[1]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                            )
                                            .origin_position(real_x_position, real_y_position),
                                    );
                                    replace_resource_list.push((
                                        basic_front_resource.display_position_size_config(),
                                        [rcr.id.name.clone(), rcr.id.discern_type.clone()],
                                    ));
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
                                        enable_scrolling,
                                    ));
                                }
                                PanelMargin::Horizontal(
                                    [top, bottom, left, right],
                                    move_to_right,
                                ) => {
                                    let mut modify_x = 0_f32;
                                    let [default_x_position, default_y_position] =
                                        match resource_panel.layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0] + (size[0] / x[1] as f32 * x[0] as f32),
                                                position[1] + (size[1] / y[1] as f32 * y[0] as f32),
                                            ],
                                        };
                                    let default_x_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .0
                                    {
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
                                    let default_y_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .1
                                    {
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
                                        if default_x_position - left + modify_x < point.1[0]
                                            && default_y_position - top < point.1[1]
                                            && default_x_position
                                                + basic_front_resource.display_size()[0]
                                                + right
                                                + modify_x
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
                                    let real_x_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .0
                                    {
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
                                    let real_y_position = match basic_front_resource
                                        .display_position_size_config()
                                        .display_method
                                        .1
                                    {
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
                                    let default_storage = if let Some(resource_storage) =
                                        resource_panel
                                            .resource_storage
                                            .iter()
                                            .find(|x| x.id == rcr.id)
                                    {
                                        (true, resource_storage.origin_size)
                                    } else {
                                        (false, [0_f32, 0_f32])
                                    };
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_size(
                                                if basic_front_resource.display_position()[0]
                                                    < position[0] + size[0]
                                                {
                                                    if resource_panel.auto_shrink[0]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                    {
                                                        position_size_config.origin_size[0]
                                                            - (basic_front_resource
                                                                .display_position()[0]
                                                                - position[0])
                                                            - right
                                                    } else if default_storage.0 {
                                                        if default_storage.1[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        {
                                                            position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        } else {
                                                            default_storage.1[0]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                                if basic_front_resource.display_position()[1]
                                                    < position[1] + size[1]
                                                {
                                                    if resource_panel.auto_shrink[1]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                    {
                                                        position_size_config.origin_size[1]
                                                            - (basic_front_resource
                                                                .display_position()[1]
                                                                - position[1])
                                                            - bottom
                                                    } else if default_storage.0 {
                                                        if default_storage.1[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        {
                                                            position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        } else {
                                                            default_storage.1[1]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                            )
                                            .origin_position(real_x_position, real_y_position),
                                    );
                                    replace_resource_list.push((
                                        basic_front_resource.display_position_size_config(),
                                        [rcr.id.name.clone(), rcr.id.discern_type.clone()],
                                    ));
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
                                        enable_scrolling,
                                    ));
                                }
                                PanelMargin::None([top, bottom, left, right], influence_layout) => {
                                    let [default_x_position, default_y_position] =
                                        match resource_panel.layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0] + (size[0] / x[1] as f32 * x[0] as f32),
                                                position[1] + (size[1] / y[1] as f32 * y[0] as f32),
                                            ],
                                        };
                                    let default_storage = if let Some(resource_storage) =
                                        resource_panel
                                            .resource_storage
                                            .iter()
                                            .find(|x| x.id == rcr.id)
                                    {
                                        (true, resource_storage.origin_size)
                                    } else {
                                        (false, [0_f32, 0_f32])
                                    };
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_size(
                                                if basic_front_resource.display_position()[0]
                                                    < position[0] + size[0]
                                                {
                                                    if resource_panel.auto_shrink[0]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                    {
                                                        position_size_config.origin_size[0]
                                                            - (basic_front_resource
                                                                .display_position()[0]
                                                                - position[0])
                                                            - right
                                                    } else if default_storage.0 {
                                                        if default_storage.1[0]
                                                            > position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        {
                                                            position_size_config.origin_size[0]
                                                                - (basic_front_resource
                                                                    .display_position()[0]
                                                                    - position[0])
                                                                - right
                                                        } else {
                                                            default_storage.1[0]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[0]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                                if basic_front_resource.display_position()[1]
                                                    < position[1] + size[1]
                                                {
                                                    if resource_panel.auto_shrink[1]
                                                        && basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                    {
                                                        position_size_config.origin_size[1]
                                                            - (basic_front_resource
                                                                .display_position()[1]
                                                                - position[1])
                                                            - bottom
                                                    } else if default_storage.0 {
                                                        if default_storage.1[1]
                                                            > position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        {
                                                            position_size_config.origin_size[1]
                                                                - (basic_front_resource
                                                                    .display_position()[1]
                                                                    - position[1])
                                                                - bottom
                                                        } else {
                                                            default_storage.1[1]
                                                        }
                                                    } else {
                                                        basic_front_resource
                                                            .display_position_size_config()
                                                            .origin_size[1]
                                                    }
                                                } else {
                                                    0_f32
                                                },
                                            )
                                            .origin_position(
                                                default_x_position,
                                                default_y_position,
                                            ),
                                    );
                                    replace_resource_list.push((
                                        basic_front_resource.display_position_size_config(),
                                        [rcr.id.name.clone(), rcr.id.discern_type.clone()],
                                    ));
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
                                            enable_scrolling,
                                        ));
                                    };
                                }
                            };
                        };
                    }
                    for (new_position_size_config, [name, discern_type]) in replace_resource_list {
                        let default_storage = if let Some(resource_storage) =
                            resource_panel.resource_storage.iter().find(|x| {
                                x.id == RustConstructorId {
                                    name: name.clone(),
                                    discern_type: discern_type.clone(),
                                }
                            }) {
                            [
                                true,
                                resource_storage.ignore_render_layer,
                                resource_storage.hidden,
                            ]
                        } else {
                            [false, true, true]
                        };
                        match &*discern_type {
                            "rust_constructor::CustomRect" => {
                                let mut custom_rect = self
                                    .get_resource::<CustomRect>(&name, &discern_type)?
                                    .clone();
                                custom_rect.basic_front_resource_config.position_size_config =
                                    new_position_size_config;
                                custom_rect.display_info.ignore_render_layer = if resource_panel
                                    .last_frame_mouse_status
                                    .is_some()
                                    || [x_scroll_delta, y_scroll_delta].iter().any(|x| *x != 0_f32)
                                {
                                    true
                                } else if default_storage[0] {
                                    default_storage[1]
                                } else {
                                    custom_rect.display_info.ignore_render_layer
                                };
                                custom_rect.basic_front_resource_config.clip_rect =
                                    Some(position_size_config);
                                custom_rect.display_info.hidden = if resource_panel.hidden {
                                    true
                                } else if default_storage[0] {
                                    default_storage[2]
                                } else {
                                    custom_rect.display_info.hidden
                                };
                                self.replace_resource(&name, custom_rect)?;
                            }
                            "rust_constructor::Image" => {
                                let mut image =
                                    self.get_resource::<Image>(&name, &discern_type)?.clone();
                                image.basic_front_resource_config.position_size_config =
                                    new_position_size_config;
                                image.display_info.ignore_render_layer = if resource_panel
                                    .last_frame_mouse_status
                                    .is_some()
                                    || [x_scroll_delta, y_scroll_delta].iter().any(|x| *x != 0_f32)
                                {
                                    true
                                } else if default_storage[0] {
                                    default_storage[1]
                                } else {
                                    image.display_info.ignore_render_layer
                                };
                                image.basic_front_resource_config.clip_rect =
                                    Some(position_size_config);
                                image.display_info.hidden = resource_panel.hidden;
                                self.replace_resource(&name, image)?;
                            }
                            "rust_constructor::Text" => {
                                let mut text =
                                    self.get_resource::<Text>(&name, &discern_type)?.clone();
                                text.basic_front_resource_config.position_size_config =
                                    new_position_size_config;
                                text.display_info.ignore_render_layer = if resource_panel
                                    .last_frame_mouse_status
                                    .is_some()
                                    || [x_scroll_delta, y_scroll_delta].iter().any(|x| *x != 0_f32)
                                {
                                    true
                                } else if default_storage[0] {
                                    default_storage[1]
                                } else {
                                    text.display_info.ignore_render_layer
                                };
                                text.auto_fit = [false, false];
                                text.basic_front_resource_config.clip_rect =
                                    Some(position_size_config);
                                text.display_info.hidden = resource_panel.hidden;
                                self.replace_resource(&name, text)?;
                            }
                            _ => unreachable!(),
                        }
                    }
                    for info in use_resource_list {
                        self.use_resource(&info[0], &info[1], ui, ctx)?;
                    }
                    if let Some(horizontal_scroll_length_method) =
                        resource_panel.scroll_length_method[0]
                    {
                        resource_panel.scroll_length[0] = match horizontal_scroll_length_method {
                            ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                            ScrollLengthMethod::AutoFit(expand) => {
                                let mut length = -background_resource.display_size()[0];
                                match resource_panel.layout.panel_margin {
                                    PanelMargin::Horizontal(_, _) => {
                                        for storage in &resource_panel.resource_storage {
                                            length += storage.origin_size[0];
                                        }
                                    }
                                    PanelMargin::Vertical(_, _) | PanelMargin::None(_, _) => {
                                        for storage in &resource_panel.resource_storage {
                                            length = if storage.origin_size[0]
                                                - background_resource.display_size()[0]
                                                > length
                                            {
                                                storage.origin_size[0]
                                                    - background_resource.display_size()[0]
                                            } else {
                                                length
                                            };
                                        }
                                    }
                                }
                                if length >= 0_f32 {
                                    length + expand.abs()
                                } else {
                                    0_f32
                                }
                            }
                        };
                        if resource_panel.scroll_progress[0] > resource_panel.scroll_length[0] {
                            resource_panel.scroll_progress[0] = resource_panel.scroll_length[0];
                        };
                    };
                    if let Some(vertical_scroll_length_method) =
                        resource_panel.scroll_length_method[1]
                    {
                        resource_panel.scroll_length[1] = match vertical_scroll_length_method {
                            ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                            ScrollLengthMethod::AutoFit(expand) => {
                                let mut length = -background_resource.display_size()[1];
                                match resource_panel.layout.panel_margin {
                                    PanelMargin::Vertical(_, _) => {
                                        for storage in &resource_panel.resource_storage {
                                            length += storage.origin_size[1];
                                        }
                                    }
                                    PanelMargin::Horizontal(_, _) | PanelMargin::None(_, _) => {
                                        for storage in &resource_panel.resource_storage {
                                            length = if storage.origin_size[1]
                                                - background_resource.display_size()[1]
                                                > length
                                            {
                                                storage.origin_size[1]
                                                    - background_resource.display_size()[1]
                                            } else {
                                                length
                                            };
                                        }
                                    }
                                }
                                if length >= 0_f32 {
                                    length + expand.abs()
                                } else {
                                    0_f32
                                }
                            }
                        };
                        if resource_panel.scroll_progress[1] > resource_panel.scroll_length[1] {
                            resource_panel.scroll_progress[1] = resource_panel.scroll_length[1];
                        };
                    };
                    match resource_panel.scroll_bar_display_method {
                        ScrollBarDisplayMethod::Always(ref config, margin, width) => {
                            let line_length = if resource_panel.scroll_length[1] == 0_f32 {
                                (size[0] - margin[0] * 2_f32)
                                    * (size[0] / (resource_panel.scroll_length[0] + size[0]))
                            } else {
                                (size[0] - width - margin[1] - margin[0] * 2_f32)
                                    * (size[0] / (resource_panel.scroll_length[0] + size[0]))
                            };
                            let line_position = if resource_panel.scroll_length[1] == 0_f32 {
                                position[0]
                                    + margin[0]
                                    + (size[0] - margin[0] * 2_f32 - line_length)
                                        * (resource_panel.scroll_progress[0]
                                            / resource_panel.scroll_length[0])
                            } else {
                                position[0]
                                    + margin[0]
                                    + (size[0]
                                        - margin[0] * 2_f32
                                        - width
                                        - margin[1]
                                        - line_length)
                                        * (resource_panel.scroll_progress[0]
                                            / resource_panel.scroll_length[0])
                            };
                            self.replace_resource(
                                &format!("{name}XScroll"),
                                background.clone().background_type(&match config.clone() {
                                    BackgroundType::CustomRect(config) => {
                                        BackgroundType::CustomRect(
                                            config
                                                .ignore_render_layer(Some(true))
                                                .hidden(Some(resource_panel.hidden))
                                                .position_size_config(Some(
                                                    PositionSizeConfig::default()
                                                        .display_method(
                                                            HorizontalAlign::Left,
                                                            VerticalAlign::Bottom,
                                                        )
                                                        .origin_position(
                                                            line_position,
                                                            position[1] + size[1] - margin[1],
                                                        )
                                                        .origin_size(line_length, width),
                                                )),
                                        )
                                    }
                                    BackgroundType::Image(config) => BackgroundType::Image(
                                        config
                                            .ignore_render_layer(Some(true))
                                            .hidden(Some(resource_panel.hidden))
                                            .position_size_config(Some(
                                                PositionSizeConfig::default()
                                                    .display_method(
                                                        HorizontalAlign::Left,
                                                        VerticalAlign::Bottom,
                                                    )
                                                    .origin_position(
                                                        line_position,
                                                        position[1] + size[1] - margin[1],
                                                    )
                                                    .origin_size(line_length, width),
                                            )),
                                    ),
                                }),
                            )?;
                            self.use_resource(
                                &format!("{name}XScroll"),
                                "rust_constructor::Background",
                                ui,
                                ctx,
                            )?;
                            let line_length = if resource_panel.scroll_length[0] == 0_f32 {
                                (size[1] - margin[0] * 2_f32)
                                    * (size[1] / (resource_panel.scroll_length[1] + size[1]))
                            } else {
                                (size[1] - width - margin[1] - margin[0] * 2_f32)
                                    * (size[1] / (resource_panel.scroll_length[1] + size[1]))
                            };
                            let line_position = if resource_panel.scroll_length[0] == 0_f32 {
                                position[1]
                                    + margin[0]
                                    + (size[1] - margin[0] * 2_f32 - line_length)
                                        * (resource_panel.scroll_progress[1]
                                            / resource_panel.scroll_length[1])
                            } else {
                                position[1]
                                    + margin[0]
                                    + (size[1]
                                        - margin[0] * 2_f32
                                        - width
                                        - margin[1]
                                        - line_length)
                                        * (resource_panel.scroll_progress[1]
                                            / resource_panel.scroll_length[1])
                            };
                            self.replace_resource(
                                &format!("{name}YScroll"),
                                background.background_type(&match config.clone() {
                                    BackgroundType::CustomRect(config) => {
                                        BackgroundType::CustomRect(
                                            config
                                                .ignore_render_layer(Some(true))
                                                .hidden(Some(resource_panel.hidden))
                                                .position_size_config(Some(
                                                    PositionSizeConfig::default()
                                                        .display_method(
                                                            HorizontalAlign::Right,
                                                            VerticalAlign::Top,
                                                        )
                                                        .origin_position(
                                                            position[0] + size[0] - margin[1],
                                                            line_position,
                                                        )
                                                        .origin_size(width, line_length),
                                                )),
                                        )
                                    }
                                    BackgroundType::Image(config) => BackgroundType::Image(
                                        config
                                            .ignore_render_layer(Some(true))
                                            .hidden(Some(resource_panel.hidden))
                                            .position_size_config(Some(
                                                PositionSizeConfig::default()
                                                    .display_method(
                                                        HorizontalAlign::Right,
                                                        VerticalAlign::Top,
                                                    )
                                                    .origin_position(
                                                        position[0] + size[0] - margin[1],
                                                        line_position,
                                                    )
                                                    .origin_size(width, line_length),
                                            )),
                                    ),
                                }),
                            )?;
                            self.use_resource(
                                &format!("{name}YScroll"),
                                "rust_constructor::Background",
                                ui,
                                ctx,
                            )?;
                        }
                        ScrollBarDisplayMethod::OnlyScroll(ref config, margin, width) => {
                            resource_panel.scroll_bar_alpha[0] = if resource_panel.scrolled[0] {
                                self.reset_split_time(&format!("{name}ScrollBarXAlphaStart"))?;
                                255
                            } else if self.timer.now_time
                                - self.get_split_time(&format!("{name}ScrollBarXAlphaStart"))?[0]
                                >= 1_f32
                                && self.timer.now_time
                                    - self.get_split_time(&format!("{name}ScrollBarXAlpha"))?[0]
                                    >= self.tick_interval
                            {
                                self.reset_split_time(&format!("{name}ScrollBarXAlpha"))?;
                                resource_panel.scroll_bar_alpha[0].saturating_sub(10)
                            } else {
                                resource_panel.scroll_bar_alpha[0]
                            };
                            resource_panel.scroll_bar_alpha[1] = if resource_panel.scrolled[1] {
                                self.reset_split_time(&format!("{name}ScrollBarYAlphaStart"))?;
                                255
                            } else if self.timer.now_time
                                - self.get_split_time(&format!("{name}ScrollBarYAlphaStart"))?[0]
                                >= 1_f32
                                && self.timer.now_time
                                    - self.get_split_time(&format!("{name}ScrollBarYAlpha"))?[0]
                                    >= self.tick_interval
                            {
                                self.reset_split_time(&format!("{name}ScrollBarYAlpha"))?;
                                resource_panel.scroll_bar_alpha[1].saturating_sub(10)
                            } else {
                                resource_panel.scroll_bar_alpha[1]
                            };
                            let line_length = if resource_panel.scroll_length[1] == 0_f32 {
                                (size[0] - margin[0] * 2_f32)
                                    * (size[0] / (resource_panel.scroll_length[0] + size[0]))
                            } else {
                                (size[0] - width - margin[1] - margin[0] * 2_f32)
                                    * (size[0] / (resource_panel.scroll_length[0] + size[0]))
                            };
                            let line_position = if resource_panel.scroll_length[1] == 0_f32 {
                                position[0]
                                    + margin[0]
                                    + (size[0] - margin[0] * 2_f32 - line_length)
                                        * (resource_panel.scroll_progress[0]
                                            / resource_panel.scroll_length[0])
                            } else {
                                position[0]
                                    + margin[0]
                                    + (size[0]
                                        - margin[0] * 2_f32
                                        - width
                                        - margin[1]
                                        - line_length)
                                        * (resource_panel.scroll_progress[0]
                                            / resource_panel.scroll_length[0])
                            };
                            self.replace_resource(
                                &format!("{name}XScroll"),
                                background.clone().background_type(&match config.clone() {
                                    BackgroundType::CustomRect(config) => {
                                        BackgroundType::CustomRect(
                                            config
                                                .ignore_render_layer(Some(true))
                                                .hidden(Some(resource_panel.hidden))
                                                .position_size_config(Some(
                                                    PositionSizeConfig::default()
                                                        .display_method(
                                                            HorizontalAlign::Left,
                                                            VerticalAlign::Bottom,
                                                        )
                                                        .origin_position(
                                                            line_position,
                                                            position[1] + size[1] - margin[1],
                                                        )
                                                        .origin_size(line_length, width),
                                                ))
                                                .alpha(Some(resource_panel.scroll_bar_alpha[0]))
                                                .border_alpha(Some(
                                                    resource_panel.scroll_bar_alpha[0],
                                                )),
                                        )
                                    }
                                    BackgroundType::Image(config) => BackgroundType::Image(
                                        config
                                            .ignore_render_layer(Some(true))
                                            .hidden(Some(resource_panel.hidden))
                                            .position_size_config(Some(
                                                PositionSizeConfig::default()
                                                    .display_method(
                                                        HorizontalAlign::Left,
                                                        VerticalAlign::Bottom,
                                                    )
                                                    .origin_position(
                                                        line_position,
                                                        position[1] + size[1] - margin[1],
                                                    )
                                                    .origin_size(line_length, width),
                                            ))
                                            .alpha(Some(resource_panel.scroll_bar_alpha[0]))
                                            .background_alpha(Some(
                                                resource_panel.scroll_bar_alpha[0],
                                            ))
                                            .overlay_alpha(Some(
                                                resource_panel.scroll_bar_alpha[0],
                                            )),
                                    ),
                                }),
                            )?;
                            self.use_resource(
                                &format!("{name}XScroll"),
                                "rust_constructor::Background",
                                ui,
                                ctx,
                            )?;
                            let line_length = if resource_panel.scroll_length[0] == 0_f32 {
                                (size[1] - margin[0] * 2_f32)
                                    * (size[1] / (resource_panel.scroll_length[1] + size[1]))
                            } else {
                                (size[1] - width - margin[1] - margin[0] * 2_f32)
                                    * (size[1] / (resource_panel.scroll_length[1] + size[1]))
                            };
                            let line_position = if resource_panel.scroll_length[0] == 0_f32 {
                                position[1]
                                    + margin[0]
                                    + (size[1] - margin[0] * 2_f32 - line_length)
                                        * (resource_panel.scroll_progress[1]
                                            / resource_panel.scroll_length[1])
                            } else {
                                position[1]
                                    + margin[0]
                                    + (size[1]
                                        - margin[0] * 2_f32
                                        - width
                                        - margin[1]
                                        - line_length)
                                        * (resource_panel.scroll_progress[1]
                                            / resource_panel.scroll_length[1])
                            };
                            self.replace_resource(
                                &format!("{name}YScroll"),
                                background.clone().background_type(&match config.clone() {
                                    BackgroundType::CustomRect(config) => {
                                        BackgroundType::CustomRect(
                                            config
                                                .ignore_render_layer(Some(true))
                                                .hidden(Some(resource_panel.hidden))
                                                .position_size_config(Some(
                                                    PositionSizeConfig::default()
                                                        .display_method(
                                                            HorizontalAlign::Right,
                                                            VerticalAlign::Top,
                                                        )
                                                        .origin_position(
                                                            position[0] + size[0] - margin[1],
                                                            line_position,
                                                        )
                                                        .origin_size(width, line_length),
                                                ))
                                                .alpha(Some(resource_panel.scroll_bar_alpha[1]))
                                                .border_alpha(Some(
                                                    resource_panel.scroll_bar_alpha[1],
                                                )),
                                        )
                                    }
                                    BackgroundType::Image(config) => BackgroundType::Image(
                                        config
                                            .ignore_render_layer(Some(true))
                                            .hidden(Some(resource_panel.hidden))
                                            .position_size_config(Some(
                                                PositionSizeConfig::default()
                                                    .display_method(
                                                        HorizontalAlign::Right,
                                                        VerticalAlign::Top,
                                                    )
                                                    .origin_position(
                                                        position[0] + size[0] - margin[1],
                                                        line_position,
                                                    )
                                                    .origin_size(width, line_length),
                                            ))
                                            .alpha(Some(resource_panel.scroll_bar_alpha[1]))
                                            .background_alpha(Some(
                                                resource_panel.scroll_bar_alpha[1],
                                            ))
                                            .overlay_alpha(Some(
                                                resource_panel.scroll_bar_alpha[1],
                                            )),
                                    ),
                                }),
                            )?;
                            self.use_resource(
                                &format!("{name}YScroll"),
                                "rust_constructor::Background",
                                ui,
                                ctx,
                            )?;
                        }
                        ScrollBarDisplayMethod::Hidden => {}
                    };
                    self.replace_resource(name, resource_panel.clone())?;
                }
                _ => {}
            };
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource \"{name}({discern_type})\" not found."),
            })
        }
    }

    /// 切换页面。
    pub fn switch_page(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(name, "rust_constructor::PageData")?;
        page_data.enter_page_updated = false;
        self.timer.start_time = self.timer.total_time;
        self.current_page = name.to_string();
        self.update_timer();
        Ok(())
    }

    /// 输出字体资源。
    pub fn get_font(&self, name: &str) -> Result<FontDefinitions, RustConstructorError> {
        let font = self.get_resource::<Font>(name, "rust_constructor::Font")?;
        Ok(font.font_definitions.clone())
    }

    /// 将所有已添加到RC的字体资源添加到egui中。
    pub fn register_all_fonts(&self, ctx: &Context) {
        let mut font_definitions_amount = FontDefinitions::default();
        let mut fonts = Vec::new();
        for i in 0..self.rust_constructor_resource.len() {
            if let Some(font) = self.rust_constructor_resource[i]
                .content
                .as_any()
                .downcast_ref::<Font>()
            {
                fonts.push((
                    self.rust_constructor_resource[i].id.name.clone(),
                    font.font_definitions.clone(),
                ));
            };
        }
        for i in &fonts {
            // 从 font_def 中提取对应字体的 Arc<FontData>
            if let Some(font_data) = i.1.font_data.get(&i.0) {
                font_definitions_amount
                    .font_data
                    .insert(i.0.clone(), Arc::clone(font_data));
                font_definitions_amount
                    .families
                    .entry(FontFamily::Name(i.0.clone().into()))
                    .or_default()
                    .push(i.0.clone());
            };

            // 将字体添加到字体列表中
            font_definitions_amount
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, i.0.to_owned());

            font_definitions_amount
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, i.0.to_owned());
        }
        ctx.set_fonts(font_definitions_amount);
    }

    /// 处理最基本的位置计算。
    pub fn position_size_processor(
        &self,
        position_size_config: PositionSizeConfig,
        ctx: &Context,
    ) -> [[f32; 2]; 2] {
        let mut position = [0_f32, 0_f32];
        let mut size = [0_f32, 0_f32];
        size[0] = match position_size_config.x_size_grid[0] {
            0_f32 => position_size_config.origin_size[0],
            _ => {
                (ctx.available_rect().width() / position_size_config.x_size_grid[1]
                    * position_size_config.x_size_grid[0])
                    + position_size_config.origin_size[0]
            }
        };
        size[1] = match position_size_config.y_size_grid[0] {
            0_f32 => position_size_config.origin_size[1],
            _ => {
                (ctx.available_rect().height() / position_size_config.y_size_grid[1]
                    * position_size_config.y_size_grid[0])
                    + position_size_config.origin_size[1]
            }
        };
        position[0] = match position_size_config.x_location_grid[1] {
            0_f32 => position_size_config.origin_position[0],
            _ => {
                (ctx.available_rect().width() / position_size_config.x_location_grid[1]
                    * position_size_config.x_location_grid[0])
                    + position_size_config.origin_position[0]
            }
        };
        position[1] = match position_size_config.y_location_grid[1] {
            0_f32 => position_size_config.origin_position[1],
            _ => {
                (ctx.available_rect().height() / position_size_config.y_location_grid[1]
                    * position_size_config.y_location_grid[0])
                    + position_size_config.origin_position[1]
            }
        };
        match position_size_config.display_method.0 {
            HorizontalAlign::Left => {}
            HorizontalAlign::Center => position[0] -= size[0] / 2.0,
            HorizontalAlign::Right => position[0] -= size[0],
        };
        match position_size_config.display_method.1 {
            VerticalAlign::Top => {}
            VerticalAlign::Center => position[1] -= size[1] / 2.0,
            VerticalAlign::Bottom => position[1] -= size[1],
        };
        position[0] += position_size_config.offset[0];
        position[1] += position_size_config.offset[1];
        [position, size]
    }

    /// 检查页面是否已完成首次加载。
    pub fn check_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        let page_data = self
            .get_resource::<PageData>(name, "rust_constructor::PageData")?
            .clone();
        if !page_data.change_page_updated {
            self.new_page_update(name)?;
        };
        Ok(page_data.change_page_updated)
    }

    /// 检查页面是否已完成加载。
    pub fn check_enter_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(name, "rust_constructor::PageData")?;
        page_data.enter_page_updated = true;
        Ok(page_data.enter_page_updated)
    }

    /// 进入新页面时的更新。
    pub fn new_page_update(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(name, "rust_constructor::PageData")?;
        page_data.change_page_updated = true;
        self.timer.start_time = self.timer.total_time;
        self.update_timer();
        Ok(())
    }

    /// 更新帧数。
    pub fn update_frame_stats(&mut self) {
        let current_time = self.timer.total_time;
        if let Some(last) = self.last_frame_time {
            let delta = current_time - last;
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
    pub fn reset_split_time(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let new_time = [self.timer.now_time, self.timer.total_time];
        let split_time = self.get_resource_mut::<SplitTime>(name, "rust_constructor::SplitTime")?;
        split_time.time = new_time;
        Ok(())
    }

    /// 输出分段时间。
    pub fn get_split_time(&self, name: &str) -> Result<[f32; 2], RustConstructorError> {
        let split_time = self.get_resource::<SplitTime>(name, "rust_constructor::SplitTime")?;
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

    /// 修改变量资源。
    pub fn modify_variable<T: Debug + 'static>(
        &mut self,
        name: &str,
        value: Option<T>,
    ) -> Result<(), RustConstructorError> {
        let variable = self.get_resource_mut::<Variable<T>>(name, "rust_constructor::Variable")?;
        variable.value = value;
        Ok(())
    }

    /// 取出变量。
    pub fn get_variable<T: Debug + Clone + 'static>(
        &self,
        name: &str,
    ) -> Result<Option<T>, RustConstructorError> {
        if let Ok(variable) = self.get_resource::<Variable<T>>(name, "rust_constructor::Variable") {
            Ok(variable.value.clone())
        } else if self
            .check_resource_exists(name, "rust_constructor::Variable")
            .is_none()
        {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!(
                    "Resource \"{name}(rust_constructor::Variable<T>)\" not found."
                ),
            })
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource \"{name}(rust_constructor::Variable<T>)\" is mismatched."
                ),
            })
        }
    }

    /// 输出图片纹理。
    pub fn get_image_texture(
        &self,
        name: &str,
    ) -> Result<Option<DebugTextureHandle>, RustConstructorError> {
        let image_texture =
            self.get_resource::<ImageTexture>(name, "rust_constructor::ImageTexture")?;
        Ok(image_texture.texture.clone())
    }

    /// 重置图片纹理。
    pub fn reset_image_texture(
        &mut self,
        name: &str,
        path: &str,
        flip: [bool; 2],
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        let image_texture =
            self.get_resource_mut::<ImageTexture>(name, "rust_constructor::ImageTexture")?;
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
            let texture = ctx.load_texture(name, color_image, TextureOptions::LINEAR);
            image_texture.texture = Some(DebugTextureHandle::new(&texture));
            image_texture.cite_path = path.to_string();
        } else {
            return Err(RustConstructorError {
                error_id: "ImageLoadFailed".to_string(),
                description: format!(
                    "Failed to load an image from the path \"{}\".",
                    image_texture.cite_path
                ),
            });
        }
        Ok(())
    }

    /// 修改开关的启用状态。
    pub fn set_switch_enable(
        &mut self,
        name: &str,
        enable: bool,
    ) -> Result<(), RustConstructorError> {
        let switch = self.get_resource_mut::<Switch>(name, "rust_constructor::Switch")?;
        switch.enable = enable;
        Ok(())
    }

    /// 查找指定开关的常用判定字段集合。
    pub fn check_switch_data(&self, name: &str) -> Result<SwitchData, RustConstructorError> {
        let switch = self.get_resource::<Switch>(name, "rust_constructor::Switch")?;
        Ok(SwitchData {
            switched: switch.switched,
            last_frame_clicked: switch.last_frame_clicked,
            state: switch.state,
        })
    }
}
