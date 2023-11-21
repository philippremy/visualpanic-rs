#![allow(warnings, unused)]
#![feature(panic_info_message)]

#[derive(Clone)]
pub enum VisualPanicLevel {
    Error,
    Warning,
    Info
}
#[derive(Clone)]
pub struct VisualPanic {
    custom_icon: Option<String>,
    custom_title: Option<String>,
    custom_level: Option<VisualPanicLevel>,
}

impl VisualPanic {
    pub fn default() -> Self {
        return VisualPanic {
            custom_icon: None,
            custom_title: None,
            custom_level: None,
        }
    }

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
