# `Rust Constructor V2`

## A cross-platform `GUI` framework built on `egui` as well as compatible with `bevy`, the simplest way to develop `GUI` projects with `Rust`

[![Author: ChepleBob](https://img.shields.io/badge/Author-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-5F4C49)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/Version-v2.12.1-7F1C13)](https://github.com/ChepleBob30/Rust-Constructor/releases)

English | [简体中文](./docs/zh-CN/README_zh-CN.md)

---

## Table of Contents

- [Version Update Information](#version-update-information)
- [Overview](#overview)
- [Quick Start](#quick-start)
- [Purpose of Creating Rust Constructor](#purpose-of-creating-rust-constructor)
- [FAQ](#faq)
- [License](#license)
- [Epilogue](#epilogue)

---

## Version Update Information

- The current version is `v2.12.1`.
  - This update fixes some known issues.
  - **Improvements**
    - Improved some code.
  - **Breaking Changes**
    - Removed the `state` method from `Switch`;
    - `DebugTextureHandle` has been extended with a `path` field, and the original field has been renamed to `texture_handle`;
    - `LoadedImageData` has been extended with a `path` field;
    - `Image` has been extended with a `texture_list` field for storing all loaded textures;
    - Modified some content.
  - **Bug Fixes**
    - Fixed the issue where textures were reloaded every time the image path was toggled;
    - Fixed some known issues.

---

## Overview

- `Rust Constructor` is a comprehensive GUI framework that leverages the powerful features of `egui` to provide a simple and intuitive tool for building cross-platform applications.
- Based on `bevy_egui`, `Rust Constructor` is also compatible with `bevy`, so you can use it directly in `bevy`.
- `Rust Constructor` is certainly not perfect. Currently, it still has problems such as poor writing logic and confusing usage methods. I will do my best to solve these problems.

---

## Quick Start

- To introduce `Rust Constructor`, please add `rust_constructor = "x.y.z"` (please replace xyz as needed) to your `toml`.
- If you want to launch an `App` and perform some simple operations, it is recommended to refer to the [official documentation of egui](https://github.com/emilk/egui).
- You can refer to the official documentation of `Rust Constructor` (not necessarily up-to-date) [Rust Constructor Guide](./docs/en/rust-constructor-guide/src/introduction.md).
- Here is a simple example:

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

## Purpose of Creating Rust Constructor

- In actual development, we often need to consider layout and resource presentation under different screen sizes, but most of `egui`'s built-in components are only suitable for quick development without fine-tuning, which does not meet my needs. Therefore, I developed `Rust Constructor`.

---

## FAQ

- Q1: Which platforms does `Rust Constructor` support?

- A1: `macOS` and `Windows` have been confirmed to be fully supported, other platforms depend on `egui`'s support.

- Q2: Why is there only `V2` of `Rust Constructor` on `crates.io`?

- A2: `Rust Constructor V0` and `Rust Constructor V1` were essentially bloated projects with many redundant functions and meaningless code, so they were never released.

- Q3: What is the relationship between `Rust Constructor` and `egui`?

- A3: `Rust Constructor` is developed based on `egui`, but there is no relationship between the developers of both sides.

- Q4: What should I do if I encounter new resources that I don't know how to handle?

- A4: Please first add it using `add_resource` or `quick_place`, then browse the source code to find methods related to that resource and try to use them.

- Q5: Does having `Rust Constructor` mean I can completely abandon `egui`?

- A5: Quite the opposite. During debugging, the clumsiness of `Rust Constructor` precisely needs `egui`'s components to compensate.

- Q6: What is `discern_type`?

- A6: `discern_type` is a string used to mark resource types, which is automatically created when adding resources, with the name the same as the resource name you used.

- Q7: Can I develop without using `Rust Constructor`'s components?

- A7: Of course you can. However, `Rust Constructor`'s advanced front-end resources do provide some practical functions that may not be replaceable.

- Q8: Under what form is `Rust Constructor` open-sourced? How to contribute?

- A8: The source code of `Rust Constructor` is open-sourced under the `MIT` license. Due to personal reasons, I do not recommend contributing to this project. If you have any ideas, please fork this project and maintain it yourself.

- Q9: I still have unresolved issues, what should I do?

- A9: Please raise an [`issue`](https://github.com/ChepleBob30/Rust-Constructor/issues) in this project repository, and I will try my best to solve all problems.

---

## License

[MIT](./LICENSE-MIT) © 2026 ChepleBob

## Epilogue

If you like this project, please give me a `star` on `GitHub`. You can also join our organization [Binder](https://github.com/Binder-organize).
