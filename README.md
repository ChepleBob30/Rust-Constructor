# `Rust Constructor V2`

## A cross-platform `GUI` framework built on `egui`, the simplest way to develop `GUI` projects with `Rust`

[![Author: ChepleBob](https://img.shields.io/badge/author-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![Language: Rust](https://img.shields.io/badge/language-Rust-5F4C49)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-v2.9.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

English | [简体中文](./README_zh.md)

---

`Rust Constructor` is an open-source project, but it has no direct relationship with the `egui` development team!

## Table of Contents

- [Version Update Information](#version-update-information)
- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Purpose of Creating Rust Constructor](#purpose-of-creating-rust-constructor)
- [FAQ](#faq)
- [License](#license)
- [Epilogue](#epilogue)

---

## Version Update Information

- Current latest version is `v2.9.0 Scientific Resource Panel`.
  - This update expands `ResourcePanel` with extensive practical features and addresses several issues.
    - `AutoFit` now supports allocating additional scroll margin;
    - Added `PanelStorage` to store internal resource metadata in `ResourcePanel`;
    - Deprecated `custom_layout` and `overall_layout` - unified layout configuration via `layout` setter;
    - Implemented dynamic resource hiding capability in `ResourcePanel`;
    - Scroll direction reversal via `reverse_scroll_direction` now supports axis-specific control with single-direction enforcement;
    - Fixed scrollbar visibility logic inconsistencies;
    - Introduced `auto_shrink` for automatic resource scaling in dynamic layout adjustments;
    - Renamed `unsafe_request_jump_render_list` to `try_request_jump_render_list`;
    - `Switch` components now support tag inheritance for background and text elements;
    - Implemented visibility control for `Switch` components;
    - Addressed `Switch` press detection logic - now requires mouse button down/up sequence on component;
    - `HintText` now respects opacity threshold detection to prevent activation when fully transparent;
    - Reduced mouse interaction area for `ResourcePanel`;
    - `ResourcePanel` now automatically brings itself to frontmost layer upon click;
    - Fixed several known issues.

---

## Introduction

- `Rust Constructor` is a Rust graphical development library built on egui, including commonly used functions such as timers, resource storage, and error handling.
- `Rust Constructor` released its first version in `2025.2`, and today's `Rust Constructor` has undergone earth-shaking changes compared to that time.

---

## Quick Start

- To introduce `Rust Constructor`, please add `rust_constructor = "x.y.z"` (please replace xyz according to your needs) to `toml`.
- If you want to start `App` and perform some simple operations, it is recommended to refer to the [official documentation of egui](https://github.com/emilk/egui).
- You can refer to the official tutorial of `Rust Constructor` [Rust Constructor Guide](https://github.com/ChepleBob30/Rust-Constructor-Guide).

---

## Purpose of Creating Rust Constructor

We encountered some problems that `egui` could not solve during the development of [Targeted Vector](https://github.com/ChepleBob30/Targeted-Vector), so we expanded many tools. In order to allow more people to develop conveniently, we created `Rust Constructor`.

---

## FAQ

- Q1: Which platforms does `Rust Constructor` support?

- A1: `macOS` and `Windows` have been confirmed to be fully supported, and other platforms depend on `egui` support.

- Q2: What is the difference between `Rust Constructor V2` and `V1`?

- A2: The original architecture was modified to conform to the structure of the library `crate`, published on [crates.io](https://crates.io/), and added an official guide.

- Q3: Why do I get an error when calling a resource?

- A3: Please ensure that you have added the resource through the `add` method and there are no spelling errors.

- Q4: How to modify resources?

- A4: Take out the resource through `get_resource_mut`.

- Q5: What should I do if I encounter unknown error prompts?

- A5: Prioritize checking the `RustConstructorError` definition in the `Rust Constructor` source code, find the problem you triggered and correct it.

- Q6: Why is there only `V2` of `Rust Constructor` on `crates.io`?

- A6: `Rust Constructor V0` and `Rust Constructor V1` are essentially bloated projects with many redundant functions and meaningless code, so they were not released.

---

## License

[MIT](./LICENSE-MIT) © 2025 ChepleBob

## Epilogue

If you like this project, please give me a `star` on `GitHub`. You can also join our organization [Binder](https://github.com/Binder-organize).
