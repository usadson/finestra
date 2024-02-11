// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
pub(crate) mod cacao_delegates;
mod dynamic_wrapper;
pub(crate) mod extensions;
pub(crate) mod objc;
pub(crate) mod resources;
mod menu;
pub(crate) mod state;
pub(crate) mod views;
pub(crate) mod window;

pub(crate) use self::app::MacOSDelegate;
pub(crate) use self::dynamic_wrapper::DynamicViewWrapper;
pub(crate) use self::dynamic_wrapper::LayoutExt;
pub(crate) use self::objc::*;
pub(crate) use self::views::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::App as CacaoApp;
use cacao::appkit::window::Window as CacaoWindow;

use crate::{App, AppDelegate, EventHandlerMapRegistry};
use self::window::MacOSWindowDelegate;

const BUNDLE_ID: &str = "com.tristangerritsen.finestra";

pub fn run_app<Delegate, State>(app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {

    let event_registry = EventHandlerMapRegistry::<State>::default();
    let delegate = Rc::new(RefCell::new(app.delegate));
    let state = Arc::new(Mutex::new(app.state));
    let macos_delegate = MacOSDelegate {
        delegate: Rc::clone(&delegate),
        window: Rc::new(CacaoWindow::with(Default::default(), MacOSWindowDelegate {
            delegate: Rc::clone(&delegate),
            delegator: None,
            window: None,
            view: Default::default(),
            content: Default::default(),
            event_registry: event_registry.clone(),
            state: Arc::clone(&state)
        })),
        event_registry,
        state,
    };

    let cacao_app = CacaoApp::new(BUNDLE_ID, macos_delegate);
    cacao_app.run();

    std::process::exit(0);
}
