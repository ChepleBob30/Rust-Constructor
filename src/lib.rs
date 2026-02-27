//! # Rust Constructor V2
//!
//! A cross-platform `GUI` framework built on `egui`, the simplest way to develop `GUI` projects with `Rust`
//!
//! 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式
//!
//! ## Overview 概述
//!
//! `Rust Constructor` is a fully functional GUI framework that leverages the power of `egui` to provide
//! a simple and intuitive instrument for building cross-platform applications.
//!
//! `Rust Constructor`是一个功能全面的GUI框架，它利用了`egui`的强大功能，为构建跨平台应用程序提供了一个简单直观的工具。
//!
//! ## Quick Start 快速入门
//!
//! ```rust
//! pub struct RcApp {
//!     pub inner: rust_constructor::app::App,
//! }
//!
//! impl eframe::App for RcApp {
//!     fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             ui.label("Hello world");
//!         });
//!     }
//! }
//!
//!
//! eframe::run_native(
//!     "Example App",
//!     eframe::NativeOptions::default(),
//!     Box::new(|_| {
//!         Ok(Box::new(RcApp {
//!             inner: rust_constructor::app::App::default(),
//!         }))
//!     }),
//! )
//! .unwrap();
//! ```
//!
//! ## Documentation 文档
//!
//! - [Rust Constructor Guide](https://github.com/ChepleBob30/Rust-Constructor-Guide)
//! - [GitHub Repository](https://github.com/ChepleBob30/Rust-Constructor)
//! - [Binder 必达](https://github.com/Binder-organize) - Other projects from our organization 我们组织的其他项目
pub mod advance_front;
pub mod app;
pub mod background;
pub mod basic_front;
use std::{
    any::Any,
    error::Error,
    fmt::{Debug, Display, Formatter},
    time::Instant,
    vec::Vec,
};

/// Core trait for managing Rust Constructor resources uniformly.
///
/// 统一管理Rust Constructor资源的核心特性。
///
/// This trait provides a common interface for all GUI resources in the framework,
/// allowing for the acquisition and modification of specific resources and their details.
///
/// 此特征为框架中的所有GUI资源提供了一个公共接口，允许获取具体资源及其细节并对其进行修改。
pub trait RustConstructorResource: Debug {
    /// Returns a reference to the resource as `Any` for extract the specific type.
    ///
    /// 以`Any`返回对资源的引用，用于取出具体类型。
    ///
    /// This allows downcasting to the concrete type when the actual type is known.
    ///
    /// 当实际类型已知时，允许向下转换到具体类型。
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the resource as `Any` for extract the specific type.
    ///
    /// 以`Any`返回对资源的可变引用，用于取出具体类型。
    ///
    /// This allows mutable downcasting when the actual type is known.
    ///
    /// 当实际类型已知时，允许向下可变转换到具体类型。
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Retrieves the display info field for this resource.
    ///
    /// 取出此资源的显示信息字段。
    ///
    /// Returns `Some(DisplayInfo)` if the resource has display info field,
    /// or `None` if it doesn't have display info field.
    ///
    /// 如果资源具有显示信息字段则返回`Some(DisplayInfo)`，如果资源没有显示信息字段则返回`None`。
    fn display_display_info(&self) -> Option<DisplayInfo>;

    /// Updates the display info field for this resource.
    ///
    /// 更新此资源的显示信息字段。
    fn modify_display_info(&mut self, display_info: DisplayInfo);

    /// Returns all tags associated with this resource.
    ///
    /// 返回与此资源关联的所有标签。
    ///
    /// Tags are stored as key-value pairs (`[String; 2]`) and can be used
    /// for categorization, filtering, or metadata storage.
    ///
    /// 标签以键值对（`[String; 2]`）的形式存储，可用于分类、过滤或元数据存储。
    fn display_tags(&self) -> Vec<[String; 2]>;

    /// Updates the tags for this resource.
    ///
    /// 更新此资源的标签。
    ///
    /// # Arguments
    ///
    /// * `replace` - If `true`, replaces all existing tags;
    ///   if `false`, merges with existing tags.
    ///
    /// # 参数
    ///
    /// * `replace` - 若为`true`，则替换所有现有的标签；
    ///   若为`false`，则与现有标签合并。
    fn modify_tags(&mut self, tags: &[[String; 2]], replace: bool);
}

/// Trait for managing basic front resources that are displayed to the user.
///
/// 用于管理显示给用户的基本前端资源的特征。
///
/// This trait extends `RustConstructorResource` with additional methods specific
/// to visual elements.
///
/// 此特征扩展了`RustConstructorResource`，添加了特定视觉元素的方法。
pub trait BasicFrontResource: RustConstructorResource {
    /// Returns the complete basic resource config.
    ///
    /// 返回完整的基本前端资源配置。
    ///
    /// This includes both position/size config and clipping information.
    ///
    /// 包括位置/大小配置和裁剪信息。
    fn display_basic_front_resource_config(&self) -> BasicFrontResourceConfig;

    /// Returns the position and size config for this resource.
    ///
    /// 返回此资源的位置和大小配置。
    ///
    /// Includes grid-based positioning, alignment settings, and offset values.
    ///
    /// 包括基于网格的定位、对齐设置和偏移值。
    fn display_position_size_config(&self) -> PositionSizeConfig;

    /// Returns the clipping rectangle config if this resource has one.
    ///
    /// 返回裁剪矩形配置（如果此资源有的话）。
    ///
    /// Clipping rectangles define the visible area of the resource.
    ///
    /// 裁剪矩形定义资源的可见区域。
    ///
    /// Returns `None` if no clipping is applied.
    ///
    /// 如果没有应用裁剪矩形，则返回`None`。
    fn display_clip_rect(&self) -> Option<PositionSizeConfig>;

    /// Returns the current display position of the resource.
    ///
    /// 返回资源的当前显示位置。
    ///
    /// The position is returned as `[x, y]` coordinates.
    ///
    /// 位置以`[x, y]`坐标返回。
    fn display_position(&self) -> [f32; 2];

    /// Returns the current display size of the resource.
    ///
    /// 返回资源的当前显示大小。
    ///
    /// The size is returned as `[width, height]`.
    ///
    /// 大小以`[width, height]`返回。
    fn display_size(&self) -> [f32; 2];

    /// Updates the complete basic resource config.
    ///
    /// 更新完整的前端资源配置。
    fn modify_basic_front_resource_config(
        &mut self,
        basic_front_resource_config: BasicFrontResourceConfig,
    );

    /// Updates the position and size config.
    ///
    /// 更新位置和大小配置。
    fn modify_position_size_config(&mut self, position_size_config: PositionSizeConfig);

    /// Updates the clipping rectangle config.
    ///
    /// 更新裁剪矩形配置。
    fn modify_clip_rect(&mut self, clip_rect: Option<PositionSizeConfig>);
}

/// Unique identifier for Rust Constructor resources.
///
/// Rust Constructor资源的唯一标识符。
///
/// This struct combines a resource name and type to create a unique identifier
/// that can be used to reference resources throughout the application.
///
/// 这个结构体结合了资源名称和类型来创建一个唯一的标识符，可以在整个应用程序中用来引用资源。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RustConstructorId {
    /// Unique name identifying the resource instance, Resources of different
    /// types can have the same name.
    ///
    /// 标识资源实例的唯一名称，不同类型的资源可以重名。
    pub name: String,

    /// Type of the resource (e.g., `Image`).
    ///
    /// 资源的类型（例如`Image`）。
    pub discern_type: String,
}

/// Container for Rust Constructor resources with type-erased storage.
///
/// 具有类型擦除存储的Rust Constructor资源的容器。
///
/// This struct pairs a resource identifier with the actual resource content,
/// allowing for heterogeneous storage of different resource types while
/// maintaining the ability to identify and manage them individually.
///
/// 这个结构体将资源标识符与实际资源内容配对，允许不同资源类型的异构存储，同时保持单独识别和管理它们的能力。
///
/// # Type Erasure 类型擦除
///
/// The `content` field uses a `Box<dyn RustConstructorResource>` to store
/// any type that implements the `RustConstructorResource` trait, enabling
/// polymorphic behavior and storage.
///
/// `content`字段使用`Box<dyn RustConstructorResource>`来存储任何实现`RustConstructorResource`
/// 特征的类型，从而启用多态行为和存储。
#[derive(Debug)]
pub struct RustConstructorResourceBox {
    /// Unique identifier for the contained resource.
    ///
    /// 所包含资源的唯一标识符。
    pub id: RustConstructorId,

    /// Type-erased resource content.
    ///
    /// 类型擦除的资源内容。
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

/// Config for basic front resources.
///
/// 基本前端资源的配置。
///
/// This struct contains all the essential config needed for positioning,
/// sizing, and clipping visual elements in the GUI.
///
/// 这个结构体包含了在GUI中定位、调整大小和裁剪可视元素所需的所有配置。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct BasicFrontResourceConfig {
    /// Config for position, size, and layout properties.
    ///
    /// 位置、大小和布局属性的配置。
    pub position_size_config: PositionSizeConfig,

    /// Optional clipping rectangle that defines the visible area.
    /// If `None`, the resource is rendered without clipping.
    ///
    /// 可选的裁剪矩形，用于定义可见区域。如果为`None`，则渲染资源时不进行裁剪。
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

/// Config for positioning and sizing resources in a flexible grid system.
///
/// 用于在灵活的网格系统中定位和调整资源大小的配置。
///
/// This struct provides comprehensive control over how resources are positioned
/// and sized within the GUI, supporting grid-based layouts with multiple alignment options.
///
/// 这个结构体提供了对GUI中资源如何定位和大小的全面控制，支持具有多种对齐选项的基于网格的布局。
///
/// # Grid System 网格系统
///
/// The grid system allows for relative positioning and sizing using fractions of
/// the available space, making layouts responsive and adaptable to different screen sizes.
///
/// 网格系统允许使用可用空间的一部分进行相对定位和大小调整，使布局响应并适应不同的屏幕尺寸。
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositionSizeConfig {
    /// Absolute position coordinates in pixels (`[x, y]`).
    ///
    /// 以像素为单位的绝对位置坐标（`[x, y]`）。
    pub origin_position: [f32; 2],

    /// Absolute size dimensions in pixels (`[width, height]`).
    ///
    /// 以像素为单位的绝对尺寸（`[width, height]`）。
    pub origin_size: [f32; 2],

    /// Grid-based X-axis positioning (`[numerator, denominator]`).
    ///
    /// 基于网格的x轴定位（`[numerator, denominator]`）。
    ///
    /// Position = (total_width * numerator / denominator)
    ///
    /// 位置 = （总宽度 * 分子 / 分母）
    pub x_location_grid: [f32; 2],

    /// Grid-based Y-axis positioning (`[numerator, denominator]`).
    ///
    /// 基于网格的y轴定位（`[numerator, denominator]`）。
    ///
    /// Position = (total_height * numerator / denominator)
    ///
    /// 位置 = （总高度 * 分子 / 分母）
    pub y_location_grid: [f32; 2],

    /// Grid-based X-axis sizing (`[numerator, denominator]`).
    ///
    /// 基于网格的x轴缩放（`[numerator, denominator]`）。
    ///
    /// Width = (total_width * numerator / denominator)
    ///
    /// 宽度 = （总宽度 * 分子 / 分母）
    pub x_size_grid: [f32; 2],

    /// Grid-based Y-axis sizing (`[numerator, denominator]`).
    ///
    /// 基于网格的y轴缩放（`[numerator, denominator]`）。
    ///
    /// Height = (total_height * numerator / denominator)
    ///
    /// 高度 = （总高度 * 分子 / 分母）
    pub y_size_grid: [f32; 2],

    /// Horizontal and vertical alignment methods.
    ///
    /// 水平和垂直对齐方法。
    pub display_method: (HorizontalAlign, VerticalAlign),

    /// Additional offset in pixels (`[x_offset, y_offset]`).
    ///
    /// 额外的像素偏移量（`[x_offset, y_offset]`）。
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

/// Timer for tracking application and page runtimes.
///
/// 用于跟踪应用程序和页面运行时间的计时器。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Timer {
    /// Time when the current page was entered, in seconds.
    ///
    /// 进入当前页面的时间（秒）。
    pub start_time: f32,

    /// Total runtime of the application since start, in seconds.
    ///
    /// 应用程序自启动以来的总运行时间（秒）。
    pub total_time: f32,

    /// Core timer instance for precise timing.
    ///
    /// 用于精确计时的核心计时器实例。
    pub timer: Instant,

    /// Runtime of the current page, in seconds.
    ///
    /// 当前页面的运行时间（秒）。
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

/// Error type for Rust Constructor framework.
///
/// Rust Constructor框架的错误类型。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RustConstructorError {
    /// Identifier for the error type.
    ///
    /// 错误类型的标识符。
    pub error_id: String,

    /// Description of the error.
    ///
    /// 对此错误的描述。
    pub description: String,
}

impl Display for RustConstructorError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for RustConstructorError {}

/// Horizontal alignment options for UI elements.
///
/// UI元素的水平对齐选项。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HorizontalAlign {
    /// Align to the left.
    ///
    /// 左对齐。
    #[default]
    Left,
    /// Center align horizontally.
    ///
    /// 居中对齐。
    Center,
    /// Align to the right.
    ///
    /// 右对齐。
    Right,
}

/// Vertical alignment options for UI elements.
///
/// UI元素的垂直对齐选项。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerticalAlign {
    /// Align to the top.
    ///
    /// 顶部对齐。
    #[default]
    Top,
    /// Center align vertically.
    ///
    /// 居中对齐。
    Center,
    /// Align to the bottom.
    ///
    /// 底部对齐。
    Bottom,
}

/// Defines the placement of borders relative to the element's bounds.
///
/// 定义边框相对于元素边界的放置方式。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BorderKind {
    /// Border is drawn inside the element's bounds, reducing the content area.
    ///
    /// 边框在元素边界内部绘制，减少内容区域。
    #[default]
    Inside,
    /// Border is centered on the element's bounds, half inside and half outside.
    ///
    /// 边框以元素边界为中心，一半在内部一半在外部。
    Middle,
    /// Border is drawn outside the element's bounds, preserving the content area.
    ///
    /// 边框在元素边界外部绘制，保持内容区域不变。
    Outside,
}

/// Config for rendering.
///
/// 渲染的配置。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RenderConfig {
    /// Line with width and color.
    ///
    /// 线段，包含宽度和颜色。
    Line(f32, [u8; 4]),
    /// Rectangle with corner radius, fill color, border color, border width, and border kind.
    ///
    /// 矩形，包含圆角半径，填充颜色，边框颜色，边框宽度，边框类型。
    Rect([u8; 4], [u8; 4], [u8; 4], f32, BorderKind),
}

/// Display config for resources, controlling visibility and rendering behavior.
///
/// 资源的显示配置，控制可见性和渲染行为。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisplayInfo {
    /// Enables or disables the resource. If false, the resource is not processed.
    ///
    /// 启用或禁用资源。如果为false，资源不会被处理。
    pub enable: bool,

    /// Hides the resource visually but keeps it active for event handling.
    ///
    /// 隐藏资源视觉显示但保持其事件处理活性。
    pub hidden: bool,

    /// If true, the resource ignores the rendering layer and does not occupy the mouse focus.
    ///
    /// 如果为true，资源忽略渲染层，不占用鼠标焦点。
    pub ignore_render_layer: bool,
}

/// The lookup method for requesting resources to skip the rendering queue.
///
/// 请求资源跳过渲染队列的查找方法。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestMethod {
    /// Request by resource identifier.
    ///
    /// 按资源标识符请求。
    Id(RustConstructorId),
    /// Request by resource reference.
    ///
    /// 通过资源引用请求。
    Citer(RustConstructorId),
}

/// Types of rendering layer requests.
///
/// 渲染层请求类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestType {
    /// Move resource to the top layer.
    ///
    /// 移动资源到顶层。
    Top,
    /// Move resource up by specified number of layers.
    ///
    /// 按指定层数向上移动资源。
    Up(usize),
}

/// Methods for displaying list information.
///
/// 用于显示列表信息的方法。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ListInfoMethod {
    /// Detailed display including resource and ID (with optional formatting).
    ///
    /// 详细显示，包括资源和ID（可选格式）。
    Detailed(bool),
    /// Simple display showing only resource IDs.
    ///
    /// 简单显示，只显示资源ID。
    #[default]
    Simple,
}
