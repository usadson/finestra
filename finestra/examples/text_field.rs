// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.change_label.set("Changed 0 times");

        Stack::vertical()
            .with({
                Stack::horizontal()
                    .with(Label::new("Input: "))
                    .with(TextField::new(state.value.clone())
                        .with_placeholder("Placeholder")
                        .with_on_change(|state: &mut AppState, _, _| {
                            state.change_count += 1;
                            state.change_label.set(format!("Changed {} times", state.change_count))
                        }))
            })
            .with({
                Stack::horizontal()
                   .with(Label::new("Value:"))
                   .with(Label::new(state.value.clone()))
            })
            .with(Label::new(state.change_label.clone()))
    }
}

#[derive(Debug, Default)]
struct AppState {
    value: TextValue,
    change_count: usize,
    change_label: TextValue,
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
