// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{Arc, Mutex};

use cacao::input::TextFieldDelegate;

use crate::{EventHandlerMapRegistry, StateChangeOrigin, TextValue, ViewId, Window};

use super::state::Event;

pub(crate) struct MacOSTextFieldDelegate {
    pub(crate) view_id: ViewId,
    pub(crate) dispatcher: Box<dyn EventDispatcher>,
    pub(crate) text: Option<TextValue>,
}

impl TextFieldDelegate for MacOSTextFieldDelegate {
    const NAME: &'static str = "finestra.textfield";

    fn text_did_change(&self, value: &str) {
        self.dispatcher.dispatch_event(Event::TextFieldChanged(self.view_id, value.to_string()));

        if let Some(text) = &self.text {
            text.set_with_origin(value, StateChangeOrigin::Owner(self.view_id));
        }
    }
}

pub(crate) trait EventDispatcher {
    fn dispatch_event(&self, event: Event);
}

pub struct StatefulEventDispatcher<State> {
    pub event_registry: EventHandlerMapRegistry<State>,
    pub state: Arc<Mutex<State>>,
    pub window: Window,
}

impl<State> Clone for StatefulEventDispatcher<State> {
    fn clone(&self) -> Self {
        Self {
            event_registry: self.event_registry.clone(),
            state: self.state.clone(),
            window: self.window.clone(),
        }
    }
}

impl<State> EventDispatcher for StatefulEventDispatcher<State> {
    fn dispatch_event(&self, event: Event) {
        let mut state = self.state.lock().unwrap();

        match event {
            Event::ButtonClicked(view_id) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.click else {
                    return;
                };

                (handler)(&mut state, self.window.clone());
            }

            Event::TextFieldChanged(view_id, text) => {
                let Some(handler) = self.event_registry.map.get(&view_id) else {
                    return;
                };

                let Some(handler) = &handler.text_changed else {
                    return;
                };

                (handler)(&mut state, text, self.window.clone());
            }
        }
    }
}
