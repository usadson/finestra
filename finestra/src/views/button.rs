// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::View;

/// A [`View`] that displays text and is clickable.
///
/// ```
/// # use finestra::Button;
/// let label = Button::new("Click Me");
/// ```
pub struct Button {
    text: String,
}

impl Button {
    /// Creates a new [`Button`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl View for Button {
    #[cfg(target_os = "macos")]
    fn build_native(&self) -> crate::platform::macos::DynamicViewWrapper {
        let button = cacao::button::Button::new(&self.text);
        button.into()
    }
}
