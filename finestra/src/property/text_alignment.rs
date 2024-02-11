// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// Set the alignment/justification of e.g. a [`TextBlock`](crate::TextBlock).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    /// The default alignment for the OS and language.
    #[default]
    Default,

    /// The lines should be aligned to the left side of the view.
    Left,

    /// The lines should be aligned to be in the center of the view.
    Center,

    /// The lines should be aligned to the right side of the view.
    Right,
}

#[cfg(target_os = "macos")]
impl crate::platform::macos::ToCacao<cacao::text::TextAlign> for TextAlignment {
    fn to_cacao(&self) -> cacao::text::TextAlign {
        match self {
            Self::Default => cacao::text::TextAlign::Natural,
            Self::Left => cacao::text::TextAlign::Left,
            Self::Center => cacao::text::TextAlign::Center,
            Self::Right => cacao::text::TextAlign::Right,
        }
    }
}
