// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self>  {
        Button::new("Surprise Me")
            .with_on_click(|_, window| {
                Timer::delayed_action(Duration::from_secs(1), move || {
                    window.create_dialog("Hello, world!")
                        .show();
                }).schedule();
            })
    }
}

fn main() {
    App::new(Application)
        .run();
}
