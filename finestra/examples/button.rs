// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.label.set("Clicked: 0");

        Button::new(&state.label)
            .with_on_click(|state: &mut AppState, _| {
                state.count += 1;
                state.label.set(format!("Clicked: {}", state.count));
            })
    }
}

#[derive(Debug, Default)]
struct AppState {
    count: usize,
    label: TextValue,
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
