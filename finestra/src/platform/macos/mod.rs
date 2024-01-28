// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod dynamic_wrapper;
pub(crate) mod resources;

use std::cell::RefCell;
use std::rc::Rc;

use cacao::appkit::{App as CacaoApp, AppDelegate as CacaoAppDelegate};
use cacao::appkit::window::{Window as CacaoWindow, WindowDelegate};
use cacao::layout::LayoutConstraint;

use crate::{App, AppDelegate, View};
pub(crate) use self::dynamic_wrapper::DynamicViewWrapper;

const BUNDLE_ID: &str = "com.tristangerritsen.finestra";

pub fn run_app<Delegate>(app: App<Delegate>) -> !
        where Delegate: AppDelegate + 'static {

    let delegate = Rc::new(RefCell::new(app.delegate));
    let macos_delegate = MacOSDelegate {
        delegate: Rc::clone(&delegate),
        window: CacaoWindow::with(Default::default(), MacOSWindowDelegate {
            delegate: Rc::clone(&delegate),
            window: None,
            view: Default::default(),
            content: Default::default(),
            // main_content: Default::default(),
        }),
    };

    let cacao_app = CacaoApp::new(BUNDLE_ID, macos_delegate);
    cacao_app.run();

    std::process::exit(0);
}

struct MacOSDelegate<Delegate> {
    delegate: Rc<RefCell<Delegate>>,
    window: CacaoWindow<MacOSWindowDelegate<Delegate>>,
}

impl<Delegate> CacaoAppDelegate for MacOSDelegate<Delegate>
        where Delegate: AppDelegate {
    fn did_finish_launching(&self) {
        CacaoApp::activate();

        self.delegate.borrow_mut().did_launch();

        let config = self.delegate.borrow_mut().configure_main_window();
        self.window.set_title(config.title.as_str());

        if config.width != 0.0 && config.height != 0.0 {
            self.window.set_content_size(config.width, config.height);
        }

        self.delegate.borrow_mut().will_show_window();
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

struct MacOSWindowDelegate<Delegate> {
    delegate: Rc<RefCell<Delegate>>,
    window: Option<CacaoWindow>,
    view: cacao::view::View,
    content: Option<DynamicViewWrapper>,
}

impl<Delegate> WindowDelegate for MacOSWindowDelegate<Delegate>
        where Delegate: AppDelegate {
    const NAME: &'static str = "finestra";

    fn did_load(&mut self, window: CacaoWindow) {
        debug_assert!(self.window.is_none());

        let mut delegate = self.delegate.borrow_mut();

        let content_view = delegate.make_content_view();
        let content_view = content_view.build_native();

        content_view.add_to_view(&self.view);

        window.set_content_view(&self.view);

        LayoutConstraint::activate(&[
            content_view.layout_constraint_center_x().constraint_equal_to(&self.view.center_x),
            content_view.layout_constraint_center_y().constraint_equal_to(&self.view.center_y),
        ]);

        self.content = Some(content_view);
        self.window = Some(window);
    }
}
