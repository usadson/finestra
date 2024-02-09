// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod cacao_delegates;
mod dynamic_wrapper;
pub(crate) mod extensions;
pub(crate) mod objc;
pub(crate) mod resources;
pub(crate) mod state;
pub(crate) mod views;

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::{App as CacaoApp, AppDelegate as CacaoAppDelegate};
use cacao::appkit::window::{Window as CacaoWindow, WindowDelegate};
use cacao::layout::LayoutConstraint;
use cacao::notification_center::Dispatcher;

use crate::event::{EventHandlerMapRegistry, ViewTree};
use crate::platform::macos::cacao_delegates::StatefulEventDispatcher;
use crate::{App, AppDelegate, DialogBuilder, View, Window, WindowDelegator};
pub(crate) use self::dynamic_wrapper::DynamicViewWrapper;
pub(crate) use self::dynamic_wrapper::LayoutExt;
use self::extensions::WindowExtensions;
pub(crate) use self::objc::*;
use self::state::Event;
pub(crate) use self::views::*;


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

pub(crate) struct MacOSDelegate<Delegate, State>
        where State: 'static {
    delegate: Rc<RefCell<Delegate>>,
    window: Rc<CacaoWindow<MacOSWindowDelegate<Delegate, State>>>,
    event_registry: EventHandlerMapRegistry<State>,
    state: Arc<Mutex<State>>,
}

impl<Delegate, State: 'static> CacaoAppDelegate for MacOSDelegate<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    fn did_finish_launching(&self) {
        CacaoApp::activate();

        let user_delegator = Window::new(Rc::new(MacOSWindowDelegator {
            window: Rc::clone(&self.window),
        }));

        let config = {
            let mut state = self.state.as_ref().lock().unwrap();

            self.delegate.as_ref().borrow_mut().did_launch(&mut state);
            self.delegate.as_ref().borrow_mut().configure_main_window(&mut state)
        };

        config.title.as_ref().with(|title| {
            self.window.set_title(title);
        });

        if let Some(state) = config.title.as_ref().as_state() {
            let window = self.window.clone();
            state.add_listener(move |value| {
                window.set_title(value);
            });
        }

        if config.width != 0.0 && config.height != 0.0 {
            self.window.set_content_size(config.width, config.height);
        }

        {
            let mut state = self.state.as_ref().lock().unwrap();
            self.delegate.as_ref().borrow_mut().will_show_window(user_delegator, &mut state);
        }
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

impl<Delegate, State> Dispatcher for MacOSDelegate<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    type Message = Event;

    fn on_ui_message(&self, message: Event) {
        let mut state = self.state.lock().unwrap();

        let window = Window::new(Rc::new(MacOSWindowDelegator {
            window: Rc::clone(&self.window),
        }));

        match message {
            Event::ButtonClicked(view_id) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.click else {
                    return;
                };

                (handler)(&mut state, window);
            }

            Event::CheckboxChanged(view_id, is_checked) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.checked else {
                    return;
                };

                (handler)(&mut state, is_checked, window);
            }

            Event::TextFieldChanged(view_id, text) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.text_changed else {
                    return;
                };

                (handler)(&mut state, text, window);
            }
        }
    }
}

struct MacOSWindowDelegate<Delegate, State>
        where State: 'static {
    delegate: Rc<RefCell<Delegate>>,
    window: Option<Rc<CacaoWindow>>,
    #[allow(unused)]
    delegator: Option<Window>,
    view: cacao::view::View,
    content: Option<DynamicViewWrapper>,
    event_registry: EventHandlerMapRegistry<State>,
    state: Arc<Mutex<State>>,
}

impl<Delegate, State> WindowDelegate for MacOSWindowDelegate<Delegate, State>
        where Delegate: AppDelegate<State> {
    const NAME: &'static str = "finestra";

    fn did_load(&mut self, window: CacaoWindow) {
        let window = Rc::new(window);
        debug_assert!(self.window.is_none());

        let user_delegator = Window::new(Rc::new(MacOSWindowDelegator {
            window: Rc::clone(&window),
        }));

        let dispatcher = StatefulEventDispatcher {
            state: self.state.clone(),
            event_registry: self.event_registry.clone(),
            window: user_delegator.clone(),
        };

        let mut delegate = self.delegate.as_ref().borrow_mut();

        let mut state = self.state.lock().unwrap();
        let mut content_view = delegate.make_content_view(&mut state, user_delegator);

        let mut tree = ViewTree::new(self.event_registry.clone(), dispatcher);
        let parent_id = tree.exchange_events_for_id(Default::default());
        tree.set_parent_id(parent_id);
        let content_view = content_view.build_native(&mut tree);

        content_view.add_to_view(&self.view);

        window.set_content_view(&self.view);

        if content_view.constraints().is_empty() {
            LayoutConstraint::activate(&[
                content_view.layout_constraint_center_x().constraint_equal_to(&self.view.center_x),
                content_view.layout_constraint_center_y().constraint_equal_to(&self.view.center_y),
            ]);
        } else {
            for constraint in content_view.constraints() {
                let constraint = constraint.convert(&content_view, &self.view);
                constraint.set_active(true);
            }
        }

        self.content = Some(content_view);
        self.window = Some(window);
    }
}

struct MacOSWindowDelegator<CacaoDelegate> {
    window: Rc<CacaoWindow<CacaoDelegate>>,
}

impl<CacaoDelegate> WindowDelegator for MacOSWindowDelegator<CacaoDelegate> {
    fn create_dialog(&self, text: Cow<'static, str>) -> crate::DialogBuilder {
        DialogBuilder::new(Box::new(NSAlert::with(text, self.window.get_title())))
    }
}
