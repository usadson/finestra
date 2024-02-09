// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::cell::RefCell;

use crate::{App, UIBackend};

thread_local! {
    pub static CURRENT: RefCell<AppContext> = RefCell::new(AppContext::default());
}

#[derive(Default)]
pub struct AppContext {
    backend: UIBackend,
}

impl AppContext {
    pub fn initialize<Delegate, State>(app: &App<Delegate, State>) {
        Self::with_current(|ctx| {
            ctx.backend = app.backend;
        });
    }

    pub fn with_current<F: FnOnce(&mut AppContext) -> R, R>(f: F) -> R {
        CURRENT.with_borrow_mut(f)
    }

    #[must_use]
    pub fn backend() -> UIBackend {
        Self::with_current(|ctx| ctx.backend)
    }
}
