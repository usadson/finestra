// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn configure_main_window(&mut self, _: &mut ()) -> WindowConfiguration {
        WindowConfiguration::new()
            .with_menu(Menu::new("Greeting")
                .with_item(MenuItem::titled("Hello"))
                .with_item(MenuItem::titled("Goodbye"))
            )
    }

    fn did_invoke_menu_action(&mut self, item: MenuItem, _: &mut (), window: Window) {
        window.create_dialog(item.title().unwrap_or("INVALID").to_string())
            .kind(DialogKind::Informational)
            .show();
    }

    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self> {
        TextBlock::new("Look at my menu!")
            .with_color(Color::system(SystemColor::Teal))
            .centered()
    }
}

fn main() {
    App::new(Application)
        .run();
}
