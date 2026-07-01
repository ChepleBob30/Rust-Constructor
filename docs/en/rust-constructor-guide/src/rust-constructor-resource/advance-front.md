Advanced front-end resources are the most distinctive feature of `Rust Constructor`.

# Background

`Background` is a practical resource that integrates `Image` and `CustomRect` resources, so you can freely choose between images and rectangles when you need to fill an area.

## Code Example (Rectangle)

```rust
self.inner
    .quick_place(
        "Example",
        rust_constructor::advance_front::Background::default().background_type(
            &rust_constructor::advance_front::BackgroundType::CustomRect(
                rust_constructor::basic_front::CustomRectConfig::default()
                    .position_size_config(Some(
                        rust_constructor::PositionSizeConfig::default()
                            .origin_size(300_f32, 300_f32)
                            .x_location_grid(1_f32, 2_f32)
                            .y_location_grid(1_f32, 2_f32)
                            .display_method(
                                rust_constructor::HorizontalAlign::Center,
                                rust_constructor::VerticalAlign::Center,
                            ),
                    ))
                    .border_width(Some(5_f32))
                    .border_color(Some([0, 0, 255]))
                    .rounding(Some(5_f32))
                    .color(Some([0, 255, 0])),
            ),
        ),
        None,
        ui,
    )
    .unwrap();
```

This brings us to another key concept: configuration. Every front-end resource has a corresponding configuration struct whose purpose is to quickly set up all the information for a resource, and each field is optional. In this code, we set the background to a rectangle and provided a rectangle configuration.

![Background Rectangle](../images/background_custom_rect.png)

## Code Example (Image)

```rust
self.inner
    .quick_place(
        "Example",
        rust_constructor::advance_front::Background::default().background_type(
            &rust_constructor::advance_front::BackgroundType::Image(
                rust_constructor::basic_front::ImageConfig::default()
                    .position_size_config(Some(
                        rust_constructor::PositionSizeConfig::default()
                            .origin_size(300_f32, 300_f32)
                            .x_location_grid(1_f32, 2_f32)
                            .y_location_grid(1_f32, 2_f32)
                            .display_method(
                                rust_constructor::HorizontalAlign::Center,
                                rust_constructor::VerticalAlign::Center,
                            ),
                    ))
                    .image_load_method(Some(
                        rust_constructor::basic_front::ImageLoadMethod::ByPath((
                            "logo.png".to_string(),
                            [false, false],
                        )),
                    )),
            ),
        ),
        None,
        ui,
    )
    .unwrap();
```

![Background Image](../images/background_image.png)

# Switch

`Switch` is a practical advanced resource with many configurable options.

A working example is shown below:

![Switch Off](../images/switch_inactive.png)

![Switch On](../images/switch_active.png)

## Code Example

```rust
self.inner
    .quick_place(
        "Example",
        rust_constructor::advance_front::Switch::default()
            .enable_animation(false, true)
            .state_amount(2)
            .background_type(
                &rust_constructor::advance_front::BackgroundType::Image(
                    rust_constructor::basic_front::ImageConfig::default()
                        .position_size_config(Some(
                            rust_constructor::PositionSizeConfig::default()
                                .origin_size(300_f32, 175_f32)
                                .x_location_grid(1_f32, 2_f32)
                                .y_location_grid(1_f32, 2_f32)
                                .display_method(
                                    rust_constructor::HorizontalAlign::Center,
                                    rust_constructor::VerticalAlign::Center,
                                ),
                        )),
                ),
            )
            .click_method(vec![
                rust_constructor::advance_front::SwitchClickConfig {
                    click_method: eframe::egui::PointerButton::Primary,
                    action: true,
                },
            ])
            .appearance(&[
                rust_constructor::advance_front::SwitchAppearanceConfig {
                    background_config: rust_constructor::advance_front::BackgroundType::Image(
                        rust_constructor::basic_front::ImageConfig::default()
                            .overlay_color(Some([255, 255, 255]))
                            .image_load_method(Some(
                                rust_constructor::basic_front::ImageLoadMethod::ByPath(
                                    ("switch_inactive.png".to_string(), [false, false]),
                                ),
                            )),
                    ),
                    text_config: rust_constructor::basic_front::TextConfig::default(),
                    hint_text_config: rust_constructor::basic_front::TextConfig::default()
                },
                rust_constructor::advance_front::SwitchAppearanceConfig {
                    background_config: rust_constructor::advance_front::BackgroundType::Image(
                        rust_constructor::basic_front::ImageConfig::default()
                            .overlay_color(Some([100, 100, 100]))
                            .image_load_method(Some(
                                rust_constructor::basic_front::ImageLoadMethod::ByPath(
                                    ("switch_inactive.png".to_string(), [false, false]),
                                ),
                            )),
                    ),
                    text_config: rust_constructor::basic_front::TextConfig::default(),
                    hint_text_config: rust_constructor::basic_front::TextConfig::default()
                },
                rust_constructor::advance_front::SwitchAppearanceConfig {
                    background_config: rust_constructor::advance_front::BackgroundType::Image(
                        rust_constructor::basic_front::ImageConfig::default()
                            .overlay_color(Some([255, 255, 255]))
                            .image_load_method(Some(
                                rust_constructor::basic_front::ImageLoadMethod::ByPath(
                                    ("switch_active.png".to_string(), [false, false]),
                                ),
                            )),
                    ),
                    text_config: rust_constructor::basic_front::TextConfig::default(),
                    hint_text_config: rust_constructor::basic_front::TextConfig::default()
                },
                rust_constructor::advance_front::SwitchAppearanceConfig {
                    background_config: rust_constructor::advance_front::BackgroundType::Image(
                        rust_constructor::basic_front::ImageConfig::default()
                            .overlay_color(Some([100, 100, 100]))
                            .image_load_method(Some(
                                rust_constructor::basic_front::ImageLoadMethod::ByPath(
                                    ("switch_active.png".to_string(), [false, false]),
                                ),
                            )),
                    ),
                    text_config: rust_constructor::basic_front::TextConfig::default(),
                    hint_text_config: rust_constructor::basic_front::TextConfig::default()
                },
            ]),
        None,
        ui,
    )
    .unwrap();
```
In this code, `enable_animation` controls whether the switch changes when the mouse hovers or clicks, `state_amount` controls the number of switch states, `appearance` controls the switch's appearance and includes configuration items for three resources, and `click_method` controls which click methods can trigger the switch and change its state.

The running effect is shown below:

![Switch Effect](../images/switch.gif)

To check the switch state, simply call the `check_switch_data` method.

# Resource Panel

`ResourcePanel` is a key resource that transforms `Rust Constructor`. This resource serves multiple roles at once; you can think of it as a window.

## Code Example

```rust
impl eframe::App for RcApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if self
            .inner
            .check_resource_exists(&rust_constructor::build_id("Launch", "PageData"))
            .is_none()
        {
            self.inner
                .add_resource(
                    "Launch",
                    rust_constructor::background::PageData::default().forced_update(true),
                )
                .unwrap();
            self.inner
                .add_resource(
                    "Example1",
                    rust_constructor::basic_front::CustomRect::default()
                        .tags(&[["panel_name".to_string(), "Example".to_string()]], false)
                        .basic_front_resource_config(
                            &rust_constructor::BasicFrontResourceConfig::default()
                                .position_size_config(
                                    rust_constructor::PositionSizeConfig::default()
                                        .origin_size(200_f32, 200_f32),
                                ),
                        ),
                )
                .unwrap();
            self.inner
                .add_resource(
                    "Example2",
                    rust_constructor::basic_front::Text::default()
                        .content("Another text")
                        .basic_front_resource_config(
                            &rust_constructor::BasicFrontResourceConfig::default()
                                .position_size_config(
                                    rust_constructor::PositionSizeConfig::default()
                                        .origin_size(200_f32, 100_f32)
                                        .display_method(
                                            rust_constructor::HorizontalAlign::Left,
                                            rust_constructor::VerticalAlign::Bottom,
                                        )
                                        .origin_position(300_f32, 600_f32),
                                ),
                        )
                        .tags(
                            &vec![["panel_name".to_string(), "Example".to_string()]],
                            false,
                        ),
                )
                .unwrap();
        };
        match &*self.inner.current_page.clone() {
            "Launch" => {
                self.inner
                    .use_resource(&rust_constructor::build_id("Launch", "PageData"), None, ui)
                    .unwrap();
                self.inner
                    .quick_place(
                        "Example",
                        rust_constructor::advance_front::ResourcePanel::default()
                            .overall_layout(rust_constructor::advance_front::PanelLayout {
                                panel_margin:
                                    rust_constructor::advance_front::PanelMargin::Vertical(
                                        [0_f32, 10_f32, 0_f32, 10_f32],
                                        true,
                                    ),
                                panel_location:
                                    rust_constructor::advance_front::PanelLocation::Relative([
                                        [0_f32, 0_f32],
                                        [0_f32, 0_f32],
                                    ]),
                            })
                            .overall_config(rust_constructor::advance_front::PanelConfig {
                                custom_rect_config:
                                    rust_constructor::basic_front::CustomRectConfig::default()
                                        .color(Some([0, 0, 255])),
                                text_config: rust_constructor::basic_front::TextConfig::default()
                                    .content(Some("Hello".to_string())),
                                image_config: rust_constructor::basic_front::ImageConfig::default(),
                            })
                            .resizable(true, true, true, true)
                            .inner_margin(16_f32, 16_f32, 16_f32, 16_f32)
                            .hidden(false)
                            .scroll_sensitivity(2_f32)
                            .scroll_length_method(
                                Some(
                                    rust_constructor::advance_front::ScrollLengthMethod::AutoFit(
                                        0_f32,
                                    ),
                                ),
                                Some(
                                    rust_constructor::advance_front::ScrollLengthMethod::AutoFit(
                                        0_f32,
                                    ),
                                ),
                            )
                            .scroll_bar_display_method(
                                rust_constructor::advance_front::ScrollBarDisplayMethod::OnlyScroll(
                                    rust_constructor::advance_front::BackgroundType::CustomRect(
                                        rust_constructor::basic_front::CustomRectConfig::default(),
                                    ),
                                    [4_f32, 4_f32],
                                    5_f32,
                                ),
                            )
                            .movable(true, true)
                            .background(
                                &rust_constructor::advance_front::BackgroundType::CustomRect(
                                    rust_constructor::basic_front::CustomRectConfig::default()
                                        .position_size_config(Some(
                                            rust_constructor::PositionSizeConfig::default()
                                                .origin_position(300_f32, 300_f32)
                                                .origin_size(300_f32, 300_f32),
                                        ))
                                        .color(Some([255, 24, 47])),
                                ),
                            ),
                        None,
                        ui,
                    )
                    .unwrap();
            }
            _ => {}
        };
    }
}
```
This section of code is too complex, so we won't provide additional explanation here.

The running effect is shown below:

![Resource Panel](../images/resource_panel.gif)
