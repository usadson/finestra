// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod dispatch;
mod window;
mod wrapper;

use crate::{App, AppDelegate};
use std::rc::Rc;

use self::window::Window;

pub fn run_app<Delegate, State>(mut app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {
    app.delegate.did_launch();

    let config = app.delegate.configure_main_window();
    let window = Window::new(config);
    let delegator = crate::Window::new(Rc::new(window.clone()));

    app.delegate.will_show_window(delegator);
    window.show();
    window.update();

    dispatch::run_message_pump();
}
