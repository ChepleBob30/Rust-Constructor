后端资源通常只需要由高级前端资源自行调用，但了解一下对你的开发也有帮助。

# 页面数据

此部分已在[前文](../overview/add-pages.md)提及。

# 变量

`Variable`是一个用于存储全局数据的后端资源，通常情况下，你可以这么使用：

## 代码示例

```rust
self.inner
    .add_resource(
        "Example",
        rust_constructor::background::Variable::default().value(Some(1_i32)),
    )
    .unwrap();
println!("{:?}", self.inner.get_variable::<i32>("Example"));
```

这段代码创建了一个存储`1`的变量，并通过`get_variable`取出变量，输出如下：

```shell
Ok(Some(1))
```

# 时间分段

`SplitTime`是`RustConstructor`的时间标签，它可以用于控制动画运行，并进行计时。

如果必要，你还可以通过`reset_split_time`对时间分段进行重置。

请注意，该资源需要在页面更新的情况下才能正常使用。

## 代码示例

```rust
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
                    "Example",
                    rust_constructor::background::SplitTime::default(),
                )
                .unwrap();
        };
        match &*self.inner.current_page.clone() {
            "Launch" => {
                self.inner
                    .use_resource(&rust_constructor::build_id("Launch", "PageData"), None, ui)
                    .unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("{:?}", self.inner.get_split_time("Example").unwrap());
                self.inner.reset_split_time("Example").unwrap();
                println!(
                    "{:?}",
                    self.inner.timer.now_time - self.inner.get_split_time("Example").unwrap()[0]
                );
            }
            _ => {}
        };
```
这段代码中还涉及了另一个关键字段`timer`，它是`Rust Constructor`的计时器，包含了进入页面时间，总运行时间，当前页面运行时间三个字段。一般来说，如果想要计算从时间分段创建到目前的时间，只需用当前页面运行时间减去时间分段即可。

输出结果可能是这样的：

```shell
[0, 0]
0
[27, 27]
0
[1064, 1064]
0
[2141, 2141]
0
[3160, 3160]
0
[4175, 4175]
0
[5193, 5193]
0
```

在这个数组中，第一个项目为页面运行时间，第二个项目为总运行时间，通常情况下只需使用第一个即可。
