// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{ColorKind, SystemColor};

pub(crate) trait ToCacao<CacaoType> {
    fn to_cacao(&self) -> CacaoType;
}

impl ToCacao<Option<cacao::color::Color>> for crate::Color {
    fn to_cacao(&self) -> Option<cacao::color::Color> {
        self.clone().into()
    }
}

impl From<crate::Color> for Option<cacao::color::Color> {
    fn from(value: crate::Color) -> Self {
        use cacao::color::Color;

        match *value.kind() {
            ColorKind::Default => None,
            ColorKind::System(system) => Some(system.into()),
            ColorKind::Transparent => Some(Color::rgba(0, 0, 0, 0)),

            ColorKind::Rgb { red, green, blue } => {
                Some(Color::rgb(red, green, blue))
            }

            ColorKind::Rgba { red, green, blue, alpha } => {
                Some(Color::rgba(red, green, blue, alpha))
            }
        }
    }
}

impl From<SystemColor> for cacao::color::Color {
    fn from(value: SystemColor) -> Self {
        use cacao::color::Color;

        match value {
            SystemColor::Background => Color::SystemBackground,
            SystemColor::Label => Color::Label,
            SystemColor::Link => Color::Link,

            SystemColor::Black => Color::SystemBlack,
            SystemColor::Blue => Color::SystemBlue,
            SystemColor::Brown => Color::SystemBrown,
            SystemColor::Gray => Color::SystemGray,
            SystemColor::Green => Color::SystemGreen,
            SystemColor::Indigo => Color::SystemIndigo,
            SystemColor::Orange => Color::SystemOrange,
            SystemColor::Pink => Color::SystemPink,
            SystemColor::Purple => Color::SystemPurple,
            SystemColor::Red => Color::SystemRed,
            SystemColor::Teal => Color::SystemTeal,
            SystemColor::White => Color::SystemWhite,
            SystemColor::Yellow => Color::SystemYellow,
        }
    }
}
