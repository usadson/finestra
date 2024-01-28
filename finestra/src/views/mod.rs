// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod button;
mod label;

pub use self::button::Button;
pub use self::label::Label;

/// A generic graphical component.
///
/// Common components are:
/// 1. [`Label`] can be used to display text.
pub trait View {
    /// Internal API: creates a native view (for macOS).
    #[cfg(target_os = "macos")]
    fn build_native(&self) -> crate::platform::macos::DynamicViewWrapper;
}

impl View for () {
    #[cfg(target_os = "macos")]
    fn build_native(&self) -> crate::platform::macos::DynamicViewWrapper {
        cacao::text::Label::new().into()
    }
}
