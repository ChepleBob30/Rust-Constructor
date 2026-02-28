//! This file contains advanced front-end resources, which can be used to handle complex tasks.
//!
//! 此文件包含高级前端资源，高级前端资源可以用于处理复杂的任务。
use crate::{
    DisplayInfo, RustConstructorId, RustConstructorResource,
    basic_front::{CustomRectConfig, ImageConfig, TextConfig},
};
use eframe::egui::PointerButton;
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

/// Background resource for UI elements.
///
/// UI元素的背景资源。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Background {
    /// Type of background to display.
    ///
    /// 要显示的背景类型。
    pub background_type: BackgroundType,

    /// If true, the background config updates automatically.
    ///
    /// 如果为true，则背景会自动更新。
    pub auto_update: bool,

    /// If true, resources created by the background use its tags.
    ///
    /// 如果为true，则背景创建的资源使用其标签。
    pub use_background_tags: bool,

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

    /// Whether to use smooth scrolling with delta values.
    ///
    /// 是否使用平滑滚动。
    pub use_smooth_scroll_delta: bool,

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
            overall_layout: (PanelLayout {
                panel_margin: PanelMargin::Vertical([0_f32, 0_f32, 0_f32, 0_f32], false),
                panel_location: PanelLocation::Absolute([0_f32, 0_f32]),
            }),
            custom_layout: Vec::new(),
            hidden: false,
            reverse_scroll_direction: [false, false],
            inner_margin: [6_f32, 6_f32, 6_f32, 6_f32],
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
    pub fn overall_layout(mut self, overall_layout: PanelLayout) -> Self {
        self.overall_layout = overall_layout;
        self
    }

    #[inline]
    pub fn push_custom_layout(mut self, custom_layout: CustomPanelLayout) -> Self {
        self.custom_layout.push(custom_layout);
        self
    }

    #[inline]
    pub fn custom_layout(mut self, custom_layout: &[CustomPanelLayout]) -> Self {
        self.custom_layout = custom_layout.to_owned();
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
    pub state: u32,
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

    /// Whether the switch is enabled (disabled shows but not interactive).
    ///
    /// 开关是否启用（disabled会显示，但无法交互）。
    pub enable: bool,

    /// Current state of the switch.
    ///
    /// 开关当前状态。
    pub state: u32,

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

    /// If true, resources created by the switch use its tags.
    ///
    /// 如果为true，则开关创建的资源使用其标签。
    pub use_switch_tags: bool,

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
    pub fn state(mut self, state: u32) -> Self {
        self.state = state;
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
