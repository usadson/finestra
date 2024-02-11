// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Default,

    Left,
    Center,
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
