//! Rust Constructor v1.2.0
//! Developer: Cheple_Bob
//! A powerful cross-platform GUI framework, the easiest way to develop GUI projects in Rust.
use egui::IconData;
use function::App;
use function::Config;
use function::GameText;
use function::read_from_json;
use std::collections::HashMap;
use std::sync::Arc;

mod function;
mod pages;
fn main() {
    let mut config = Config {
        language: 0,
        amount_languages: 0,
        rc_strict_mode: false,
        enable_debug_mode: false,
    };

    if let Ok(json_value) = read_from_json("Resources/config/Preferences.json") {
        if let Some(read_config) = Config::from_json_value(&json_value) {
            config = read_config;
        };
    };

    let mut gametext = GameText {
        game_text: HashMap::new(),
    };

    if let Ok(json_value) = read_from_json("Resources/config/GameText.json") {
        if let Some(read_gametext) = GameText::from_json_value(&json_value) {
            gametext = read_gametext;
        };
    };

    let img = image::load_from_memory_with_format(
        include_bytes!("../Resources/assets/images/icon.png"),
        image::ImageFormat::Png,
    )
    .unwrap();

    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();
    let options = eframe::NativeOptions {
        centered: true,
        vsync: false,
        viewport: egui::ViewportBuilder::default()
            .with_icon(Arc::<IconData>::new(IconData {
                rgba: raw_data,
                width: w,
                height: h,
            }))
            .with_active(true)
            .with_maximized(true)
            .with_title(gametext.game_text["debug_game_version"][config.language as usize].clone())
            .with_min_inner_size([1280_f32, 720_f32]),
        ..Default::default()
    };

    println!(
        "{}\n{} https://github.com/ChepleBob30/Rust-Constructor :)",
        gametext.game_text["debug_game_version"][config.language as usize],
        gametext.game_text["hello"][config.language as usize]
    );

    eframe::run_native(
    "Rust Constructor",
    options,
    Box::new(|_cc: &eframe::CreationContext| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
        let app: App = App::new();
        Ok(Box::new(app))
    }),
    ).unwrap();
}
