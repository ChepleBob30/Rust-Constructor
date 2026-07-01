Backend resources are usually only called by advanced frontend resources, but understanding them can help your development too.

# Page Data

This section has already been mentioned [earlier](../overview/add-pages.md).

# Variable

`Variable` is a backend resource for storing global data. Typically, you can use it like this:

## Code Example

```rust
self.inner
    .add_resource(
        "Example",
        rust_constructor::background::Variable::default().value(Some(1_i32)),
    )
    .unwrap();
println!("{:?}", self.inner.get_variable::<i32>("Example"));
```

This code creates a variable storing `1`, retrieves it via `get_variable`, and produces the following output:

```shell
Ok(Some(1))
```

# SplitTime

`SplitTime` is `RustConstructor`'s time marker. It can be used to control animation execution and perform timing.

If necessary, you can also reset the time marker via `reset_split_time`.

Note that this resource requires page updates to function properly.

## Code Example

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
This code also involves another key field, `timer`, which is `Rust Constructor`'s timer containing three fields: page entry time, total running time, and current page running time. Generally, to calculate the time from when the SplitTime was created to the present, simply subtract the SplitTime from the current page running time.

The output might look like this:

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

In this array, the first entry is the page running time, and the second is the total running time. In most cases, you only need the first one.
