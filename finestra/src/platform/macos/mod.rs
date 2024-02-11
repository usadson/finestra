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

use cacao::appkit::App as CacaoApp;

use crate::{App, AppDelegate};

const BUNDLE_ID: &str = "com.tristangerritsen.finestra";

pub(crate) fn run_app<Delegate, State>(app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    let delegate = MacOSDelegate::new(app.delegate, app.state);

    let cacao_app = CacaoApp::new(BUNDLE_ID, delegate);
    cacao_app.run();

    std::process::exit(0);
}
