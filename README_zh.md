# `Rust Constructor V2`

## 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式

[![作者: ChepleBob](https://img.shields.io/badge/作者-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![语言: Rust](https://img.shields.io/badge/语言-Rust-5F4C49)](https://www.rust-lang.org/)
[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![版本](https://img.shields.io/badge/版本-v2.7.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

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

- 目前的最新版本为`v2.7.0 拨乱反正`。主要更新了以下内容：
  - 移除了问题报告机制及与其相关的所有内容；
  - 移除了安全模式机制及与其相关的所有内容；
  - 添加了`render_layer` `active_list` `render_list` `event_list` `event_map`用于适配新的渲染机制；
  - 移除了`MouseDetector` `Switch` `MessageBox` `ResourcePanel`及与其相关的所有内容；
  - 添加了渲染队列机制，现在调用资源会先将其加入渲染队列，等到页面刷新时统一进行渲染；
  - 添加`request_jump_render_list`以允许资源跳过渲染队列提前渲染；
  - 添加事件处理机制，当你尝试加载外部库的资源时，会发送事件到事件列表中，等待外部库进行处理；
  - 添加`quick_place`，支持自动添加资源并运行资源；
  - `RustConstructorResource`现在添加了`display_display_info` `modify_display_info` `display_tags` `modify_tags`方法用于快速获取或修改资源；
  - 修改了`BasicFrontResource`的部分方法；
  - 添加了`RustConstructorResourceBox`，用于封装资源进入`App`；
  - 添加了`RustConstructorId`，包含资源名称与类型；
  - 添加了`BasicFrontResourceConfig`，用于快速设置基本前端资源的样式；
  - `center_display`现已重命名为`display_method`；
  - `PositionConfig`重命名为`PositionSizeConfig`，并添加了`position`和`size`两个字段；
  - `ReportState`现已重命名为`EventState`；
  - 移除了`Problem`和`SeverityLevel`；
  - 添加了`NeedPlaceholder`用于标记外部库资源是否需要预留占位符；
  - 为所有资源添加了`tags`用于自定义，并将一系列字段合并；
  - `ImageTexture`的`ctx`现已重命名为`context`；
  - 添加了`BorderKind`用于指定`CustomRect`的边缘显示方法；
  - 现在`ImageConfig` `TextConfig` `CustomRectConfig`的所有字段都带有`Option`，未设置的字段不会覆盖资源原有设置；
  - 基本前端资源添加了`display_info`和`basic_front_resource_config`用于管理显示信息和位置尺寸等基本信息；
  - `CustomRect`现已支持设置边缘显示方法；
  - 添加了`DisplayInfo`，用于控制资源是否允许显示，是否隐藏，是否忽略渲染层级；
  - 修改了`RustConstructorError`，现在仅保留一个错误类型和一个对错误的描述；
  - 添加了`RenderConfig`，用于指定调试工具渲染资源时的外观；
  - 添加了`Event`，用于说明事件的类型和状态；
  - 添加了一系列方法和字段以适配新渲染机制；
  - 大范围改进了代码。

---

## 简介

- `Rust Constructor`是一个基于egui搭建的Rust图形化开发库，包含了计时器、资源存储、错误处理等常用功能。
- `Rust Constructor`在`2025.2`发布了第一个版本，今天的`Rust Constructor`相较于那时有了翻天覆地的变化。

---

## 快速上手

- 若想引入`Rust Constructor`，请添加`rust_constructor = "x.y.z"`(请根据需要自行替换xyz)到`toml`中。
- 如果想要启动`App`并进行一些简单的操作，建议查阅[egui的官方文档](https://github.com/emilk/egui)。
- 你可以参考`Rust Constructor`的官方教程[Rust Constructor 指南](https://github.com/ChepleBob30/Rust-Constructor-Guide)。

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
