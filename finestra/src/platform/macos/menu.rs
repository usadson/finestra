// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::marker::PhantomData;

use cacao::appkit::App;

use cacao::appkit::menu::{
    Menu as CacaoMenu,
    MenuItem as CacaoMenuItem,
};

use crate::{AppDelegate, Menu, MenuBar, MenuItem, MenuItemKind};

use super::{state::Event, MacOSDelegate};

pub(super) fn set_menu_bar<Delegate, State>(mut menubar: MenuBar)
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    Wrapper::<Delegate, State>::fill_appropriate_menus(&mut menubar);
    let menubar = Wrapper::<Delegate, State>::convert_menubar(menubar);
    App::set_menu(menubar);
}

struct Wrapper<Delegate, State> {
    _delegate: PhantomData<Delegate>,
    _state: PhantomData<State>,
}

impl<Delegate, State> Wrapper<Delegate, State>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn fill_appropriate_menus(menubar: &mut MenuBar) {
        _ = menubar.ensure_menu_at("", 0);

        let menu = menubar.ensure_menu_at("File", 1);
        if menu.items.is_empty() {
            menu.items.push(MenuItem::titled("Close Window"));
        }

        let menu = menubar.ensure_menu_at("Edit", 2);
        if menu.items.is_empty() {
            menu.add_item("Undo");
            menu.add_item("Redo");
            menu.add_item(MenuItem::separator());
            menu.add_item("Cut");
            menu.add_item("Copy");
            menu.add_item("Paste");
            menu.add_item(MenuItem::separator());
            menu.add_item("Select All");
        }

        if menubar.menu_opt("Selection").is_some() {
            _ = menubar.ensure_menu_at("Selection", 3);
        }

        if menubar.menu_opt("Help").is_some() {
            _ = menubar.ensure_menu_at("Help", menubar.menus.len() - 1);
        }
    }

    pub(super) fn convert_menubar(menubar: MenuBar) -> Vec<CacaoMenu> {
        menubar.menus
            .into_iter()
            .map(Self::convert_menu)
            .collect()
    }

    fn convert_menu(menu: Menu) -> CacaoMenu {
        if menu.name.is_empty() {
            return CacaoMenu::standard().into_iter().next().unwrap();
        }

        let items = menu.items
            .into_iter()
            .map(Self::convert_menu_item)
            .collect();

        CacaoMenu::new(&menu.name, items)
    }

    fn convert_menu_item(item: MenuItem) -> CacaoMenuItem {
        match item.kind {
            MenuItemKind::Separator => CacaoMenuItem::Separator,

            MenuItemKind::Titled(name) => match name.as_str() {
                "Close Window" => CacaoMenuItem::CloseWindow,
                "Copy" => CacaoMenuItem::Copy,
                "Cut" => CacaoMenuItem::Cut,
                "Enter FullScreen" => CacaoMenuItem::EnterFullScreen,
                "Hide" => CacaoMenuItem::Hide,
                "Hide Others" => CacaoMenuItem::HideOthers,
                "Minimize" => CacaoMenuItem::Minimize,
                "Paste" => CacaoMenuItem::Paste,
                "Quit" => CacaoMenuItem::Quit,
                "Redo" => CacaoMenuItem::Redo,
                "Select All" => CacaoMenuItem::SelectAll,
                // "Services" => CacaoMenuItem::Services,
                "Show All" => CacaoMenuItem::ShowAll,
                "Toggle Sidebar" => CacaoMenuItem::ToggleSidebar,
                "Undo" => CacaoMenuItem::Undo,
                "Zoom" => CacaoMenuItem::Zoom,

                _ => {
                    let item = MenuItem::titled(name.clone());
                    CacaoMenuItem::new(name)
                        .action(move || {
                            App::<MacOSDelegate<Delegate, State>, Event>::dispatch_main(Event::MenuAction {
                                item: item.clone(),
                            });
                        })
                }
            }
        }
    }
}
