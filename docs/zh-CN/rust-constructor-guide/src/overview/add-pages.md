拥有一个页面并进行循环加载是`Rust Constructor`的一个至关重要且容易被忽视的事情，因为所有的更新逻辑都放在了页面加载中。

# 应用页面

在`Rust Constructor`中，页面也属于一个资源，它叫做`PageData`，是本项目中较为简单的一个资源。你只需要关注一个叫做`forced_update`的字段，它的作用是在开启时每一帧都强制重绘页面。如果不开启，那么当用户不进行任何操作时（例如鼠标的移动，键盘的操作），界面就会停止绘制（在用户看来相当于画面卡住了），直至用户进行了操作。

添加页面只需使用上文提及的`add_resource`即可，

要想使用页面，你只需调用`use_resource`即可。如果想要让程序正常运作，使用页面是必不可少的。

# 简单示例

以下是一个应用了页面的简单示例：
```rust
pub struct RcApp {
    pub inner: rust_constructor::app::App,
}

fn main() {
    eframe::run_native(
        "Example App",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            Ok(Box::new(RcApp {
                inner: rust_constructor::app::App::default().current_page("Launch"),
            }))
        }),
    )
    .unwrap();
}

impl eframe::App for RcApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if self
            .inner
            .check_resource_exists(&rust_constructor::build_id("Launch", "PageData"))
            .is_none()
        {
            self.inner
                .add_resource(
                    "Launch",
                    rust_constructor::background::PageData::default().forced_update(true),
                )
                .unwrap();
        };
        match &*self.inner.current_page.clone() {
            "Launch" => self
                .inner
                .use_resource(&rust_constructor::build_id("Launch", "PageData"), None, ui)
                .unwrap(),
            _ => {}
        };
    }
}
```
需要注意的是，如果你需要自定义初始页面名称，就需要在创建`App`时指定。同时，代码中的`build_id`是一个用于快速构建`RustConstructorId`的方法。

执行效果大致如下：

![运行窗口](../images/add_pages.png)

非常好，现在你已经基本掌握了使用`Rust Constructor`的方法。接下来，让我们来具体看看各个资源的作用。
