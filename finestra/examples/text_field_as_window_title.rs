// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn configure_main_window(&mut self, state: &mut AppState) -> WindowConfiguration {
        state.title.set("My Application");

        WindowConfiguration::new()
            .with_title(state.title.clone())
    }

    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        Stack::horizontal()
            .with(Label::new("Choose a title:"))
            .with(TextField::new(state.title.clone())
                    .with_placeholder("Window title"))
    }
}

#[derive(Debug, Default)]
struct AppState {
    title: TextValue,
}

fn main() {
    App::with_state(Application::default(), AppState::default())
        .run();
}
