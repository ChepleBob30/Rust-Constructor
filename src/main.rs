use egui::IconData;
use screen_size::get_primary_screen_size;
use std::sync::Arc;

mod function;
mod pages;
use function::App;

fn main() {
    let (mut width, mut height): (u64, u64) = (1280, 720);
    if let Ok((try_catch_width, try_catch_height)) = get_primary_screen_size() {
        (width, height) = (try_catch_width, try_catch_height);
    }
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([width as f32, height as f32]),
        ..Default::default()
    };
    let img = image::load_from_memory_with_format(
        include_bytes!("../assets/images/icon.png"),
        image::ImageFormat::Png,
    )
    .unwrap();
    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();
    options.viewport.icon = Some(Arc::<IconData>::new(IconData {
        rgba: raw_data,
        width: w,
        height: h,
    }));
    let _ = eframe::run_native(
        "Rust Constructor v0.2.0",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
