// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Size2D;

use crate::{Menu, MenuBar, Number, StateOrRaw, Theme};


#[derive(Default)]
pub struct WindowConfiguration {
    pub(crate) title: TitleWrapper,
    pub(crate) width: Number,
    pub(crate) height: Number,
    pub(crate) theme: StateOrRaw<Theme>,
    pub(crate) menubar: MenuBar,
}

impl WindowConfiguration {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_size<T>(self, size: Size2D<Number, T>) -> Self {
        Self {
            width: size.width,
            height: size.height,
            ..self
        }
    }

    #[must_use]
    pub fn with_title(self, title: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            title: TitleWrapper(title.into()),
            ..self
        }
    }

    /// Use the specified theme for the window.
    #[must_use]
    pub fn with_theme(self, theme: impl Into<StateOrRaw<Theme>>) -> Self {
        Self {
            theme: theme.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_menubar(self, menubar: impl Into<MenuBar>) -> Self {
        Self {
            menubar: menubar.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_menu(mut self, menu: impl Into<Menu>) -> Self {
        self.menubar.add_menu(menu);
        self
    }
}

/// A simple wrapper, just to allow us to add a [`Default`] value.
#[derive(Debug)]
pub(crate) struct TitleWrapper(StateOrRaw<String>);

impl Default for TitleWrapper {
    fn default() -> Self {
        Self("My Application".into())
    }
}

impl AsRef<StateOrRaw<String>> for TitleWrapper {
    fn as_ref(&self) -> &StateOrRaw<String> {
        &self.0
    }
}
