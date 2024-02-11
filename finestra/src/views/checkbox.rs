// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, Color, StateOrRaw, View, ViewBase, Window};

use super::base::BaseView;

/// A [`View`] that displays text and is clickable.
///
/// ```
/// # // This is usually used in a context where the `State` generic parameter
/// # // is inferred by the compiler.
/// # type Checkbox = finestra::Checkbox<()>;
/// let checkbox = Checkbox::new("Click Me");
/// ```
pub struct Checkbox<State> {
    pub(crate) base: ViewBase,
    pub(crate) text: StateOrRaw<String>,
    pub(crate) text_color: StateOrRaw<Color>,
    pub(crate) background_color: StateOrRaw<Color>,
    pub(crate) checked: StateOrRaw<bool>,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> Checkbox<State> {
    /// Creates a new [`Checkbox`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            base: ViewBase::default(),
            text: text.into(),
            text_color: StateOrRaw::Raw(Color::default()),
            background_color: StateOrRaw::Raw(Color::default()),
            checked: StateOrRaw::State(crate::State::new(false)),
            event_handler_map: Default::default(),
        }
    }

    /// Get notified when the checkbox has been switched to on or off.
    pub fn set_on_checked(&mut self, action: impl Fn(&mut State, bool, Window) + 'static) {
        self.event_handler_map.checked = Some(Box::new(action));
    }

    /// Get notified when the checkbox has been switched to on or off.
    pub fn with_on_checked(mut self, action: impl Fn(&mut State, bool, Window) + 'static) -> Self {
        self.set_on_checked(action);
        self
    }

    /// Returns `Self` with the given text `color`.
    #[must_use]
    pub fn with_text_color(self, color: impl Into<StateOrRaw<Color>>) -> Self {
        Self {
            text_color: color.into(),
            ..self
        }
    }

    /// Returns `Self` with the given background `color`.
    #[must_use]
    pub fn with_background_color(self, color: impl Into<StateOrRaw<Color>>) -> Self {
        Self {
            background_color: color.into(),
            ..self
        }
    }

    /// Sets the text color of the [`Checkbox`]. Use [`Checkbox::with_text_color()`]
    /// to avoid making a `mut` variable.
    pub fn set_text_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.text_color = color.into();
    }

    /// Sets the background color of the [`Checkbox`]. Use
    /// [`Checkbox::with_background_color()`] to avoid making a `mut` variable.
    pub fn set_background_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.background_color = color.into();
    }
}

impl<State> BaseView for Checkbox<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for Checkbox<State>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use cacao::{appkit::App, button::BezelStyle};
        use crate::platform::macos::{ButtonExtensions, Event, MacOSDelegate, NSButtonType, ToCacao};

        let map = std::mem::take(&mut self.event_handler_map);
        let id = tree.exchange_events_for_id(map);

        let mut checkbox = self.text.with(|text| {
            cacao::button::Button::new(text)
        });

        checkbox.set_button_type(NSButtonType::Switch);

        if let Some(color) = self.text_color.clone_inner().to_cacao() {
            checkbox.set_text_color(color);
        }

        if let Some(color) = self.background_color.clone_inner().to_cacao() {
            checkbox.set_bezel_color(color);
            checkbox.set_bezel_style(BezelStyle::Rounded);
        }

        crate::platform::macos::state::attach_checkbox_state(id, self, &checkbox);

        let state = self.checked.as_state().unwrap().clone();
        checkbox.set_action(move || {
            let is_checked = !state.clone_inner();
            state.set(is_checked);

            App::<MacOSDelegate<Delegate, State>, Event>::dispatch_main(Event::CheckboxChanged(id, is_checked));
        });
        checkbox.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView {
        use crate::{platform::win32::view::{WinButton, WinView, WinViewKind}, ViewId};

        let checkbox = self.text.with(|text| {
            WinButton::new(parent, text)
        });

        checkbox.as_ref().subscribe_text_update(self.text.as_state());

        let id = ViewId(checkbox.as_ref().get_control_id().0 as _);

        let map = std::mem::take(&mut self.event_handler_map);
        tree.put_event_handlers_with_id(id, map);

        WinView::new(id, WinViewKind::Button(checkbox))
    }
}

impl<Delegate, State> From<Checkbox<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: Checkbox<State>) -> Self {
        Box::new(value)
    }
}
