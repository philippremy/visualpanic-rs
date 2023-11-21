//! # VisualPanic
//!
//! Visualize panics with native GUI dialogs on supported systems.
//!
//! Provides a solution to panic visually, useful for GUI applications where a console view might not be available at all times. Customizable in some ways, e.g., which icon, title and dialog level should be used.
//!
//! ## Example 1: Use the default settings and register for the whole application
//! ```rust
//! # use visualpanic_rs::VisualPanic;
//!
//! fn main() {
//!     VisualPanic::default().register_global();
//! }
//! ```
//!
//! ## Example 2: Use custom settings and register for the whole application
//! ```rust
//! # use visualpanic_rs::VisualPanic;
//! # use visualpanic_rs::VisualPanicLevel;
//!
//! fn main() {
//!     VisualPanic::new(
//!         Some("path/to/custom_icon.png"),
//!         Some("Custom Title"),
//!         Some(VisualPanicLevel::Info))
//!     .register_global();
//! }
//!
//! ```

#![allow(warnings, unused)]
#![feature(panic_info_message)]

/// An enum stating the possible dialog levels
#[derive(Clone)]
pub enum VisualPanicLevel {
    /// Error level
    Error,
    /// Warning level (
    Warning,
    /// Info level
    Info
}

/// The struct containing information on the current VisualPanic settings.
/// Because all fields are optional to set, each one is wrapped in an [`Option<T>`].
#[derive(Clone)]
pub struct VisualPanic {
    /// <div class="warning">Currently not implemented!</div>
    /// Option to set a custom icon to be used.
    /// Value must be set to a valid path, e.g.,
    /// ```rust
    /// Some(String::from("path/to/icon.png"));
    /// ```
    custom_icon: Option<String>,
    /// Option to set a custom title to be used.
    /// Value can be set to any UTF-8 compliant [`String`], e.g.,
    /// ```rust
    /// Some(String::from("Custom String"));
    /// ```
    custom_title: Option<String>,
    /// Option to set a custom dialog level.
    /// Value can be one option of [`VisualPanicLevel`], e.g.,
    /// ```rust
    /// # use visualpanic_rs::VisualPanicLevel;
    /// Some(VisualPanicLevel::Error);
    /// ```
    custom_level: Option<VisualPanicLevel>,
}

/// Provide public methods for [`VisualPanic`].
impl VisualPanic {
    /// Implements a default struct with all fields set to [`None`].
    pub fn default() -> Self {
        return VisualPanic {
            custom_icon: None,
            custom_title: None,
            custom_level: None,
        }
    }

    /// Implements a new struct with custom options.
    /// The icon, title and level of the dialog can be set using [`Option<T>`], e.g.,
    /// ```rust
    /// # use visualpanic_rs::{VisualPanic, VisualPanicLevel};
    /// let visual_panic_options: VisualPanic = VisualPanic::new(
    ///     Some("path/to/custom_icon.png"),
    ///     Some("Custom Title"),
    ///     Some(VisualPanicLevel::Info)
    /// );
    /// ```
    pub fn new(custom_icon: Option<&str>, custom_title: Option<&str>, custom_level: Option<VisualPanicLevel>) -> Self {
        let mut return_val = VisualPanic{
            custom_icon: None,
            custom_title: None,
            custom_level: None,
        };
        if let Some(icon_str) = custom_icon {
            return_val.custom_icon = Some(String::from(icon_str));
        }
        if let Some(title_str) = custom_title {
            return_val.custom_title = Some(String::from(title_str));
        }
        if let Some(custom_level) = custom_level {
            return_val.custom_level = Some(custom_level);
        }
        return return_val;
    }

    /// Registers a [`VisualPanic`] globally, i.e., for the whole application.
    /// Returns currently nothing.
    /// Will panic, if handling the &[`PanicInfo`] fails in any way or the native message dialog can not be spawned.
    pub fn register_global(self) {

        let clone = self.clone();

        std::panic::set_hook(Box::new(move |panic_info| {
            let mut level: native_dialog::MessageType = native_dialog::MessageType::Error;
            let mut icon: Option<String> = None;
            let mut title = String::from(env!("CARGO_PKG_NAME"));
            let payload_as_str = panic_info.payload().downcast_ref::<&str>().expect("Failed to extract Panic Payload to &str.");
            let location = panic_info.location().expect("Failed to get location where the panic occured.");
            let message = panic_info.message().expect("Failed to extract message.").to_string();
            let location_str = format!("File: {} at Line: {}, Column: {}", location.file(), location.line(), location.column());
            let display_message = format!("An error occured.\n({})\n\nPayload: {}\n\nMessage: {}\n\nThe program will terminate.\n", location_str, payload_as_str, message);

            match &clone.custom_title {
                None => {},
                Some(custom) => { title = custom.clone() }
            }
            match &clone.custom_level {
                None => {},
                Some(custom) => {
                    match custom {
                        VisualPanicLevel::Error => { level = native_dialog::MessageType::Error }
                        VisualPanicLevel::Warning => { level = native_dialog::MessageType::Warning }
                        VisualPanicLevel::Info => { level = native_dialog::MessageType::Info }
                    }
                }
            }
            match &clone.custom_icon {
                None => {}
                Some(custom) => { icon = Some(custom.clone()) }
            }

            let _ = native_dialog::MessageDialog::new()
                .set_title(&title)
                .set_type(level)
                .set_text(&display_message)
                .show_alert()
                .expect("Failed to launch VisualPanic Dialog.");
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_error() {
        VisualPanic::new(None, Some("VisualPanic v1.0"), Some(VisualPanicLevel::Warning)).register_global();
        let num1: i32 = 2;
        num1.checked_div(0).unwrap();
    }
}
