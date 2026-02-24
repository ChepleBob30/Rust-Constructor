//! Main application struct containing all GUI resources and state management.
//!
//! 程序主体，包含所有GUI资源和状态管理。
use crate::{
    ActiveListInfoMethod, BasicFrontResource, BorderKind, DisplayInfo, HorizontalAlign,
    PositionSizeConfig, RenderConfig, RequestMethod, RequestType, RustConstructorError,
    RustConstructorId, RustConstructorResource, RustConstructorResourceBox, Timer, VerticalAlign,
    advance_front::{
        Background, BackgroundType, ClickAim, PanelLocation, PanelMargin, PanelStorage,
        ResourcePanel, ScrollBarDisplayMethod, ScrollLengthMethod, Switch, SwitchData,
    },
    background::{Font, PageData, SplitTime, Variable},
    basic_front::{
        CustomRect, DebugTextureHandle, HyperlinkSelectMethod, Image, ImageLoadMethod, Text,
    },
};
use eframe::{
    emath::Rect,
    epaint::{Stroke, textures::TextureOptions},
};
use egui::{
    Color32, ColorImage, Context, CornerRadius, CursorIcon, FontData, FontDefinitions, FontFamily,
    FontId, Galley, Id, ImageSource, Key, OpenUrl, Pos2, Sense, StrokeKind, Ui, Vec2,
    text::CCursor,
};
use std::{
    any::type_name_of_val,
    char,
    cmp::Ordering,
    fmt::Debug,
    fs::{File, read},
    io::Read,
    sync::Arc,
    vec::Vec,
};

/// This struct serves as the central hub for the Rust Constructor framework.
///
/// 该结构体是Rust Constructor框架的中心枢纽。
#[derive(Debug)]
pub struct App {
    /// Collection of all Rust Constructor resources with type-erased storage.
    ///
    /// 所有Rust Constructor资源的集合，使用类型擦除存储。
    pub rust_constructor_resource: Vec<RustConstructorResourceBox>,

    /// Refresh rate for resource updates in seconds.
    ///
    /// 资源更新的刷新率（秒）。
    pub tick_interval: f32,

    /// Name of the current active page.
    ///
    /// 当前活动页面的名称。
    pub current_page: String,

    /// Timer for tracking application runtime and page durations.
    ///
    /// 用于跟踪应用程序运行时间和页面持续时间的计时器。
    pub timer: Timer,

    /// Record of recent frame times for performance monitoring.
    ///
    /// 最近帧时间的记录，用于性能监控。
    pub frame_times: Vec<f32>,

    /// Time taken to render the previous frame in seconds.
    ///
    /// 渲染上一帧所用的时间（秒）。
    pub last_frame_time: Option<f32>,

    /// List of resource IDs that are basic front resources.
    ///
    /// 基本前端资源的资源ID列表。
    ///
    /// This list should not be modified manually.
    ///
    /// 此列表不应手动修改。
    pub basic_front_resource_list: Vec<String>,

    /// Rendering layer information: (resource_id, [position, size], ignore_render_layer).
    ///
    /// 渲染层级信息：(资源ID, [位置, 尺寸], 是否忽略渲染层级)。
    pub render_layer: Vec<(RustConstructorId, [[f32; 2]; 2], bool)>,

    /// List of currently active resources.
    ///
    /// 当前活动的资源列表。
    pub active_list: Vec<RustConstructorId>,

    /// Queue of resources to be rendered in the current frame.
    ///
    /// 要在当前帧中呈现的资源队列。
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
                String::from("Image"),
                String::from("Text"),
                String::from("CustomRect"),
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

    /// Obtain the type name of the target resource.
    ///
    /// 获取目标资源的类型名称。
    ///
    /// # Arguments
    ///
    /// * `target` - Target resource
    ///
    /// # Returns
    ///
    /// Returns the type name of the target resource.
    ///
    /// # 参数
    ///
    /// * `target` - 目标资源
    ///
    /// # 返回
    ///
    /// 目标资源的类型名称。
    pub fn type_processor(&self, target: &impl RustConstructorResource) -> String {
        let result: Vec<_> = if let Some(list) = type_name_of_val(target).split_once("<") {
            list.0
        } else {
            type_name_of_val(&target)
        }
        .split("::")
        .collect();
        result[result.len() - 1].to_string()
    }

    /// Gets a tag value from the specified tag list by name.
    ///
    /// 从指定标签列表中根据名称获取标签值。
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag to retrieve
    /// * `target` - The list of tags to search through
    ///
    /// # Returns
    ///
    /// Returns `Some((index, value))` if the tag is found, or `None` if not found.
    ///
    /// # 参数
    ///
    /// * `tag_name` - 要检索的标签名称
    /// * `target` - 要搜索的标签列表
    ///
    /// # 返回值
    ///
    /// 如果找到标签则返回`Some((索引, 值))`，否则返回`None`。
    pub fn get_tag(&self, tag_name: &str, target: &[[String; 2]]) -> Option<(usize, String)> {
        target
            .iter()
            .position(|x| x[0] == tag_name)
            .map(|index| (index, target[index][1].clone()))
    }

    /// Draws all resources in the rendering queue at once, discarding all return values.
    ///
    /// 一次性绘制渲染队列中的所有资源，会丢弃所有返回值。
    ///
    /// This method iterates through all resources in the render list and draws them.
    /// It's not recommended for production use due to error handling limitations.
    ///
    /// 此方法遍历渲染列表中的所有资源并绘制它们。由于错误处理限制，不建议在生产环境中使用。
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI context for drawing
    /// * `ctx` - The rendering context
    ///
    /// # 参数
    ///
    /// * `ui` - 用于绘制的UI上下文
    /// * `ctx` - 渲染上下文
    pub fn draw_resources(&mut self, ui: &mut Ui, ctx: &Context) {
        for i in 0..self.render_list.len() {
            #[allow(warnings)]
            self.draw_resource_by_index(ui, ctx, i);
        }
    }

    /// Draws a specific resource by its index in the rendering queue.
    ///
    /// 根据资源在渲染队列中的索引值绘制特定资源。
    ///
    /// This method handles the rendering of different resource types including:
    /// - Images with various loading methods and transformations
    /// - Text with formatting, selection, and hyperlink support
    /// - Custom rectangles with borders and styling
    ///
    /// 此方法处理不同类型资源的渲染，包括：
    /// - 具有各种加载方法和变换的图像
    /// - 具有格式设置、选择和超链接支持的文本
    /// - 具有边框和样式的自定义矩形
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI context for drawing
    /// * `ctx` - The rendering context
    /// * `index` - The index of the resource in the render list
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource
    /// cannot be found or drawn.
    ///
    /// # 参数
    ///
    /// * `ui` - 用于绘制的UI上下文
    /// * `ctx` - 渲染上下文
    /// * `index` - 资源在渲染列表中的索引
    ///
    /// # 返回值
    ///
    /// 成功时返回`Ok(())`，如果资源无法找到或绘制则返回`Err(RustConstructorError)`。
    pub fn draw_resource_by_index(
        &mut self,
        ui: &mut Ui,
        ctx: &Context,
        index: usize,
    ) -> Result<(), RustConstructorError> {
        if let Some(render_resource) = self.render_list.clone().get(index) {
            match &*render_resource.discern_type {
                "Image" => {
                    let image = self.get_resource::<Image>(&RustConstructorId {
                        name: render_resource.name.clone(),
                        discern_type: "Image".to_string(),
                    })?;
                    if image.display_info.enable {
                        let mut image = image.clone();
                        match image.image_load_method {
                            ImageLoadMethod::ByPath((ref path, flip)) => {
                                if *path != image.last_frame_path {
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

                                        let color_image = ColorImage::from_rgba_unmultiplied(
                                            [w as usize, h as usize],
                                            &raw_data,
                                        );
                                        let loaded_image_texture = ctx.load_texture(
                                            &render_resource.name,
                                            color_image,
                                            TextureOptions::LINEAR,
                                        );
                                        image.texture =
                                            Some(DebugTextureHandle::new(&loaded_image_texture));
                                    } else {
                                        return Err(RustConstructorError {
                                            error_id: "ImageLoadFailed".to_string(),
                                            description: format!(
                                                "Failed to load an image from the path '{path}'.",
                                            ),
                                        });
                                    };
                                };
                            }
                            ImageLoadMethod::ByTexture(ref texture) => {
                                image.texture = Some(texture.clone());
                            }
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
                        match image.image_load_method {
                            ImageLoadMethod::ByPath((ref path, _)) => {
                                image.last_frame_path = path.clone()
                            }
                            ImageLoadMethod::ByTexture(_) => {}
                        };
                        self.replace_resource(&render_resource.name, image)?;
                    };
                }
                "Text" => {
                    let text = self.get_resource::<Text>(&RustConstructorId {
                        name: render_resource.name.clone(),
                        discern_type: "Text".to_string(),
                    })?;
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
                                        .check_resource_exists(&RustConstructorId {
                                            name: text.font.clone(),
                                            discern_type: "Font".to_string(),
                                        })
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

                                if let Some(index) =
                                    self.get_render_layer_resource(&RustConstructorId {
                                        name: render_resource.name.clone(),
                                        discern_type: "Text".to_string(),
                                    })
                                    && let Some(mouse_pos) = fullscreen_detect_result.interact_pos()
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
                                    if let Some(index) =
                                        self.get_render_layer_resource(&RustConstructorId {
                                            name: render_resource.name.clone(),
                                            discern_type: "Text".to_string(),
                                        })
                                        && let Some(mouse_pos) =
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
                "CustomRect" => {
                    let custom_rect = self.get_resource::<CustomRect>(&RustConstructorId {
                        name: render_resource.name.clone(),
                        discern_type: "CustomRect".to_string(),
                    })?;
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

    /// Generates information about currently active resources.
    ///
    /// 生成当前活跃资源的信息。
    ///
    /// This method returns a formatted string containing details about all resources
    /// in the active list. The level of detail depends on the specified method.
    ///
    /// 此方法返回一个格式化字符串，包含活动列表中所有资源的详细信息。
    /// 详细程度取决于指定的方法。
    ///
    /// # Arguments
    ///
    /// * `method` - Determines the level of detail in the output
    ///
    /// # Returns
    ///
    /// A formatted string with resource information.
    ///
    /// # 参数
    ///
    /// * `method` - 决定输出信息的详细程度
    ///
    /// # 返回值
    ///
    /// 包含资源信息的格式化字符串。
    pub fn active_list_info(&self, method: ActiveListInfoMethod) -> String {
        let mut text = String::from("Resource Active Info:\n");
        for info in &self.active_list {
            if let ActiveListInfoMethod::Detailed(format) = method {
                if let Some(index) = self.check_resource_exists(info) {
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

    /// Generates information about the current rendering layers.
    ///
    /// This method returns a formatted string containing details about the rendering
    /// layer stack, including resource positions and rendering behavior.
    ///
    /// # Returns
    ///
    /// A formatted string with rendering layer information
    ///
    /// 生成当前渲染层级的信息。
    ///
    /// 此方法返回一个格式化字符串，包含渲染层级堆栈的详细信息，
    /// 包括资源位置和渲染行为。
    ///
    /// # 返回值
    ///
    /// 包含渲染层级信息的格式化字符串
    pub fn render_layer_info(&self) -> String {
        let mut text = String::from("Render Layer Info:\n");
        for (
            RustConstructorId { name, discern_type },
            [min_position, max_position],
            ignore_render_layer,
        ) in &self.render_layer
        {
            text += &format!(
                "\nName: {}\nType: {}\nMin Position: {:?}\nMax Position: {:?}\nIgnore Render Layer: {}\n",
                name, discern_type, min_position, max_position, ignore_render_layer
            );
        }
        text
    }

    /// Generates information about the current render queue.
    ///
    /// 生成当前渲染队列的信息。
    ///
    /// This method returns a formatted string listing all resources in the
    /// render queue with their names and types.
    ///
    /// 此方法返回一个格式化字符串，列出渲染队列中的所有资源及其名称和类型。
    ///
    /// # Returns
    ///
    /// A formatted string with render queue information.
    ///
    /// # 返回值
    ///
    /// 包含渲染队列信息的格式化字符串。
    pub fn render_list_info(&self) -> String {
        let mut text = String::from("Render List Info:\n");
        for RustConstructorId { name, discern_type } in &self.render_list {
            text += &format!("\nName: {}\nType: {}\n", name, discern_type);
        }
        text
    }

    /// Updates the render queue based on active resources.
    ///
    /// 根据活跃资源更新渲染队列。
    ///
    /// This method synchronizes the render list with the active list, ensuring that
    /// only active basic front resources are included in the rendering queue.
    ///
    /// 此方法将渲染列表与活跃列表同步，确保只有活跃的基本前端资源包含在渲染队列中。
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

    /// Attempts to move a resource to the front of the render queue, ignoring whether it exists.
    ///
    /// 请求在渲染队列中插队，且无视申请跳过队列的资源是否存在。
    ///
    /// This is a safe wrapper around `request_jump_render_list` that suppresses errors.
    /// Use when you want to attempt reordering without handling potential errors.
    ///
    /// 这是`request_jump_render_list`的安全包装器，会抑制错误。当您想要尝试重新排序而不处理潜在错误时使用。
    ///
    /// # Arguments
    ///
    /// * `requester` - The resource to move in the queue
    /// * `request_type` - How to move the resource (to top or up N layers)
    ///
    /// # 参数
    ///
    /// * `requester` - 要在队列中移动的资源
    /// * `request_type` - 如何移动资源（到顶部或上移N层）
    pub fn try_request_jump_render_list(
        &mut self,
        requester: RequestMethod,
        request_type: RequestType,
    ) {
        #[allow(warnings)]
        self.request_jump_render_list(requester, request_type);
    }

    /// Moves a resource to the front of the render queue with error handling.
    ///
    /// 将资源移动到渲染队列的前面(含错误处理)。
    ///
    /// This method allows changing the rendering order of resources by moving a specific
    /// resource to the top of the queue or up a specified number of layers.
    ///
    /// 此方法允许通过将特定资源移动到队列顶部或上移指定层数来更改资源的渲染顺序。
    ///
    /// # Arguments
    ///
    /// * `requester` - The resource to move in the queue
    /// * `request_type` - How to move the resource (to top or up N layers)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource
    /// cannot be found.
    ///
    /// # 参数
    ///
    /// * `requester` - 要在队列中移动的资源
    /// * `request_type` - 如何移动资源（到顶部或上移N层）
    ///
    /// # 返回值
    ///
    /// 成功时返回`Ok(())`，如果资源无法找到则返回`Err(RustConstructorError)`。
    pub fn request_jump_render_list(
        &mut self,
        requester: RequestMethod,
        request_type: RequestType,
    ) -> Result<(), RustConstructorError> {
        match requester {
            RequestMethod::Id(id) => {
                if let Some(index) = self.render_list.iter().position(|x| *x == id) {
                    self.jump_render_list_processor(index, request_type)?;
                    Ok(())
                } else {
                    Err(RustConstructorError {
                        error_id: "RenderResourceNotFound".to_string(),
                        description: format!(
                            "Render resource '{}({})' not found.",
                            id.name, id.discern_type
                        ),
                    })
                }
            }
            RequestMethod::Citer(RustConstructorId { name, discern_type }) => {
                for (i, render_resource) in self.render_list.iter().enumerate() {
                    let tags = self.get_box_resource(render_resource)?.display_tags();
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
                    description: format!("Render resource '{name}({discern_type})' not found.",),
                })
            }
        }
    }

    /// Handle the operation of skipping the rendering queue.
    ///
    /// 处理跳过渲染队列操作。
    ///
    /// # Arguments
    ///
    /// * `requester_index` - The index of the resources to be moved in the queue
    /// * `request_type` - How to move the resource (to top or up N layers)
    ///
    /// # Returns
    ///
    /// When successful, return `Ok(())`. If the index is out of bounds, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `requester_index` - 要在队列中移动的资源的索引
    /// * `request_type` - 如何移动资源（到顶部或上移N层）
    ///
    /// # 返回值
    ///
    /// 成功时返回`Ok(())`，如果索引越界则返回`Err(RustConstructorError)`。
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
                    if requester_index + up <= self.render_list.len() {
                        requester_index + up
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

    /// Updates the rendering layer information for all rendering resources.
    ///
    /// 更新所有渲染资源的渲染层信息。
    ///
    /// This method recalculates the rendering layer by processing all resources
    /// in the render list and updating their position, size, and rendering properties.
    ///
    /// 此方法通过处理渲染列表中的所有资源并更新它们的位置、尺寸和渲染属性来重新计算渲染层级。
    pub fn update_render_layer(&mut self) {
        self.render_layer.clear();
        for info in &self.render_list {
            if let Some(index) = self.check_resource_exists(info) {
                let basic_front_resource: Box<dyn BasicFrontResource> = match &*info.discern_type {
                    "Image" => Box::new(
                        self.rust_constructor_resource[index]
                            .content
                            .as_any()
                            .downcast_ref::<Image>()
                            .unwrap()
                            .clone(),
                    ),
                    "Text" => Box::new(
                        self.rust_constructor_resource[index]
                            .content
                            .as_any()
                            .downcast_ref::<Text>()
                            .unwrap()
                            .clone(),
                    ),
                    "CustomRect" => Box::new(
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

    /// Draw the rendering layer.
    ///
    /// 绘制渲染层。
    ///
    /// This method can visually inspect the rendering status of all rendering
    /// resources and promptly correct any issues.
    ///
    /// 此方法可以直观检查所有渲染资源的渲染情况，并及时修正问题。
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI context for drawing
    /// * `render_config` - The config of the rendering layer area
    /// * `ignore_render` - The config of ignore the rendering layer area
    ///
    /// # 参数
    ///
    /// * `ui` - 用于绘制的UI上下文
    /// * `render_config` - 渲染层区域的配置
    /// * `ignore_render_config` - 无视渲染层区域的配置
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

    /// Search for resources in the render list by ID.
    ///
    /// 通过ID在渲染列表中查找资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the resource to search for
    ///
    /// # Returns
    ///
    /// The index of the resource in the render list, or None if not found
    ///
    /// # 参数
    ///
    /// * `id` - 要查找的资源的ID
    ///
    /// # 返回值
    ///
    /// 渲染列表中的资源索引，如果没有找到则为None
    pub fn get_render_layer_resource(&self, id: &RustConstructorId) -> Option<usize> {
        self.render_layer
            .iter()
            .position(|x| x.0.name == id.name && x.0.discern_type == id.discern_type)
    }

    /// Check whether the resource has obtained the mouse focus.
    ///
    /// 检查资源是否获取鼠标焦点。
    ///
    /// Use this method to ensure that mouse operations do not trigger
    /// multiple components simultaneously, causing confusion.
    ///
    /// 使用此方法以保证鼠标操作不会同时触发多个组件产生混乱。
    ///
    /// # Arguments
    ///
    /// * `index` - The index value of the rendering resource
    /// * `mouse_pos` - The position of the mouse
    ///
    /// # Returns
    ///
    /// Return true if the resource is not blocked; otherwise, return false.
    ///
    /// # 参数
    ///
    /// * `index` - 渲染资源的索引值
    /// * `mouse_pos` - 鼠标的位置
    ///
    /// # 返回值
    ///
    /// 如果资源未被阻挡，返回true，否则返回false。
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

    /// Mark active resources.
    ///
    /// 标记活跃资源。
    ///
    /// This method will be automatically called by the Rust Constructor without
    /// the need for manual control.
    ///
    /// 此方法会被Rust Constructor自动调用，无需手动控制。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// When a success mark is made, return `Ok(())`, and when the resource is not found,
    /// return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 成功标记时返回`Ok(())`，找不到资源时返回`Err(RustConstructorError)`。
    pub fn add_active_resource(
        &mut self,
        id: &RustConstructorId,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(id).is_some() {
            self.active_list.push(id.clone());
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{}({})' not found.", id.name, id.discern_type),
            })
        }
    }

    /// Adds a new resource to the application with the specified name.
    ///
    /// 添加一个新资源到应用程序中，并指定名称。
    ///
    /// This method registers a resource instance with a unique name. If the name is already in use
    /// or invalid, an error is returned. For certain resource types like SplitTime, it automatically
    /// initializes time values.
    ///
    /// 此方法使用唯一名称注册资源实例。如果名称已存在或无效，则返回错误。
    /// 对于某些资源类型（如 SplitTime），它会自动初始化时间值。
    ///
    /// # Arguments
    ///
    /// * `name` - A unique identifier for the resource
    /// * `resource` - The resource instance to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be added.
    ///
    /// # 参数
    ///
    /// * `name` - 资源的唯一标识符
    /// * `resource` - 要添加的资源实例
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法添加则返回 `Err(RustConstructorError)`。
    pub fn add_resource<T: RustConstructorResource + 'static>(
        &mut self,
        name: &str,
        mut resource: T,
    ) -> Result<(), RustConstructorError> {
        let discern_type = &*self.type_processor(&resource);
        if self
            .check_resource_exists(&RustConstructorId {
                name: name.to_string(),
                discern_type: discern_type.to_string(),
            })
            .is_some()
        {
            return Err(RustConstructorError {
                error_id: "ResourceNameRepetition".to_string(),
                description: format!("Resource '{name}({discern_type})' has already existed."),
            });
        };
        if name.is_empty() {
            return Err(RustConstructorError {
                error_id: "ResourceUntitled".to_string(),
                description: "All resources must have a valid name.".to_string(),
            });
        };
        match discern_type {
            "SplitTime" => {
                if let Some(split_time) = resource.as_any_mut().downcast_mut::<SplitTime>() {
                    split_time.time = [self.timer.now_time, self.timer.total_time];
                };
            }
            "Font" => {
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
                                "Failed to load a font from the path '{}'.",
                                font.path
                            ),
                        });
                    }
                };
            }
            "Background" => {
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
            "Switch" => {
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
            "ResourcePanel" => {
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

    /// Removes a resource from the application. This method is very dangerous! Ensure the resource is no longer in use before deletion.
    ///
    /// 移除资源。此方法非常危险！务必确保资源一定不再使用后删除。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn drop_resource(&mut self, id: &RustConstructorId) -> Result<(), RustConstructorError> {
        if let Some(index) = self.check_resource_exists(id) {
            self.rust_constructor_resource.remove(index);
            if let Some(index) = self.active_list.iter().position(|x| x == id) {
                self.active_list.remove(index);
            };
            if let Some(index) = self
                .render_layer
                .iter()
                .position(|x| x.0.name == id.name && x.0.discern_type == id.discern_type)
            {
                self.render_layer.remove(index);
            };
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{}({})' not found.", id.name, id.discern_type),
            })
        }
    }

    /// Replaces an existing resource with a new one in the application.
    ///
    /// 用应用程序中的新资源替换现有资源。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the resource to replace
    /// * `resource` - The new resource instance
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be found or replaced.
    ///
    /// # 参数
    ///
    /// * `name` - 要替换的资源名称
    /// * `resource` - 新的资源实例
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法找到或替换则返回 `Err(RustConstructorError)`。
    pub fn replace_resource<T>(
        &mut self,
        name: &str,
        resource: T,
    ) -> Result<(), RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        let discern_type = &*self.type_processor(&resource);
        if let Some(index) = self.check_resource_exists(&RustConstructorId {
            name: name.to_string(),
            discern_type: discern_type.to_string(),
        }) {
            self.rust_constructor_resource[index] =
                RustConstructorResourceBox::new(name, discern_type, Box::new(resource));
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{name}({discern_type})' not found."),
            })
        }
    }

    /// Obtain the boxed immutable resources from the list.
    ///
    /// 从列表中获取封装的不可变资源。
    ///
    /// If you need to use a resource without knowing its type, please use this method to retrieve the resource.
    ///
    /// 如果需要在不知道类型的情况下使用资源，请使用此方法取出资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// If the resource is found, return the reference of the resource; otherwise, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 如果找到资源，返回资源的引用，否则返回`Err(RustConstructorError)`。
    pub fn get_box_resource(
        &self,
        id: &RustConstructorId,
    ) -> Result<&dyn RustConstructorResource, RustConstructorError> {
        if let Some(index) = self.check_resource_exists(id) {
            Ok(&*self.rust_constructor_resource[index].content)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{}({})' not found.", id.name, id.discern_type),
            })
        }
    }

    /// Obtain the boxed mutable resources from the list.
    ///
    /// 从列表中获取封装的可变资源。
    ///
    /// If you need to use a resource without knowing its type, please use this method to retrieve the resource.
    ///
    /// 如果需要在不知道类型的情况下使用资源，请使用此方法取出资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// If the resource is found, return the mutable reference of the resource; otherwise, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 如果找到资源，返回资源的可变引用，否则返回`Err(RustConstructorError)`。
    pub fn get_box_resource_mut(
        &mut self,
        id: &RustConstructorId,
    ) -> Result<&mut dyn RustConstructorResource, RustConstructorError> {
        if let Some(index) = self.check_resource_exists(id) {
            Ok(&mut *self.rust_constructor_resource[index].content)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{}({})' not found.", id.name, id.discern_type),
            })
        }
    }

    /// Obtain the immutable resources from the list.
    ///
    /// 从列表中获取不可变资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// If the resource is found, return the reference of the resource; otherwise, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 如果找到资源，返回资源的引用，否则返回`Err(RustConstructorError)`。
    pub fn get_resource<T>(&self, id: &RustConstructorId) -> Result<&T, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if let Some(resource) = self.get_box_resource(id)?.as_any().downcast_ref::<T>() {
            Ok(resource)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource '{}({})' is mismatched.",
                    id.name, id.discern_type
                ),
            })
        }
    }

    /// Obtain the mutable resources from the list.
    ///
    /// 从列表中获取可变资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// If the resource is found, return the reference of the resource; otherwise, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 如果找到资源，返回资源的引用，否则返回`Err(RustConstructorError)`。
    pub fn get_resource_mut<T>(
        &mut self,
        id: &RustConstructorId,
    ) -> Result<&mut T, RustConstructorError>
    where
        T: RustConstructorResource + 'static,
    {
        if let Some(resource) = self
            .get_box_resource_mut(id)?
            .as_any_mut()
            .downcast_mut::<T>()
        {
            Ok(resource)
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource '{}({})' is mismatched.",
                    id.name, id.discern_type
                ),
            })
        }
    }

    /// Checks if a specific resource exists in the application.
    ///
    /// 检查应用程序中是否存在特定资源。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    ///
    /// # Returns
    ///
    /// Returns `Some(index)` if the resource exists, or `None` if not found.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    ///
    /// # 返回值
    ///
    /// 如果资源存在则返回 `Some(索引)`，否则返回 `None`。
    pub fn check_resource_exists(&self, id: &RustConstructorId) -> Option<usize> {
        self.rust_constructor_resource
            .iter()
            .position(|x| &x.id == id)
    }

    /// Quickly adds and uses a resource in one operation.
    ///
    /// 快速添加并使用资源。
    ///
    /// This method combines adding a resource to the application and immediately using it.
    ///
    /// 此方法将资源添加到应用程序并立即使用它。
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the resource
    /// * `resource` - The resource instance to add and draw
    /// * `ui` - The UI context for drawing
    /// * `ctx` - The rendering context
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be added or drawn.
    ///
    /// # 参数
    ///
    /// * `name` - 资源的名称
    /// * `resource` - 要添加和绘制的资源实例
    /// * `ui` - 用于绘制的UI上下文
    /// * `ctx` - 渲染上下文
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法添加或绘制则返回 `Err(RustConstructorError)`。
    pub fn quick_place<T: RustConstructorResource + 'static>(
        &mut self,
        name: &str,
        resource: T,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        let discern_type = &*self.type_processor(&resource);
        if self
            .check_resource_exists(&RustConstructorId {
                name: name.to_string(),
                discern_type: discern_type.to_string(),
            })
            .is_none()
        {
            self.add_resource(name, resource)
        } else {
            self.use_resource(
                &RustConstructorId {
                    name: name.to_string(),
                    discern_type: discern_type.to_string(),
                },
                ui,
                ctx,
            )
        }
    }

    /// Use the existing resources.
    ///
    /// 使用已存在的资源。
    ///
    /// This method invokes existing resources and performs operations.
    ///
    /// 此方法调用存在的资源并进行操作。
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the resource
    /// * `ui` - The UI context for drawing
    /// * `ctx` - The rendering context
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be handled.
    ///
    /// # 参数
    ///
    /// * `id` - 资源的唯一标识符
    /// * `ui` - 用于绘制的UI上下文
    /// * `ctx` - 渲染上下文
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法处理则返回 `Err(RustConstructorError)`。
    pub fn use_resource(
        &mut self,
        id: &RustConstructorId,
        ui: &mut Ui,
        ctx: &Context,
    ) -> Result<(), RustConstructorError> {
        if self.check_resource_exists(id).is_some() {
            match &*id.discern_type {
                "CustomRect" | "Text" | "Image" => {
                    self.add_active_resource(id)?;
                }
                "PageData" => {
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
                    let page_data = self.get_resource::<PageData>(&RustConstructorId {
                        name: self.current_page.clone(),
                        discern_type: "PageData".to_string(),
                    })?;
                    if page_data.forced_update {
                        ctx.request_repaint();
                    };
                }
                "Background" => {
                    let background = self.get_resource::<Background>(id)?.clone();
                    if background.auto_update {
                        match &background.background_type {
                            BackgroundType::CustomRect(config) => {
                                let mut custom_rect = self
                                    .get_resource::<CustomRect>(&RustConstructorId {
                                        name: id.name.clone(),
                                        discern_type: "CustomRect".to_string(),
                                    })?
                                    .clone()
                                    .from_config(config);
                                if background.use_background_tags {
                                    custom_rect.modify_tags(&background.tags, false);
                                };
                                self.replace_resource(&id.name, custom_rect)?;
                            }
                            BackgroundType::Image(config) => {
                                let mut image = self
                                    .get_resource::<Image>(&RustConstructorId {
                                        name: id.name.clone(),
                                        discern_type: "Image".to_string(),
                                    })?
                                    .clone()
                                    .from_config(config);
                                if background.use_background_tags {
                                    image.modify_tags(&background.tags, false);
                                };
                                self.replace_resource(&id.name, image)?;
                            }
                        };
                    };
                    match background.background_type {
                        BackgroundType::CustomRect(_) => self.use_resource(
                            &RustConstructorId {
                                name: id.name.clone(),
                                discern_type: "CustomRect".to_string(),
                            },
                            ui,
                            ctx,
                        ),
                        BackgroundType::Image(_) => self.use_resource(
                            &RustConstructorId {
                                name: id.name.clone(),
                                discern_type: "Image".to_string(),
                            },
                            ui,
                            ctx,
                        ),
                    }?;
                }
                "Switch" => {
                    let mut switch = self.get_resource::<Switch>(id)?.clone();
                    let mut background = self
                        .get_resource::<Background>(&RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: "Background".to_string(),
                        })?
                        .clone();
                    let background_resource_type = match switch.background_type {
                        BackgroundType::CustomRect(_) => "CustomRect",
                        BackgroundType::Image(_) => "Image",
                    };
                    let background_resource: Box<dyn BasicFrontResource> =
                        match background_resource_type {
                            "CustomRect" => Box::new(
                                self.get_resource::<CustomRect>(&RustConstructorId {
                                    name: format!("{}Background", &id.name),
                                    discern_type: background_resource_type.to_string(),
                                })?
                                .clone(),
                            ),
                            "Image" => Box::new(
                                self.get_resource::<Image>(&RustConstructorId {
                                    name: format!("{}Background", &id.name),
                                    discern_type: background_resource_type.to_string(),
                                })?
                                .clone(),
                            ),
                            _ => {
                                unreachable!()
                            }
                        };
                    let mut text = self
                        .get_resource::<Text>(&RustConstructorId {
                            name: format!("{}Text", &id.name),
                            discern_type: "Text".to_string(),
                        })?
                        .clone();
                    let mut hint_text = self
                        .get_resource::<Text>(&RustConstructorId {
                            name: format!("{}HintText", &id.name),
                            discern_type: "Text".to_string(),
                        })?
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
                    if let Some(index) = self.get_render_layer_resource(&RustConstructorId {
                        name: format!("{}Background", &id.name),
                        discern_type: background_resource_type.to_string(),
                    }) && switch.enable
                        && let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
                        && self.resource_get_focus(index, mouse_pos.into())
                        && let Some(display_info) = background_resource.display_display_info()
                        && !display_info.hidden
                    {
                        // 判断是否在矩形内
                        if rect.contains(mouse_pos) {
                            if !switch.last_frame_hovered {
                                self.reset_split_time(&format!("{}StartHoverTime", &id.name))?;
                            } else if self.timer.total_time
                                - self.get_split_time(&format!("{}StartHoverTime", &id.name))?[1]
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
                            self.reset_split_time(&format!("{}HintFadeAnimation", &id.name))?;
                        };
                        if self.timer.total_time
                            - self.get_split_time(&format!("{}HintFadeAnimation", &id.name))?[1]
                            >= self.tick_interval
                        {
                            self.reset_split_time(&format!("{}HintFadeAnimation", &id.name))?;
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

                    self.replace_resource(&format!("{}Text", &id.name), text)?;
                    self.replace_resource(&format!("{}HintText", &id.name), hint_text)?;
                    self.replace_resource(&id.name, switch)?;
                    self.replace_resource(&format!("{}Background", &id.name), background)?;

                    self.use_resource(
                        &RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: "Background".to_string(),
                        },
                        ui,
                        ctx,
                    )?;
                    self.use_resource(
                        &RustConstructorId {
                            name: format!("{}Text", &id.name),
                            discern_type: "Text".to_string(),
                        },
                        ui,
                        ctx,
                    )?;
                    if alpha != 0 {
                        if let [Some(hint_text_index), Some(switch_background_index)] = [
                            self.get_render_layer_resource(&RustConstructorId {
                                name: format!("{}HintText", &id.name),
                                discern_type: "Text".to_string(),
                            }),
                            self.get_render_layer_resource(&RustConstructorId {
                                name: format!("{}Background", &id.name),
                                discern_type: background_resource_type.to_string(),
                            }),
                        ] && hint_text_index < switch_background_index
                        {
                            self.try_request_jump_render_list(
                                RequestMethod::Id(RustConstructorId {
                                    name: format!("{}HintText", &id.name),
                                    discern_type: "Text".to_string(),
                                }),
                                RequestType::Up(switch_background_index - hint_text_index),
                            );
                        };
                        self.use_resource(
                            &RustConstructorId {
                                name: format!("{}HintText", &id.name),
                                discern_type: "Text".to_string(),
                            },
                            ui,
                            ctx,
                        )?;
                    };
                }
                "ResourcePanel" => {
                    let mut resource_panel = self
                        .get_resource::<ResourcePanel>(&RustConstructorId {
                            name: id.name.clone(),
                            discern_type: "ResourcePanel".to_string(),
                        })?
                        .clone();
                    let background = self
                        .get_resource::<Background>(&RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: "Background".to_string(),
                        })?
                        .clone();
                    let background_resource: Box<dyn BasicFrontResource> =
                        match background.background_type.clone() {
                            BackgroundType::CustomRect(_) => Box::new(
                                self.get_resource::<CustomRect>(&RustConstructorId {
                                    name: format!("{}Background", &id.name),
                                    discern_type: "CustomRect".to_string(),
                                })?
                                .clone(),
                            ),
                            BackgroundType::Image(_) => Box::new(
                                self.get_resource::<Image>(&RustConstructorId {
                                    name: format!("{}Background", &id.name),
                                    discern_type: "Image".to_string(),
                                })?
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
                        if let Some(index) = self.get_render_layer_resource(&RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: match background.background_type {
                                BackgroundType::CustomRect(_) => "CustomRect",
                                BackgroundType::Image(_) => "Image",
                            }
                            .to_string(),
                        }) && self.resource_get_focus(index, mouse_pos.into())
                        {
                            if ui.input(|i| i.pointer.primary_pressed())
                                && Rect::from_min_size(position.into(), size.into())
                                    .contains(mouse_pos)
                            {
                                self.request_jump_render_list(
                                    RequestMethod::Id(RustConstructorId {
                                        name: format!("{}Background", &id.name),
                                        discern_type: match background.background_type {
                                            BackgroundType::CustomRect(_) => "CustomRect",
                                            BackgroundType::Image(_) => "Image",
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
                                        && panel_name.1 == id.name
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
                                            name: format!("{}XScroll", &id.name),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => "CustomRect",
                                                BackgroundType::Image(_) => "Image",
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{}YScroll", &id.name),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => "CustomRect",
                                                BackgroundType::Image(_) => "Image",
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
                                            name: format!("{}XScroll", &id.name),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => "CustomRect",
                                                BackgroundType::Image(_) => "Image",
                                            }
                                            .to_string(),
                                        }),
                                        RequestType::Top,
                                    );
                                    self.try_request_jump_render_list(
                                        RequestMethod::Id(RustConstructorId {
                                            name: format!("{}YScroll", &id.name),
                                            discern_type: match background_type {
                                                BackgroundType::CustomRect(_) => "CustomRect",
                                                BackgroundType::Image(_) => "Image",
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
                        &format!("{}Background", &id.name),
                        background.clone().background_type(&background_type).clone(),
                    )?;
                    self.use_resource(
                        &RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: "Background".to_string(),
                        },
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
                            && panel_name.1 == id.name
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
                                "Image" => Box::new(
                                    rcr.content
                                        .as_any()
                                        .downcast_ref::<Image>()
                                        .unwrap()
                                        .clone(),
                                ),
                                "Text" => Box::new(
                                    rcr.content.as_any().downcast_ref::<Text>().unwrap().clone(),
                                ),
                                "CustomRect" => Box::new(
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
                        let id = RustConstructorId {
                            name: name.clone(),
                            discern_type: discern_type.clone(),
                        };
                        let default_storage = if let Some(resource_storage) =
                            resource_panel.resource_storage.iter().find(|x| x.id == id)
                        {
                            [
                                true,
                                resource_storage.ignore_render_layer,
                                resource_storage.hidden,
                            ]
                        } else {
                            [false, true, true]
                        };
                        match &*discern_type {
                            "CustomRect" => {
                                let mut custom_rect = self.get_resource::<CustomRect>(&id)?.clone();
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
                            "Image" => {
                                let mut image = self.get_resource::<Image>(&id)?.clone();
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
                            "Text" => {
                                let mut text = self.get_resource::<Text>(&id)?.clone();
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
                        self.use_resource(
                            &RustConstructorId {
                                name: info[0].clone(),
                                discern_type: info[1].clone(),
                            },
                            ui,
                            ctx,
                        )?;
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
                                &format!("{}XScroll", &id.name),
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
                                &RustConstructorId {
                                    name: format!("{}XScroll", &id.name),
                                    discern_type: "Background".to_string(),
                                },
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
                                &format!("{}YScroll", &id.name),
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
                                &RustConstructorId {
                                    name: format!("{}YScroll", &id.name),
                                    discern_type: "Background".to_string(),
                                },
                                ui,
                                ctx,
                            )?;
                        }
                        ScrollBarDisplayMethod::OnlyScroll(ref config, margin, width) => {
                            resource_panel.scroll_bar_alpha[0] = if resource_panel.scrolled[0] {
                                self.reset_split_time(&format!(
                                    "{}ScrollBarXAlphaStart",
                                    &id.name
                                ))?;
                                255
                            } else if self.timer.now_time
                                - self
                                    .get_split_time(&format!("{}ScrollBarXAlphaStart", &id.name))?
                                    [0]
                                >= 1_f32
                                && self.timer.now_time
                                    - self
                                        .get_split_time(&format!("{}ScrollBarXAlpha", &id.name))?[0]
                                    >= self.tick_interval
                            {
                                self.reset_split_time(&format!("{}ScrollBarXAlpha", &id.name))?;
                                resource_panel.scroll_bar_alpha[0].saturating_sub(10)
                            } else {
                                resource_panel.scroll_bar_alpha[0]
                            };
                            resource_panel.scroll_bar_alpha[1] = if resource_panel.scrolled[1] {
                                self.reset_split_time(&format!(
                                    "{}ScrollBarYAlphaStart",
                                    &id.name
                                ))?;
                                255
                            } else if self.timer.now_time
                                - self
                                    .get_split_time(&format!("{}ScrollBarYAlphaStart", &id.name))?
                                    [0]
                                >= 1_f32
                                && self.timer.now_time
                                    - self
                                        .get_split_time(&format!("{}ScrollBarYAlpha", &id.name))?[0]
                                    >= self.tick_interval
                            {
                                self.reset_split_time(&format!("{}ScrollBarYAlpha", &id.name))?;
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
                                &format!("{}XScroll", &id.name),
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
                                &RustConstructorId {
                                    name: format!("{}XScroll", &id.name),
                                    discern_type: "Background".to_string(),
                                },
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
                                &format!("{}YScroll", &id.name),
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
                                &RustConstructorId {
                                    name: format!("{}YScroll", &id.name),
                                    discern_type: "Background".to_string(),
                                },
                                ui,
                                ctx,
                            )?;
                        }
                        ScrollBarDisplayMethod::Hidden => {}
                    };
                    self.replace_resource(&id.name, resource_panel.clone())?;
                }
                _ => {}
            };
            Ok(())
        } else {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{}({})' not found.", id.name, id.discern_type),
            })
        }
    }

    /// Switches to a different page and resets page-specific state.
    ///
    /// 切换到不同页面并重置页面特定状态。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the page to switch to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the page cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 要切换到的页面名称
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果页面无法找到则返回 `Err(RustConstructorError)`。
    pub fn switch_page(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "PageData".to_string(),
        })?;
        page_data.enter_page_updated = false;
        self.timer.start_time = self.timer.total_time;
        self.current_page = name.to_string();
        self.update_timer();
        Ok(())
    }

    /// Retrieves font definitions for a font resource.
    ///
    /// 获取字体资源的字体定义。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the font resource
    ///
    /// # Returns
    ///
    /// Returns `Ok(FontDefinitions)` if the font exists, or `Err(RustConstructorError)` if not found.
    ///
    /// # 参数
    ///
    /// * `name` - 字体资源的名称
    ///
    /// # 返回值
    ///
    /// 如果字体存在则返回 `Ok(FontDefinitions)`，否则返回 `Err(RustConstructorError)`。
    pub fn get_font(&self, name: &str) -> Result<FontDefinitions, RustConstructorError> {
        let font = self.get_resource::<Font>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "Font".to_string(),
        })?;
        Ok(font.font_definitions.clone())
    }

    /// Registers all font resources with the egui context.
    ///
    /// 向egui上下文中注册所有字体资源。
    ///
    /// This method loads and registers all fonts that have been added to the application
    /// with the egui rendering system for text display.
    ///
    /// 此方法加载并注册应用程序中已添加的所有字体到egui渲染系统中，用于文本显示。
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for font registration
    ///
    /// # 参数
    ///
    /// * `ctx` - 用于字体注册的egui上下文
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

    /// Processes position and size calculations for resources.
    ///
    /// 处理资源的最基本位置和尺寸计算。
    ///
    /// This method handles the complex positioning logic including grid-based layout,
    /// alignment, and offset calculations for UI resources.
    ///
    /// 此方法处理复杂的定位逻辑，包括基于网格的布局、对齐方式和UI资源的偏移计算。
    ///
    /// # Arguments
    ///
    /// * `position_size_config` - The configuration for position and size
    /// * `ctx` - The egui context for available space calculations
    ///
    /// # Returns
    ///
    /// Returns `[position, size]` as computed from the configuration
    ///
    /// # 参数
    ///
    /// * `position_size_config` - 位置和尺寸的配置
    /// * `ctx` - 用于可用空间计算的egui上下文
    ///
    /// # 返回值
    ///
    /// 返回根据配置计算出的 `[位置, 尺寸]`
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

    /// Checks if a page has completed its initial loading phase.
    ///
    /// 检查页面是否已完成首次加载。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the page to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the page has completed loading, or `Ok(false)` if the page has not completed loading.
    /// Returns `Err(RustConstructorError)` if the page cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 要检查的页面名称
    ///
    /// # 返回值
    ///
    /// 如果页面已完成加载则返回 `Ok(true)`，如果未加载则返回 `Ok(false)`。
    /// 如果页面无法找到则返回 `Err(RustConstructorError)`。
    pub fn check_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        let page_data = self
            .get_resource::<PageData>(&RustConstructorId {
                name: name.to_string(),
                discern_type: "PageData".to_string(),
            })?
            .clone();
        if !page_data.change_page_updated {
            self.new_page_update(name)?;
        };
        Ok(page_data.change_page_updated)
    }

    /// Checks if a page has completed its enter transition.
    ///
    /// 检查页面是否已完成进入过渡。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the page to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the page has completed entering, or `Ok(false)` if the page has not completed entering.
    /// Returns `Err(RustConstructorError)` if the page cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 要检查的页面名称
    ///
    /// # 返回值
    ///
    /// 如果页面已完成进入则返回 `Ok(true)`，如果未过渡则返回 `Ok(false)`。
    /// 如果页面无法找到则返回 `Err(RustConstructorError)`。
    pub fn check_enter_updated(&mut self, name: &str) -> Result<bool, RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "PageData".to_string(),
        })?;
        page_data.enter_page_updated = true;
        Ok(page_data.enter_page_updated)
    }

    /// Updates when entering a new page.
    ///
    /// 进入新页面时的更新。
    ///
    /// This method is used to ensure the accuracy of the content based on the page, and the Rust Constructor will automatically call this method.
    ///
    /// 此方法用于确保基于页面的内容的准确性，Rust Constructor会自动调用此方法。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the page to be updated
    ///
    /// # Returns
    ///
    /// If the update is successful, return `Ok(())`; if the resource is not found, return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `name` - 要更新的页面名称
    ///
    /// # 返回值
    ///
    /// 如果更新成功则返回`Ok(())`，找不到资源则返回`Err(RustConstructorError)`。
    pub fn new_page_update(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let page_data = self.get_resource_mut::<PageData>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "PageData".to_string(),
        })?;
        page_data.change_page_updated = true;
        self.timer.start_time = self.timer.total_time;
        self.update_timer();
        Ok(())
    }

    /// Updates frame timing statistics for performance monitoring.
    ///
    /// 更新帧数统计信息用于性能监控。
    ///
    /// This method maintains a rolling window of frame times and calculates
    /// performance metrics like frame rate.
    ///
    /// 此方法维护帧时间的滚动窗口并计算帧率等性能指标。
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

    /// Update the frame rate.
    ///
    /// 更新帧数。
    ///
    /// This method is used to obtain the number of program frames and conduct analysis.
    ///
    /// 此方法用于获取程序帧数并进行分析。
    ///
    /// # Returns
    ///
    /// Return the number of frames.
    ///
    /// # 返回值
    ///
    /// 返回帧数。
    pub fn current_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            1.0 / (self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32)
        }
    }

    /// Resets the split time for a specific resource.
    ///
    /// 重置特定资源的分段计时器。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the split time resource to reset
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 要重置的分段时间资源名称
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn reset_split_time(&mut self, name: &str) -> Result<(), RustConstructorError> {
        let new_time = [self.timer.now_time, self.timer.total_time];
        let split_time = self.get_resource_mut::<SplitTime>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "SplitTime".to_string(),
        })?;
        split_time.time = new_time;
        Ok(())
    }

    /// Retrieves the timing information from a split time resource.
    ///
    /// 获取分段计时器资源的时间信息。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the split time resource
    ///
    /// # Returns
    ///
    /// Returns `Ok([page_runtime, total_runtime])` if found, or `Err(RustConstructorError)` if not found.
    ///
    /// # 参数
    ///
    /// * `name` - 分段计时器资源的名称
    ///
    /// # 返回值
    ///
    /// 如果找到则返回 `Ok([页面运行时间, 总运行时间])`，否则返回 `Err(RustConstructorError)`。
    pub fn get_split_time(&self, name: &str) -> Result<[f32; 2], RustConstructorError> {
        let split_time = self.get_resource::<SplitTime>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "SplitTime".to_string(),
        })?;
        Ok(split_time.time)
    }

    /// Updates the application timer with current timing information.
    ///
    /// 更新应用程序计时器的当前时间信息。
    ///
    /// This method updates both the total runtime and current page runtime.
    ///
    /// 此方法更新总运行时间和当前页面运行时间。
    pub fn update_timer(&mut self) {
        let elapsed = self.timer.timer.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();
        self.timer.total_time = seconds as f32 + milliseconds as f32 / 1000.0;
        self.timer.now_time = self.timer.total_time - self.timer.start_time
    }

    /// Modifies the value of a variable resource.
    ///
    /// 修改变量资源的值。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable resource
    /// * `value` - The new value to set (use `None` to clear)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 变量资源的名称
    /// * `value` - 要设置的新值（使用 `None` 来清除）
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn modify_variable<T: Debug + 'static>(
        &mut self,
        name: &str,
        value: Option<T>,
    ) -> Result<(), RustConstructorError> {
        let variable = self.get_resource_mut::<Variable<T>>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "Variable".to_string(),
        })?;
        variable.value = value;
        Ok(())
    }

    /// Take the variable out of the list.
    ///
    /// 从列表中取出变量。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable resource
    ///
    /// # Returns
    ///
    /// Returns `Ok(Option<T>)` on success, or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 变量资源的名称
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(Option<T>)`，如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn get_variable<T: Debug + Clone + 'static>(
        &self,
        name: &str,
    ) -> Result<Option<T>, RustConstructorError> {
        if let Ok(variable) = self.get_resource::<Variable<T>>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "Variable".to_string(),
        }) {
            Ok(variable.value.clone())
        } else if self
            .check_resource_exists(&RustConstructorId {
                name: name.to_string(),
                discern_type: "Variable".to_string(),
            })
            .is_none()
        {
            Err(RustConstructorError {
                error_id: "ResourceNotFound".to_string(),
                description: format!("Resource '{name}(Variable<T>)' not found."),
            })
        } else {
            Err(RustConstructorError {
                error_id: "ResourceGenericMismatch".to_string(),
                description: format!(
                    "The generic type of the resource '{name}(Variable<T>)' is mismatched."
                ),
            })
        }
    }

    /// Modify the enable status of the switch.
    ///
    /// 修改开关的启用状态。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the switch resource
    /// * `enable` - The new enable status
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 开关资源的名称
    /// * `enable` - 新的启用状态
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`，如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn set_switch_enable(
        &mut self,
        name: &str,
        enable: bool,
    ) -> Result<(), RustConstructorError> {
        let switch = self.get_resource_mut::<Switch>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "Switch".to_string(),
        })?;
        switch.enable = enable;
        Ok(())
    }

    /// Retrieves the current state and interaction data from a switch resource.
    ///
    /// 获取开关资源的当前状态和交互数据。
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the switch resource
    ///
    /// # Returns
    ///
    /// Returns `Ok(SwitchData)` containing the switch state and interaction history,
    /// or `Err(RustConstructorError)` if the resource cannot be found.
    ///
    /// # 参数
    ///
    /// * `name` - 开关资源的名称
    ///
    /// # 返回值
    ///
    /// 返回包含开关状态和交互历史的 `Ok(SwitchData)`，
    /// 如果资源无法找到则返回 `Err(RustConstructorError)`。
    pub fn check_switch_data(&self, name: &str) -> Result<SwitchData, RustConstructorError> {
        let switch = self.get_resource::<Switch>(&RustConstructorId {
            name: name.to_string(),
            discern_type: "Switch".to_string(),
        })?;
        Ok(SwitchData {
            switched: switch.switched,
            last_frame_clicked: switch.last_frame_clicked,
            state: switch.state,
        })
    }
}
