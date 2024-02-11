// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::{App as CacaoApp, AppDelegate as CacaoAppDelegate};
use cacao::appkit::window::Window as CacaoWindow;
use cacao::notification_center::Dispatcher;

use crate::{AppDelegate, StateChangeOrigin, EventHandlerMapRegistry, Window};

use super::extensions::WindowExtensions;
use super::state::Event;
use super::window::{MacOSWindowDelegate, MacOSWindowDelegator};

pub(crate) struct MacOSDelegate<Delegate, State>
        where State: 'static {
    pub(super) delegate: Rc<RefCell<Delegate>>,
    pub(super) window: Rc<CacaoWindow<MacOSWindowDelegate<Delegate, State>>>,
    pub(super) event_registry: EventHandlerMapRegistry<State>,
    pub(super) state: Arc<Mutex<State>>,
}

impl<Delegate: AppDelegate<State> + 'static, State: 'static> MacOSDelegate<Delegate, State> {
    pub fn new(
        delegate: Delegate,
        state: State,
    ) -> Self {
        let delegate = Rc::new(RefCell::new(delegate));
        let event_registry = EventHandlerMapRegistry::default();
        let state = Arc::new(Mutex::new(state));

        let window_delegate = MacOSWindowDelegate::new(
            Rc::clone(&delegate),
            event_registry.clone(),
            Arc::clone(&state)
        );

        Self {
            delegate,
            event_registry,
            state,

            window: Rc::new(
                CacaoWindow::with(Default::default(), window_delegate)
            ),
        }
    }
}

impl<Delegate, State: 'static> CacaoAppDelegate for MacOSDelegate<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    fn did_finish_launching(&self) {
        CacaoApp::activate();

        let user_delegator = Window::new(Arc::new(MacOSWindowDelegator {
            window: Rc::clone(&self.window),
        }));

        let config = {
            let mut state = self.state.as_ref().lock().unwrap();

            self.delegate.as_ref().borrow_mut().did_launch(&mut state);
            self.delegate.as_ref().borrow_mut().configure_main_window(&mut state)
        };

        super::set_menu_bar::<Delegate, State>(config.menubar);

        config.title.as_ref().with(|title| {
            self.window.set_title(title);
        });

        if let Some(state) = config.title.as_ref().as_state() {
            let window = self.window.clone();
            state.add_listener_with_origin(move |value| {
                window.set_title(value);
            }, StateChangeOrigin::System);
        }

        config.theme.with(|theme| {
            self.window.set_appearance(*theme);
        });

        if let Some(state) = config.theme.as_state() {
            let window = self.window.clone();
            state.add_listener_with_origin(move |theme| {
                window.set_appearance(*theme);
            }, StateChangeOrigin::System);
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

        let window = Window::new(Arc::new(MacOSWindowDelegator {
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

            Event::MenuAction { item } => {
                self.delegate.borrow_mut().did_invoke_menu_action(item, &mut state, window);
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
