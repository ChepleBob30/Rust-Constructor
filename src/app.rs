//! Main application struct containing all GUI resources and state management.
//!
//! 程序主体，包含所有GUI资源和状态管理。
use crate::{
    BasicFrontResource, BorderKind, DisplayInfo, HorizontalAlign, ListInfoDescribeMethod,
    PositionSizeConfig, RenderConfig, RequestMethod, RequestType, RustConstructorError,
    RustConstructorId, RustConstructorResource, RustConstructorResourceBox, Timer, VerticalAlign,
    advance_front::{
        Background, BackgroundType, ClickAim, CustomPanelLayout, PanelLocation, PanelMargin,
        PanelStorage, ResourcePanel, ScrollBarDisplayMethod, ScrollLengthMethod, Switch,
        SwitchData,
    },
    background::{PageData, SplitTime, Variable},
    basic_front::{
        CustomRect, DebugTextureHandle, HyperlinkSelectMethod, Image, ImageLoadMethod, Text,
    },
    downcast_resource, downcast_resource_mut, get_tag, position_size_processor, type_processor,
};
use eframe::{
    egui::{
        Color32, ColorImage, Context, CornerRadius, CursorIcon, FontData, FontDefinitions,
        FontFamily, FontId, Galley, Id, ImageSource, Key, OpenUrl, Pos2, Sense, StrokeKind, Ui,
        Vec2, text::CCursor,
    },
    emath::Rect,
    epaint::{Stroke, textures::TextureOptions},
};
use std::{
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

    /// Refresh rate for resource updates in milliseconds.
    ///
    /// 资源更新的刷新率（毫秒）。
    pub tick_interval: u128,

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
    pub frame_times: Vec<u128>,

    /// The time for rendering the previous frame in milliseconds.
    ///
    /// 渲染上一帧的时间（毫秒）。
    pub last_frame_time: Option<u128>,

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
    pub active_list: Vec<(RustConstructorId, Option<RustConstructorId>)>,

    /// Queue of resources to be rendered in the current frame.
    ///
    /// 要在当前帧中呈现的资源队列。
    pub render_list: Vec<(RustConstructorId, Option<RustConstructorId>)>,

    /// List the loaded fonts.
    ///
    /// 列出已加载的字体。
    pub loaded_fonts: Vec<[String; 2]>,

    /// List the fonts that are currently loading.
    ///
    /// 列出正在加载的字体。
    pub loading_fonts: Vec<[String; 2]>,
}

impl Default for App {
    fn default() -> Self {
        App {
            rust_constructor_resource: Vec::new(),
            tick_interval: 50,
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
            loaded_fonts: Vec::new(),
            loading_fonts: Vec::new(),
        }
    }
}

impl App {
    #[inline]
    pub fn tick_interval(mut self, tick_interval: u128) -> Self {
        self.tick_interval = tick_interval;
        self
    }

    #[inline]
    pub fn current_page(mut self, current_page: &str) -> Self {
        self.current_page = current_page.to_string();
        self
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
            let _ = self.draw_resource_by_index(ui, ctx, i);
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
            match &*render_resource.0.discern_type {
                "Image" => {
                    let image = self.get_resource::<Image>(&RustConstructorId {
                        name: render_resource.0.name.clone(),
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
                                            &render_resource.0.name,
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
                        [image.position, image.size] = position_size_processor(
                            image.basic_front_resource_config.position_size_config,
                            ctx,
                        );
                        if !image.display_info.hidden {
                            if let Some(clip_rect) = image.basic_front_resource_config.clip_rect {
                                let [min, size] = position_size_processor(clip_rect, ctx);
                                ui.set_clip_rect(Rect::from_min_size(min.into(), size.into()));
                            };
                            if let Some(texture) = &image.texture {
                                let rect = Rect::from_min_size(
                                    Pos2::new(image.position[0], image.position[1]),
                                    Vec2::new(image.size[0], image.size[1]),
                                );

                                // 直接绘制图片
                                eframe::egui::Image::new(ImageSource::Texture((&texture.0).into()))
                                    .tint(Color32::from_rgba_unmultiplied(
                                        image.overlay_color[0],
                                        image.overlay_color[1],
                                        image.overlay_color[2],
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
                        self.replace_resource(&render_resource.0.name, image)?;
                    };
                }
                "Text" => {
                    let text = self.get_resource::<Text>(&RustConstructorId {
                        name: render_resource.0.name.clone(),
                        discern_type: "Text".to_string(),
                    })?;
                    if text.display_info.enable {
                        let mut text = text.clone();
                        [_, text.truncate_size] = position_size_processor(
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
                                    if self.loaded_fonts.iter().any(|x| x[0] == text.font) {
                                        FontId::new(
                                            text.font_size,
                                            FontFamily::Name(text.font.clone().into()),
                                        )
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
                        [text.position, _] = position_size_processor(
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
                                let [min, size] = position_size_processor(clip_rect, ctx);
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
                                    Id::new(&render_resource.0.name),
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
                                        name: render_resource.0.name.clone(),
                                        discern_type: "Text".to_string(),
                                    })
                                    && let Some(mouse_pos) = fullscreen_detect_result.interact_pos()
                                    && self.resource_get_focus(
                                        index,
                                        mouse_pos.into(),
                                        false,
                                        vec![],
                                    )
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
                                        Id::new(format!(
                                            "link_{}_{}_{}",
                                            render_resource.0.name, start, end
                                        )),
                                        Sense::click(),
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
                                                    render_resource.0.name, start, end, row
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
                                            name: render_resource.0.name.clone(),
                                            discern_type: "Text".to_string(),
                                        })
                                        && let Some(mouse_pos) =
                                            ui.input(|i| i.pointer.interact_pos())
                                        && self.resource_get_focus(
                                            index,
                                            mouse_pos.into(),
                                            false,
                                            vec![],
                                        )
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
                        self.replace_resource(&render_resource.0.name, text)?;
                    };
                }
                "CustomRect" => {
                    let custom_rect = self.get_resource::<CustomRect>(&RustConstructorId {
                        name: render_resource.0.name.clone(),
                        discern_type: "CustomRect".to_string(),
                    })?;
                    if custom_rect.display_info.enable {
                        let mut custom_rect = custom_rect.clone();
                        [custom_rect.position, custom_rect.size] = position_size_processor(
                            custom_rect.basic_front_resource_config.position_size_config,
                            ctx,
                        );
                        if !custom_rect.display_info.hidden {
                            if let Some(clip_rect) =
                                custom_rect.basic_front_resource_config.clip_rect
                            {
                                let [min, size] = position_size_processor(clip_rect, ctx);
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
                                if let Some(overlay_alpha) = custom_rect.overlay_alpha {
                                    Color32::from_rgba_unmultiplied(
                                        (custom_rect.color[0] as f32
                                            * custom_rect.overlay_color[0] as f32
                                            / 255_f32)
                                            as u8,
                                        (custom_rect.color[1] as f32
                                            * custom_rect.overlay_color[1] as f32
                                            / 255_f32)
                                            as u8,
                                        (custom_rect.color[2] as f32
                                            * custom_rect.overlay_color[2] as f32
                                            / 255_f32)
                                            as u8,
                                        (custom_rect.alpha as f32 * overlay_alpha as f32 / 255_f32)
                                            as u8,
                                    )
                                } else {
                                    Color32::from_rgba_unmultiplied(
                                        custom_rect.color[0],
                                        custom_rect.color[1],
                                        custom_rect.color[2],
                                        custom_rect.alpha,
                                    )
                                },
                                Stroke {
                                    width: custom_rect.border_width,
                                    color: if let Some(overlay_border_alpha) =
                                        custom_rect.overlay_border_alpha
                                    {
                                        Color32::from_rgba_unmultiplied(
                                            (custom_rect.border_color[0] as f32
                                                * custom_rect.overlay_border_color[0] as f32
                                                / 255_f32)
                                                as u8,
                                            (custom_rect.border_color[1] as f32
                                                * custom_rect.overlay_border_color[1] as f32
                                                / 255_f32)
                                                as u8,
                                            (custom_rect.border_color[2] as f32
                                                * custom_rect.overlay_border_color[2] as f32
                                                / 255_f32)
                                                as u8,
                                            (custom_rect.border_alpha as f32
                                                * overlay_border_alpha as f32
                                                / 255_f32)
                                                as u8,
                                        )
                                    } else {
                                        Color32::from_rgba_unmultiplied(
                                            custom_rect.border_color[0],
                                            custom_rect.border_color[1],
                                            custom_rect.border_color[2],
                                            custom_rect.border_alpha,
                                        )
                                    },
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
                        self.replace_resource(&render_resource.0.name, custom_rect)?;
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

    /// Generate information for Rust Constructor resources.
    ///
    /// 生成Rust Constructor资源的信息。
    ///
    /// This method returns a formatted string containing details about all resources.
    /// The level of detail depends on the specified method.
    ///
    /// 此方法返回一个格式化字符串，包含所有资源的详细信息。
    /// 详细程度取决于指定的方法。
    ///
    /// # Arguments
    ///
    /// * `describe` - Determines the level of detail in the output
    /// * `print` - Determines whether to print
    ///
    /// # Returns
    ///
    /// A formatted string with resource information.
    ///
    /// # 参数
    ///
    /// * `describe` - 决定输出信息的详细程度
    /// * `print` - 决定是否打印
    ///
    /// # 返回值
    ///
    /// 包含资源信息的格式化字符串。
    pub fn rust_constructor_resource_info(
        &self,
        describe: ListInfoDescribeMethod,
        print: bool,
    ) -> String {
        let mut text =
            String::from("————————————————————————————————————\nRust Constructor Resource Info:\n");
        for info in &self.rust_constructor_resource {
            if let ListInfoDescribeMethod::Detailed(format) = describe {
                text += &if format {
                    format!(
                        "\nName: {}\nType: {}\nDetail: {:#?}\n",
                        info.id.name, info.id.discern_type, info.content,
                    )
                } else {
                    format!(
                        "\nName: {}\nType: {}\nDetail: {:?}\n",
                        info.id.name, info.id.discern_type, info.content,
                    )
                };
            } else {
                text += &format!("\nName: {}\nType: {}\n", info.id.name, info.id.discern_type,)
            };
        }
        if print {
            println!("{text}");
        };
        text
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
    /// * `describe` - Determines the level of detail in the output
    /// * `print` - Determines whether to print
    ///
    /// # Returns
    ///
    /// A formatted string with resource information.
    ///
    /// # 参数
    ///
    /// * `describe` - 决定输出信息的详细程度
    /// * `print` - 决定是否打印
    ///
    /// # 返回值
    ///
    /// 包含资源信息的格式化字符串。
    pub fn active_list_info(&self, describe: ListInfoDescribeMethod, print: bool) -> String {
        let mut text =
            String::from("————————————————————————————————————\nResource Active Info:\n");
        for info in &self.active_list {
            if let ListInfoDescribeMethod::Detailed(format) = describe {
                if let Some(index) = self.check_resource_exists(&info.0) {
                    text += &if format {
                        format!(
                            "\nName: {}\nType: {}\nCiter: {:?}\nDetail: {:#?}\n",
                            info.0.name,
                            info.0.discern_type,
                            info.1,
                            self.rust_constructor_resource[index],
                        )
                    } else {
                        format!(
                            "\nName: {}\nType: {}\nCiter: {:?}\nDetail: {:?}\n",
                            info.0.name,
                            info.0.discern_type,
                            info.1,
                            self.rust_constructor_resource[index],
                        )
                    };
                };
            } else {
                text += &format!(
                    "\nName: {}\nType: {}\nCiter: {:?}\n",
                    info.0.name, info.0.discern_type, info.1
                );
            };
        }
        if print {
            println!("{text}");
        };
        text
    }

    /// Generates information about the current rendering layers.
    ///
    /// 生成当前渲染层级的信息。
    ///
    /// This method returns a formatted string containing details about the rendering
    /// layer stack, including resource positions and rendering behavior.
    ///
    /// 此方法返回一个格式化字符串，包含渲染层级堆栈的详细信息，
    /// 包括资源位置和渲染行为。
    ///
    /// # Arguments
    ///
    /// * `print` - Determines whether to print
    ///
    /// # Returns
    ///
    /// A formatted string with rendering layer information.
    ///
    /// # 参数
    ///
    /// * `print` - 决定是否打印
    ///
    /// # 返回值
    ///
    /// 包含渲染层级信息的格式化字符串。
    pub fn render_layer_info(&self, print: bool) -> String {
        let mut text = String::from("————————————————————————————————————\nRender Layer Info:\n");
        for (
            RustConstructorId { name, discern_type },
            [min_position, max_position],
            ignore_render_layer,
        ) in &self.render_layer
        {
            text += &format!(
                "\nName: {}\nType: {}\nMin Position: {:?}\nMax Position: {:?}\nIgnore Render Layer: {}\n",
                name, discern_type, min_position, max_position, ignore_render_layer
            )
        }
        if print {
            println!("{text}");
        };
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
    /// # Arguments
    ///
    /// * `print` - Determines whether to print
    ///
    /// # Returns
    ///
    /// A formatted string with render queue information.
    ///
    /// # 参数
    ///
    /// * `print` - 决定是否打印
    ///
    /// # 返回值
    ///
    /// 包含渲染队列信息的格式化字符串。
    pub fn render_list_info(&self, print: bool) -> String {
        let mut text = String::from("————————————————————————————————————\nRender List Info:\n");
        for (RustConstructorId { name, discern_type }, citer) in &self.render_list {
            text += &format!(
                "\nName: {}\nType: {}\nCiter: {:?}\n",
                name, discern_type, citer
            )
        }
        if print {
            println!("{text}");
        };
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
                if self
                    .basic_front_resource_list
                    .contains(&info.0.discern_type)
                {
                    self.render_list.push(info.clone());
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
                if self
                    .basic_front_resource_list
                    .contains(&info.0.discern_type)
                {
                    if !self.render_list.contains(info) {
                        self.render_list.insert(insert_index, info.clone());
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
        let _ = self.request_jump_render_list(requester, request_type);
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
                if let Some(index) = self.render_list.iter().position(|x| x.0 == id) {
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
            RequestMethod::Citer(citer) => {
                for (i, render_resource) in self.render_list.iter().enumerate() {
                    if let Some(render_citer) = &render_resource.1
                        && render_citer == &citer
                    {
                        self.jump_render_list_processor(i, request_type)?;
                        return Ok(());
                    };
                }
                Err(RustConstructorError {
                    error_id: "RenderResourceNotFound".to_string(),
                    description: format!(
                        "Render resource citer '{}({})' not found.",
                        citer.name, citer.discern_type
                    ),
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
    ///
    /// # Arguments
    ///
    /// * `ctx` - The rendering context
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(RustConstructorError)` if the resource
    /// cannot be found.
    ///
    /// # 参数
    ///
    /// * `ctx` - 渲染上下文
    ///
    /// # 返回值
    ///
    /// 成功时返回`Ok(())`，如果资源无法找到则返回`Err(RustConstructorError)`。
    pub fn update_render_layer(&mut self, ctx: &Context) -> Result<(), RustConstructorError> {
        self.render_layer.clear();
        for info in &self.render_list {
            let basic_front_resource = self.get_basic_front_resource(&info.0)?;
            if let Some(display_info) = basic_front_resource.display_display_info() {
                self.render_layer.push((
                    info.0.clone(),
                    if let Some(clip_rect) = basic_front_resource
                        .display_basic_front_resource_config()
                        .clip_rect
                    {
                        let [position, size] = position_size_processor(clip_rect, ctx);
                        let [resource_rect, clip_rect] = [
                            Rect::from_min_max(
                                basic_front_resource.display_position().into(),
                                [
                                    basic_front_resource.display_position()[0]
                                        + basic_front_resource.display_size()[0],
                                    basic_front_resource.display_position()[1]
                                        + basic_front_resource.display_size()[1],
                                ]
                                .into(),
                            ),
                            Rect::from_min_size(position.into(), size.into()),
                        ];
                        let min = resource_rect.min.max(clip_rect.min);
                        let max = resource_rect.max.min(clip_rect.max);

                        // 检查是否有交集
                        if min.x < max.x && min.y < max.y {
                            [min.into(), max.into()]
                        } else {
                            [[0_f32, 0_f32], [0_f32, 0_f32]]
                        }
                    } else {
                        [
                            basic_front_resource.display_position(),
                            [
                                basic_front_resource.display_position()[0]
                                    + basic_front_resource.display_size()[0],
                                basic_front_resource.display_position()[1]
                                    + basic_front_resource.display_size()[1],
                            ],
                        ]
                    },
                    display_info.ignore_render_layer,
                ));
            };
        }
        Ok(())
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
    /// * `hover_config` - The config of hover the rendering layer area
    ///
    /// # 参数
    ///
    /// * `ui` - 用于绘制的UI上下文
    /// * `render_config` - 渲染层区域的配置
    /// * `ignore_render_config` - 无视渲染层区域的配置
    /// * `hover_config` - 鼠标悬停时的配置
    pub fn display_render_layer(
        &self,
        ui: &mut Ui,
        render_config: &RenderConfig,
        ignore_render_config: &RenderConfig,
        hover_config: Option<&RenderConfig>,
    ) {
        for (i, (_, point, ignore_render_layer)) in self.render_layer.iter().enumerate() {
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
            if let Some(hover_config) = hover_config
                && let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
                && self.resource_get_focus(i, mouse_pos.into(), true, vec![])
            {
                match hover_config {
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
                                Color32::from_rgba_unmultiplied(
                                    color[0], color[1], color[2], color[3],
                                ),
                            ),
                        );
                    }
                };
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
        self.render_layer.iter().position(|x| &x.0 == id)
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
    /// * `need_contains_mouse` - Is it necessary to include the mouse position
    /// * `ignore_render_layer` - The range of indices to ignore in the render layer
    ///
    /// # Returns
    ///
    /// Return true if the resource is not blocked; otherwise, return false.
    ///
    /// # 参数
    ///
    /// * `index` - 渲染资源的索引值
    /// * `mouse_pos` - 鼠标的位置
    /// * `need_contains_mouse` - 是否需要包含鼠标位置
    /// * `ignore_render_layer` - 要忽略的渲染层索引范围
    ///
    /// # 返回值
    ///
    /// 如果资源未被阻挡，返回true，否则返回false。
    pub fn resource_get_focus(
        &self,
        index: usize,
        mouse_pos: [f32; 2],
        need_contains_mouse: bool,
        ignore_render_layer: Vec<[usize; 2]>,
    ) -> bool {
        let mut ignore_list = Vec::new();
        for range in ignore_render_layer {
            for i in 0..range[1] {
                ignore_list.push(range[0] + i);
            }
        }
        for i in index + 1..self.render_layer.len() {
            let point = self.render_layer[i].1;
            if mouse_pos[0] >= point[0][0]
                && mouse_pos[1] >= point[0][1]
                && mouse_pos[0] <= point[1][0]
                && mouse_pos[1] <= point[1][1]
                && !self.render_layer[i].2
                && !ignore_list.contains(&i)
            {
                return false;
            };
        }
        let target_point = self.render_layer[index].1;
        !need_contains_mouse
            || mouse_pos[0] <= target_point[1][0]
                && mouse_pos[0] >= target_point[0][0]
                && mouse_pos[1] <= target_point[1][1]
                && mouse_pos[1] >= target_point[0][1]
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
        self.active_list.push((
            id.clone(),
            if let [Some(citer_name), Some(citer_type)] = [
                get_tag("citer_name", &self.get_box_resource(id)?.display_tags()),
                get_tag("citer_type", &self.get_box_resource(id)?.display_tags()),
            ] {
                Some(RustConstructorId {
                    name: citer_name.1,
                    discern_type: citer_type.1,
                })
            } else {
                None
            },
        ));
        Ok(())
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
        let discern_type = &*type_processor(&resource);
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
                let split_time = downcast_resource_mut::<SplitTime>(&mut resource)?;
                split_time.time = [self.timer.now_time, self.timer.total_time];
            }
            "Background" => {
                let background = downcast_resource_mut::<Background>(&mut resource)?;
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
            }
            "Switch" => {
                let switch = downcast_resource_mut::<Switch>(&mut resource)?;
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
                if !switch.radio_group.is_empty() {
                    if !self.rust_constructor_resource.iter().any(|x| {
                        if let Ok(check_switch) = downcast_resource::<Switch>(&*x.content) {
                            switch.radio_group == check_switch.radio_group
                        } else {
                            false
                        }
                    }) {
                        switch.state = 1;
                    };
                    if switch.state_amount != 2 {
                        return Err(RustConstructorError {
                            error_id: "SwitchAppearanceConfigMismatch".to_string(),
                            description: format!(
                                "Radio group is only supported for switches with 2 states, found {}.",
                                switch.state_amount
                            ),
                        });
                    };
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
                                ["panel_layout_group".to_string(), name.to_string()],
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
                                ["panel_layout_group".to_string(), name.to_string()],
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
            }
            "ResourcePanel" => {
                let resource_panel = downcast_resource_mut::<ResourcePanel>(&mut resource)?;
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
            if let Some(index) = self.active_list.iter().position(|x| &x.0 == id) {
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
        let discern_type = &*type_processor(&resource);
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

    /// Obtain basic front resource from the list.
    ///
    /// 从列表中获取基本前端资源。
    ///
    /// If you want to use the basic front resource method, please call this method to retrieve the resource.
    ///
    /// 如果想要使用基本前端资源的方法，请调用此方法来取出资源。
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
    pub fn get_basic_front_resource(
        &self,
        id: &RustConstructorId,
    ) -> Result<&dyn BasicFrontResource, RustConstructorError> {
        match &*id.discern_type {
            "Image" => Ok(downcast_resource::<Image>(self.get_box_resource(id)?)?),
            "Text" => Ok(downcast_resource::<Text>(self.get_box_resource(id)?)?),
            "CustomRect" => Ok(downcast_resource::<CustomRect>(self.get_box_resource(id)?)?),
            _ => unreachable!(),
        }
    }

    /// Obtain mutable basic front resource from the list.
    ///
    /// 从列表中获取可变基本前端资源。
    ///
    /// If you want to use the basic front resource method and modify the basic front resource, please call
    /// this method to retrieve the resource.
    ///
    /// 如果想要使用基本前端资源的方法并修改基本前端资源，请调用此方法来取出资源。
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
    pub fn get_basic_front_resource_mut(
        &mut self,
        id: &RustConstructorId,
    ) -> Result<&mut dyn BasicFrontResource, RustConstructorError> {
        match &*id.discern_type {
            "Image" => Ok(downcast_resource_mut::<Image>(
                self.get_box_resource_mut(id)?,
            )?),
            "Text" => Ok(downcast_resource_mut::<Text>(
                self.get_box_resource_mut(id)?,
            )?),
            "CustomRect" => Ok(downcast_resource_mut::<CustomRect>(
                self.get_box_resource_mut(id)?,
            )?),
            _ => unreachable!(),
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
        downcast_resource(self.get_box_resource(id)?)
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
        downcast_resource_mut(self.get_box_resource_mut(id)?)
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
        let discern_type = &*type_processor(&resource);
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
                    self.update_render_layer(ctx)?;
                    // 更新资源活跃状态。
                    self.active_list.clear();
                    // 更新字体加载情况。
                    if !self.loading_fonts.is_empty() {
                        self.loaded_fonts = self.loading_fonts.clone();
                        self.loading_fonts.clear();
                    };
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
                    let background_resource =
                        self.get_basic_front_resource(&RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: background_resource_type.to_string(),
                        })?;
                    let display_info = background_resource.display_display_info();
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
                        && self.resource_get_focus(index, mouse_pos.into(), true, vec![])
                        && let Some(display_info) = background_resource.display_display_info()
                        && !display_info.hidden
                    {
                        if !switch.last_frame_hovered {
                            self.reset_split_time(&format!("{}StartHoverTime", &id.name))?;
                        } else if self.timer.total_time
                            - self.get_split_time(&format!("{}StartHoverTime", &id.name))?[1]
                            >= 2000
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
                                if !switch.radio_group.is_empty() {
                                    self.rust_constructor_resource
                                        .iter_mut()
                                        .filter(|x| &x.id.discern_type == "Switch")
                                        .for_each(|x| {
                                            if let Ok(check_switch) =
                                                downcast_resource_mut::<Switch>(&mut *x.content)
                                                && switch.radio_group == check_switch.radio_group
                                            {
                                                check_switch.state = 0;
                                            };
                                        });
                                };
                                if switch.radio_group.is_empty() || switch.state == 0 {
                                    if switch.state < switch.appearance.len() / animation_count - 1
                                    {
                                        switch.state += 1;
                                    } else {
                                        switch.state = 0;
                                    };
                                }
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
                        [switch.state * animation_count + appearance_count]
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
                            &switch.appearance[switch.state * animation_count + appearance_count]
                                .hint_text_config,
                        )
                        .ignore_render_layer(true);
                    hint_text.background_alpha = alpha;
                    hint_text.alpha = alpha;
                    hint_text.display_info.hidden = if let Some(display_info) = display_info
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
                            &switch.appearance[switch.state * animation_count + appearance_count]
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
                        self.try_request_jump_render_list(
                            RequestMethod::Id(RustConstructorId {
                                name: format!("{}HintText", &id.name),
                                discern_type: "Text".to_string(),
                            }),
                            RequestType::Top,
                        );
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
                        position_size_config.display_method =
                            (HorizontalAlign::Left, VerticalAlign::Top);
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
                    [position, size] = position_size_processor(position_size_config, ctx);
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
                    let mut resource_get_focus = [false, false];
                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
                        && !resource_panel.hidden
                        && let Some(index) = self.get_render_layer_resource(&RustConstructorId {
                            name: format!("{}Background", &id.name),
                            discern_type: match background.background_type {
                                BackgroundType::CustomRect(_) => "CustomRect",
                                BackgroundType::Image(_) => "Image",
                            }
                            .to_string(),
                        })
                    {
                        resource_get_focus = [
                            self.resource_get_focus(index, mouse_pos.into(), false, vec![]),
                            self.resource_get_focus(
                                index,
                                mouse_pos.into(),
                                true,
                                vec![[index + 1, resource_panel.resource_storage.len()]],
                            ),
                        ];
                        if resource_get_focus[1] {
                            if resource_panel.scroll_length_method[0].is_some()
                                && x_scroll_delta != 0_f32
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
                            if resource_panel.raise_on_focus
                                && ui.input(|i| i.pointer.primary_pressed())
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
                                            get_tag("panel_name", &rcr.content.display_tags())
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
                        }
                        if resource_get_focus[0] {
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
                                ctx.set_cursor_icon(match resource_panel.movable {
                                    [true, true] => CursorIcon::Move,
                                    [true, false] => CursorIcon::ResizeColumn,
                                    [false, true] => CursorIcon::ResizeRow,
                                    [false, false] => CursorIcon::NotAllowed,
                                });
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
                    [position, size] = position_size_processor(position_size_config, ctx);
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
                    type PointList = Vec<([f32; 2], [f32; 2], [bool; 2], Option<String>)>;
                    let mut resource_point_list: PointList = Vec::new();
                    let mut use_resource_list = Vec::new();
                    let mut replace_resource_list = Vec::new();
                    for rcr in &self.rust_constructor_resource {
                        if self
                            .basic_front_resource_list
                            .contains(&rcr.id.discern_type)
                            && let Some(panel_name) =
                                get_tag("panel_name", &rcr.content.display_tags())
                            && panel_name.1 == id.name
                        {
                            if let [Some(citer_name), Some(citer_type)] = [
                                get_tag("citer_name", &rcr.content.display_tags()),
                                get_tag("citer_type", &rcr.content.display_tags()),
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
                            let mut basic_front_resource: Box<dyn BasicFrontResource> =
                                match &*rcr.id.discern_type {
                                    "Image" => {
                                        Box::new(downcast_resource::<Image>(&*rcr.content)?.clone())
                                    }
                                    "Text" => {
                                        Box::new(downcast_resource::<Text>(&*rcr.content)?.clone())
                                    }
                                    "CustomRect" => Box::new(
                                        downcast_resource::<CustomRect>(&*rcr.content)?.clone(),
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
                                });
                            };
                            let enable_scrolling = [
                                get_tag("disable_x_scrolling", &rcr.content.display_tags())
                                    .is_none(),
                                get_tag("disable_y_scrolling", &rcr.content.display_tags())
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
                            let mut layout = resource_panel.overall_layout;
                            for custom_layout in &resource_panel.custom_layout {
                                match custom_layout {
                                    CustomPanelLayout::Id(layout_id, panel_layout) => {
                                        if rcr.id.cmp(layout_id) == Ordering::Equal {
                                            layout = *panel_layout;
                                            break;
                                        };
                                    }
                                    CustomPanelLayout::Type(layout_type, panel_layout) => {
                                        if *layout_type == rcr.id.discern_type {
                                            layout = *panel_layout;
                                        }
                                    }
                                };
                            }
                            let panel_layout_group = if let Some(panel_layout_group) =
                                get_tag("panel_layout_group", &basic_front_resource.display_tags())
                            {
                                Some(panel_layout_group.1)
                            } else {
                                None
                            };
                            match layout.panel_margin {
                                PanelMargin::Vertical(
                                    [top, bottom, left, right],
                                    move_to_bottom,
                                ) => {
                                    let mut modify_y = 0_f32;
                                    let [default_x_position, default_y_position] =
                                        match layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0]
                                                    + if x[1] != 0_f32 {
                                                        size[0] / x[1] * x[0]
                                                    } else {
                                                        0_f32
                                                    },
                                                position[1]
                                                    + if y[1] != 0_f32 {
                                                        size[1] / y[1] * y[0]
                                                    } else {
                                                        0_f32
                                                    },
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
                                        if let Some(ref point_panel_layout_group) = point.3
                                            && let Some(ref panel_layout_group) = panel_layout_group
                                            && panel_layout_group == point_panel_layout_group
                                        {
                                            continue;
                                        };
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
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_position(
                                                real_x_position
                                                    + left
                                                    + resource_panel.inner_margin[2],
                                                real_y_position
                                                    + top
                                                    + resource_panel.inner_margin[0],
                                            ),
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
                                        panel_layout_group,
                                    ));
                                }
                                PanelMargin::Horizontal(
                                    [top, bottom, left, right],
                                    move_to_right,
                                ) => {
                                    let mut modify_x = 0_f32;
                                    let [default_x_position, default_y_position] =
                                        match layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0]
                                                    + if x[1] != 0_f32 {
                                                        size[0] / x[1] * x[0]
                                                    } else {
                                                        0_f32
                                                    },
                                                position[1]
                                                    + if y[1] != 0_f32 {
                                                        size[1] / y[1] * y[0]
                                                    } else {
                                                        0_f32
                                                    },
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
                                        if let Some(ref point_panel_layout_group) = point.3
                                            && let Some(ref panel_layout_group) = panel_layout_group
                                            && panel_layout_group == point_panel_layout_group
                                        {
                                            continue;
                                        };
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
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_position(
                                                real_x_position
                                                    + left
                                                    + resource_panel.inner_margin[2],
                                                real_y_position
                                                    + top
                                                    + resource_panel.inner_margin[0],
                                            ),
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
                                        panel_layout_group,
                                    ));
                                }
                                PanelMargin::None([top, bottom, left, right], influence_layout) => {
                                    let [default_x_position, default_y_position] =
                                        match layout.panel_location {
                                            PanelLocation::Absolute([x, y]) => {
                                                [position[0] + x, position[1] + y]
                                            }
                                            PanelLocation::Relative([x, y]) => [
                                                position[0]
                                                    + if x[1] != 0_f32 {
                                                        size[0] / x[1] * x[0]
                                                    } else {
                                                        0_f32
                                                    },
                                                position[1]
                                                    + if y[1] != 0_f32 {
                                                        size[1] / y[1] * y[0]
                                                    } else {
                                                        0_f32
                                                    },
                                            ],
                                        };
                                    basic_front_resource.modify_position_size_config(
                                        basic_front_resource
                                            .display_position_size_config()
                                            .origin_position(
                                                default_x_position
                                                    + left
                                                    + resource_panel.inner_margin[2],
                                                default_y_position
                                                    + top
                                                    + resource_panel.inner_margin[0],
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
                                            panel_layout_group,
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
                        let basic_front_resource = self.get_basic_front_resource_mut(&id)?;
                        basic_front_resource.modify_position_size_config(new_position_size_config);
                        basic_front_resource.modify_clip_rect(Some(
                            position_size_config
                                .origin_size(
                                    position_size_config.origin_size[0]
                                        - resource_panel.inner_margin[2]
                                        - resource_panel.inner_margin[3],
                                    position_size_config.origin_size[1]
                                        - resource_panel.inner_margin[0]
                                        - resource_panel.inner_margin[1],
                                )
                                .origin_position(
                                    position_size_config.origin_position[0]
                                        + resource_panel.inner_margin[2],
                                    position_size_config.origin_position[1]
                                        + resource_panel.inner_margin[0],
                                ),
                        ));
                        basic_front_resource.modify_display_info({
                            let mut display_info =
                                basic_front_resource.display_display_info().unwrap();
                            display_info.ignore_render_layer =
                                if resource_panel.last_frame_mouse_status.is_some()
                                    && resource_get_focus[1]
                                {
                                    true
                                } else if default_storage[0] {
                                    default_storage[1]
                                } else {
                                    display_info.ignore_render_layer
                                };
                            display_info.hidden = if resource_panel.hidden {
                                true
                            } else if default_storage[0] {
                                default_storage[2]
                            } else {
                                display_info.hidden
                            };
                            display_info
                        });
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
                    let mut resource_length = [None, None];
                    for point in resource_point_list {
                        resource_length = [
                            if resource_length[0].is_none()
                                || resource_length[0].is_some()
                                    && point.1[0] > resource_length[0].unwrap()
                                    && point.2[0]
                            {
                                Some(point.1[0])
                            } else {
                                resource_length[0]
                            },
                            if resource_length[1].is_none()
                                || resource_length[1].is_some()
                                    && point.1[1] > resource_length[1].unwrap()
                                    && point.2[1]
                            {
                                Some(point.1[1])
                            } else {
                                resource_length[1]
                            },
                        ]
                    }
                    if let Some(horizontal_scroll_length_method) =
                        resource_panel.scroll_length_method[0]
                    {
                        let margin = match resource_panel.overall_layout.panel_margin {
                            PanelMargin::Horizontal([_, _, left, right], _) => left + right,
                            PanelMargin::Vertical([_, _, left, right], _) => left + right,
                            PanelMargin::None([_, _, left, right], _) => left + right,
                        };
                        resource_panel.scroll_length[0] = match horizontal_scroll_length_method {
                            ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                            ScrollLengthMethod::AutoFit(expand) => {
                                if let Some(max) = resource_length[0] {
                                    let width = max - position[0];
                                    if width - size[0]
                                        + expand
                                        + margin
                                        + resource_panel.inner_margin[3]
                                        + resource_panel.inner_margin[2]
                                        > 0_f32
                                    {
                                        width - size[0]
                                            + expand
                                            + margin
                                            + resource_panel.inner_margin[3]
                                            + resource_panel.inner_margin[2]
                                    } else {
                                        0_f32
                                    }
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
                        let margin = match resource_panel.overall_layout.panel_margin {
                            PanelMargin::Horizontal([top, bottom, _, _], _) => top + bottom,
                            PanelMargin::Vertical([top, bottom, _, _], _) => top + bottom,
                            PanelMargin::None([top, bottom, _, _], _) => top + bottom,
                        };
                        resource_panel.scroll_length[1] = match vertical_scroll_length_method {
                            ScrollLengthMethod::Fixed(fixed_length) => fixed_length,
                            ScrollLengthMethod::AutoFit(expand) => {
                                if let Some(max) = resource_length[1] {
                                    let height = max - position[1];
                                    if height - size[1]
                                        + expand
                                        + margin
                                        + resource_panel.inner_margin[1]
                                        + resource_panel.inner_margin[0]
                                        > 0_f32
                                    {
                                        height - size[1]
                                            + expand
                                            + margin
                                            + resource_panel.inner_margin[1]
                                            + resource_panel.inner_margin[0]
                                    } else {
                                        0_f32
                                    }
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
                                >= 1000
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
                                >= 1000
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

    /// Try to register all fonts in the egui context.
    ///
    /// 尝试向egui上下文中注册所有字体。
    ///
    /// This method loads and registers all fonts with the egui rendering system for
    /// text display.
    ///
    /// 此方法加载并注册所有字体到egui渲染系统中，用于文本显示。
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for font registration
    /// * `font_info` - Font information, including font names and paths
    ///
    /// # 参数
    ///
    /// * `ctx` - 用于字体注册的egui上下文
    /// * `font_info` - 字体信息，包含字体名称和路径
    pub fn try_register_all_fonts(&mut self, ctx: &Context, font_info: Vec<[&str; 2]>) {
        let mut font_definitions_amount = FontDefinitions::default();
        let mut loaded_fonts = Vec::new();
        for font_info in font_info {
            let mut font = FontDefinitions::default();
            if let Ok(font_read_data) = read(font_info[1]) {
                let font_data: Arc<Vec<u8>> = Arc::new(font_read_data);
                font.font_data.insert(
                    font_info[0].to_owned(),
                    Arc::new(FontData::from_owned(
                        Arc::try_unwrap(font_data).ok().unwrap(),
                    )),
                );
                // 将字体添加到字体列表中
                font.families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_info[0].to_owned());

                font.families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_info[0].to_owned());
                if let Some(font_data) = font.font_data.get(font_info[0]) {
                    font_definitions_amount
                        .font_data
                        .insert(font_info[0].to_string(), Arc::clone(font_data));
                    font_definitions_amount
                        .families
                        .entry(FontFamily::Name(font_info[0].into()))
                        .or_default()
                        .push(font_info[0].to_string());
                    // 将字体添加到字体列表中
                    font_definitions_amount
                        .families
                        .entry(FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_info[0].to_owned());

                    font_definitions_amount
                        .families
                        .entry(FontFamily::Monospace)
                        .or_default()
                        .insert(0, font_info[0].to_owned());
                    loaded_fonts.push(font_info);
                };
            }
        }
        self.loading_fonts = loaded_fonts
            .iter()
            .map(|x| [x[0].to_string(), x[1].to_string()])
            .collect();
        ctx.set_fonts(font_definitions_amount);
    }

    /// Registers all fonts with the egui context.
    ///
    /// 向egui上下文中注册所有字体。
    ///
    /// This method loads and registers all fonts with the egui rendering system for
    /// text display.
    ///
    /// 此方法加载并注册所有字体到egui渲染系统中，用于文本显示。
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for font registration
    /// * `font_info` - Font information, including font names and paths
    ///
    /// # Returns
    ///
    /// If the loading is successfully completed, return `Ok(())`; otherwise,
    /// return `Err(RustConstructorError)`.
    ///
    /// # 参数
    ///
    /// * `ctx` - 用于字体注册的egui上下文
    /// * `font_info` - 字体信息，包含字体名称和路径
    ///
    /// # 返回值
    ///
    /// 如果成功完成加载返回`Ok(())`，否则返回`Err(RustConstructorError)`。
    pub fn register_all_fonts(
        &mut self,
        ctx: &Context,
        font_info: Vec<[&str; 2]>,
    ) -> Result<(), RustConstructorError> {
        let mut font_definitions_amount = FontDefinitions::default();
        let mut loaded_fonts = Vec::new();
        for font_info in font_info {
            let mut font = FontDefinitions::default();
            if let Ok(font_read_data) = read(font_info[1]) {
                let font_data: Arc<Vec<u8>> = Arc::new(font_read_data);
                font.font_data.insert(
                    font_info[0].to_owned(),
                    Arc::new(FontData::from_owned(
                        Arc::try_unwrap(font_data).ok().unwrap(),
                    )),
                );
                // 将字体添加到字体列表中
                font.families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_info[0].to_owned());

                font.families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_info[0].to_owned());
                if let Some(font_data) = font.font_data.get(font_info[0]) {
                    font_definitions_amount
                        .font_data
                        .insert(font_info[0].to_string(), Arc::clone(font_data));
                    font_definitions_amount
                        .families
                        .entry(FontFamily::Name(font_info[0].into()))
                        .or_default()
                        .push(font_info[0].to_string());
                    // 将字体添加到字体列表中
                    font_definitions_amount
                        .families
                        .entry(FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_info[0].to_owned());

                    font_definitions_amount
                        .families
                        .entry(FontFamily::Monospace)
                        .or_default()
                        .insert(0, font_info[0].to_owned());
                    loaded_fonts.push(font_info);
                };
            } else {
                return Err(RustConstructorError {
                    error_id: "FontLoadFailed".to_string(),
                    description: format!("Failed to load a font from the path '{}'.", font_info[1]),
                });
            }
        }
        self.loading_fonts = loaded_fonts
            .iter()
            .map(|x| [x[0].to_string(), x[1].to_string()])
            .collect();
        ctx.set_fonts(font_definitions_amount);
        Ok(())
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
            if self.frame_times.len() > 120 {
                self.frame_times.drain(0..120);
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
            1000_f32
                / (self.frame_times.iter().sum::<u128>() as f32 / self.frame_times.len() as f32)
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
    pub fn get_split_time(&self, name: &str) -> Result<[u128; 2], RustConstructorError> {
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
        self.timer.total_time = elapsed.as_millis();
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

    /// Find out which switch in the radio switch group is activated.
    ///
    /// 查找单选开关组中哪个开关被激活了。
    ///
    /// # Arguments
    ///
    /// * `radio_group` - The name of the radio switch group
    ///
    /// # Returns
    ///
    /// Returns the name of the activated switch. If there is no activated switch or the
    /// radio switch group does not exist, return an empty string.
    ///
    /// # 参数
    ///
    /// * `radio_group` - 单选开关组的名称
    ///
    /// # 返回值
    ///
    /// 返回激活的开关的名称，如果没有激活的开关或单选开关组不存在则返回空字符串。
    pub fn check_radio_switch(&self, radio_group: &str) -> String {
        let mut activate_switch = String::new();
        for rcr in &self.rust_constructor_resource {
            if let Ok(switch) = downcast_resource::<Switch>(&*rcr.content)
                && switch.radio_group == radio_group
                && switch.state == 1
            {
                activate_switch = rcr.id.name.clone();
                break;
            };
        }
        activate_switch
    }
}
