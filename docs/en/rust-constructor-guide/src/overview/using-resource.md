Resources are the foundation of all functionality. You can use resources through the following two approaches, though their underlying logic is the same.

# Add First, Use Later

This approach is older and is currently used mainly for the basic frontend resources within advanced frontend resources.

First, you need to use the `add_resource` method. This method accepts a resource name and resource content. No matter what resource you use, you can directly use this method to add the resource and use it later.

When creating resource content, you need to first find the resource struct you want to use, then create the resource content in the form of `xxx::default()`. If further modifications are needed, you can add methods like `.something(true)` after `default()`.

Important: never add resources directly inside loop code! Doing so will quickly cause duplicate resource name errors, leading to a program crash.

Typically, you can use the `check_resource_exists` method. It accepts only a `RustConstructorId`, and after checking, if the resource exists, it returns its index value; otherwise, it returns `None`. With this method, you can safely add resources inside loop code and avoid duplicate resource name errors.

There is another method based on page loading, which we will introduce later in [Adding Pages](./add-pages.md).

After adding a resource, you need to use the `use_resource` method to use the added resource. This method accepts a `RustConstructorId`, an `egui::Ui` for rendering content, and an optional `Box<dyn Config>` (this is advanced content, which I will explain later). Unlike adding resources, using resources must be placed inside loop code so that your resources always remain active.

# Quick Place

This approach is simpler and more straightforward. If you don't have special requirements, it's recommended to use this approach for working with resources.

You only need to use the `quick_place` method. What it accepts is almost identical to `use_resource`. The difference from `use_resource` is that `quick_place` automatically creates a resource when it doesn't exist and invokes it when it does, greatly reducing your workload.

Having methods to use resources alone isn't enough. Next, we need to understand how to make resources update automatically.
