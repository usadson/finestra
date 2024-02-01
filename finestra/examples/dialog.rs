// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self>  {
        Button::new("Show Message")
            .with_on_click(|_, window: Window| {
                window.create_dialog("You are awesome :)")
                        .title("Good Job!")
                        .kind(DialogKind::Warning)
                        .show();
            })
    }
}

fn main() {
    App::new(Application::default())
        .run();
}
