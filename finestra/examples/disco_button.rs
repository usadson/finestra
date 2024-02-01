// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

const COLORS: &[Color] = &[
    Color::system(SystemColor::Red),
    Color::system(SystemColor::Green),
    Color::system(SystemColor::Blue),
    Color::system(SystemColor::Yellow),
    Color::system(SystemColor::Indigo),
    Color::system(SystemColor::Orange),
    Color::system(SystemColor::Teal),
];

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.color.set(COLORS[0].clone());

        Button::new("Click to Change")
            .with_on_click(|state: &mut AppState, _| {
                state.count += 1;
                state.color.set(COLORS[state.count % COLORS.len()].clone());
            })
            .with_background_color(state.color.clone())
    }
}

#[derive(Debug, Default)]
struct AppState {
    count: usize,
    color: ColorValue,
}

fn main() {
    App::with_state(Application::default(), AppState::default())
        .run();
}
