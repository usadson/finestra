// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::View;

/// A [`View`] that displays text.
///
/// ```
/// # use finestra::Label;
/// let label = Label::new("This is some text");
/// let label = Label::new(format!("What is 2 + 2? Correct, {}!", 2 + 2));
/// ```
pub struct Label {
    text: String,
}

impl Label {
    /// Creates a new [`Label`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl View for Label {
    #[cfg(target_os = "macos")]
    fn build_native(&self) -> crate::platform::macos::DynamicViewWrapper {
        let label = cacao::text::Label::new();
        label.set_text(&self.text);
        label.set_font(&cacao::text::Font::system(30.));
        label.set_text_color(cacao::color::Color::SystemRed);
        label.into()
    }
}
