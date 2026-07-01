# 安装

和其他库一样，使用`Cargo`自带的`cargo add rust-constructor`即可完成安装。

需要注意的是，由于作者代码水平问题，`Rust Constructor`的更新时常伴随破坏性改动，
所以更新依赖版本时务必谨慎。如果遇到问题，请先查看文档。

# 使用

由于`Rust Constructor`是`egui`的一个扩展库，你可以先尝试使用`eframe`来快速启动一个`Rust Constructor`项目。
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
这段代码展示了一个`Rust Constructor`应用的最小实现，尽管代码中并没有用到任何`Rust Constructor`的功能。

由于孤儿规则的存在，在使用前，你必须用一个自定义的结构体包裹`Rust Constructor`的`App`结构体并为其实现`eframe::App`才能正常使用。
