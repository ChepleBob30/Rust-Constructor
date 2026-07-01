# Installation

Like any other library, you can install it using Cargo's built-in `cargo add rust-constructor`.

One thing to note: due to the author's coding ability, updates to `Rust Constructor` often come with breaking changes, so be careful when upgrading dependency versions. If you run into problems, check the documentation first.

# Usage

Since `Rust Constructor` is an extension library for `egui`, you can first try using `eframe` to quickly spin up a `Rust Constructor` project.
```rust
pub struct RcApp {
    pub inner: rust_constructor::app::App,
}

impl eframe::App for RcApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Hello world");
        });
    }
}

fn main() {
    eframe::run_native(
        "Example App",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            Ok(Box::new(RcApp {
                inner: rust_constructor::app::App::default(),
            }))
        }),
    )
    .unwrap();
}
```
This code shows a minimal `Rust Constructor` application, even though it doesn't actually use any of `Rust Constructor`'s features.

Due to the orphan rule, before using it you must wrap `Rust Constructor`'s `App` struct in a custom struct and implement `eframe::App` on it for everything to work properly.
