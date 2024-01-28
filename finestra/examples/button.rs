// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::{App, AppDelegate, Button};

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self) -> impl finestra::View {
        Button::new("Click Me")
    }
}

fn main() {
    App::new(Application::default())
        .run();
}
