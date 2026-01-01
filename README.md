# `Rust Constructor V2`

## A cross-platform `GUI` framework built on `egui`, the simplest way to develop `GUI` projects in `Rust`

[![Author: ChepleBob](https://img.shields.io/badge/Author-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-5F4C49)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/Version-v2.6.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

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

- The current latest version is `v2.6.0 Layout Revolution`. The main updates include the following:
  - Added `ResourcePanel` resource, which can automatically layout resources, lock display ranges, split windows, and other features.
  - Removed the `reg_render_resource` method from `RustConstructorResource`, replacing it with `active` and `modify_active` methods.
  - Optimized active resource management logic, now you can directly print active resource information through the `print_resource_active_info` method in `App`.
  - Extended a large number of methods for `BasicFrontResource`.
  - Renamed `x_grid` and `y_grid` to `x_location_grid` and `y_location_grid` in all basic front-end resources.
  - Extended all basic front-end resources with `x_size_grid` and `y_size_grid` fields for quickly setting resource sizes.
  - Extended a series of enumerations to assist with resource calls.
  - Added an `active` field to all resources to mark whether the resource should be invoked.
  - Migrated some fields that were originally passed in `add` methods of certain resources to inside the structures.
  - Added `panel_name`, `panel_layout`, and `allow_scrolling` to all basic front-end resources for customizing resource styles in the resource panel.
  - Extended all basic front-end resources with a `clip_rect` field to control resource display range, with parts exceeding the range not displayed.
  - Added text box functionality to `Text`, where text exceeding specified dimensions will now be truncated and replaced with ... .
  - Extended `Text` with `actual_size` and `origin_size` fields to get the actual size of the rendered text portion and the original size of the text box.
  - `MouseDetector` can now detect mouse scroll amounts.
  - Extended some resources with fields to adapt to `PositionConfig`.
  - Consolidated all `add` methods into an `add_resource` method that can automatically perform operations based on resource type and add to the resource list.
  - Updated the return value of `get_resource_mut`.
  - Extended `RustConstructorError` to accommodate new resources.
  - Improved some code.

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
