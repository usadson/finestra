// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Size2D;

use crate::{Menu, MenuBar, Number, StateOrRaw, Theme};

/// Use this to configure the look and feel of the Window.
#[derive(Default)]
pub struct WindowConfiguration {
    pub(crate) title: TitleWrapper,
    pub(crate) width: Number,
    pub(crate) height: Number,
    pub(crate) theme: StateOrRaw<Theme>,
    pub(crate) menubar: MenuBar,
}

impl WindowConfiguration {
    /// Use this to configure the look and feel of the Window.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Change the initial size of the window.
    #[must_use]
    pub fn with_size<T>(self, size: Size2D<Number, T>) -> Self {
        Self {
            width: size.width,
            height: size.height,
            ..self
        }
    }

    /// Set the title of the window. You can also use a [`State`](crate::State)
    /// to update it automatically, when you change it in your AppState.
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

    /// A menu bar is a bar containing [Menus][Menu]. Use this function to set
    /// the menu, and react to the user invoking items using
    /// [`AppDelegate::did_invoke_menu_action()`](crate::AppDelegate::did_invoke_menu_action).
    #[must_use]
    pub fn with_menubar(self, menubar: impl Into<MenuBar>) -> Self {
        Self {
            menubar: menubar.into(),
            ..self
        }
    }

    /// Use this function to add a new menu, and react to the user invoking
    /// items using
    /// [`AppDelegate::did_invoke_menu_action()`](crate::AppDelegate::did_invoke_menu_action).
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
