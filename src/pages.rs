use crate::function;
use eframe::egui::{self, Color32, Pos2};
use egui::Stroke;
use function::App;
use function::Page;
use std::process::exit;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_timer();
        match self.page {
            Page::Launch => {
                if self.page_status[0].change_page_updated == false {
                    self.all_page_preload(ctx);
                    self.new_page_update(0);
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.rect(ui, "Background", ctx);
                    if self.timer.now_time >= 1.0 {
                        self.image(ctx, "Logo", ui);
                    };
                    if self.timer.now_time >= 4.0 {
                        ui.painter().line_segment(
                            [
                                Pos2::new(
                                    ctx.available_rect().width() / 8.0 * 3.0,
                                    ctx.available_rect().height() / 8.0 * 5.0,
                                ),
                                Pos2::new(
                                    ctx.available_rect().width() / 8.0 * 5.0,
                                    ctx.available_rect().height() / 8.0 * 5.0,
                                ),
                            ],
                            Stroke::new(8.0, Color32::from_rgb(110, 110, 110)),
                        );
                        ui.painter().line_segment(
                            [
                                Pos2::new(
                                    ctx.available_rect().width() / 8.0 * 3.0,
                                    ctx.available_rect().height() / 8.0 * 5.0,
                                ),
                                Pos2::new(
                                    ctx.available_rect().width() / 8.0
                                        * (3.0 + 2.0 * ((4.0 - (8.0 - self.timer.now_time)) / 4.0)),
                                    ctx.available_rect().height() / 8.0 * 5.0,
                                ),
                            ],
                            Stroke::new(6.0, Color32::from_rgb(200, 200, 200)),
                        );
                    };
                    if self.timer.now_time >= 8.0 {
                        self.page = Page::Home;
                    };
                });
            }
            Page::Home => {
                let scroll_background =
                    self.track_resource("scroll_background", "ScrollWallpaper");
                if self.page_status[1].change_page_updated == false {
                    self.new_page_update(1);
                    self.resource_scroll_background[scroll_background].resume_point =
                        ctx.available_rect().width();
                    for i in 0..self.resource_scroll_background[scroll_background]
                        .image_name
                        .len()
                    {
                        let id = self.track_resource(
                            "image",
                            &*self.resource_scroll_background[scroll_background].image_name[i]
                                .clone(),
                        );
                        self.resource_image[id].image_size =
                            [ctx.available_rect().width(), ctx.available_rect().height()];
                        self.resource_image[id].image_position[0] =
                            i as f32 * self.resource_image[id].image_size[0];
                        self.resource_scroll_background[scroll_background].boundary =
                            -ctx.available_rect().width();
                    }
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.last_window_size[0] != ctx.available_rect().width()
                        || self.last_window_size[1] != ctx.available_rect().height()
                    {
                        self.resource_scroll_background[scroll_background].resume_point =
                            ctx.available_rect().width();
                        for i in 0..self.resource_scroll_background[scroll_background]
                            .image_name
                            .len()
                        {
                            let id = self.track_resource(
                                "image",
                                &*self.resource_scroll_background[scroll_background].image_name[i]
                                    .clone(),
                            );
                            self.resource_image[id].image_size =
                                [ctx.available_rect().width(), ctx.available_rect().height()];
                            self.resource_image[id].image_position[0] =
                                i as f32 * self.resource_image[id].image_size[0];
                            self.resource_scroll_background[scroll_background].boundary =
                                -ctx.available_rect().width();
                        }
                    };
                    // 将加载的图片作为参数
                    self.image(ctx, "Background", ui);
                    self.scroll_background(ui, "ScrollWallpaper", ctx);
                    self.text(ui, "Title", ctx);
                });
                self.last_window_size =
                    [ctx.available_rect().width(), ctx.available_rect().height()];
            }
            Page::Update => {
                if self.page_status[2].change_page_updated == false {
                    self.new_page_update(2);
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("v0.2.0主要更新概述");
                    ui.separator();
                    ui.label(format!("1.新增方法:track_resource，可使用资源名称查找特定资源在列表中的索引值（选择列表错误/不存在该名称资源会返回0）\n\
                    2.新增配置文件Preferences.json与配套的修改方法\n\
                    {:?}\n\
                    3.App结构体新增项目:\n\
                    - Timer:用于获取程序全程/单页面全程的运行时间\n\
                    当前页面运行时间:{:?}秒\n\
                    RustConstructor总运行时间:{:?}秒\n\
                    - last_window_size:用于确定上一次渲染的页面大小，以确定RustConstructor窗口是否被拉扯\n\
                    上一次渲染时该区域的可用空间:{:?}\n\
                    - page_id:与Page枚举配合，可以给每个页面确定一个id值并进行操作\n\
                    本页面id:{:?}\n\
                    - page_status:项目数和页面数量相同，每个对应一个PageData结构体\n\
                    - PageData:一个页面配置结构体，包含两个参数，change_page_updated用于标记该页面是否被刷新过，forced_update用于标记该页面是否要进行强制渲染\n\
                    该页面是否被刷新:{:?} / 该页面是否强制要求渲染:{:?}\n\
                    4.新增矩形和滚动背景（对应方法和存储位置：add_text&text + resource_text / add_scroll_background&scroll_background + resource+scroll_background）\n\
                    提示：首页的背景和启动页面的黑色背景分别是使用滚动背景和矩形实现的\n\
                    5.图片现已支持进行翻转处理和输入图片路径处理\n\
                    以上内容皆为概述，具体更新内容请查阅源代码", self.config, self.timer.now_time as u32, self.timer.total_time as u32, self.last_window_size, self.page_id, self.page_status[2].change_page_updated, self.page_status[2].forced_update));
                });
                self.last_window_size =
                    [ctx.available_rect().width(), ctx.available_rect().height()];
            }
            Page::NewFeatureShow => {
                let scroll_background =
                    self.track_resource("scroll_background", "SampleScroll");
                if self.page_status[3].change_page_updated == false {
                    self.resource_scroll_background[scroll_background].resume_point =
                        ctx.available_rect().width();
                    for i in 0..self.resource_scroll_background[scroll_background]
                        .image_name
                        .len()
                    {
                        let id = self.track_resource(
                            "image",
                            &*self.resource_scroll_background[scroll_background].image_name[i]
                                .clone(),
                        );
                        self.resource_image[id].image_size =
                            [ctx.available_rect().width() / 2_f32, 50_f32];
                        self.resource_image[id].image_position[0] =
                            i as f32 * self.resource_image[id].image_size[0];
                        self.resource_image[id].image_position[1] =
                            ctx.available_rect().height() - 50_f32;
                        self.resource_scroll_background[scroll_background].boundary =
                            -ctx.available_rect().width() / 2_f32;
                    }
                    self.new_page_update(3);
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.last_window_size[0] != ctx.available_rect().width()
                        || self.last_window_size[1] != ctx.available_rect().height() {
                        self.resource_scroll_background[scroll_background].resume_point =
                            ctx.available_rect().width();
                        for i in 0..self.resource_scroll_background[scroll_background]
                            .image_name
                            .len()
                        {
                            let id = self.track_resource(
                                "image",
                                &*self.resource_scroll_background[scroll_background].image_name[i]
                                    .clone(),
                            );
                            self.resource_image[id].image_size =
                                [ctx.available_rect().width() / 2_f32, 50_f32];
                            self.resource_image[id].image_position[0] =
                                i as f32 * self.resource_image[id].image_size[0];
                            self.resource_image[id].image_position[1] =
                                ctx.available_rect().height() - 50_f32;
                            self.resource_scroll_background[scroll_background].boundary =
                                -ctx.available_rect().width() / 2_f32;
                        }
                    };
                    self.image(ctx, "Sample1", ui);
                    self.rect(ui, "Sample2", ctx);
                    ui.heading("v0.2.0新功能展示");
                    ui.separator();
                    self.scroll_background(ui, "SampleScroll", ctx);
                });
                self.last_window_size =
                    [ctx.available_rect().width(), ctx.available_rect().height()];
            }
        };
        if self.page_id != 0 {
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("退出").clicked() {
                        exit(0);
                    };
                    if ui.button("Rust Constructor v0.2.0").clicked() {
                        self.page_status[1].change_page_updated = false;
                        self.page = Page::Home;
                    };
                    if ui.button("更新概览").clicked() {
                        self.page_status[2].change_page_updated = false;
                        self.page = Page::Update;
                    };
                    if ui.button("效果展示").clicked() {
                        self.page_status[3].change_page_updated = false;
                        self.page = Page::NewFeatureShow;
                    };
                });
            });
        }
        if self.page_status[self.page_id as usize].forced_update == true {
            // 请求重新绘制界面
            ctx.request_repaint();
        };
    }
}
