// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod dynamic_wrapper;
pub(crate) mod resources;
pub(crate) mod state;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::{App as CacaoApp, AppDelegate as CacaoAppDelegate};
use cacao::appkit::window::{Window as CacaoWindow, WindowDelegate};
use cacao::layout::LayoutConstraint;
use cacao::notification_center::Dispatcher;

use crate::event::EventHandlerMapRegistry;
use crate::platform::macos::state::ViewTree;
use crate::{App, AppDelegate, View};
pub(crate) use self::dynamic_wrapper::DynamicViewWrapper;
use self::state::Event;


const BUNDLE_ID: &str = "com.tristangerritsen.finestra";

pub fn run_app<Delegate, State>(app: App<Delegate, State>) -> !
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {

    let event_registry = EventHandlerMapRegistry::<State>::default();
    let delegate = Rc::new(RefCell::new(app.delegate));
    let state = Arc::new(Mutex::new(app.state));
    let macos_delegate = MacOSDelegate {
        delegate: Rc::clone(&delegate),
        window: CacaoWindow::with(Default::default(), MacOSWindowDelegate {
            delegate: Rc::clone(&delegate),
            window: None,
            view: Default::default(),
            content: Default::default(),
            event_registry: event_registry.clone(),
            state: Arc::clone(&state)
        }),
        event_registry,
        state,
    };

    let cacao_app = CacaoApp::new(BUNDLE_ID, macos_delegate);
    cacao_app.run();

    std::process::exit(0);
}

pub(crate) struct MacOSDelegate<Delegate, State> {
    delegate: Rc<RefCell<Delegate>>,
    window: CacaoWindow<MacOSWindowDelegate<Delegate, State>>,
    event_registry: EventHandlerMapRegistry<State>,
    state: Arc<Mutex<State>>,
}

impl<Delegate, State> CacaoAppDelegate for MacOSDelegate<Delegate, State>
        where Delegate: AppDelegate<State> {
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

impl<Delegate, State> Dispatcher for MacOSDelegate<Delegate, State>
        where Delegate: AppDelegate<State> {
    type Message = Event;

    fn on_ui_message(&self, message: Event) {
        let mut state = self.state.lock().unwrap();

        match message {
            Event::ButtonClicked(view_id) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.click else {
                    return;
                };

                (handler)(&mut state);
            }
        }
    }
}

struct MacOSWindowDelegate<Delegate, State> {
    delegate: Rc<RefCell<Delegate>>,
    window: Option<CacaoWindow>,
    view: cacao::view::View,
    content: Option<DynamicViewWrapper>,
    event_registry: EventHandlerMapRegistry<State>,
    state: Arc<Mutex<State>>,
}

impl<Delegate, State> WindowDelegate for MacOSWindowDelegate<Delegate, State>
        where Delegate: AppDelegate<State> {
    const NAME: &'static str = "finestra";

    fn did_load(&mut self, window: CacaoWindow) {
        debug_assert!(self.window.is_none());

        let mut delegate = self.delegate.borrow_mut();

        let mut state = self.state.lock().unwrap();
        let mut content_view = delegate.make_content_view(&mut state);

        let mut tree = ViewTree::new(self.event_registry.clone());
        let content_view = content_view.build_native(&mut tree);

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
