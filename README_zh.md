# Rust Constructor V2

## 一个强大的跨平台GUI框架, 用Rust开发GUI项目最简单的方式

[![作者: ChepleBob](https://img.shields.io/badge/作者-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![语言: Rust](https://img.shields.io/badge/语言-Rust-5F4C49)](https://www.rust-lang.org/)
[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Github 星星](https://img.shields.io/github/stars/ChepleBob30/Rust-Constructor?style=flat&color=red)
[![版本](https://img.shields.io/badge/版本-v2.1.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

[English](./README.md) | 简体中文

---

Rust Constructor是开源项目，但与egui开发组无直接关系！

## 目录

- [版本更新信息](#版本更新信息)
- [创造Rust Constructor的目的](#创造rust-constructor的目的)
- [常见问题](#常见问题)
- [许可证](#许可证)
- [尾声](#尾声)

---

## 版本更新信息

- 目前的最新版本为v2.1.0: 标准化更新(2025.10.7发布)。主要更新了以下内容：
  - 对所有`RCR`进行大范围改动，现在可以使用更常见的`xxx::default().xxx()`格式来创建资源了。
  - 对所有`add`方法使用方法进行修改。
  - 添加`image_texture`方法，可以直接取出图片纹理。
  - 修改了`Text`的超链接定义方法，现在你可以直接提供链接所属文本内容，并选择选中所有匹配项或特定匹配项。
  - 现在`Switch`的状态数目取决于`appearance`的项目数，少于最小值则会创建失败。
  - 添加`reset_split_time`和`reset_image_texture`方法，用于重置特定资源。
  - 移除了`App`中的`frame`参数。
  - 将`GameText`重命名为`AppText`。
  - 现在`App`的`new`方法直接接收`Config`和`AppText`而非文件路径。
  - 修复了`Switch`的提示文本在出现一次留空情况就无法显示的问题。
  - 修复了`Text`的超链接文本下划线不会被文本框覆盖的问题。
  - 移除了`ScrollBackground`。

---

## 创造Rust Constructor的目的

我们在开发[靶向载体](https://github.com/ChepleBob30/Targeted-Vector/)的过程中遇到了一些egui无法解决的问题，因此我们拓展了许多工具。为了能让更多人便利地进行开发，我们就创造了Rust Constructor。

---

## 常见问题

- Rust Constructor支持哪些平台？

    *macOS和Windows已确认完全支持，Linux尚未经过测试。*

- Rust Constructor如何使用？

    *在toml中添加 rust_constructor = "x.y.z"。

- Rust Constructor V2比起V1有什么区别？

    *将原先的架构修改为了符合库crate的结构，发布在了[crates.io](https://crates.io/)上，并添加了官方指南。

---

## 许可证

[MIT](./LICENSE-MIT) © 2025 ChepleBob

## 尾声

如果你喜欢这个项目，请在GitHub上给我点个star。你也可以加入我们的组织[必达](https://github.com/Binder-organize)。
