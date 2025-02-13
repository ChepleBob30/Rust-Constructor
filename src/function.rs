use anyhow::Context;
use eframe::emath::Rect;
use eframe::epaint::textures::TextureOptions;
use eframe::epaint::Stroke;
use egui::{Color32, FontId, Pos2, Ui, Vec2};
use json::{object, JsonValue};
use rodio::{Decoder, OutputStream};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

pub fn write_to_json<P: AsRef<std::path::Path>>(path: P, config: &Config) {
    let json_value = config.to_json_value();
    let serialized = json_value.dump(); // 将 JSON 值序列化为字符串

    if let Err(e) = fs::write(path, serialized) {
        eprintln!("写入文件失败: {}", e);
    }
}

pub fn read_from_json<P: AsRef<std::path::Path>>(path: P) -> Option<Config> {
    match fs::read_to_string(path) {
        Ok(data) => match json::parse(&data) {
            Ok(parsed) => Config::from_json_value(&parsed),
            Err(e) => {
                eprintln!("解析JSON失败: {}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("阅读文件失败: {}", e);
            None
        }
    }
}

pub fn wav_player(wav_path: String) -> anyhow::Result<f64> {
    // 打开 WAV 文件
    let reader = hound::WavReader::open(&wav_path).context("无法打开 WAV 文件")?;

    // 获取 WAV 文件的规格
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f64;
    let total_samples = reader.len() as f64;

    // 计算时长（秒）
    let duration = total_samples / sample_rate;

    // 打开文件并创建解码器
    let file = BufReader::new(File::open(&wav_path).context("无法打开文件")?);
    let source = Decoder::new(file).context("无法解码音频文件")?;

    // 获取默认物理声音设备的输出流句柄
    let (_stream, stream_handle) = OutputStream::try_default().context("无法获取默认输出流")?;

    // 创建一个新的 Sink 来管理播放
    let sink = rodio::Sink::try_new(&stream_handle).context("无法创建 Sink")?;
    sink.append(source);

    sink.sleep_until_end(); // 等待音频播放结束
    Ok(duration)
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Text.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

#[derive(Debug)]
pub struct Config {
    pub version: String,
    pub wallpaper: String,
}

impl Config {
    fn to_json_value(&self) -> JsonValue {
        object! {
            version: self.version.clone(),
            wallpaper: self.wallpaper.clone()
        }
    }

    fn from_json_value(value: &JsonValue) -> Option<Config> {
        Some(Config {
            version: value["version"].as_str()?.to_string(),
            wallpaper: value["wallpaper"].as_str()?.to_string(),
        })
    }
}

pub struct PageData {
    pub forced_update: bool,
    pub change_page_updated: bool,
}

pub struct Timer {
    pub start_time: f32,
    pub total_time: f32,
    pub timer: Instant,
    pub now_time: f32,
}

pub struct CustomRect {
    name: String,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub rounding: f32,
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub center_display: [bool; 4],
    pub color: [u8; 4],
    pub border_width: f32,
    pub border_color: [u8; 4],
}

pub struct Image {
    name: String,
    pub image_texture: Option<egui::TextureHandle>,
    pub image_position: [f32; 2],
    pub image_size: [f32; 2],
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub center_display: [bool; 4],
    pub image_path: String,
    pub flip: [bool; 2],
}

pub struct Text {
    name: String,
    pub text_content: String,
    pub font_size: f32,
    pub rgba: [u8; 4],
    pub position: [f32; 2],
    pub center_display: [bool; 4],
    pub wrap_width: f32,
    pub write_background: bool,
    pub background_rgb: [u8; 3],
    pub rounding: f32,
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
}

pub struct ScrollBackground {
    name: String,
    pub image_name: Vec<String>,
    pub horizontal_or_vertical: bool,
    pub left_and_top_or_right_and_bottom: bool,
    pub scroll_speed: u32,
    pub boundary: f32,
    pub resume_point: f32,
}

pub struct App {
    pub config: Config,
    pub page_id: i32,
    pub page: Page,
    pub page_status: [PageData; 4],
    pub resource_image: Vec<Image>,
    pub resource_text: Vec<Text>,
    pub resource_rect: Vec<CustomRect>,
    pub resource_scroll_background: Vec<ScrollBackground>,
    pub timer: Timer,
    pub last_window_size: [f32; 2],
}

pub enum Page {
    Launch,
    Home,
    Update,
    NewFeatureShow,
}
#[allow(dead_code)]
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut config = Config {
            version: "0.2.0".to_string(),
            wallpaper: "assets/images/default_wallpaper.jpg".to_string(),
        };

        // // 写入 JSON 文件
        // write_to_json("config/Preferences.json", &config);
        // 读取 JSON 文件
        if let Some(read_config) = read_from_json("config/Preferences.json") {
            config = read_config
        } else {
            eprintln!("阅读/解析 JSON 文件失败");
        };
        Self {
            config,
            page_id: 0,
            page: Page::Launch,
            page_status: [
                PageData {
                    forced_update: true,
                    change_page_updated: false,
                },
                PageData {
                    forced_update: true,
                    change_page_updated: false,
                },
                PageData {
                    forced_update: true,
                    change_page_updated: false,
                },
                PageData {
                    forced_update: true,
                    change_page_updated: false,
                },
            ],
            resource_image: vec![],
            resource_text: vec![],
            resource_rect: vec![],
            resource_scroll_background: vec![],
            timer: Timer {
                start_time: 0.0,
                total_time: 0.0,
                timer: Instant::now(),
                now_time: 0.0,
            },
            last_window_size: [0.0, 0.0],
        }
    }

    pub fn all_page_preload(&mut self, ctx: &egui::Context) {
        for i in 0..self.page_status.len() {
            if i == 0 {
                self.add_image(
                    false,
                    "Logo",
                    [0_f32, 0_f32],
                    [250_f32, 250_f32],
                    [1, 2],
                    [1, 4],
                    [false, false, true, true],
                    "assets/images/icon.png",
                    [false, false],
                );
                self.add_rect(
                    false,
                    "Background",
                    [0_f32, 0_f32],
                    [ctx.available_rect().width(), ctx.available_rect().height()],
                    0.0,
                    [1, 2],
                    [1, 2],
                    [false, false, true, true],
                    [0, 0, 0, 255],
                    0.0,
                    [255, 255, 255, 255],
                );
                let _ = std::thread::spawn(|| {
                    let _ = wav_player("assets/sounds/Launch.wav".to_string());
                });
            } else if i == 1 {
                self.add_image(
                    false,
                    "Background",
                    [0_f32, 0_f32],
                    [ctx.available_rect().width(), ctx.available_rect().height()],
                    [1, 0],
                    [1, 0],
                    [true, true, false, false],
                    &*self.config.wallpaper.clone(),
                    [false, false],
                );
                self.add_text(
                    false,
                    "Title",
                    [0_f32, 0_f32],
                    70.0,
                    [255, 255, 255, 255],
                    "Rust Constructor v0.2.0",
                    [false, true, true, false],
                    1000_f32,
                    true,
                    [0, 0, 0],
                    0.0,
                    [1, 2],
                    [1, 4],
                );
                self.add_image(
                    false,
                    "Wallpaper1",
                    [0_f32, 0_f32],
                    [0_f32, 0_f32],
                    [1, 0],
                    [1, 0],
                    [true, false, false, false],
                    "assets/images/default_wallpaper.jpg",
                    [false, false],
                );
                self.add_image(
                    false,
                    "Wallpaper2",
                    [0_f32, 0_f32],
                    [0_f32, 0_f32],
                    [1, 0],
                    [1, 0],
                    [true, false, false, false],
                    "assets/images/default_wallpaper.jpg",
                    [false, false],
                );
                self.add_scroll_background(
                    false,
                    "ScrollWallpaper",
                    vec!["Wallpaper1".to_string(), "Wallpaper2".to_string()],
                    true,
                    true,
                    3,
                    [ctx.available_rect().width(), ctx.available_rect().height()],
                    [0_f32, 0_f32],
                    -ctx.available_rect().width(),
                );
            } else if i == 3 {
                self.add_image(false, "Sample1", [0_f32, 0_f32], [200_f32, 200_f32], [1, 2], [1, 2], [false, false, true, false], "assets/images/SampleImage.png", [true, true]);
                self.add_image(false, "SampleScroll1", [0_f32, 0_f32], [200_f32, 200_f32], [0, 0], [0, 0], [true, true, false, false], "assets/images/warn.png", [false, false]);
                self.add_image(false, "SampleScroll2", [0_f32, 0_f32], [200_f32, 200_f32], [0, 0], [0, 0], [true, true, false, false], "assets/images/warn.png", [false, false]);
                self.add_image(false, "SampleScroll3", [0_f32, 0_f32], [200_f32, 200_f32], [0, 0], [0, 0], [true, true, false, false], "assets/images/warn.png", [false, false]);
                self.add_image(false, "SampleScroll4", [0_f32, 0_f32], [200_f32, 200_f32], [0, 0], [0, 0], [true, true, false, false], "assets/images/warn.png", [false, false]);
                self.add_scroll_background(false, "SampleScroll", vec!["SampleScroll1".to_string(), "SampleScroll2".to_string(), "SampleScroll3".to_string(), "SampleScroll4".to_string()], true, true, 1, [ctx.available_rect().width() / 2_f32, 50_f32], [0_f32, ctx.available_rect().height() - 50_f32], -ctx.available_rect().width() / 2_f32);
                self.add_rect(false, "Sample2", [0_f32, 0_f32], [200_f32, 200_f32], 10_f32, [1, 2], [1, 2], [false, true, true, false], [255, 255, 255, 255], 5_f32, [0, 0, 0, 255]);
            };
        }
    }

    pub fn new_page_update(&mut self, page_id: i32) {
        self.renew_timer();
        self.page_id = page_id;
        self.page_status[page_id as usize].change_page_updated = true;
    }

    pub fn renew_timer(&mut self) {
        self.timer.start_time = self.timer.total_time;
    }

    pub fn update_timer(&mut self) {
        let elapsed = self.timer.timer.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();
        self.timer.total_time = seconds as f32 + milliseconds as f32 / 1000.0;
        self.timer.now_time = self.timer.total_time - self.timer.start_time
    }

    pub fn success_create_message_sender(
        &mut self,
        subject: &str,
        subject_name: String,
        id: usize,
    ) {
        println!("已成功创建{}{},id为{}", subject, subject_name, id);
    }

    pub fn add_rect(
        &mut self,
        send_message: bool,
        name: &str,
        position: [f32; 2],
        size: [f32; 2],
        rounding: f32,
        x_grid: [u32; 2],
        y_grid: [u32; 2],
        center_display: [bool; 4],
        color: [u8; 4],
        border_width: f32,
        border_color: [u8; 4],
    ) {
        self.resource_rect.push(CustomRect {
            name: name.to_string(),
            position,
            size,
            rounding,
            x_grid,
            y_grid,
            center_display,
            color,
            border_width,
            border_color,
        });
        if send_message {
            self.success_create_message_sender(
                "矩形",
                self.resource_rect[self.resource_rect.len() - 1]
                    .name
                    .clone(),
                self.resource_rect.len() - 1,
            );
        }
    }

    pub fn rect(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        let id = self.track_resource("rect", name);
        self.resource_rect[id].position[0] = match self.resource_rect[id].x_grid[1] {
            0 => self.resource_rect[id].position[0],
            _ => {
                (ctx.available_rect().width() as f64 / self.resource_rect[id].x_grid[1] as f64
                    * self.resource_rect[id].x_grid[0] as f64) as f32
            }
        };
        self.resource_rect[id].position[1] = match self.resource_rect[id].y_grid[1] {
            0 => self.resource_rect[id].position[1],
            _ => {
                (ctx.available_rect().height() as f64 / self.resource_rect[id].y_grid[1] as f64
                    * self.resource_rect[id].y_grid[0] as f64) as f32
            }
        };
        let pos_x;
        let pos_y;
        if self.resource_rect[id].center_display[2] {
            pos_x = self.resource_rect[id].position[0] - self.resource_rect[id].size[0] / 2.0;
        } else if self.resource_rect[id].center_display[0] {
            pos_x = self.resource_rect[id].position[0];
        } else {
            pos_x = self.resource_rect[id].position[0] - self.resource_rect[id].size[0];
        };
        if self.resource_rect[id].center_display[3] {
            pos_y = self.resource_rect[id].position[1] - self.resource_rect[id].size[1] / 2.0;
        } else if self.resource_rect[id].center_display[1] {
            pos_y = self.resource_rect[id].position[1];
        } else {
            pos_y = self.resource_rect[id].position[1] - self.resource_rect[id].size[1];
        };
        ui.painter().rect(
            Rect::from_min_max(
                Pos2::new(pos_x, pos_y),
                Pos2::new(
                    pos_x + self.resource_rect[id].size[0],
                    pos_y + self.resource_rect[id].size[1],
                ),
            ),
            self.resource_rect[id].rounding,
            Color32::from_rgba_premultiplied(
                self.resource_rect[id].color[0],
                self.resource_rect[id].color[1],
                self.resource_rect[id].color[2],
                self.resource_rect[id].color[3],
            ),
            Stroke {
                width: self.resource_rect[id].border_width,
                color: Color32::from_rgba_premultiplied(
                    self.resource_rect[id].border_color[0],
                    self.resource_rect[id].border_color[1],
                    self.resource_rect[id].border_color[2],
                    self.resource_rect[id].border_color[3],
                ),
            },
        );
    }

    #[allow(dead_code)]
    pub fn add_text(
        &mut self,
        send_message: bool,
        name: &str,
        position: [f32; 2],
        font_size: f32,
        rgba: [u8; 4],
        text_content: &str,
        center_display: [bool; 4],
        wrap_width: f32,
        write_background: bool,
        background_rgb: [u8; 3],
        rounding: f32,
        x_grid: [u32; 2],
        y_grid: [u32; 2],
    ) {
        self.resource_text.push(Text {
            name: name.to_string(),
            text_content: text_content.to_string(),
            font_size,
            rgba,
            position,
            center_display,
            wrap_width,
            write_background,
            background_rgb,
            rounding,
            x_grid,
            y_grid,
        });
        if send_message {
            self.success_create_message_sender(
                "文本",
                self.resource_text[self.resource_text.len() - 1]
                    .name
                    .clone(),
                self.resource_text.len() - 1,
            );
        };
    }

    pub fn text(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        let id = self.track_resource("text", name);
        // 计算文本大小
        let galley = ui.fonts(|f| {
            f.layout(
                self.resource_text[id].text_content.to_string(),
                FontId::proportional(self.resource_text[id].font_size),
                Color32::from_rgba_premultiplied(
                    self.resource_text[id].rgba[0],
                    self.resource_text[id].rgba[1],
                    self.resource_text[id].rgba[2],
                    self.resource_text[id].rgba[3],
                ),
                self.resource_text[id].wrap_width,
            )
        });
        let text_size = galley.size();
        self.resource_text[id].position[0] = match self.resource_text[id].x_grid[1] {
            0 => self.resource_text[id].position[0],
            _ => {
                (ctx.available_rect().width() as f64 / self.resource_text[id].x_grid[1] as f64
                    * self.resource_text[id].x_grid[0] as f64) as f32
            }
        };
        self.resource_text[id].position[1] = match self.resource_text[id].y_grid[1] {
            0 => self.resource_text[id].position[1],
            _ => {
                (ctx.available_rect().height() as f64 / self.resource_text[id].y_grid[1] as f64
                    * self.resource_text[id].y_grid[0] as f64) as f32
            }
        };
        let pos_x;
        let pos_y;
        if self.resource_text[id].center_display[2] {
            pos_x = self.resource_text[id].position[0] - text_size.x / 2.0;
        } else if self.resource_text[id].center_display[0] {
            pos_x = self.resource_text[id].position[0];
        } else {
            pos_x = self.resource_text[id].position[0] - text_size.x;
        };
        if self.resource_text[id].center_display[3] {
            pos_y = self.resource_text[id].position[1] - text_size.y / 2.0;
        } else if self.resource_text[id].center_display[1] {
            pos_y = self.resource_text[id].position[1];
        } else {
            pos_y = self.resource_text[id].position[1] - text_size.y;
        };
        // 使用绝对定位放置文本
        let position = Pos2::new(pos_x, pos_y);
        if self.resource_text[id].write_background {
            let rect = Rect::from_min_size(position, text_size);
            // 绘制背景颜色
            ui.painter().rect_filled(
                rect,
                self.resource_text[id].rounding,
                Color32::from_rgb(
                    self.resource_text[id].background_rgb[0],
                    self.resource_text[id].background_rgb[1],
                    self.resource_text[id].background_rgb[2],
                ),
            ); // 背景色
        };
        // 绘制文本
        ui.painter().galley(position, galley, Color32::BLACK);
    }

    fn read_image_to_vec(&mut self, path: &str) -> Vec<u8> {
        let mut file = File::open(path).expect("打开图片文件失败");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("读取图片文件失败");
        buffer
    }

    pub fn track_resource(&mut self, resource_list_name: &str, resource_name: &str) -> usize {
        let mut id = 0;
        match resource_list_name.to_lowercase().as_str() {
            "image" => {
                for i in 0..self.resource_image.len() {
                    if self.resource_image[i].name == resource_name {
                        id = i;
                        break;
                    }
                }
            }
            "text" => {
                for i in 0..self.resource_text.len() {
                    if self.resource_text[i].name == resource_name {
                        id = i;
                        break;
                    }
                }
            }
            "rect" => {
                for i in 0..self.resource_rect.len() {
                    if self.resource_rect[i].name == resource_name {
                        id = i;
                        break;
                    }
                }
            }
            "scroll_background" => {
                for i in 0..self.resource_scroll_background.len() {
                    if self.resource_scroll_background[i].name == resource_name {
                        id = i;
                        break;
                    }
                }
            }
            _ => eprintln!("无效输入!"),
        };
        id
    }

    pub fn add_scroll_background(
        &mut self,
        send_message: bool,
        name: &str,
        image_name: Vec<String>,
        horizontal_or_vertical: bool,
        left_and_top_or_right_and_bottom: bool,
        scroll_speed: u32,
        size: [f32; 2],
        position: [f32; 2],
        boundary: f32,
    ) {
        let mut image_id = vec![];
        for i in 0..image_name.len() {
            image_id.push(self.track_resource("image", &image_name[i]));
            continue;
        }
        for i in 0..image_id.len() {
            self.resource_image[image_id[i]].x_grid = [0, 0];
            self.resource_image[image_id[i]].y_grid = [0, 0];
            self.resource_image[image_id[i]].center_display = [true, true, false, false];
            self.resource_image[image_id[i]].image_size = [size[0], size[1]];
            let mut temp_position;
            if horizontal_or_vertical == true {
                temp_position = position[0];
            } else {
                temp_position = position[1];
            };
            if horizontal_or_vertical == true {
                for _j in 0..i {
                    if left_and_top_or_right_and_bottom == true {
                        temp_position += size[0];
                    } else {
                        temp_position -= size[0];
                    };
                }
                self.resource_image[image_id[i]].image_position = [temp_position, position[1]];
            } else {
                for _j in 0..i {
                    if left_and_top_or_right_and_bottom == true {
                        temp_position += size[1];
                    } else {
                        temp_position -= size[1];
                    };
                }
                self.resource_image[image_id[i]].image_position = [position[0], temp_position];
            };
        }
        let resume_point;
        if horizontal_or_vertical == true {
            resume_point = self.resource_image[image_id[image_id.len() - 1]].image_position[0];
        } else {
            resume_point = self.resource_image[image_id[image_id.len() - 1]].image_position[1];
        };
        self.resource_scroll_background.push(ScrollBackground {
            name: name.to_string(),
            image_name,
            horizontal_or_vertical,
            left_and_top_or_right_and_bottom,
            scroll_speed,
            boundary,
            resume_point,
        });
        if send_message {
            self.success_create_message_sender(
                "动态背景",
                self.resource_scroll_background[self.resource_scroll_background.len() - 1]
                    .name
                    .clone(),
                self.resource_scroll_background.len() - 1,
            );
        };
    }

    pub fn scroll_background(&mut self, ui: &mut Ui, name: &str, ctx: &egui::Context) {
        let id = self.track_resource("scroll_background", name);
        let mut id2;
        for i in 0..self.resource_scroll_background[id].image_name.len() {
            self.image(
                ctx,
                &self.resource_scroll_background[id].image_name[i].clone(),
                ui,
            );
        };
        for i in 0..self.resource_scroll_background[id].image_name.len() {
            id2 = self.track_resource(
                "image",
                &self.resource_scroll_background[id].image_name[i].clone(),
            );
            if self.resource_scroll_background[id].horizontal_or_vertical == true {
                if self.resource_scroll_background[id].left_and_top_or_right_and_bottom == true {
                    for _j in 0..self.resource_scroll_background[id].scroll_speed {
                        self.resource_image[id2].image_position[0] -= 1_f32;
                        self.scroll_background_check_boundary(id, id2);
                    }
                } else {
                    for _j in 0..self.resource_scroll_background[id].scroll_speed {
                        self.resource_image[id2].image_position[0] += 1_f32;
                        self.scroll_background_check_boundary(id, id2);
                    }
                };
            } else {
                if self.resource_scroll_background[id].left_and_top_or_right_and_bottom == true {
                    for _j in 0..self.resource_scroll_background[id].scroll_speed {
                        self.resource_image[id2].image_position[1] -= 1_f32;
                        self.scroll_background_check_boundary(id, id2);
                    }
                } else {
                    for _j in 0..self.resource_scroll_background[id].scroll_speed {
                        self.resource_image[id2].image_position[1] += 1_f32;
                        self.scroll_background_check_boundary(id, id2);
                    }
                };
            };
        }
    }

    fn scroll_background_check_boundary(&mut self, id: usize, id2: usize) {
        if self.resource_scroll_background[id].horizontal_or_vertical == true {
            if self.resource_scroll_background[id].left_and_top_or_right_and_bottom == true {
                if self.resource_image[id2].image_position[0]
                    <= self.resource_scroll_background[id].boundary
                {
                    self.resource_image[id2].image_position[0] =
                        self.resource_scroll_background[id].resume_point;
                };
            } else {
                if self.resource_image[id2].image_position[0]
                    >= self.resource_scroll_background[id].boundary
                {
                    self.resource_image[id2].image_position[0] =
                        self.resource_scroll_background[id].resume_point;
                };
            }
        } else {
            if self.resource_scroll_background[id].left_and_top_or_right_and_bottom == true {
                if self.resource_image[id2].image_position[1]
                    <= self.resource_scroll_background[id].boundary
                {
                    self.resource_image[id2].image_position[1] =
                        self.resource_scroll_background[id].resume_point;
                };
            } else {
                if self.resource_image[id2].image_position[1]
                    >= self.resource_scroll_background[id].boundary
                {
                    self.resource_image[id2].image_position[1] =
                        self.resource_scroll_background[id].resume_point;
                };
            }
        };
    }

    pub fn add_image(
        &mut self,
        send_message: bool,
        name: &str,
        position: [f32; 2],
        size: [f32; 2],
        x_grid_locate_position: [u32; 2],
        y_grid_locate_position: [u32; 2],
        center_display: [bool; 4],
        image_path: &str,
        flip: [bool; 2],
    ) {
        self.resource_image.push(Image {
            name: name.to_string(),
            image_texture: None,
            image_position: position,
            image_size: size,
            x_grid: x_grid_locate_position,
            y_grid: y_grid_locate_position,
            center_display,
            image_path: image_path.to_string(),
            flip,
        });
        if send_message {
            self.success_create_message_sender(
                "图像",
                self.resource_image[self.resource_image.len() - 1]
                    .name
                    .clone(),
                self.resource_image.len() - 1,
            );
        };
    }

    pub fn image(&mut self, ctx: &egui::Context, name: &str, ui: &mut Ui) {
        let id = self.track_resource("image", name);
        if self.resource_image[id].image_texture.is_none() {
            let img_bytes = self.read_image_to_vec(&*self.resource_image[id].image_path.clone());
            let img = image::load_from_memory(&img_bytes).unwrap();
            let rgba_data;
            match self.resource_image[id].flip {
                [true, true] => {
                    rgba_data = img.fliph().flipv().into_rgba8();
                }
                [true, false] => {
                    rgba_data = img.fliph().into_rgba8();
                }
                [false, true] => {
                    rgba_data = img.flipv().into_rgba8();
                }
                _ => {
                    rgba_data = img.into_rgba8();
                }
            }
            let (w, h) = (rgba_data.width(), rgba_data.height());
            let raw_data: Vec<u8> = rgba_data.into_raw();

            let color_image =
                egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
            self.resource_image[id].image_texture =
                Some(ctx.load_texture("", color_image, TextureOptions::LINEAR));
        };

        self.resource_image[id].image_position[0] = match self.resource_image[id].x_grid[1] {
            0 => self.resource_image[id].image_position[0],
            _ => {
                (ctx.available_rect().width() as f64 / self.resource_image[id].x_grid[1] as f64
                    * self.resource_image[id].x_grid[0] as f64) as f32
            }
        };
        self.resource_image[id].image_position[1] = match self.resource_image[id].y_grid[1] {
            0 => self.resource_image[id].image_position[1],
            _ => {
                (ctx.available_rect().height() as f64 / self.resource_image[id].y_grid[1] as f64
                    * self.resource_image[id].y_grid[0] as f64) as f32
            }
        };
        let pos_x;
        let pos_y;
        if self.resource_image[id].center_display[2] {
            pos_x = self.resource_image[id].image_position[0]
                - self.resource_image[id].image_size[0] / 2.0;
        } else if self.resource_image[id].center_display[0] {
            pos_x = self.resource_image[id].image_position[0];
        } else {
            pos_x =
                self.resource_image[id].image_position[0] - self.resource_image[id].image_size[0];
        };
        if self.resource_image[id].center_display[3] {
            pos_y = self.resource_image[id].image_position[1]
                - self.resource_image[id].image_size[1] / 2.0;
        } else if self.resource_image[id].center_display[1] {
            pos_y = self.resource_image[id].image_position[1];
        } else {
            pos_y =
                self.resource_image[id].image_position[1] - self.resource_image[id].image_size[1];
        };
        if let Some(texture) = &self.resource_image[id].image_texture {
            let rect = Rect::from_min_size(
                Pos2::new(pos_x, pos_y),
                Vec2::new(
                    self.resource_image[id].image_size[0],
                    self.resource_image[id].image_size[1],
                ),
            );
            ui.painter().image(
                texture.into(),
                rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        };
    }
}
