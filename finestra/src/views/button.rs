// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, StateOrRaw, View, Window};

/// A [`View`] that displays text and is clickable.
///
/// ```
/// # // This is usually used in a context where the `State` generic parameter
/// # // is inferred by the compiler.
/// # type Button = finestra::Button<()>;
/// let button = Button::new("Click Me");
/// ```
pub struct Button<State> {
    text: StateOrRaw<String>,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> Button<State> {
    /// Creates a new [`Button`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            text: text.into(),
            event_handler_map: Default::default(),
        }
    }

    pub fn set_on_click(&mut self, action: impl Fn(&mut State, Window) + 'static) {
        self.event_handler_map.click = Some(Box::new(action));
    }

    pub fn with_on_click(mut self, action: impl Fn(&mut State, Window) + 'static) -> Self {
        self.event_handler_map.click = Some(Box::new(action));
        self
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for Button<State>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::platform::macos::state::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use cacao::appkit::App;
        use crate::platform::macos::{
            MacOSDelegate,
            state::Event,
        };

        use cacao::{foundation::NSString, objc::{msg_send, sel, sel_impl}};

        let map = std::mem::take(&mut self.event_handler_map);
        let id = tree.exchange_events_for_id(map);

        let mut button = self.text.with(|text| {
            cacao::button::Button::new(text)
        });

        let objc = button.objc.clone();
        if let StateOrRaw::State(text_state) = &self.text {
            text_state.add_listener(move |val| {
                let s = NSString::new(val);

                objc.with_mut(|obj| unsafe {
                    let _: () = msg_send![obj, setTitle:&*s];
                });
            });
        }

        button.set_action(move || {
            App::<MacOSDelegate<Delegate, State>, Event>::dispatch_main(Event::ButtonClicked(id));
        });
        button.into()
    }
}
