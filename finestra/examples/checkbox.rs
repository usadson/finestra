// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.label.set("Value: false");

        Stack::vertical()
            .with(Label::new(&state.label))
            .with(Checkbox::new(&state.label)
                .with_on_checked(|state: &mut AppState, is_checked, _| {
                    state.label.set(format!("Value: {is_checked}"));
                }))
    }
}

#[derive(Debug, Default)]
struct AppState {
    label: TextValue,
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
