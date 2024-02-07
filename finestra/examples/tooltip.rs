// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self> {
        Label::new("Hello, tooltip!")
            .with_tooltip("Greeting")
    }
}

fn main() {
    App::new(Application)
        .run();
}
