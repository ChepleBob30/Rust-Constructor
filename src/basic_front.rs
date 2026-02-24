//! This file contains basic front resources. Basic front resources can be used independently or to create advanced front-end resources.
//!
//! 此文件包含基本前端资源。基本前端资源可以单独使用，也可被用于创建高级前端资源。
use crate::{
    BasicFrontResource, BasicFrontResourceConfig, BorderKind, DisplayInfo, PositionSizeConfig,
    RustConstructorResource,
};
use egui::TextureHandle;
use std::{
    any::Any,
    fmt::{Debug, Formatter},
};

/// Config options for custom rectangles.
///
/// 矩形的可配置选项。
///
/// This struct contains all configurable properties for creating and modifying
/// rectangular UI elements with various visual properties.
///
/// 该结构体包含用于创建和修改具有各种视觉属性的矩形UI元素的所有可配置属性。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct CustomRectConfig {
    /// Config for position, size, and layout of the rectangle.
    ///
    /// 矩形的位置、尺寸和布局配置。
    pub position_size_config: Option<PositionSizeConfig>,

    /// Optional clipping rectangle that defines the visible area.
    ///
    /// 定义可见区域的可选裁剪矩形。
    pub clip_rect: Option<Option<PositionSizeConfig>>,

    /// Controls whether the rectangle is visible or hidden.
    ///
    /// 控制矩形是否可见或隐藏。
    pub hidden: Option<bool>,

    /// If true, the rectangle ignores render layer.
    ///
    /// 如果为true，矩形忽略渲染层。
    pub ignore_render_layer: Option<bool>,

    /// Radius for rounded corners. Zero for sharp corners.
    ///
    /// 圆角半径。零表示直角。
    pub rounding: Option<f32>,

    /// Fill color of the rectangle as [R, G, B].
    ///
    /// 矩形的填充颜色，格式为[R, G, B]。
    pub color: Option<[u8; 3]>,

    /// Opacity of the rectangle (0-255).
    ///
    /// 矩形的不透明度（0-255）。
    pub alpha: Option<u8>,

    /// Width of the border.
    ///
    /// 边框宽度。
    pub border_width: Option<f32>,

    /// Color of the border as [R, G, B].
    ///
    /// 边框颜色，格式为[R, G, B]。
    pub border_color: Option<[u8; 3]>,

    /// Opacity of the border (0-255).
    ///
    /// 边框的不透明度（0-255）。
    pub border_alpha: Option<u8>,

    /// Placement of the border relative to the rectangle's bounds.
    ///
    /// 边框相对于矩形边界的放置方式。
    pub border_kind: Option<BorderKind>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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

/// Custom rectangle resource for drawing rectangles with various visual properties.
///
/// 自定义矩形资源，用于绘制具有各种视觉属性的矩形。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRect {
    /// Config for basic front resource properties.
    ///
    /// 基本前端资源属性配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,

    /// Current display position of the rectangle as [x, y].
    ///
    /// 矩形的当前显示位置，为[x, y]。
    pub position: [f32; 2],

    /// Current display size of the rectangle as [width, height].
    ///
    /// 矩形的当前显示尺寸，为[width, height]。
    pub size: [f32; 2],

    /// Display info controlling visibility and rendering.
    ///
    /// 显示信息，控制可见性和渲染。
    pub display_info: DisplayInfo,

    /// Radius for rounded corners.
    ///
    /// 圆角。
    pub rounding: f32,

    /// Fill color of the rectangle as [R, G, B].
    ///
    /// 填充矩形颜色，为[R, G, B]。
    pub color: [u8; 3],

    /// Opacity of the rectangle (0-255).
    ///
    /// 矩形的不透明度（0-255）。
    pub alpha: u8,

    /// Width of the border.
    ///
    /// 边框宽度。
    pub border_width: f32,

    /// Color of the border as [R, G, B].
    ///
    /// 边框颜色，为[R, G, B]。
    pub border_color: [u8; 3],

    /// Opacity of the border (0-255).
    ///
    /// 边框的不透明度（0-255）。
    pub border_alpha: u8,

    /// Placement of the border relative to the rectangle's bounds.
    ///
    /// 边框相对于矩形边界的位置。
    pub border_kind: BorderKind,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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

/// Wrapper for TextureHandle that supports Debug trait derivation.
///
/// 支持Debug特征派生的TextureHandle包装器。
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DebugTextureHandle(pub TextureHandle);

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

/// Methods for loading images into the resource.
///
/// 将图像加载到资源中的方法。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageLoadMethod {
    /// Load image from a file path.
    ///
    /// 从文件路径加载图像。
    ByPath((String, [bool; 2])),

    /// Use an existing TextureHandle for the image.
    ///
    /// 使用现有的TextureHandle作为图像。
    ByTexture(DebugTextureHandle),
}

/// Config options for image resources.
///
/// 图像资源的配置选项。
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageConfig {
    /// Config for position, size, and layout.
    ///
    /// 位置、尺寸和布局配置。
    pub position_size_config: Option<PositionSizeConfig>,

    /// Optional clipping rectangle that defines the visible area.
    ///
    /// 定义可见区域的可选裁剪矩形。
    pub clip_rect: Option<Option<PositionSizeConfig>>,

    /// Controls whether the image is visible or hidden.
    ///
    /// 控制图像是否可见或隐藏。
    pub hidden: Option<bool>,

    /// If true, the image ignores render layer.
    ///
    /// 如果为true，图像忽略渲染层。
    pub ignore_render_layer: Option<bool>,

    /// Opacity of the image (0-255).
    ///
    /// 图像的不透明度（0-255）。
    pub alpha: Option<u8>,

    /// Color overlay applied to the image as [R, G, B].
    ///
    /// 应用于图像的色彩覆盖，格式为[R, G, B]。
    pub overlay_color: Option<[u8; 3]>,

    /// Opacity of the overlay (0-255).
    ///
    /// 覆盖层的不透明度（0-255）。
    pub overlay_alpha: Option<u8>,

    /// Background color behind the image as [R, G, B].
    ///
    /// 图像背后的背景颜色，格式为[R, G, B]。
    pub background_color: Option<[u8; 3]>,

    /// Opacity of the background (0-255).
    ///
    /// 背景的不透明度（0-255）。
    pub background_alpha: Option<u8>,

    /// Rotation angle of the image in degrees.
    ///
    /// 图像的旋转角度（度）。
    pub rotate_angle: Option<f32>,

    /// Center point for rotation, compare it with the actual size to obtain as [width, height].
    ///
    /// 旋转中心点，通过与实际大小的比得出，为[width, height]。
    pub rotate_center: Option<[f32; 2]>,

    /// Method used to load the image.
    ///
    /// 用于加载图像的方法。
    pub image_load_method: Option<ImageLoadMethod>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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
            image_load_method: Some(image.image_load_method.clone()),
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
    pub fn image_load_method(mut self, image_load_method: Option<ImageLoadMethod>) -> Self {
        self.image_load_method = image_load_method;
        self
    }

    #[inline]
    pub fn tags(mut self, tags: Option<Vec<[String; 2]>>) -> Self {
        self.tags = tags;
        self
    }
}

/// Image resource for displaying graphical content in the GUI.
///
/// 用于在GUI中显示图形内容的图像资源。
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// Config for basic front resource properties.
    ///
    /// 基本前端资源属性配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,

    /// Current display position of the image as [x, y].
    ///
    /// 图像的当前显示位置，坐标为[x, y]。
    pub position: [f32; 2],

    /// Current display size of the image as [width, height].
    ///
    /// 图像的当前显示尺寸，为[width, height]。
    pub size: [f32; 2],

    /// Display info controlling visibility and rendering.
    ///
    /// 显示信息，控制可见性和渲染。
    pub display_info: DisplayInfo,

    /// Handle to the loaded texture, if available.
    ///
    /// 已加载纹理的句柄（如果可用）。
    pub texture: Option<DebugTextureHandle>,

    /// Opacity of the image (0-255).
    ///
    /// 图像的不透明度（0-255）。
    pub alpha: u8,

    /// Color overlay applied to the image as [R, G, B].
    ///
    /// 应用于图像的色彩覆盖，格式为[R, G, B]。
    pub overlay_color: [u8; 3],

    /// Opacity of the overlay (0-255).
    ///
    /// 覆盖层的不透明度（0-255）。
    pub overlay_alpha: u8,

    /// Background color behind the image as [R, G, B].
    ///
    /// 图像背后的背景颜色，格式为[R, G, B]。
    pub background_color: [u8; 3],

    /// Opacity of the background (0-255).
    ///
    /// 背景的不透明度（0-255）。
    pub background_alpha: u8,

    /// Rotation angle of the image in degrees.
    ///
    /// 图像的旋转角度（度）。
    pub rotate_angle: f32,

    /// Center point for rotation, compare it with the actual size to obtain as [width, height].
    ///
    /// 旋转中心点，通过与实际大小的比得出，为[width, height]。
    pub rotate_center: [f32; 2],

    /// Method used to load the image.
    ///
    /// 用于加载图像的方法。
    pub image_load_method: ImageLoadMethod,

    /// The path for loading the image in the previous frame.
    ///
    /// 上一帧加载图片的路径。
    pub last_frame_path: String,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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
            image_load_method: ImageLoadMethod::ByPath((String::new(), [false, false])),
            last_frame_path: String::new(),
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
        if let Some(image_load_method) = config.image_load_method.clone() {
            self.image_load_method = image_load_method;
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
    pub fn image_load_method(mut self, image_load_method: &ImageLoadMethod) -> Self {
        self.image_load_method = image_load_method.clone();
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

/// Control the selection method of hyperlinks.
///
/// 控制超链接的选取方法。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HyperlinkSelectMethod {
    /// Selects all occurrences of the hyperlink text.
    ///
    /// 选取所有匹配的超链接文本。
    All(String),
    /// Selects specific segments of the hyperlink text with indices.
    ///
    /// 选取指定的超链接文本段。
    Segment(Vec<(usize, String)>),
}

/// Config options for text resources.
///
/// 文本资源的配置选项。
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct TextConfig {
    /// Config for position, size, and layout.
    ///
    /// 位置、尺寸和布局配置。
    pub position_size_config: Option<PositionSizeConfig>,

    /// Optional clipping rectangle that defines the visible area.
    ///
    /// 定义可见区域的可选裁剪矩形。
    pub clip_rect: Option<Option<PositionSizeConfig>>,

    /// Controls whether the text is visible or hidden.
    ///
    /// 控制文本是否可见或隐藏。
    pub hidden: Option<bool>,

    /// If true, the text ignores render layer.
    ///
    /// 如果为true，文本忽略渲染层。
    pub ignore_render_layer: Option<bool>,

    /// Text content to be displayed.
    ///
    /// 要显示的文本内容。
    pub content: Option<String>,

    /// Font size in points.
    ///
    /// 字体大小（点）。
    pub font_size: Option<f32>,

    /// Text color as [R, G, B].
    ///
    /// 文本颜色，格式为[R, G, B]。
    pub color: Option<[u8; 3]>,

    /// Opacity of the text (0-255).
    ///
    /// 文本的不透明度（0-255）。
    pub alpha: Option<u8>,

    /// Background color behind the text as [R, G, B].
    ///
    /// 文本背后的背景颜色，格式为[R, G, B]。
    pub background_color: Option<[u8; 3]>,

    /// Opacity of the background (0-255).
    ///
    /// 背景的不透明度（0-255）。
    pub background_alpha: Option<u8>,

    /// Radius for rounded corners of the background.
    ///
    /// 背景圆角半径。
    pub background_rounding: Option<f32>,

    /// The font used for the specified text.
    ///
    /// 指定文本使用的字体。
    pub font: Option<String>,

    /// Whether the text can be selected by the user.
    ///
    /// 文本是否可以被用户选择。
    pub selectable: Option<bool>,

    /// Hyperlink texts for clickable regions.
    ///
    /// 可点击区域的超链接文本。
    pub hyperlink_text: Option<Vec<(String, HyperlinkSelectMethod)>>,

    /// Automatically adjust size to fit content.
    ///
    /// 自动调整尺寸以适应内容。
    pub auto_fit: Option<[bool; 2]>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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

/// Text resource for displaying and interacting with textual content.
///
/// 用于显示和交互文本内容的文本资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    /// Config for basic front resource properties.
    ///
    /// 基本前端资源属性配置。
    pub basic_front_resource_config: BasicFrontResourceConfig,

    /// Current display position of the text as [x, y].
    ///
    /// 文本的当前显示位置，坐标为[x, y]。
    pub position: [f32; 2],

    /// Current display size of the text as [width, height].
    ///
    /// 文本的当前显示尺寸，为[width, height]。
    pub size: [f32; 2],

    /// Display info controlling visibility and rendering.
    ///
    /// 显示信息，控制可见性和渲染。
    pub display_info: DisplayInfo,

    /// Text content to be displayed.
    ///
    /// 要显示的文本内容。
    pub content: String,

    /// Font size in points.
    ///
    /// 字体大小（点）。
    pub font_size: f32,

    /// Text color as [R, G, B].
    ///
    /// 文本颜色，格式为[R, G, B]。
    pub color: [u8; 3],

    /// Opacity of the text (0-255).
    ///
    /// 文本的不透明度（0-255）。
    pub alpha: u8,

    /// Background color behind the text as [R, G, B].
    ///
    /// 文本背后的背景颜色，格式为[R, G, B]。
    pub background_color: [u8; 3],

    /// Opacity of the background (0-255).
    ///
    /// 背景的不透明度（0-255）。
    pub background_alpha: u8,

    /// Radius for rounded corners of the background.
    ///
    /// 背景圆角半径。
    pub background_rounding: f32,

    /// The font used for the specified text.
    ///
    /// 指定文本使用的字体。
    pub font: String,

    /// Whether the text can be selected by the user.
    ///
    /// 文本是否可以被用户选择。
    pub selectable: bool,

    /// Hyperlink texts with their selection methods for clickable regions.
    ///
    /// 可点击区域的超链接文本及其选择方法。
    pub hyperlink_text: Vec<(String, HyperlinkSelectMethod)>,

    /// Hyperlink indices and URLs: (start_index, end_index, url).
    ///
    /// 超链接索引值和链接：(起始索引, 结束索引, 链接)。
    pub hyperlink_index: Vec<(usize, usize, String)>,

    /// Auto-fit behavior: [horizontal_fit, vertical_fit].
    ///
    /// 是否让渲染层大小自动匹配实际大小：[水平适应, 垂直适应]。
    pub auto_fit: [bool; 2],

    /// Text content from the previous frame for change detection.
    ///
    /// 上一帧的文本内容，用于变化检测。
    pub last_frame_content: String,

    /// Currently selected text range (start_index, end_index).
    ///
    /// 框选选中的文本范围（起始索引, 结束索引）。
    pub selection: Option<(usize, usize)>,

    /// Size at which text is truncated for display.
    ///
    /// 文本被截断以供显示的尺寸。
    pub truncate_size: [f32; 2],

    /// Actual size of the text content.
    ///
    /// 文本内容的实际尺寸。
    pub actual_size: [f32; 2],

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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
