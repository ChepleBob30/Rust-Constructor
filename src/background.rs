//! This file contains backend resources, which can store some key data and be called upon when needed.
//!
//! 此文件包含后端资源，后端资源可以存储一些关键数据并在有需要时调用。
use crate::{DisplayInfo, RustConstructorResource};
use egui::FontDefinitions;
use std::{any::Any, fmt::Debug};

/// Storage Rust Constructor resource for page-specific data and state management.
///
/// 用于指定页面的数据和状态管理的Rust Constructor存储资源。
///
/// This resource provides metadata for page transitions and update cycles.
///
/// 该资源为页面转换和更新周期提供元数据。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PageData {
    /// Forces the page to refresh every frame regardless of changes.
    ///
    /// 强制页面刷新每一帧，无论更改。
    pub forced_update: bool,

    /// Indicates if resources needed for the initial page transition have been loaded.
    ///
    /// 指示是否已加载初始化页面转换所需的资源。
    pub change_page_updated: bool,

    /// Indicates if resources needed for entering the page have been loaded.
    ///
    /// 指示是否已加载进入该页所需的资源。
    pub enter_page_updated: bool,

    /// Key-value pairs for categorization and metadata storage.
    ///
    /// 用于分类和元数据存储的键值对。
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

/// Generic variable resource for storing any type of data with metadata.
///
/// 用于存储任意类型数据及元数据的通用变量资源。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable<T> {
    /// The stored value of the variable.
    ///
    /// 变量的存储值。
    pub value: Option<T>,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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

/// Font resource for text rendering.
///
/// 用于文本渲染的字体资源。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Font {
    /// Font definitions containing glyphs and styles.
    ///
    /// 包含字形和样式的字体定义。
    pub font_definitions: FontDefinitions,

    /// Path to the font file.
    ///
    /// 字体文件路径。
    pub path: String,

    /// Key-value pairs for categorization and metadata.
    ///
    /// 用于分类和元数据的键值对标签。
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

/// Time segmentation resource for tracking and managing timing information.
///
/// 时间分段资源，用于跟踪和管理时间信息。
///
/// This resource allows for precise timing control by storing both page-specific
/// and total application runtime, enabling coordinated animations.
///
/// 该资源通过存储页面特定运行时间和应用程序总运行时间实现精确的时间控制，支持协调动画。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SplitTime {
    /// Timing values: [page_runtime, total_runtime] in seconds.
    ///
    /// 时间点：[页面运行时间, 总运行时间]，单位为秒。
    pub time: [f32; 2],

    /// Key-value pairs for categorization and metadata storage.
    ///
    /// 用于分类和元数据存储的键值对标签。
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
