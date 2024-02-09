// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self>  {
        Stack::horizontal()
            .with(Button::new("Crosshair")
                .with_on_click(|_, _| {
                    Cursor::system(SystemCursor::CrossHair)
                        .show_for(Duration::from_secs(2));
                }))
            .with(Button::new("Not Allowed")
                .with_on_click(|_, _| {
                    Cursor::system(SystemCursor::NotAllowed)
                        .show_for(Duration::from_secs(2));
                }))
            .with(Button::new("Unstable: Disappearing Item")
                .with_on_click(|_, _| {
                    Cursor::unstable(UnstableCursor::DisappearingItem, SystemCursor::Arrow)
                        .show_for(Duration::from_secs(2));
                }))
    }
}

fn main() {
    App::new(Application)
        .run();
}
