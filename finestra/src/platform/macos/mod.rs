// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
mod appkit;
pub mod cacao_delegates;
mod dynamic_wrapper;
mod extensions;
mod resources;
pub(crate) mod state;
mod window;

pub(crate) use self::app::MacOSDelegate;
pub(crate) use self::appkit::*;
pub(crate) use self::extensions::*;
pub(crate) use self::state::Event;
pub(crate) use self::resources::ToCacao;
pub(crate) use self::dynamic_wrapper::DynamicViewWrapper;
pub(crate) use self::dynamic_wrapper::LayoutExt;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::App as CacaoApp;
use cacao::appkit::window::Window as CacaoWindow;

use crate::{App, AppDelegate, EventHandlerMapRegistry};
use self::window::MacOSWindowDelegate;

const BUNDLE_ID: &str = "com.tristangerritsen.finestra";

pub(crate) fn run_app<Delegate, State>(app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    let event_registry = EventHandlerMapRegistry::<State>::default();
    let delegate = Rc::new(RefCell::new(app.delegate));
    let state = Arc::new(Mutex::new(app.state));
    let macos_delegate = MacOSDelegate {
        delegate: Rc::clone(&delegate),
        window: Rc::new(CacaoWindow::with(Default::default(), MacOSWindowDelegate::new(Rc::clone(&delegate), event_registry.clone(), Arc::clone(&state)))),
        event_registry,
        state,
    };

    let cacao_app = CacaoApp::new(BUNDLE_ID, macos_delegate);
    cacao_app.run();

    std::process::exit(0);
}
