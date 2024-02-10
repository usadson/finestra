// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn configure_main_window(&mut self, state: &mut AppState) -> WindowConfiguration {
        WindowConfiguration::default()
            .with_theme(&state.theme)
            .with_title(&state.theme_name)
    }

    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.theme_name.set("Theme: Automatic");

        Stack::vertical()
            .with(Label::new(&state.theme_name))
            .with({
                Stack::horizontal()
                    .with(Button::new("Auto")
                        .with_on_click(|state: &mut AppState, _| {
                            state.theme.set(Theme::Automatic);
                            state.theme_name.set("Theme: Automatic");
                        }))
                    .with(Button::new("Light")
                        .with_on_click(|state: &mut AppState, _| {
                            state.theme.set(Theme::Light);
                            state.theme_name.set("Theme: Light");
                        }))
                    .with(Button::new("Dark")
                        .with_on_click(|state: &mut AppState, _| {
                            state.theme.set(Theme::Dark);
                            state.theme_name.set("Theme:  Dark");
                        }))
            })
    }
}

#[derive(Debug, Default)]
struct AppState {
    theme: ThemeValue,
    theme_name: TextValue,
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
