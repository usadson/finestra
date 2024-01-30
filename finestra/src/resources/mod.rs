// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// A platform-independent color type.
///
/// ### System Colors
/// Each platform has its own defaults, which can be applied using the
/// [`SystemColor`] enumeration. To create a [`Color`], simply call
/// [`Into::into`] or [`Color::system()`].
/// ```
/// # use finestra::Color;
/// # use finestra::SystemColor;
/// let label_color = Color::system(SystemColor::Label);
/// ```
///
/// ### Usage
/// These colors can be set on [`Views`](crate::View) using their respective
/// methods. For example:
/// ```
/// # // This is usually used in a context where the `State` generic parameter
/// # // is inferred by the compiler.
/// # type Label = finestra::Label<()>;
/// # use finestra::Color;
/// # use finestra::SystemColor;
/// let label = Label::new("Hello, Red World!")
///         .with_color(Color::system(SystemColor::Red));
/// ```
///
/// ### RGB
/// You can also create raw RGB and RGBA colors using:
/// ```
/// # use finestra::Color;
/// let blue = Color::rgb(0x00, 0x00, 0xFF);
/// let translucent_red = Color::rgba(255, 0, 0, 127);
/// ```
#[derive(Clone, Debug, Default)]
pub struct Color {
    kind: ColorKind,
}

impl Color {
    /// Create a new color, based on platform-dependent
    /// [`SystemColors`](SystemColor).
    #[must_use]
    pub const fn system(color: SystemColor) -> Self {
        Self {
            kind: ColorKind::System(color),
        }
    }

    /// Create a new color from 8-bit RGB colors.
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            kind: ColorKind::Rgb { red, green, blue }
        }
    }

    /// Create a new color from 8-bit RGBA colors.
    #[must_use]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            kind: ColorKind::Rgba { red, green, blue, alpha }
        }
    }

    /// Create a fully transparent color.
    #[must_use]
    pub const fn transparent() -> Self {
        Self {
            kind: ColorKind::Transparent,
        }
    }

    #[inline]
    #[must_use]
    /// Returns the associated [`ColorKind`].
    pub(crate) const fn kind(&self) -> &ColorKind {
        &self.kind
    }
}

/// A platform-dependent color type. As each platform has its own default
/// colors, this enumeration can be used to conform to the UI Guidelines
/// of the respective platform.
///
/// ### Source
/// These values are sourced on the following platforms, using the following
/// methods:
/// - macOS using [NSColor UI Element Colors](https://developer.apple.com/documentation/appkit/nscolor/ui_element_colors?language=objc)
///   and [NSColor Standard Colors](https://developer.apple.com/documentation/appkit/nscolor/standard_colors?language=objc).
/// - Windows using [GetSysColor (winuser.h)](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SystemColor {
    /// The default foreground color of a label.
    Label,

    /// The default background color.
    Background,

    /// The default foreground color of a link.
    Link,

    Black,
    White,
    Brown,
    Blue,
    Green,
    Indigo,
    Orange,
    Pink,
    Purple,
    Red,
    Teal,
    Yellow,
    Gray,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum ColorKind {
    /// A context-dependent default color.
    #[default]
    Default,
    Transparent,

    System(SystemColor),

    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },

    Rgba {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
}

impl From<ColorKind> for Color {
    fn from(value: ColorKind) -> Self {
        Color {
            kind: value,
        }
    }
}

impl From<SystemColor> for ColorKind {
    fn from(value: SystemColor) -> Self {
        ColorKind::System(value)
    }
}

impl From<SystemColor> for Color {
    fn from(value: SystemColor) -> Self {
        Color::system(value)
    }
}
