# `Rust Constructor V2`

## A cross-platform `GUI` framework built on `egui`, the simplest way to develop `GUI` projects in `Rust`

[![Author: ChepleBob](https://img.shields.io/badge/Author-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-5F4C49)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Stars](https://img.shields.io/github/stars/ChepleBob30/Rust-Constructor?style=flat&color=red)
[![Version](https://img.shields.io/badge/Version-v2.2.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

English | [简体中文](./README_zh.md)

---

`Rust Constructor` is an open-source project, but is not directly affiliated with the `egui` development team!

## Table of Contents

- [Version Update Information](#version-update-information)
- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Purpose of Creating Rust Constructor](#purpose-of-creating-rust-constructor)
- [Frequently Asked Questions](#frequently-asked-questions)
- [License](#license)
- [Conclusion](#conclusion)

---

## Version Update Information

- The current latest version is `v2.2.0 Health Update`. Major updates include:
  - Improved creation methods for some structs.
  - `RustConstructorResource` now supports `Debug`.
  - The `reg_render_resource` method of `RustConstructorResource` can now be automatically implemented.
  - `RustConstructorResource` extends two methods `as_any` and `as_any_mut` for type conversion.
  - `Image` and `ImageTexture` now implement `Debug`.
  - Added `DebugTextureHandle` struct to implement `Debug` for `ImageTexture`.
  - `RustConstructorError` now implements `Display` and `Error`, and added two new error types: `ImageTextureNotFound` and `RectNotFound`.
  - Fields `page` and `vertrefresh` in `App` are now renamed to `current_page` and `tick_interval`.
  - The storage content of `rust_constructor_resource` is now `Vec<Box<dyn RustConstructorResource>>`.
  - Extensively improved error handling.
  - Removed `get_resource_index`.
  - Added `get_resource` and `get_resource_mut` methods to directly fetch resources from the list and operate on them.
  - Added `replace_resource` and `replace_resource_custom` methods to replace specified resources from the list.
  - Modified many methods to adapt to the new storage approach.
  - Added `problem_report_custom` method for custom problem reporting lists.
  - Removed some functions and methods with low relevance to `Rust Constructor`.

---

## Introduction

- `Rust Constructor` is a Rust GUI development library built on egui, including common features such as timers, resource storage, and error handling.
- `Rust Constructor` released its first version in `2025.2`, and today's `Rust Constructor` has undergone earth-shattering changes compared to that time.

---

## Quick Start

- To introduce `Rust Constructor`, please add `rust_constructor = "x.y.z"` (please replace xyz as needed) to your `toml`.
- If you want to start `App` and perform some simple operations, it is recommended to refer to the [official egui documentation](https://github.com/emilk/egui).
- Due to the large content and complex operations of `Rust Constructor`, it is inconvenient to fully describe here. You can refer to the official tutorial of `Rust Constructor` [Rust Constructor Guide](https://github.com/ChepleBob30/Rust-Constructor-Guide).

---

## Purpose of Creating Rust Constructor

We encountered some problems that `egui` could not solve during the development of [Targeted Vector](https://github.com/ChepleBob30/Targeted-Vector), so we expanded many tools. To enable more people to develop conveniently, we created `Rust Constructor`.

---

## Frequently Asked Questions

- Q1: Which platforms does `Rust Constructor` support?

- A1: `macOS` and `Windows` are confirmed to be fully supported, other platforms depend on `egui` support.

- Q2: What are the differences between `Rust Constructor V2` and `V1`?

- A2: The original architecture was modified to conform to the `crate` structure, published on [crates.io](https://crates.io/), and added an official guide.

- Q3: Why do I get errors when calling resources?

- A3: Please ensure you have added the resources through the `add` method and there are no spelling errors.

- Q4: How to modify resources?

- A4: You can retrieve resources via `get_resource_mut`.

- Q5: What should I do if I encounter unknown error messages?

- A5: First check the `RustConstructorError` definition in the `Rust Constructor` source code, find the problem you triggered and fix it.

- Q6: Why is only `V2` of `Rust Constructor` on `crates.io`?

- A6: `Rust Constructor V0` and `Rust Constructor V1` were essentially bloated projects with many redundant features and meaningless code, so they were not published.

---

## License

[MIT](./LICENSE-MIT) © 2025 ChepleBob

## Conclusion

If you like this project, please give me a `star` on `GitHub`. You can also join our organization [Binder](https://github.com/Binder-organize).
