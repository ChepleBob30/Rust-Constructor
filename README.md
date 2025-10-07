# Rust Constructor V2

## A powerful cross-platform GUI framework, the easiest way to develop GUI projects in Rust

[![Author: ChepleBob](https://img.shields.io/badge/Author-ChepleBob-00B4D8)](https://github.com/ChepleBob30)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-5F4C49)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Github Stars](https://img.shields.io/github/stars/ChepleBob30/Rust-Constructor?style=flat&color=red)
[![Version](https://img.shields.io/badge/Version-v2.1.0-421463)](https://github.com/ChepleBob30/Rust-Constructor/releases)

English | [简体中文](./README_zh.md)

---

Rust Constructor is an open-source project, but it has no direct connection with the development team of egui!

## Contents

- [Version update information](#version-update-information)
- [The purpose of creating Rust Constructor](#the-purpose-of-creating-rust-constructor)
- [FAQ](#faq)
- [License](#license)
- [Epilogue](#epilogue)

---

## Version update information

- The latest version currently available is v2.1.0: Standardized Update (released on October 7, 2025). The main updates are as follows:
  - Make extensive changes to all `RCR`s. Now, you can create resources using the more common `xxx::default().xxx()` format.
  - Modify the usage methods of all `add` methods.
  - Adding the `image_texture` method can directly extract the texture of the image.
  - The hyperlink definition method of `Text` has been modified. Now you can directly provide the text content to which the link belongs and select all matches or specific matches.
  - The current number of states on `Switch` depends on the number of items in `appearance`. If it is less than the minimum value, the creation will fail.
  - Add the `reset_split_time` and `reset_image_texture` methods to reset specific resources.
  - Removed the `frame` parameter from `App`.
  - Rename `GameText` to `AppText`.
  - Now, the `new` method of the `App` directly receives `Config` and `AppText` instead of file paths.
  - Fixed the issue where the prompt text of `Switch` failed to display after a single blank space.
  - Fixed the issue where the underline of the hyperlink in `Text` would not be covered by the text box.
  - Removed `ScrollBackground`.

---

## The purpose of creating Rust Constructor

During the development of [Targeted Vector](https://github.com/ChepleBob30/Targeted-Vector/), we encountered some problems that egui couldn't solve, so we expanded many tools. To enable more people to develop conveniently, we created Rust Constructor.

---

## FAQ

- Which platforms does Rust Constructor support?

    *MacOS and Windows have been confirmed to fully support it, while Linux has not yet been tested.*

- How to use Rust Constructor?

    *Add rust_constructor = "x.y.z" in toml.

- What are the differences between Rust Constructor V2 and V1?  
    *The original architecture was modified to fit the structure of the library crate and published on [crates.io](https://crates.io/) with an official guide added.

---

## License

[MIT](./LICENSE-MIT) © 2025 ChepleBob

## Epilogue

If you like this project, please give me a star on GitHub. You can also join our organization [Binder](https://github.com/Binder-organize).
