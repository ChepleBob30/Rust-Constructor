# `Rust Constructor V2`

## 基于`egui`构建的跨平台`GUI`框架, 用`Rust`开发`GUI`项目最简单的方式

[![作者: ChepleBob](https://img.shields.io/badge/作者-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![语言: Rust](https://img.shields.io/badge/语言-Rust-5F4C49)](https://www.rust-lang.org/)
[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![版本](https://img.shields.io/badge/版本-v2.8.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

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

- 目前的最新版本为`v2.8.0 三个重点`。
  - 本更新围绕“多维度”“快运行”“好使用”三个重点进行了一系列改进。
    - 修改了`RustConstructorResource`中`modify_tags`的定义，现在可以选择是否要清空原有标签；
    - 将`position`和`size`从`position_size_config`中取出，并在`BasicFrontResource`中添加了`display_position``display_size`两个新方法；
    - 移除了事件机制及与其相关的所有内容；
    - 将`Rust Constructor Extra`重新整合到`Rust Constructor`中(也就是说，添加了`Background` `Switch` `ResourcePanel`三个新资源及其相关方法)；
    - 将颜色和透明度拆成了`color`和`alpha`两个字段；
    - 移除了`Text`的`truncate`，现在`Text`强制截断文本，且可以通过`auto_fit`指定是否让渲染层大小(设定`size`等于实际文本大小还是文本框大小)；
    - 改进了`ResourcePanel`的布局方式，现在只需在`ResourcePanel`内部定义排版方式即可；
    - 添加了`ResourcePanel`的滚动条；
    - 修复了`ResourcePanel`的布局错乱问题；
    - 修复了一些已知问题。
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
