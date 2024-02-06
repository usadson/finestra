// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::*;
use super::base::BaseView;

pub struct TextField<State> {
    pub(crate) base: ViewBase,
    pub(crate) text: StateOrRaw<String>,
    pub(crate) placeholder_text: StateOrRaw<String>,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> TextField<State> {
    /// Creates a new [`TextField`] with an associated string that contains an
    /// initial value. If the value is a [`TextValue`][crate::TextValue], this
    /// value will be updated if the text changes.
    #[must_use]
    pub fn new(text: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            base: ViewBase::default(),
            text: text.into(),
            placeholder_text: Default::default(),
            event_handler_map: Default::default(),
        }
    }

    pub fn set_on_change(&mut self, action: impl Fn(&mut State, String, Window) + 'static) {
        self.event_handler_map.text_changed = Some(Box::new(action));
    }

    pub fn with_on_change(mut self, action: impl Fn(&mut State, String, Window) + 'static) -> Self {
        self.set_on_change(action);
        self
    }

    #[must_use]
    pub fn with_placeholder(mut self, placeholder: impl Into<StateOrRaw<String>>) -> Self {
        self.set_placeholder(placeholder);
        self
    }

    pub fn set_placeholder(&mut self, placeholder: impl Into<StateOrRaw<String>>) {
        self.placeholder_text = placeholder.into();
    }
}

impl<State> BaseView for TextField<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for TextField<State>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use self::platform::macos::cacao_delegates::MacOSTextFieldDelegate;

        let map = std::mem::take(&mut self.event_handler_map);
        let view_id = tree.exchange_events_for_id(map);

        let delegate = MacOSTextFieldDelegate{
            view_id,
            dispatcher: tree.create_dispatcher(),
            text: self.text.as_state(),
        };

        let text_field = cacao::input::TextField::with(delegate);
        text_field.set_uses_single_line(true);

        self.placeholder_text.with(|placeholder| {
            if !placeholder.is_empty() {
                text_field.set_placeholder_text(&placeholder);
            }
        });

        crate::platform::macos::state::attach_text_field_state(view_id, &self, &text_field);
        text_field.width.constraint_equal_to_constant(100.).set_active(true);
        text_field.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinView, WinViewKind};

        _ = &self.text;
        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Empty)
    }
}

impl<Delegate, State> From<TextField<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: TextField<State>) -> Self {
        Box::new(value)
    }
}
