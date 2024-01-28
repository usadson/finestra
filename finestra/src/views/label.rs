// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{Color, View};

/// A [`View`] that displays text.
///
/// ```
/// # use finestra::Label;
/// let label = Label::new("This is some text");
/// let label = Label::new(format!("What is 2 + 2? Correct, {}!", 2 + 2));
/// ```
pub struct Label {
    text: String,
    text_color: Color,
    background_color: Color,
}

impl Label {
    /// Creates a new [`Label`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            text_color: Color::default(),
            background_color: Color::default(),
        }
    }

    /// Returns `Self` with the given `color`.
    #[must_use]
    pub fn with_color(self, color: Color) -> Self {
        Self {
            text_color: color,
            ..self
        }
    }

    /// Returns `Self` with the given background `color`.
    #[must_use]
    pub fn with_background_color(self, color: Color) -> Self {
        Self {
            background_color: color,
            ..self
        }
    }

    #[must_use]
    #[inline]
    pub const fn color(&self) -> &Color {
        &self.text_color
    }

    /// Sets the color of the [`Label`]. Use [`Label::with_color()`] to avoid
    /// making a `mut` variable.
    pub fn set_color(&mut self, color: Color) {
        self.text_color = color;
    }

    /// Returns the background color.
    #[must_use]
    #[inline]
    pub const fn background_color(&self) -> &Color {
        &self.background_color
    }

    /// Sets the background color of the [`Label`]. Use
    /// [`Label::with_background_color()`] to avoid making a `mut` variable.
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}

impl View for Label {
    #[cfg(target_os = "macos")]
    fn build_native(&self) -> crate::platform::macos::DynamicViewWrapper {
        use crate::platform::macos::resources::ToCacao;

        let label = cacao::text::Label::new();
        label.set_text(&self.text);
        label.set_font(&cacao::text::Font::system(30.));

        if let Some(color) = self.color().to_cacao() {
            label.set_text_color(color);
        }

        if let Some(color) = self.background_color().to_cacao() {
            label.set_background_color(color);
        }

        label.into()
    }
}
