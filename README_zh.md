# `Rust Constructor V2`

## 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式

[![作者: ChepleBob](https://img.shields.io/badge/作者-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![语言: Rust](https://img.shields.io/badge/语言-Rust-5F4C49)](https://www.rust-lang.org/)
[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Github 星星](https://img.shields.io/github/stars/ChepleBob30/Rust-Constructor?style=flat&color=red)
[![版本](https://img.shields.io/badge/版本-v2.2.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

[English](./README.md) | 简体中文

---

`Rust Constructor`是开源项目，但与`egui`开发组无直接关系！

## 目录

- [版本更新信息](#版本更新信息)
- [简介](#简介)
- [快速上手](#快速上手)
- [创造Rust Constructor的目的](#创造rust-constructor的目的)
- [常见问题](#常见问题)
- [许可证](#许可证)
- [尾声](#尾声)

---

## 版本更新信息

- 目前的最新版本为`v2.2.0 健全更新`。主要更新了以下内容：
  - 完善了部分结构体的创建方法。
  - `RustConstructorResource`现已支持`Debug`。
  - 现在`RustConstructorResource`的方法`reg_render_resource`可以自动实现。
  - `RustConstructorResource`扩展了`as_any`和`as_any_mut`两个方法，用于类型转换。
  - 现在`Image`和`ImageTexture`也实现了`Debug`。
  - 新增`DebugTextureHandle`结构体，用于为`ImageTexture`实现`Debug`。
  - `RustConstructorError`现已实现`Display`和`Error`，并增加了两个新错误类型：`ImageTextureNotFound`和`RectNotFound`。
  - 现在`App`中的字段`page`和`vertrefresh`被重命名为`current_page`和`tick_interval`。
  - 现在`rust_constructor_resource`的存储内容变为`Vec<Box<dyn RustConstructorResource>>`。
  - 大范围完善了错误处理。
  - 移除了`get_resource_index`。
  - 添加`get_resource`和`get_resource_mut`方法，可以直接从列表中抓取资源并进行操作。
  - 添加了`replace_resource`和`replace_resource_custom`方法，可以从列表中替代指定资源。
  - 对大量方法进行修改以适应新的存储方式。
  - 添加`problem_report_custom`方法，用于自定义问题推送列表。
  - 移除了一些与`Rust Constructor`关联性不大的函数及方法。

---

## 简介

- `Rust Constructor`是一个基于egui搭建的Rust图形化开发库，包含了计时器、资源存储、错误处理等常用功能。
- `Rust Constructor`在`2025.2`发布了第一个版本，今天的`Rust Constructor`相较于那时有了翻天覆地的变化。

---

## 快速上手

- 若想引入`Rust Constructor`，请添加`rust_constructor = "x.y.z"`(请根据需要自行替换xyz)到`toml`中。
- 如果想要启动`App`并进行一些简单的操作，建议查阅[egui的官方文档](https://github.com/emilk/egui)。
- 由于`Rust Constructor`的内容量较大且操作复杂，这里不方便完整进行叙述。你可以参考`Rust Constructor`的官方教程[Rust Constructor 指南](https://github.com/ChepleBob30/Rust-Constructor-Guide)。

---

## 创造Rust Constructor的目的

我们在开发[靶向载体](https://github.com/ChepleBob30/Targeted-Vector)的过程中遇到了一些`egui`无法解决的问题，因此我们拓展了许多工具。为了能让更多人便利地进行开发，我们就创造了`Rust Constructor`。

---

## 常见问题

- Q1: `Rust Constructor`支持哪些平台？

- A1: `macOS`和`Windows`已确认完全支持，其他平台视`egui`支持情况而定。

- Q2: `Rust Constructor V2`比起`V1`有什么区别？

- A2: 将原先的架构修改为了符合库`crate`的结构，发布在了[crates.io](https://crates.io/)上，并添加了官方指南。

- Q3: 为什么我调用资源就会报错？

- A3: 请确保你已经通过`add`方法添加了资源，并且没有拼写错误。

- Q4: 如何修改资源？

- A4: 通过`get_resource_mut`取出资源即可。

- Q5: 如果出现未知的报错提示该怎么办？

- A5: 优先查看`Rust Constructor`源代码中的`RustConstructorError`定义处，找到你触发的问题并修正。

- Q6: 为什么`Rust Constructor`在`crates.io`上只有`V2`？

- A6: `Rust Constructor V0`和`Rust Constructor V1`本质上是一个臃肿的项目，有很多冗余的功能和无意义的代码，所以并未发布。

---

## 许可证

[MIT](./LICENSE-MIT) © 2025 ChepleBob

## 尾声

如果你喜欢这个项目，请在`GitHub`上给我点个`star`。你也可以加入我们的组织[必达](https://github.com/Binder-organize)。
