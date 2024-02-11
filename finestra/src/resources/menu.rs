// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// A menu bar is a bar containing [Menus][Menu].
///
/// ## Builder
/// You can construct a [`MenuBar`] using the builder pattern with the
/// [`MenuBar::with_menu()`] function.
/// ```
/// # use finestra::*;
/// MenuBar::new()
///     .with_menu(Menu::new("File")
///         .with_item(MenuItem::titled("Open"))
///         .with_item(MenuItem::titled("Save"))
///         .with_item(MenuItem::separator())
///         .with_item(MenuItem::titled("Quit"))
///     );
/// ```
///
/// ## Mutable
/// You can construct a [`MenuBar`] using the [`MenuBar::menu()`] function.
///
/// ```
/// # use finestra::*;
/// let mut bar = MenuBar::new();
/// bar.menu("File").add_item(MenuItem::titled("Open"));
/// bar.menu("File").add_item(MenuItem::titled("Save"));
/// bar.menu("File").add_item(MenuItem::separator());
/// bar.menu("File").add_item(MenuItem::titled("Quit"));
/// ```
#[derive(Debug)]
pub struct MenuBar {
    pub(crate) menus: Vec<Menu>,
}

impl MenuBar {
    /// A menu bar is a bar containing [Menus][Menu].
    pub fn new() -> Self {
        Self {
            menus: Default::default(),
        }
    }

    /// Get or create the menu by a certain name.
    pub fn menu(&mut self, name: impl Into<String>) -> &mut Menu {
        let name = name.into();

        // Work around the borrow checker :(
        let idx = self.menus.iter()
                .enumerate()
                .find(|(_, x)| x.name == name)
                .map(|(idx, _)| idx);

        if let Some(idx) = idx {
            &mut self.menus[idx]
        } else {
            self.menus.push(Menu::new(name));
            self.menus.last_mut().unwrap()
        }
    }

    pub(crate) fn menu_opt(&mut self, name: &str) -> Option<&mut Menu> {
        for menu in &mut self.menus {
            if menu.name == name {
                return Some(menu);
            }
        }

        None
    }

    pub(crate) fn ensure_menu_at(&mut self, name: impl Into<String>, pos: usize) -> &mut Menu {
        let name = name.into();

        let mut actual_pos = None;

        for (idx, menu) in self.menus.iter_mut().enumerate() {
            if menu.name == name {
                if idx == pos {
                    return &mut self.menus[pos];
                }

                actual_pos = Some(idx);
                break;
            }
        }

        let menu = if let Some(actual_pos) = actual_pos {
            self.menus.remove(actual_pos)
        } else {
            Menu::new(name)
        };

        self.menus.insert(pos, menu);
        &mut self.menus[pos]
    }

    /// Insert or merge a menu with a given name.
    pub fn add_menu(&mut self, menu: impl Into<Menu>) {
        let menu = menu.into();

        let existing = self.menu(menu.name.clone());

        for item in menu.items {
            if !existing.items.iter().any(|x| x == &item) {
                existing.items.push(item);
            }
        }
    }

    /// Insert or merge a menu with a given name.
    pub fn with_menu(mut self, menu: impl Into<Menu>) -> Self {
        self.add_menu(menu);
        self
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

/// A menu is a (drop-down) menu in an [`MenuBar`]. It contains zero or more
/// [`MenuItems`][MenuItem].
///
/// ## Builder
/// You can construct a [`Menu`] using the builder pattern with the
/// [`MenuBar::with_menu()`] function.
/// ```
/// # use finestra::*;
/// Menu::new("File")
///     .with_item(MenuItem::titled("Open"))
///     .with_item(MenuItem::titled("Save"))
///     .with_item(MenuItem::separator())
///     .with_item(MenuItem::titled("Quit"));
/// ```
///
/// ## Mutable
/// You can construct a [`MenuBar`] using the [`MenuBar::menu()`] function.
///
/// ```
/// # use finestra::*;
/// let mut menu = Menu::new("File");
/// menu.add_item(MenuItem::titled("Open"));
/// menu.add_item(MenuItem::titled("Save"));
/// menu.add_item(MenuItem::separator());
/// menu.add_item(MenuItem::titled("Quit"));
/// ```
#[derive(Debug)]
pub struct Menu {
    pub(crate) name: String,
    pub(crate) items: Vec<MenuItem>,
}

impl Menu {
    /// Create a new menu with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: Default::default(),
        }
    }

    /// Adds an item to the Menu. Use [`Self::with_item()`] to avoid making a
    /// mutable variable.
    pub fn add_item(&mut self, item: impl Into<MenuItem>) {
        self.items.push(item.into());
    }

    /// Adds an item to the Menu. Use [`Self::add_item()`] when using a mutable
    /// variable.
    pub fn with_item(mut self, item: impl Into<MenuItem>) -> Self {
        self.items.push(item.into());
        self
    }
}

/// An item in a [`Menu`], such as "Open" and "Save" in the "File" menu.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub(crate) kind: MenuItemKind,
}

impl MenuItem {
    /// Create a new item in a [`Menu`] with a given title.
    pub fn titled(name: impl Into<String>) -> Self {
        Self {
            kind: MenuItemKind::Titled(name.into()),
        }
    }

    /// Create a new separator item in a [`Menu`].
    pub fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
        }
    }

    /// Get the title of the item, and returns [`None`] if the item is a
    /// separator.
    pub fn title(&self) -> Option<&str> {
        if let MenuItemKind::Titled(name) = &self.kind {
            Some(name)
        } else {
            None
        }
    }
}

impl From<String> for MenuItem {
    fn from(value: String) -> Self {
        Self::titled(value)
    }
}

impl From<&str> for MenuItem {
    fn from(value: &str) -> Self {
        Self::titled(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MenuItemKind {
    Separator,
    Titled(String),
}
