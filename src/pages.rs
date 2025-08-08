//! pages.rs是Rust Constructor的页面部分。
use crate::function::{App, RCR, SeverityLevel, general_click_feedback, play_wav};
use chrono::{Local, Timelike};
use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Pos2, Shadow, Stroke};
use std::{process::exit, thread, vec::Vec};
use tray_icon::menu::MenuEvent;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 更新帧数
        self.update_frame_stats(ctx);
        // 更新渲染资源列表
        self.render_resource_list = Vec::new();
        // 夜间模式
        if Local::now().hour() >= 18 {
            ctx.set_visuals(egui::Visuals::dark());
            self.frame = Frame {
                inner_margin: egui::Margin::same(10),
                outer_margin: egui::Margin::same(0),
                shadow: Shadow {
                    offset: [1, 2],
                    color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 125),
                    blur: 20,
                    spread: 5,
                },
                fill: egui::Color32::from_rgb(39, 39, 39),
                stroke: Stroke {
                    width: 2.0,
                    color: egui::Color32::from_rgb(13, 14, 115),
                },
                corner_radius: CornerRadius::same(10),
            };
        } else {
            ctx.set_visuals(egui::Visuals::light());
            self.frame = Frame {
                inner_margin: egui::Margin::same(10),
                outer_margin: egui::Margin::same(0),
                shadow: Shadow {
                    offset: [1, 2],
                    color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 125),
                    blur: 20,
                    spread: 5,
                },
                fill: egui::Color32::from_rgb(255, 255, 255),
                stroke: Stroke {
                    width: 2.0,
                    color: egui::Color32::from_rgb(200, 200, 200),
                },
                corner_radius: CornerRadius::same(10),
            };
        };
        let game_text = self.game_text.game_text.clone();
        // 更新计时器
        self.update_timer();
        if self.tray_icon_created {
            // 接收托盘事件
            if let Ok(MenuEvent { id }) = MenuEvent::receiver().try_recv() {
                #[cfg(target_os = "macos")]
                match id.0.as_str() {
                    "3" => {
                        thread::spawn(|| {
                            play_wav("Resources/assets/sounds/Notification.wav").unwrap();
                        });
                    }
                    "4" => exit(0),
                    _ => {}
                }
                #[cfg(target_os = "windows")]
                match id.0.as_str() {
                    "1001" => {
                        thread::spawn(|| {
                            play_wav("Resources/assets/sounds/Notification.wav").unwrap();
                        });
                    }
                    "1002" => exit(0),
                    _ => {}
                }
            };
        };
        match &*self.page.clone() {
            "Launch" => {
                // 初始更新
                if !self.check_updated(&self.page.clone()).unwrap() {
                    self.launch_page_preload(ctx);
                    self.add_var("enable_debug_mode", false);
                    self.add_var("debug_fps_window", false);
                    self.add_var("debug_resource_list_window", false);
                    self.add_var("debug_render_list_window", false);
                    self.add_var("debug_problem_window", false);
                    self.add_var("cut_to", false);
                    self.add_split_time("cut_to_animation", false);
                    self.add_split_time("launch_time", false);
                };
                self.check_enter_updated(&self.page.clone()).unwrap();
                if let Ok(id) = self.get_resource_index("CustomRect", "Launch_Background") {
                    if let RCR::CustomRect(cr) = &mut self.rust_constructor_resource[id] {
                        cr.size = [ctx.available_rect().width(), ctx.available_rect().height()];
                    };
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.rect(ui, "Launch_Background", ctx);
                    self.image(ui, "RC_Logo", ctx);
                    ui.painter().line(
                        vec![
                            Pos2 {
                                x: ctx.available_rect().width() / 2_f32 - 100_f32,
                                y: ctx.available_rect().height() / 4_f32 * 3_f32,
                            },
                            Pos2 {
                                x: ctx.available_rect().width() / 2_f32 + 100_f32,
                                y: ctx.available_rect().height() / 4_f32 * 3_f32,
                            },
                        ],
                        Stroke {
                            width: 8_f32,
                            color: Color32::from_rgb(100, 100, 100),
                        },
                    );
                    ui.painter().line(
                        vec![
                            Pos2 {
                                x: ctx.available_rect().width() / 2_f32 - 98_f32,
                                y: ctx.available_rect().height() / 4_f32 * 3_f32,
                            },
                            Pos2 {
                                x: ctx.available_rect().width() / 2_f32 - 98_f32
                                    + 196_f32
                                        * ((self.timer.now_time
                                            - self.split_time("launch_time").unwrap()[0])
                                            / if self.timer.now_time
                                                - self.split_time("launch_time").unwrap()[0]
                                                > 6_f32
                                            {
                                                self.timer.now_time
                                                    - self.split_time("launch_time").unwrap()[0]
                                            } else {
                                                6_f32
                                            }),
                                y: ctx.available_rect().height() / 4_f32 * 3_f32,
                            },
                        ],
                        Stroke {
                            width: 5_f32,
                            color: Color32::from_rgb(200, 200, 200),
                        },
                    );
                    self.message_box_display(ctx, ui);
                    if self.timer.now_time - self.split_time("launch_time").unwrap()[0] >= 6_f32
                        && self
                            .cut_to(true, ctx, ui, "cut_to_animation", "Cut_To_Background", 10)
                            .unwrap()
                            == 255
                    {
                        self.switch_page("Demo_Desktop");
                        self.add_split_time("cut_to_animation", true);
                    };
                });
            }
            "Demo_Desktop" => {
                self.check_updated(&self.page.clone()).unwrap();
                self.check_enter_updated(&self.page.clone()).unwrap();
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.cut_to(false, ctx, ui, "cut_to_animation", "Cut_To_Background", 10)
                        .unwrap();
                    self.message_box_display(ctx, ui);
                });
            }
            _ => self.switch_page("Demo_Desktop"),
        };
        // 调试模式
        egui::TopBottomPanel::top("Debug mode")
            .frame(egui::Frame {
                fill: egui::Color32::TRANSPARENT,
                inner_margin: egui::Margin::symmetric(8, 4),
                ..Default::default()
            })
            .show_separator_line(false)
            .show(ctx, |ui| {
                // 启用方法
                if ctx.input(|i| i.key_pressed(egui::Key::F3)) && self.config.enable_debug_mode {
                    std::thread::spawn(|| {
                        play_wav("Resources/assets/sounds/Notification.wav").unwrap();
                    });
                    let enable_debug_mode = self.var_b("enable_debug_mode").unwrap();
                    self.modify_var("enable_debug_mode", !enable_debug_mode);
                };
                if self.var_b("enable_debug_mode").unwrap() {
                    egui::Window::new("performance")
                    .frame(self.frame)
                    .title_bar(false)
                    .open(&mut self.var_b("debug_fps_window").unwrap())
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(game_text["debug_frame_number_details"][self.config.language as usize].clone());
                        });
                        ui.separator();
                        ui.label(format!("{}: {:.3}{}", game_text["debug_fps"][self.config.language as usize].clone(), self.current_fps(), game_text["debug_fps2"][self.config.language as usize].clone()));
                        ui.separator();
                        ui.label(format!("{}:", game_text["debug_last_ten_frames"][self.config.language as usize].clone()));
                        self.frame_times
                            .iter()
                            .rev()
                            .take(10)
                            .enumerate()
                            .for_each(|(i, &t)| {
                                ui.label(format!("{} {}: {:.2}{}", game_text["debug_frame"][self.config.language as usize].clone(), i + 1, t * 1000.0, game_text["debug_game_millisecond"][self.config.language as usize].clone()));
                            });
                    });
                    egui::Window::new("render_list")
                    .frame(self.frame)
                    .title_bar(false)
                    .open(&mut self.var_b("debug_render_list_window").unwrap())
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(game_text["debug_render_list"][self.config.language as usize].clone());
                        });
                        ui.separator();
                        egui::ScrollArea::vertical()
                        .max_height(ctx.available_rect().height() - 100.0)
                        .max_width(ctx.available_rect().width() - 100.0)
                        .show(ui, |ui| {
                            self.render_resource_list
                                    .iter()
                                    .rev()
                                    .take(self.render_resource_list.len())
                                    .for_each(|t| {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.separator();
                                    });
                        })});
                    egui::Window::new("resource_list")
                    .frame(self.frame)
                    .title_bar(false)
                    .open(&mut self.var_b("debug_resource_list_window").unwrap())
                    .show(ctx, |ui| {
                        self.rust_constructor_resource.sort_by(|a, b| {
                            // 首先按类型排序
                            let type_a = match a {
                                RCR::Image(_) => 0,
                                RCR::Text(_) => 1,
                                RCR::CustomRect(_) => 2,
                                RCR::ScrollBackground(_) => 3,
                                RCR::Variable(_) => 4,
                                RCR::Font(_) => 5,
                                RCR::SplitTime(_) => 6,
                                RCR::Switch(_) => 7,
                                RCR::MessageBox(_) => 8,
                                RCR::ImageTexture(_) => 9,
                                RCR::PageData(_) => 10,
                            };

                            let type_b = match b {
                                RCR::Image(_) => 0,
                                RCR::Text(_) => 1,
                                RCR::CustomRect(_) => 2,
                                RCR::ScrollBackground(_) => 3,
                                RCR::Variable(_) => 4,
                                RCR::Font(_) => 5,
                                RCR::SplitTime(_) => 6,
                                RCR::Switch(_) => 7,
                                RCR::MessageBox(_) => 8,
                                RCR::ImageTexture(_) => 9,
                                RCR::PageData(_) => 10,
                            };

                            // 如果类型不同，按类型排序
                            match type_a.cmp(&type_b) {
                                std::cmp::Ordering::Equal => {
                                    // 如果类型相同，按名称排序
                                    let name_a = match a {
                                        RCR::Image(img) => &img.name,
                                        RCR::Text(txt) => &txt.name,
                                        RCR::CustomRect(rect) => &rect.name,
                                        RCR::ScrollBackground(bg) => &bg.name,
                                        RCR::Variable(var) => &var.name,
                                        RCR::Font(font) => &font.name,
                                        RCR::SplitTime(st) => &st.name,
                                        RCR::Switch(sw) => &sw.name,
                                        RCR::MessageBox(mb) => &mb.name,
                                        RCR::ImageTexture(it) => &it.name,
                                        RCR::PageData(pd) => &pd.name,
                                    };

                                    let name_b = match b {
                                        RCR::Image(img) => &img.name,
                                        RCR::Text(txt) => &txt.name,
                                        RCR::CustomRect(rect) => &rect.name,
                                        RCR::ScrollBackground(bg) => &bg.name,
                                        RCR::Variable(var) => &var.name,
                                        RCR::Font(font) => &font.name,
                                        RCR::SplitTime(st) => &st.name,
                                        RCR::Switch(sw) => &sw.name,
                                        RCR::MessageBox(mb) => &mb.name,
                                        RCR::ImageTexture(it) => &it.name,
                                        RCR::PageData(pd) => &pd.name,
                                    };

                                    name_a.cmp(name_b)
                                },
                                other => other,
                            }
                        });
                        ui.vertical_centered(|ui| {
                            ui.heading(game_text["debug_resource_list"][self.config.language as usize].clone());
                        });
                        ui.separator();
                        egui::ScrollArea::vertical()
                        .max_height(ctx.available_rect().height() - 100.0)
                        .max_width(ctx.available_rect().width() - 100.0)
                        .show(ui, |ui| {
                            for i in 0..self.rust_constructor_resource.len() {
                                match self.rust_constructor_resource[i].clone() {
                                    RCR::CustomRect(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {:#?}", game_text["debug_resource_position"][self.config.language as usize].clone(), t.position));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {:#?}", game_text["debug_resource_size"][self.config.language as usize].clone(), t.size));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {:#?}", game_text["debug_resource_origin_or_excursion_position"][self.config.language as usize].clone(), t.origin_position));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {}", game_text["debug_resource_rect_rounding"][self.config.language as usize].clone(), t.rounding));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {:#?}", game_text["debug_resource_color"][self.config.language as usize].clone(), t.color));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {}", game_text["debug_resource_rect_border_width"][self.config.language as usize].clone(), t.border_width));
                                        ui.colored_label(egui::Color32::CYAN, format!("{}: {:#?}", game_text["debug_resource_rect_border_color"][self.config.language as usize].clone(), t.border_color));
                                        ui.separator();
                                    }
                                    RCR::Font(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(Color32::MAGENTA, format!("{}: {}", game_text["debug_resource_font_path"][self.config.language as usize].clone(), t.path));
                                        ui.colored_label(Color32::MAGENTA, format!("{}: ", game_text["debug_resource_font_test"][self.config.language as usize].clone()));
                                        let mut test_text = String::new();
                                        for i in 0..self.config.amount_languages {
                                            test_text = format!("{}\n{}({}): {}", test_text, game_text["debug_amount_languages"][i as usize], game_text[&format!("debug_language_{}", i)][self.config.language as usize], game_text["debug_hello_world"][i as usize]);
                                        };
                                        ui.colored_label(
                                            Color32::MAGENTA,
                                            egui::RichText::new(test_text)
                                                .family(egui::FontFamily::Name(t.name.into())) // 使用资源中定义的字体名称
                                        );
                                        ui.separator();
                                    }
                                    RCR::Image(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::RED, format!("{}: {:#?}", game_text["debug_resource_size"][self.config.language as usize].clone(), t.image_size));
                                        ui.colored_label(egui::Color32::RED, format!("{}: {:#?}", game_text["debug_resource_position"][self.config.language as usize].clone(), t.image_position));
                                        ui.colored_label(egui::Color32::RED, format!("{}: {:#?}", game_text["debug_resource_origin_or_excursion_position"][self.config.language as usize].clone(), t.origin_position));
                                        ui.colored_label(egui::Color32::RED, format!("{}: {}", game_text["debug_resource_alpha"][self.config.language as usize].clone(), t.alpha));
                                        if t.use_overlay_color {
                                            ui.colored_label(egui::Color32::RED, format!("{}: {:#?}", game_text["debug_resource_image_overlay"][self.config.language as usize].clone(), t.overlay_color));
                                        };
                                        ui.colored_label(egui::Color32::RED, format!("{}: {}", game_text["debug_resource_origin_cite_texture"][self.config.language as usize].clone(), t.origin_cite_texture));
                                        ui.separator();
                                    }
                                    RCR::ImageTexture(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::GRAY, format!("{}: {}", game_text["debug_resource_image_path"][self.config.language as usize].clone(), t.cite_path));
                                        ui.separator();
                                    }
                                    RCR::MessageBox(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {:#?}", game_text["debug_resource_message_box_size"][self.config.language as usize].clone(), t.box_size));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_content_name"][self.config.language as usize].clone(), t.box_content_name));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_title_name"][self.config.language as usize].clone(), t.box_title_name));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_image_name"][self.config.language as usize].clone(), t.box_image_name));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_keep_existing"][self.config.language as usize].clone(), t.box_keep_existing));
                                        if !t.box_keep_existing {
                                            ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_existing_time"][self.config.language as usize].clone(), t.box_existing_time));
                                        };
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_exist"][self.config.language as usize].clone(), t.box_exist));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_speed"][self.config.language as usize].clone(), t.box_speed));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_restore_speed"][self.config.language as usize].clone(), t.box_restore_speed));
                                        ui.colored_label(egui::Color32::BROWN, format!("{}: {}", game_text["debug_resource_message_box_memory_offset"][self.config.language as usize].clone(), t.box_memory_offset));
                                        ui.separator();
                                    }
                                    RCR::PageData(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(Color32::BLACK, format!("{}: {}", game_text["debug_resource_page_data_forced_update"][self.config.language as usize].clone(), t.forced_update));
                                        ui.separator();
                                    }
                                    RCR::ScrollBackground(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::GREEN, format!("{}: {:#?}", game_text["debug_resource_all_image_name"][self.config.language as usize].clone(), t.image_name));
                                        ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_horizontal"][self.config.language as usize].clone(), t.horizontal_or_vertical));
                                        if t.horizontal_or_vertical {
                                            ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_left"][self.config.language as usize].clone(), t.left_and_top_or_right_and_bottom));
                                        } else {
                                            ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_top"][self.config.language as usize].clone(), t.left_and_top_or_right_and_bottom));
                                        };
                                        ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_speed"][self.config.language as usize].clone(), t.scroll_speed));
                                        ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_boundary"][self.config.language as usize].clone(), t.boundary));
                                        ui.colored_label(egui::Color32::GREEN, format!("{}: {}", game_text["debug_resource_scroll_resume_point"][self.config.language as usize].clone(), t.resume_point));
                                        ui.separator();
                                    }
                                    RCR::SplitTime(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::KHAKI, format!("{}: {}", game_text["debug_resource_split_time_single_page"][self.config.language as usize].clone(), t.time[0]));
                                        ui.colored_label(egui::Color32::KHAKI, format!("{}: {}", game_text["debug_resource_split_time_total"][self.config.language as usize].clone(), t.time[1]));
                                        ui.separator();
                                    }
                                    RCR::Switch(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_image_name"][self.config.language as usize].clone(), t.switch_image_name));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_enable_hover_animation"][self.config.language as usize].clone(), t.enable_hover_click_image[0]));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_enable_click_animation"][self.config.language as usize].clone(), t.enable_hover_click_image[1]));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_state"][self.config.language as usize].clone(), t.state));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {:#?}", game_text["debug_resource_switch_appearance"][self.config.language as usize].clone(), t.appearance));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {:#?}", game_text["debug_resource_switch_click_method"][self.config.language as usize].clone(), t.click_method));
                                        ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_click_state"][self.config.language as usize].clone(), t.last_time_clicked));
                                        if t.last_time_clicked {
                                            ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_clicked_method"][self.config.language as usize].clone(), t.last_time_clicked_index));
                                        };
                                        if !t.hint_text.is_empty() {
                                            ui.colored_label(egui::Color32::ORANGE, format!("{}: {:#?}", game_text["debug_resource_switch_hint_text"][self.config.language as usize].clone(), t.hint_text));
                                            ui.colored_label(egui::Color32::ORANGE, format!("{}: {}", game_text["debug_resource_switch_hint_text_name"][self.config.language as usize].clone(), t.hint_text_name));
                                        };
                                        ui.separator();
                                    }
                                    RCR::Variable(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::GOLD, format!("{}: {:#?}", game_text["debug_resource_variable_value"][self.config.language as usize].clone(), t.value));
                                        ui.separator();
                                    }
                                    RCR::Text(t) => {
                                        ui.label(format!("{}: {}", game_text["debug_resource_name"][self.config.language as usize].clone(), t.name));
                                        ui.label(format!("{}: {}", game_text["debug_resource_type"][self.config.language as usize].clone(), t.discern_type));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {}", game_text["debug_resource_text_content"][self.config.language as usize].clone(), t.text_content));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {}", game_text["debug_resource_size"][self.config.language as usize].clone(), t.font_size));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}", game_text["debug_resource_position"][self.config.language as usize].clone(), t.position));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}", game_text["debug_resource_origin_or_excursion_position"][self.config.language as usize].clone(), t.origin_position));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {}", game_text["debug_resource_text_wrap_width"][self.config.language as usize].clone(), t.wrap_width));
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}", game_text["debug_resource_color"][self.config.language as usize].clone(), t.rgba));
                                        if self.check_resource_exists("Font", &t.font) {
                                            ui.colored_label(egui::Color32::BLUE, egui::RichText::new(format!("{}: {}", game_text["debug_resource_text_font"][self.config.language as usize].clone(), t.font)).family(egui::FontFamily::Name(t.font.into())));
                                        } else {
                                            ui.colored_label(egui::Color32::BLUE, format!("{}: {} ({})", game_text["debug_resource_text_font"][self.config.language as usize].clone(), t.font, game_text["debug_resource_text_font_not_found"][self.config.language as usize].clone()));
                                        };
                                        if t.write_background {
                                            ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}", game_text["debug_resource_text_background_color"][self.config.language as usize].clone(), t.background_rgb));
                                            ui.colored_label(egui::Color32::BLUE, format!("{}: {}", game_text["debug_resource_text_background_rounding"][self.config.language as usize].clone(), t.rounding));
                                        };
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {}", game_text["debug_resource_text_selectable"][self.config.language as usize].clone(), t.selectable));
                                        let get_text_range = |text: &str, start: usize, end: usize| -> String {
                                            let chars: Vec<char> = text.chars().collect();
                                            let safety_start = if start >= chars.len() { chars.len() } else { start };
                                            let safety_end = if end >= chars.len() { chars.len() } else { end };
                                            chars[safety_start.min(safety_end)..safety_start.max(safety_end)].iter().collect()
                                        };
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}\n[{}]", game_text["debug_resource_text_selection"][self.config.language as usize].clone(), t.selection, if let Some(selection) = t.selection { get_text_range(&t.text_content, selection.0, selection.1) } else { game_text["debug_resource_none"][self.config.language as usize].clone() }));
                                        let mut hyperlink_list = Vec::new();
                                        for i in t.hyperlink_text {
                                            hyperlink_list.push(format!("{}[{}]", get_text_range(&t.text_content, i.0, i.1), i.2));
                                        };
                                        ui.colored_label(egui::Color32::BLUE, format!("{}: {:#?}", game_text["debug_resource_text_hyperlink"][self.config.language as usize].clone(), hyperlink_list));
                                        ui.separator();
                                    }
                                };
                            };
                        });
                    });
                    egui::Window::new("problem_report")
                    .frame(self.frame)
                    .title_bar(false)
                    .open(&mut self.var_b("debug_problem_window").unwrap())
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(game_text["debug_problem_report"][self.config.language as usize].clone());
                        });
                        ui.separator();
                        egui::ScrollArea::vertical()
                        .max_height(ctx.available_rect().height() - 100.0)
                        .max_width(ctx.available_rect().width() - 100.0)
                        .show(ui, |ui| {
                            self.problem_list
                                    .iter()
                                    .rev()
                                    .take(self.problem_list.len())
                                    .for_each(|t| {
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_problem"][self.config.language as usize].clone(), t.problem));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_severity_level"][self.config.language as usize].clone(), match t.severity_level {
                                            SeverityLevel::Error => game_text["debug_severity_level_error"][self.config.language as usize].clone(),
                                            SeverityLevel::SevereWarning => game_text["debug_severity_level_severe_warning"][self.config.language as usize].clone(),
                                            SeverityLevel::MildWarning => game_text["debug_severity_level_mild_warning"][self.config.language as usize].clone(),
                                        }));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_annotation"][self.config.language as usize].clone(), t.annotation));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {:#?}", game_text["debug_problem_type"][self.config.language as usize].clone(), t.problem_type));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_problem_current_page"][self.config.language as usize].clone(), t.report_state.current_page));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_problem_current_page_runtime"][self.config.language as usize].clone(), t.report_state.current_page_runtime));
                                        ui.colored_label(match t.severity_level {
                                            SeverityLevel::Error => egui::Color32::RED,
                                            SeverityLevel::SevereWarning => egui::Color32::ORANGE,
                                            SeverityLevel::MildWarning => egui::Color32::YELLOW,
                                        }, format!("{}: {}", game_text["debug_problem_current_total_runtime"][self.config.language as usize].clone(), t.report_state.current_total_runtime));
                                        ui.separator();
                                    });
                        });
                    });
                    ui.horizontal(|ui| {
                        // 使用WidgetText进行复杂布局
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(game_text["debug_mode"][self.config.language as usize].clone())
                                        .color(egui::Color32::YELLOW)
                                        .text_style(egui::TextStyle::Heading)
                                        .background_color(egui::Color32::from_black_alpha(220)),
                                )
                                .wrap(),
                            );
                            ui.separator();
                            ui.vertical(|ui| {
                                if ui.button(format!("{}: {}", game_text["debug_frame_number_details"][self.config.language as usize].clone(), if self.var_b("debug_fps_window").unwrap() { game_text["debug_on"][self.config.language as usize].clone() } else { game_text["debug_off"][self.config.language as usize].clone() })).clicked()
                                {
                                    general_click_feedback();
                                    let flip = !self.var_b("debug_fps_window").unwrap();
                                    self.modify_var("debug_fps_window", flip);
                                };
                                if ui.button(format!("{}: {}", game_text["debug_resource_list"][self.config.language as usize].clone(), if self.var_b("debug_resource_list_window").unwrap() { game_text["debug_on"][self.config.language as usize].clone() } else { game_text["debug_off"][self.config.language as usize].clone() })).clicked()
                                {
                                    general_click_feedback();
                                    let flip = !self.var_b("debug_resource_list_window").unwrap();
                                    self.modify_var("debug_resource_list_window", flip);
                                };
                                if ui.button(format!("{}: {}", game_text["debug_render_list"][self.config.language as usize].clone(), if self.var_b("debug_render_list_window").unwrap() { game_text["debug_on"][self.config.language as usize].clone() } else { game_text["debug_off"][self.config.language as usize].clone() })).clicked() {
                                    general_click_feedback();
                                    let flip = !self.var_b("debug_render_list_window").unwrap();
                                    self.modify_var("debug_render_list_window", flip);
                                };
                                if ui.button(format!("{}: {}", game_text["debug_problem_report"][self.config.language as usize].clone(), if self.var_b("debug_problem_window").unwrap() { game_text["debug_on"][self.config.language as usize].clone() } else { game_text["debug_off"][self.config.language as usize].clone() })).clicked()
                                {
                                    general_click_feedback();
                                    let flip = !self.var_b("debug_problem_window").unwrap();
                                    self.modify_var("debug_problem_window", flip);
                                };
                            });
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::WidgetText::from(game_text["debug_game_version"][self.config.language as usize].clone().to_string())
                                    .color(egui::Color32::GRAY)
                                    .background_color(egui::Color32::from_black_alpha(220)),
                                );
                                ui.label(
                                    egui::WidgetText::from(format!("{}: {}", game_text["debug_game_page"][self.config.language as usize].clone(), self.page))
                                        .color(egui::Color32::GRAY)
                                        .background_color(egui::Color32::from_black_alpha(220)),
                                );
                                ui.label(
                                    egui::WidgetText::from(format!("{}: {:.0}{}", game_text["debug_fps"][self.config.language as usize].clone(), self.current_fps(), game_text["debug_fps2"][self.config.language as usize].clone()))
                                        .color(egui::Color32::GRAY)
                                        .background_color(egui::Color32::from_black_alpha(220)),
                                );
                                ui.label(
                                    egui::WidgetText::from(format!("{}: {:.2}{}", game_text["debug_game_now_time"][self.config.language as usize].clone(), self.timer.now_time, game_text["debug_game_second"][self.config.language as usize].clone()))
                                        .color(egui::Color32::GRAY)
                                        .background_color(egui::Color32::from_black_alpha(220)),
                                );
                                ui.label(
                                    egui::WidgetText::from(format!("{}: {:.2}{}", game_text["debug_game_total_time"][self.config.language as usize].clone(), self.timer.total_time, game_text["debug_game_second"][self.config.language as usize].clone()))
                                        .color(egui::Color32::GRAY)
                                        .background_color(egui::Color32::from_black_alpha(220)),
                                );
                                for i in 0..self.rust_constructor_resource.len() {
                                    if let RCR::Font(f) = self.rust_constructor_resource[self.rust_constructor_resource.len() - i - 1].clone() {
                                        ui.label(
                                            egui::WidgetText::from(format!("{}: {}", game_text["debug_game_current_default_font"][self.config.language as usize].clone(), f.name))
                                                .color(egui::Color32::GRAY)
                                                .background_color(egui::Color32::from_black_alpha(220)),
                                        );
                                        break
                                    };
                                };
                            });
                        });
                    });
                };
            });
        if let Ok(id) = self.get_resource_index("PageData", &self.page.clone()) {
            if let RCR::PageData(pd) = self.rust_constructor_resource[id].clone() {
                if pd.forced_update {
                    // 请求重新绘制界面
                    ctx.request_repaint();
                };
            };
        };
    }
}
