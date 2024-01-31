// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[cfg(debug_assertions)]
pub(crate) mod debug;
pub(crate) mod dispatch;
pub(crate) mod view;
pub(crate) mod window;
pub(crate) mod wrapper;

use crate::{App, AppDelegate};

use self::window::Window;

pub fn run_app<Delegate, State>(mut app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {
    app.delegate.did_launch();

    let config = app.delegate.configure_main_window();

    let window = Window::new(config, app.delegate, app.state);

    window.show();
    window.update();

    dispatch::run_message_pump();
}
