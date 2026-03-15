# `Rust Constructor V2`

## 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式

[![作者: ChepleBob](https://img.shields.io/badge/作者-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![语言: Rust](https://img.shields.io/badge/语言-Rust-5F4C49)](https://www.rust-lang.org/)
[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![版本](https://img.shields.io/badge/版本-v2.11.0-0000CD)](https://github.com/ChepleBob30/Rust-Constructor/releases)

[English](./README.md) | 简体中文

---

## 目录

- [版本更新信息](#版本更新信息)
- [概述](#概述)
- [快速上手](#快速上手)
- [创造Rust Constructor的目的](#创造rust-constructor的目的)
- [常见问题](#常见问题)
- [许可证](#许可证)
- [尾声](#尾声)

---

## 版本更新信息

- 目前的最新版本为`v2.11.0 全面改进`。
  - 本更新全面对资源进行改进，扩展了多项实用功能。
  - **实用改动**
    - `ResourcePanel`添加`raise_on_focus`字段，用于控制被点击后是否将窗口前置；
    - `Switch`添加`radio_group`字段，用于制作单选开关组；
    - 现在`Switch`的提示文本会尝试在最高层级渲染；
    - `lib.rs`中添加`downcast_resource` `downcast_resource_mut`方法，用于安全地转换资源类型；
    - `App`添加`loading_fonts` `loaded_fonts`字段，用于列出正在加载和已加载的字体；
    - `App`中的`active_list`和`render_list`字段额外添加了一个`Option<RustConstructorId>`以支持显示引用者；
    - 所有`info`类方法都扩展了`print`参数，用于控制是否打印信息；
    - `display_render_layer`添加`hover_config`参数，用于在鼠标悬停时高亮显示资源；
    - `resource_get_focus`添加`ignore_render_layer`参数，用于在获取焦点时忽略指定范围的渲染层；
    - 添加`get_basic_front_resource` `get_basic_front_resource_mut`方法，用于获取基本前端资源；
    - `ResourcePanel`在被拖拽时会更换鼠标指针外观；
    - 新增`panel_layout_group`标签，用于指定哪些资源互相无视排版；
    - 新增`try_register_all_fonts`方法，用于尝试注册所有字体；
    - 改进了部分代码。
  - **破坏性改动**
    - 将`Switch`和`SwitchData`的`state`修改为了`usize`类型；
    - 移除了`Font`资源；
    - 将`type_processor` `get_tag` `position_size_processor`移动到`lib.rs`中；
    - 将`ListInfoMethod`重命名为`ListInfoDescribeMethod`；
    - `update_render_layer`现在包含返回值`Result<(), RustConstructorError>`；
    - 现在`register_all_fonts`需要给出字体文件路径并加载，且包含返回值`Result<(), RustConstructorError>`；
    - 对部分内容进行了修改。
  - **漏洞修复**
    - 修复了`resource_get_focus`在检测最高层级资源时不会检查鼠标位置的问题；
    - 修复了`ResourcePanel`点击面板内资源无法前置窗口的问题；
    - 修复了鼠标滚动会导致所有`ResourcePanel`的内部资源全部忽略渲染层的问题；
    - 修复了一些已知问题。

---

## 概述

- `Rust Constructor`是一个功能全面的GUI框架，它利用了`egui`的强大功能，为构建跨平台应用程序提供了一个简单直观的工具。
- `Rust Constructor`当然不是完美的，目前它还存在编写逻辑糟糕，使用方法混乱等问题，我会尽力解决这些问题。

---

## 快速上手

- 若想引入`Rust Constructor`，请添加`rust_constructor = "x.y.z"`(请根据需要自行替换xyz)到`toml`中。
- 如果想要启动`App`并进行一些简单的操作，建议查阅[egui的官方文档](https://github.com/emilk/egui)。
- 你可以参考`Rust Constructor`的官方教程（不一定与时俱进）[Rust Constructor 指南](https://github.com/ChepleBob30/Rust-Constructor-Guide)。
- 以下是一个简单的示例：

```rust
pub struct RcApp {
    pub inner: rust_constructor::app::App,
}

impl eframe::App for RcApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world");
        });
    }
}


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
```

---

## 创造Rust Constructor的目的

- 在实际开发过程中，我们往往需要考虑不同屏幕尺寸下的排版以及资源呈现方式，但是`egui`自带的组件大部分情况下只适用于无需精细打磨的快捷开发，并不能满足我的需求。因此，我开发了`Rust Constructor`。

---

## 常见问题

- Q1: `Rust Constructor`支持哪些平台？

- A1: `macOS`和`Windows`已确认完全支持，其他平台视`egui`支持情况而定。

- Q2: 为什么`Rust Constructor`在`crates.io`上只有`V2`？

- A2: `Rust Constructor V0`和`Rust Constructor V1`本质上是一个臃肿的项目，有很多冗余的功能和无意义的代码，所以并未发布。

- Q3: `Rust Constructor`与`egui`有什么关系？

- A3: `Rust Constructor`基于`egui`开发，但双方开发人员之间没有任何关系。

- Q4: 遇到新的资源不知道如何处理该怎么办？

- A4: 请先用`add_resource`或`quick_place`添加它，然后翻阅源代码查找与该资源有关的方法并尝试使用。

- Q5: 有了`Rust Constructor`是否就意味着可以完全抛弃`egui`了？

- A5: 恰恰相反。在调试过程中，`Rust Constructor`的笨重劣势正需要`egui`的组件来弥补。

- Q6: 什么是`discern_type`？

- A6: `discern_type`是一个用于标记资源类型的字符串，它会在添加资源时被自动创建，名称和你使用的资源名相同。

- Q7: 能否不使用`Rust Constructor`的组件进行开发？

- A7: 当然可以。不过`Rust Constructor`的高级前端资源的确提供了一些实用功能，他们不一定能被替代。

- Q8: 如何查看`Rust Constructor`的官方文档？

- A8: 你现在看到的就是最核心的文档内容。如果还有问题，请查看[`Rust Constructor 指南`](https://github.com/ChepleBob30/Rust-Constructor-Guide)。这是`Rust Constructor`的官方教程，但不一定与时俱进。

- Q9: `Rust Constructor`以什么形式开源？如何做贡献？

- A9: `Rust Constructor`的源代码以`MIT`许可证开源。由于个人原因，我不推荐为此项目做贡献。如果你有什么想法，请选择fork本项目并自行维护。

- Q10: 我还是有问题没解决，怎么办？

- A10: 请在本项目仓库提[`issue`](https://github.com/ChepleBob30/Rust-Constructor/issues)，我会尽力解决所有问题。

---

## 许可证

[MIT](./LICENSE-MIT) © 2026 ChepleBob

## 尾声

如果你喜欢这个项目，请在`GitHub`上给我点个`star`。你也可以加入我们的组织[必达](https://github.com/Binder-organize)。
