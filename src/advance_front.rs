//! This file contains advanced front-end resources, which can be used to handle complex tasks.
//!
//! 此文件包含高级前端资源，高级前端资源可以用于处理复杂的任务。
use crate::{
    BasicFrontResource, Config, FrontResource, RustConstructorId, RustConstructorResource,
    basic_front::{CustomRectConfig, ImageConfig, TextConfig},
};
#[cfg(feature = "rc_bevy")]
use egui_bevy::PointerButton;
#[cfg(feature = "rc_standard")]
use egui_standard::PointerButton;
use std::any::Any;

/// Control the basic front resource type for Background selection.
///
/// 控制Background选择的基础前端资源类型。
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundType {
    /// Use an image as the background.
    ///
    /// 选择图像作为背景。
    Image(ImageConfig),

    /// Use a custom rectangle as the background.
    ///
    /// 选择自定义矩形作为背景。
    CustomRect(CustomRectConfig),
}

impl Default for BackgroundType {
    fn default() -> Self {
        BackgroundType::CustomRect(CustomRectConfig::default())
    }
}

/// Config options for background resources.
///
/// 背景资源的配置选项。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BackgroundConfig {
    /// Type of background to display.
    ///
    /// 要显示的背景类型。
    pub background_type: Option<BackgroundType>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl Config for BackgroundConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn convert_to_resource(&self) -> Box<dyn FrontResource> {
        Box::new(Background::default().from_config(self))
    }

    fn convert_from_resource(&self, resource: Box<dyn FrontResource>) -> Option<Box<dyn Config>> {
        if let Some(resource) = resource.as_any().downcast_ref::<Background>() {
            Some(Box::new(BackgroundConfig::from_resource(resource)))
        } else {
            None
        }
    }
}

impl BackgroundConfig {
    pub fn from_resource(resource: &Background) -> Self {
        Self {
            background_type: Some(resource.background_type.clone()),
            tags: Some(resource.tags.clone()),
        }
    }

    #[inline]
    pub fn background_type(mut self, background_type: Option<BackgroundType>) -> Self {
        self.background_type = background_type;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// Background resource for UI elements.
///
/// UI元素的背景资源。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Background {
    /// Type of background to display.
    ///
    /// 要显示的背景类型。
    pub background_type: BackgroundType,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Background {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn convert_to_front(&self) -> Option<Box<dyn FrontResource>> {
        Some(Box::new(self.clone()))
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_front_dyn(&self) -> Option<&dyn FrontResource> {
        Some(self)
    }

    fn convert_to_front_dyn_mut(&mut self) -> Option<&mut dyn FrontResource> {
        Some(self)
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
    }
}

impl FrontResource for Background {
    fn convert_to_config(&self) -> Box<dyn Config> {
        Box::new(BackgroundConfig::from_resource(self))
    }

    fn convert_from_config(&mut self, config: Box<dyn Config>) -> Option<Box<dyn FrontResource>> {
        if let Some(config) = config.as_any().downcast_ref::<BackgroundConfig>() {
            Some(Box::new(self.clone().from_config(config)))
        } else {
            None
        }
    }

    fn convert_to_original(&self) -> Box<dyn RustConstructorResource> {
        Box::new(self.clone())
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_original_dyn(&self) -> &dyn RustConstructorResource {
        self
    }

    fn convert_to_original_dyn_mut(&mut self) -> &mut dyn RustConstructorResource {
        self
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
    }
}

impl Background {
    pub fn from_config(mut self, config: &BackgroundConfig) -> Self {
        if let Some(ref background_type) = config.background_type {
            self.background_type = background_type.clone();
        };
        if let Some(ref tags) = config.tags {
            self.tags = tags.clone();
        };
        self
    }

    #[inline]
    pub fn background_type(mut self, background_type: &BackgroundType) -> Self {
        self.background_type = background_type.clone();
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

/// Methods for determining scroll bar length in panels.
///
/// 面板中滚动条长度的确定方法。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ScrollLengthMethod {
    /// Fixed length in pixels regardless of content size.
    ///
    /// 固定的像素长度，与内容大小无关。
    Fixed(f32),
    /// Automatically adjusts based on visible content proportion.
    ///
    /// 根据可见内容比例自动调整。
    AutoFit(f32),
}

/// Mouse click interaction types for panels.
///
/// 面板的鼠标点击交互类型。
///
/// Defines the intended action when clicking on different parts of a panel's border or interior.
///
/// 定义点击面板边框或内部不同区域时的预期操作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClickAim {
    /// Move the entire panel.
    ///
    /// 移动资源板。
    Move,
    /// Resize from the top edge.
    ///
    /// 在上方缩放。
    TopResize,
    /// Resize from the bottom edge.
    ///
    /// 在下方缩放。
    BottomResize,
    /// Resize from the left edge.
    ///
    /// 在左侧缩放。
    LeftResize,
    /// Resize from the right edge.
    ///
    /// 在右侧缩放。
    RightResize,
    /// Resize from the top-left corner.
    ///
    /// 在左上方缩放。
    LeftTopResize,
    /// Resize from the top-right corner.
    ///
    /// 在右上方缩放。
    RightTopResize,
    /// Resize from the bottom-left corner.
    ///
    /// 在左下方缩放。
    LeftBottomResize,
    /// Resize from the bottom-right corner.
    ///
    /// 在右下方缩放。
    RightBottomResize,
}

/// Scroll bar display behavior for panels.
///
/// 面板的滚动条显示行为。
///
/// Defines when and how the scroll bar should be displayed to the user.
///
/// 定义滚动条何时以及如何向用户显示。
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollBarDisplayMethod {
    /// Always show the scroll bar with specified background, offset, and width.
    ///
    /// 持续显示滚动条，使用指定的背景、偏移量和宽度。
    Always(BackgroundType, [f32; 2], f32),
    /// Show the scroll bar only during scrolling with specified properties.
    ///
    /// 仅在滚动时显示滚动条，使用指定的属性。
    OnlyScroll(BackgroundType, [f32; 2], f32),
    /// Never show the scroll bar (scrollable but no visual indicator).
    ///
    /// 隐藏滚动条（可滚动但无视觉指示器）。
    Hidden,
}

/// Margin config for resources within panels.
///
/// 面板内资源的外边距配置。
///
/// Defines spacing and layout behavior for resources placed inside panel containers.
///
/// 定义放置在面板容器内的资源的间距和布局行为。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PanelMargin {
    /// Vertical layout with margins [top, bottom, left, right] and reverse flag.
    ///
    /// 垂直布局，外边距为[top, bottom, left, right]，包含反转标志。
    Vertical([f32; 4], bool),
    /// Horizontal layout with margins [top, bottom, left, right] and reverse flag.
    ///
    /// 水平布局，外边距为[top, bottom, left, right]，包含反转标志。
    Horizontal([f32; 4], bool),
    /// No layout with margins [top, bottom, left, right] and influence layout flag.
    ///
    /// 无布局，外边距为[top, bottom, left, right]，包含影响布局标志。
    None([f32; 4], bool),
}

/// Panel layout config determining how resources are arranged within panels.
///
/// 面板布局配置，确定资源如何在面板内排列。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PanelLayout {
    /// Margin config for resources within the panel.
    ///
    /// 面板内资源的边距配置。
    pub panel_margin: PanelMargin,
    /// Location config for resources within the panel.
    ///
    /// 面板内资源的位置配置。
    pub panel_location: PanelLocation,
}

/// Positioning method for resources within panels.
///
/// 面板内资源的定位方式。
///
/// Defines how resources are positioned relative to their containing panel.
///
/// 定义资源相对于其包含面板的定位方法。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PanelLocation {
    /// Absolute positioning using pixel coordinates relative to panel's top-left corner.
    ///
    /// 依照此资源到资源板左上角的距离定位（绝对定位）。
    Absolute([f32; 2]),
    /// Relative positioning using grid-based coordinates.
    ///
    /// 依照网格式定位方法进行定位（相对定位）。
    Relative([[f32; 2]; 2]),
}

/// Used for customizing the layout of resources.
///
/// 用于自定义资源排版方式。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CustomPanelLayout {
    /// Locate resources by type.
    ///
    /// 通过类型定位资源。
    Type(String, PanelLayout),
    /// Locate resources through ID.
    ///
    /// 通过ID定位资源。
    Id(RustConstructorId, PanelLayout),
}

/// Storage structure for panel resource metadata.
///
/// 面板资源元数据的存储结构。
///
/// This struct holds essential information about resources stored within panels,
/// including visibility, rendering behavior, and sizing information.
///
/// 该结构体保存面板内存储资源的基本信息，包括可见性、渲染行为和尺寸信息。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct PanelStorage {
    /// Unique identifier for the stored resource.
    ///
    /// 存储资源的唯一标识符。
    pub id: RustConstructorId,

    /// If true, the resource ignores render layer.
    ///
    /// 如果为true，则资源忽略渲染层。
    pub ignore_render_layer: bool,

    /// Controls whether the resource is visible.
    ///
    /// 控制资源是否可见。
    pub hidden: bool,
}

/// Used to config the appearance of each basic front resource of the panel.
///
/// 用于配置面板各基本前端资源外观的结构。
#[derive(Debug, Clone, PartialEq)]
pub struct PanelConfig {
    /// Image config.
    ///
    /// 图片配置。
    pub image_config: ImageConfig,

    /// Custom rect config.
    ///
    /// 自定义矩形配置。
    pub custom_rect_config: CustomRectConfig,

    /// Text config.
    ///
    /// 文本配置。
    pub text_config: TextConfig,
}

/// Used for customizing the appearance of each basic front resource.
///
/// 用于自定义基本前端资源外观。
#[derive(Debug, Clone, PartialEq)]
pub enum CustomPanelConfig {
    /// Locate resources by type.
    ///
    /// 通过类型定位资源。
    Type(String, PanelConfig),
    /// Locate resources through ID.
    ///
    /// 通过ID定位资源。
    Id(RustConstructorId, PanelConfig),
}

/// Config options for resource panels.
///
/// 资源板的可配置选项。
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ResourcePanelConfig {
    /// Which edges can be resized: [top, bottom, left, right].
    ///
    /// 哪些边可以调整尺寸：[top, bottom, left, right]。
    pub resizable: Option<[bool; 4]>,

    /// Background display for the panel.
    ///
    /// 面板的背景显示。
    pub background: Option<BackgroundType>,

    /// Minimum size constraints for the panel.
    ///
    /// 面板的最小尺寸限制。
    pub min_size: Option<[f32; 2]>,

    /// Optional maximum size constraints for the panel.
    ///
    /// 面板的可选最大尺寸限制。
    pub max_size: Option<Option<[f32; 2]>>,

    /// Whether the panel can be moved: [horizontal, vertical].
    ///
    /// 面板是否可以移动：[horizontal, vertical]。
    pub movable: Option<[bool; 2]>,

    /// Methods for calculating scroll length: [horizontal, vertical].
    ///
    /// 计算滚动长度的方法：[horizontal, vertical]。
    pub scroll_length_method: Option<[Option<ScrollLengthMethod>; 2]>,

    /// Sensitivity of scrolling interactions.
    ///
    /// 滚动交互的敏感性。
    pub scroll_sensitivity: Option<f32>,

    /// Display behavior of the scroll bar.
    ///
    /// 显示滚动条的方法。
    pub scroll_bar_display_method: Option<ScrollBarDisplayMethod>,

    /// Layout config for resources within the panel.
    ///
    /// 面板内资源的布局配置。
    pub overall_layout: Option<PanelLayout>,

    /// Custom layout config of specific resources within the panel.
    ///
    /// 面板内特定资源的自定义布局配置。
    pub custom_layout: Option<Vec<CustomPanelLayout>>,

    /// Custom panel config of specific resources within the panel.
    ///
    /// 面板内特定资源的自定义面板配置。
    pub overall_config: Option<PanelConfig>,

    /// Panel config for specific resources.
    ///
    /// 特定资源的面板配置。
    pub custom_config: Option<Vec<CustomPanelConfig>>,

    /// Whether the panel is visible.
    ///
    /// 面板是否可见。
    pub hidden: Option<bool>,

    /// Reverse scroll direction: [horizontal, vertical].
    ///
    /// 反转滚动方向：[horizontal, vertical]。
    pub reverse_scroll_direction: Option<[bool; 2]>,

    /// Inner margin of the panel.
    ///
    /// 面板内边距。
    ///
    /// Use this field to ensure that functions such as resizing can be used normally.
    ///
    /// 使用此字段以保证缩放等功能可以正常使用。
    pub inner_margin: Option<[f32; 4]>,

    /// Whether to place the panel in the front when clicking
    ///
    /// 是否在点击时将面板前置。
    pub raise_on_focus: Option<bool>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl Config for ResourcePanelConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn convert_to_resource(&self) -> Box<dyn FrontResource> {
        Box::new(ResourcePanel::default().from_config(self))
    }

    fn convert_from_resource(&self, resource: Box<dyn FrontResource>) -> Option<Box<dyn Config>> {
        if let Some(resource) = resource.as_any().downcast_ref::<ResourcePanel>() {
            Some(Box::new(ResourcePanelConfig::from_resource(resource)))
        } else {
            None
        }
    }
}

impl ResourcePanelConfig {
    pub fn from_resource(resource: &ResourcePanel) -> Self {
        Self {
            resizable: Some(resource.resizable),
            background: Some(resource.background.clone()),
            min_size: Some(resource.min_size),
            max_size: Some(resource.max_size),
            movable: Some(resource.movable),
            scroll_length_method: Some(resource.scroll_length_method),
            scroll_sensitivity: Some(resource.scroll_sensitivity),
            scroll_bar_display_method: Some(resource.scroll_bar_display_method.clone()),
            overall_layout: Some(resource.overall_layout),
            custom_layout: Some(resource.custom_layout.clone()),
            overall_config: Some(resource.overall_config.clone()),
            custom_config: Some(resource.custom_config.clone()),
            hidden: Some(resource.hidden),
            reverse_scroll_direction: Some(resource.reverse_scroll_direction),
            inner_margin: Some(resource.inner_margin),
            raise_on_focus: Some(resource.raise_on_focus),
            tags: Some(resource.tags.clone()),
        }
    }

    #[inline]
    pub fn resizable(mut self, resizable: Option<[bool; 4]>) -> Self {
        self.resizable = resizable;
        self
    }

    #[inline]
    pub fn background(mut self, background: Option<BackgroundType>) -> Self {
        self.background = background;
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: Option<[f32; 2]>) -> Self {
        self.min_size = min_size;
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: Option<Option<[f32; 2]>>) -> Self {
        self.max_size = max_size;
        self
    }

    #[inline]
    pub fn movable(mut self, movable: Option<[bool; 2]>) -> Self {
        self.movable = movable;
        self
    }

    #[inline]
    pub fn scroll_length_method(
        mut self,
        scroll_length_method: Option<[Option<ScrollLengthMethod>; 2]>,
    ) -> Self {
        self.scroll_length_method = scroll_length_method;
        self
    }

    #[inline]
    pub fn scroll_sensitivity(mut self, scroll_sensitivity: Option<f32>) -> Self {
        self.scroll_sensitivity = scroll_sensitivity;
        self
    }

    #[inline]
    pub fn scroll_bar_display_method(
        mut self,
        scroll_bar_display_method: Option<ScrollBarDisplayMethod>,
    ) -> Self {
        self.scroll_bar_display_method = scroll_bar_display_method;
        self
    }

    #[inline]
    pub fn overall_layout(mut self, overall_layout: Option<PanelLayout>) -> Self {
        self.overall_layout = overall_layout;
        self
    }

    #[inline]
    pub fn custom_layout(mut self, custom_layout: Option<Vec<CustomPanelLayout>>) -> Self {
        self.custom_layout = custom_layout;
        self
    }

    #[inline]
    pub fn overall_config(mut self, overall_config: Option<PanelConfig>) -> Self {
        self.overall_config = overall_config;
        self
    }

    #[inline]
    pub fn custom_config(mut self, custom_config: Option<Vec<CustomPanelConfig>>) -> Self {
        self.custom_config = custom_config;
        self
    }

    #[inline]
    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }

    #[inline]
    pub fn reverse_scroll_direction(mut self, reverse_scroll_direction: Option<[bool; 2]>) -> Self {
        self.reverse_scroll_direction = reverse_scroll_direction;
        self
    }

    #[inline]
    pub fn inner_margin(mut self, inner_margin: Option<[f32; 4]>) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    #[inline]
    pub fn raise_on_focus(mut self, raise_on_focus: Option<bool>) -> Self {
        self.raise_on_focus = raise_on_focus;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// Resource panel for organizing and managing UI elements with scrolling capabilities.
///
/// 资源板，用于组织和管理具有滚动能力的UI元素。
#[derive(Debug, Clone, PartialEq)]
pub struct ResourcePanel {
    /// Which edges can be resized: [top, bottom, left, right].
    ///
    /// 哪些边可以调整尺寸：[top, bottom, left, right]。
    pub resizable: [bool; 4],

    /// Background display for the panel.
    ///
    /// 面板的背景显示。
    pub background: BackgroundType,

    /// Minimum size constraints for the panel.
    ///
    /// 面板的最小尺寸限制。
    pub min_size: [f32; 2],

    /// Optional maximum size constraints for the panel.
    ///
    /// 面板的可选最大尺寸限制。
    pub max_size: Option<[f32; 2]>,

    /// Whether the panel can be moved: [horizontal, vertical].
    ///
    /// 面板是否可以移动：[horizontal, vertical]。
    pub movable: [bool; 2],

    /// Methods for calculating scroll length: [horizontal, vertical].
    ///
    /// 计算滚动长度的方法：[horizontal, vertical]。
    pub scroll_length_method: [Option<ScrollLengthMethod>; 2],

    /// Sensitivity of scrolling interactions.
    ///
    /// 滚动交互的敏感性。
    pub scroll_sensitivity: f32,

    /// Display behavior of the scroll bar.
    ///
    /// 显示滚动条的方法。
    pub scroll_bar_display_method: ScrollBarDisplayMethod,

    /// Layout config for resources within the panel.
    ///
    /// 面板内资源的布局配置。
    pub overall_layout: PanelLayout,

    /// Custom layout config of specific resources within the panel.
    ///
    /// 面板内特定资源的自定义布局配置。
    pub custom_layout: Vec<CustomPanelLayout>,

    /// Custom panel config of specific resources within the panel.
    ///
    /// 面板内特定资源的自定义面板配置。
    pub overall_config: PanelConfig,

    /// Panel config for specific resources.
    ///
    /// 特定资源的面板配置。
    pub custom_config: Vec<CustomPanelConfig>,

    /// Whether the panel is visible.
    ///
    /// 面板是否可见。
    pub hidden: bool,

    /// Reverse scroll direction: [horizontal, vertical].
    ///
    /// 反转滚动方向：[horizontal, vertical]。
    pub reverse_scroll_direction: [bool; 2],

    /// Inner margin of the panel.
    ///
    /// 面板内边距。
    ///
    /// Use this field to ensure that functions such as resizing can be used normally.
    ///
    /// 使用此字段以保证缩放等功能可以正常使用。
    pub inner_margin: [f32; 4],

    /// Whether to place the panel in the front when clicking
    ///
    /// 是否在点击时将面板前置。
    pub raise_on_focus: bool,

    /// Current scroll length: [horizontal, vertical].
    ///
    /// 当前滚动长度：[horizontal, vertical]。
    pub scroll_length: [f32; 2],

    /// Current scroll progress: [horizontal, vertical].
    ///
    /// 当前滚动进度：[horizontal, vertical]。
    pub scroll_progress: [f32; 2],

    /// Mouse state from previous frame: (position, click_aim, scroll_delta).
    ///
    /// 上一帧的鼠标状态：(位置，点击目标，滚动增量)。
    pub last_frame_mouse_status: Option<([f32; 2], ClickAim, [f32; 2])>,

    /// Whether scrolling occurred in this frame: [horizontal, vertical].
    ///
    /// 在这一帧中是否发生了滚动：[horizontal, vertical]。
    pub scrolled: [bool; 2],

    /// Scroll bar transparency: [horizontal, vertical].
    ///
    /// 滚动条透明度：[horizontal, vertical]。
    pub scroll_bar_alpha: [u8; 2],

    /// Storage for resource metadata within the panel.
    ///
    /// 面板内资源元数据的存储。
    pub resource_storage: Vec<PanelStorage>,

    /// The overall offset of all resources on the panel.
    ///
    /// 面板所有资源整体的偏移量。
    ///
    /// It is used to ensure that resources with different alignment methods can all be displayed correctly.
    ///
    /// 用于确保不同对齐方式的资源都能正确显示。
    pub overall_offset: [f32; 2],

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for ResourcePanel {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn convert_to_front(&self) -> Option<Box<dyn FrontResource>> {
        Some(Box::new(self.clone()))
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_front_dyn(&self) -> Option<&dyn FrontResource> {
        Some(self)
    }

    fn convert_to_front_dyn_mut(&mut self) -> Option<&mut dyn FrontResource> {
        Some(self)
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
    }
}

impl FrontResource for ResourcePanel {
    fn convert_to_config(&self) -> Box<dyn Config> {
        Box::new(ResourcePanelConfig::from_resource(self))
    }

    fn convert_from_config(&mut self, config: Box<dyn Config>) -> Option<Box<dyn FrontResource>> {
        if let Some(config) = config.as_any().downcast_ref::<ResourcePanelConfig>() {
            Some(Box::new(self.clone().from_config(config)))
        } else {
            None
        }
    }

    fn convert_to_original(&self) -> Box<dyn RustConstructorResource> {
        Box::new(self.clone())
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_original_dyn(&self) -> &dyn RustConstructorResource {
        self
    }

    fn convert_to_original_dyn_mut(&mut self) -> &mut dyn RustConstructorResource {
        self
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
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
            scroll_bar_display_method: ScrollBarDisplayMethod::OnlyScroll(
                BackgroundType::default(),
                [4_f32, 2_f32],
                4_f32,
            ),
            overall_layout: (PanelLayout {
                panel_margin: PanelMargin::Vertical([0_f32, 0_f32, 0_f32, 0_f32], false),
                panel_location: PanelLocation::Absolute([0_f32, 0_f32]),
            }),
            custom_layout: Vec::new(),
            overall_config: PanelConfig {
                image_config: ImageConfig::default(),
                custom_rect_config: CustomRectConfig::default(),
                text_config: TextConfig::default(),
            },
            custom_config: Vec::new(),
            hidden: false,
            reverse_scroll_direction: [false, false],
            inner_margin: [6_f32, 6_f32, 6_f32, 6_f32],
            raise_on_focus: true,
            scroll_length: [0_f32, 0_f32],
            scroll_progress: [0_f32, 0_f32],
            last_frame_mouse_status: None,
            scrolled: [false, false],
            scroll_bar_alpha: [0, 0],
            resource_storage: Vec::new(),
            overall_offset: [0_f32, 0_f32],
            tags: Vec::new(),
        }
    }
}

impl ResourcePanel {
    pub fn from_config(mut self, config: &ResourcePanelConfig) -> Self {
        if let Some(resizable) = config.resizable {
            self.resizable = resizable;
        };
        if let Some(ref background) = config.background {
            self.background = background.clone();
        };
        if let Some(min_size) = config.min_size {
            self.min_size = min_size;
        };
        if let Some(max_size) = config.max_size {
            self.max_size = max_size;
        };
        if let Some(movable) = config.movable {
            self.movable = movable;
        };
        if let Some(scroll_length_method) = config.scroll_length_method {
            self.scroll_length_method = scroll_length_method;
        };
        if let Some(scroll_sensitivity) = config.scroll_sensitivity {
            self.scroll_sensitivity = scroll_sensitivity;
        };
        if let Some(ref scroll_bar_display_method) = config.scroll_bar_display_method {
            self.scroll_bar_display_method = scroll_bar_display_method.clone();
        };
        if let Some(overall_layout) = config.overall_layout {
            self.overall_layout = overall_layout;
        };
        if let Some(ref custom_layout) = config.custom_layout {
            self.custom_layout = custom_layout.clone();
        };
        if let Some(ref overall_config) = config.overall_config {
            self.overall_config = overall_config.clone();
        };
        if let Some(ref custom_config) = config.custom_config {
            self.custom_config = custom_config.clone();
        };
        if let Some(hidden) = config.hidden {
            self.hidden = hidden;
        };
        if let Some(reverse_scroll_direction) = config.reverse_scroll_direction {
            self.reverse_scroll_direction = reverse_scroll_direction;
        };
        if let Some(inner_margin) = config.inner_margin {
            self.inner_margin = inner_margin;
        };
        if let Some(raise_on_focus) = config.raise_on_focus {
            self.raise_on_focus = raise_on_focus;
        };
        if let Some(ref tags) = config.tags {
            self.tags = tags.clone();
        };
        self
    }

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
    pub fn scroll_bar_display_method(
        mut self,
        scroll_bar_display_method: ScrollBarDisplayMethod,
    ) -> Self {
        self.scroll_bar_display_method = scroll_bar_display_method;
        self
    }

    #[inline]
    pub fn overall_layout(mut self, overall_layout: PanelLayout) -> Self {
        self.overall_layout = overall_layout;
        self
    }

    #[inline]
    pub fn custom_layout(mut self, custom_layout: &[CustomPanelLayout]) -> Self {
        self.custom_layout = custom_layout.to_owned();
        self
    }

    #[inline]
    pub fn overall_config(mut self, overall_config: PanelConfig) -> Self {
        self.overall_config = overall_config;
        self
    }

    #[inline]
    pub fn custom_config(mut self, custom_config: &[CustomPanelConfig]) -> Self {
        self.custom_config = custom_config.to_owned();
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
    pub fn inner_margin(mut self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.inner_margin = [top, bottom, left, right];
        self
    }

    #[inline]
    pub fn raise_on_focus(mut self, raise_on_focus: bool) -> Self {
        self.raise_on_focus = raise_on_focus;
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

/// Appearance config for switch resources.
///
/// 开关资源的外观配置。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SwitchAppearanceConfig {
    /// Config for the background element.
    ///
    /// 背景元素的配置项。
    pub background_config: BackgroundType,

    /// Config for the main text.
    ///
    /// 主要文本的配置项。
    pub text_config: TextConfig,

    /// Config for the hint text.
    ///
    /// 提示文本的配置项。
    pub hint_text_config: TextConfig,
}

/// Click config for switch resources.
///
/// 开关资源的点击配置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchClickConfig {
    /// Mouse button used to trigger the switch.
    ///
    /// 用于触发开关的鼠标按钮。
    pub click_method: PointerButton,

    /// Whether clicking changes the switch state.
    ///
    /// 单击是否改变开关状态。
    pub action: bool,
}

/// Data structure for tracking switch state and interactions.
///
/// 用于跟踪开关状态和交互的数据结构。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwitchData {
    /// Whether the switch was toggled by a click.
    ///
    /// 是否通过点击打开开关。
    pub switched: bool,

    /// Click method from the previous frame, if any.
    ///
    /// 前一帧中的单击方法（如果有的话）。
    pub last_frame_clicked: Option<usize>,

    /// Current state of the switch.
    ///
    /// 开关当前的状态。
    pub state: usize,
}

/// Config options for switch resources.
///
/// 开关资源的配置选项。
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SwitchConfig {
    /// Appearance configs for each state combination.
    ///
    /// 每个状态组合的外观配置。
    pub appearance: Option<Vec<SwitchAppearanceConfig>>,

    /// Type of background to display.
    ///
    /// 要显示的背景类型。
    pub background_type: Option<BackgroundType>,

    /// Config for the main text display.
    ///
    /// 主文本显示的配置。
    pub text_config: Option<TextConfig>,

    /// Config for the hint text display.
    ///
    /// 提示文本显示的配置。
    pub hint_text_config: Option<TextConfig>,

    /// Enable animations for hover and click: [hover, click].
    ///
    /// 启用悬停动画和单击动画：[hover, click]。
    pub enable_animation: Option<[bool; 2]>,

    /// Total number of possible switch states.
    ///
    /// 开关可能的状态总数。
    pub state_amount: Option<u32>,

    /// Configs for click interactions.
    ///
    /// 单击交互的配置。
    pub click_method: Option<Vec<SwitchClickConfig>>,

    /// Set the single-choice grouping of the switch.
    ///
    /// 设置开关的单选分组。
    ///
    /// Only one switch can be activated within the same single-choice group.
    ///
    /// 同一单选分组内只能有一个开关被激活。
    pub radio_group: Option<String>,

    /// Whether the switch is enabled (disabled shows but not interactive).
    ///
    /// 开关是否启用（disabled会显示，但无法交互）。
    pub enable: Option<bool>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Option<Vec<[String; 2]>>,
}

impl Config for SwitchConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn convert_to_resource(&self) -> Box<dyn FrontResource> {
        Box::new(Switch::default().from_config(self))
    }

    fn convert_from_resource(&self, resource: Box<dyn FrontResource>) -> Option<Box<dyn Config>> {
        if let Some(resource) = resource.as_any().downcast_ref::<Switch>() {
            Some(Box::new(SwitchConfig::from_resource(resource)))
        } else {
            None
        }
    }
}

impl SwitchConfig {
    pub fn from_resource(resource: &Switch) -> Self {
        Self {
            appearance: Some(resource.appearance.clone()),
            background_type: Some(resource.background_type.clone()),
            text_config: Some(resource.text_config.clone()),
            hint_text_config: Some(resource.hint_text_config.clone()),
            enable_animation: Some(resource.enable_animation),
            state_amount: Some(resource.state_amount),
            click_method: Some(resource.click_method.clone()),
            radio_group: Some(resource.radio_group.clone()),
            enable: Some(resource.enable),
            tags: Some(resource.tags.clone()),
        }
    }

    #[inline]
    pub fn appearance(mut self, appearance: Option<Vec<SwitchAppearanceConfig>>) -> Self {
        self.appearance = appearance;
        self
    }

    #[inline]
    pub fn background_type(mut self, background_type: Option<BackgroundType>) -> Self {
        self.background_type = background_type;
        self
    }

    #[inline]
    pub fn text_config(mut self, text_config: Option<TextConfig>) -> Self {
        self.text_config = text_config;
        self
    }

    #[inline]
    pub fn hint_text_config(mut self, hint_text_config: Option<TextConfig>) -> Self {
        self.hint_text_config = hint_text_config;
        self
    }

    #[inline]
    pub fn enable_animation(mut self, enable_animation: Option<[bool; 2]>) -> Self {
        self.enable_animation = enable_animation;
        self
    }

    #[inline]
    pub fn state_amount(mut self, state_amount: Option<u32>) -> Self {
        self.state_amount = state_amount;
        self
    }

    #[inline]
    pub fn click_method(mut self, click_method: Option<Vec<SwitchClickConfig>>) -> Self {
        self.click_method = click_method;
        self
    }

    #[inline]
    pub fn radio_group(mut self, radio_group: Option<String>) -> Self {
        self.radio_group = radio_group;
        self
    }

    #[inline]
    pub fn enable(mut self, enable: Option<bool>) -> Self {
        self.enable = enable;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// Switch resource for toggleable UI elements.
///
/// 用于可切换UI元素的开关资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    /// Appearance configs for each state combination.
    ///
    /// 每个状态组合的外观配置。
    pub appearance: Vec<SwitchAppearanceConfig>,

    /// Type of background to display.
    ///
    /// 要显示的背景类型。
    pub background_type: BackgroundType,

    /// Config for the main text display.
    ///
    /// 主文本显示的配置。
    pub text_config: TextConfig,

    /// Config for the hint text display.
    ///
    /// 提示文本显示的配置。
    pub hint_text_config: TextConfig,

    /// Enable animations for hover and click: [hover, click].
    ///
    /// 启用悬停动画和单击动画：[hover, click]。
    pub enable_animation: [bool; 2],

    /// Total number of possible switch states.
    ///
    /// 开关可能的状态总数。
    pub state_amount: u32,

    /// Configs for click interactions.
    ///
    /// 单击交互的配置。
    pub click_method: Vec<SwitchClickConfig>,

    /// Set the single-choice grouping of the switch.
    ///
    /// 设置开关的单选分组。
    ///
    /// Only one switch can be activated within the same single-choice group.
    ///
    /// 同一单选分组内只能有一个开关被激活。
    pub radio_group: String,

    /// Whether the switch is enabled (disabled shows but not interactive).
    ///
    /// 开关是否启用（disabled会显示，但无法交互）。
    pub enable: bool,

    /// Current state of the switch.
    ///
    /// 开关当前状态。
    pub state: usize,

    /// Whether the mouse was hovering in the previous frame.
    ///
    /// 鼠标是否在前一帧中悬停。
    pub last_frame_hovered: bool,

    /// Click method from the previous frame, if any.
    ///
    /// 前一帧中的单击方法（如果有的话）。
    pub last_frame_clicked: Option<usize>,

    /// Whether the switch was toggled.
    ///
    /// 开关是否被切换。
    pub switched: bool,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
    pub tags: Vec<[String; 2]>,
}

impl RustConstructorResource for Switch {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn convert_to_front(&self) -> Option<Box<dyn FrontResource>> {
        Some(Box::new(self.clone()))
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_front_dyn(&self) -> Option<&dyn FrontResource> {
        Some(self)
    }

    fn convert_to_front_dyn_mut(&mut self) -> Option<&mut dyn FrontResource> {
        Some(self)
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
    }
}

impl FrontResource for Switch {
    fn convert_to_config(&self) -> Box<dyn Config> {
        Box::new(SwitchConfig::from_resource(self))
    }

    fn convert_from_config(&mut self, config: Box<dyn Config>) -> Option<Box<dyn FrontResource>> {
        if let Some(config) = config.as_any().downcast_ref::<SwitchConfig>() {
            Some(Box::new(self.clone().from_config(config)))
        } else {
            None
        }
    }

    fn convert_to_original(&self) -> Box<dyn RustConstructorResource> {
        Box::new(self.clone())
    }

    fn convert_to_basic_front(&self) -> Option<Box<dyn BasicFrontResource>> {
        None
    }

    fn convert_to_original_dyn(&self) -> &dyn RustConstructorResource {
        self
    }

    fn convert_to_original_dyn_mut(&mut self) -> &mut dyn RustConstructorResource {
        self
    }

    fn convert_to_basic_front_dyn(&self) -> Option<&dyn BasicFrontResource> {
        None
    }

    fn convert_to_basic_front_dyn_mut(&mut self) -> Option<&mut dyn BasicFrontResource> {
        None
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
            radio_group: String::new(),
            enable: true,
            state: 0,
            last_frame_hovered: false,
            last_frame_clicked: None,
            switched: false,
            tags: Vec::new(),
        }
    }
}

impl Switch {
    pub fn from_config(mut self, config: &SwitchConfig) -> Self {
        if let Some(ref appearance) = config.appearance {
            self.appearance = appearance.clone();
        };
        if let Some(ref background_type) = config.background_type {
            self.background_type = background_type.clone();
        };
        if let Some(ref text_config) = config.text_config {
            self.text_config = text_config.clone();
        };
        if let Some(ref hint_text_config) = config.hint_text_config {
            self.hint_text_config = hint_text_config.clone();
        };
        if let Some(enable_animation) = config.enable_animation {
            self.enable_animation = enable_animation;
        };
        if let Some(state_amount) = config.state_amount {
            self.state_amount = state_amount;
        };
        if let Some(ref click_method) = config.click_method {
            self.click_method = click_method.clone();
        };
        if let Some(ref radio_group) = config.radio_group {
            self.radio_group = radio_group.clone();
        };
        if let Some(enable) = config.enable {
            self.enable = enable;
        };
        if let Some(ref tags) = config.tags {
            self.tags = tags.clone();
        };
        self
    }

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
    pub fn radio_group(mut self, radio_group: &str) -> Self {
        self.radio_group = radio_group.to_string();
        self
    }

    #[inline]
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = enable;
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
