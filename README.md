# VisualPanic (visualpanic-rs)

[![Crates.io](https://img.shields.io/crates/v/visualpanic_rs)](https://crates.io/crates/visualpanic_rs)
[![License](https://img.shields.io/crates/l/visualpanic_rs)](LICENSE)

A library providing a panic hook for Rust applications that visualizes the panic with a native error dialog on supported systems (see listed OSes at [native-dialog](https://crates.io/crates/native-dialog)).

## Installation

```
cargo add visualpanic_rs
```

## Example 1: Use the default settings and register for the whole application
```rust
use visualpanic_rs::VisualPanic;
fn main() {
     VisualPanic::default().register_global();
}
```

## Example 2: Use custom settings and register for the whole application
```rust
use visualpanic_rs::VisualPanic;
use visualpanic_rs::VisualPanicLevel;
fn main() {
     VisualPanic::new(
         Some("path/to/custom_icon.png"),
         Some("Custom Title"),
         Some(VisualPanicLevel::Info))
     .register_global();
}
```
