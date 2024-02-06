// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, Color, StateOrRaw, View, ViewBase, Window};

use super::base::BaseView;

/// A [`View`] that displays text and is clickable.
///
/// ```
/// # // This is usually used in a context where the `State` generic parameter
/// # // is inferred by the compiler.
/// # type Button = finestra::Button<()>;
/// let button = Button::new("Click Me");
/// ```
pub struct Button<State> {
    pub(crate) base: ViewBase,
    pub(crate) text: StateOrRaw<String>,
    pub(crate) text_color: StateOrRaw<Color>,
    pub(crate) background_color: StateOrRaw<Color>,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> Button<State> {
    /// Creates a new [`Button`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            base: ViewBase::default(),
            text: text.into(),
            text_color: StateOrRaw::Raw(Color::default()),
            background_color: StateOrRaw::Raw(Color::default()),
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

    /// Sets the text color of the [`Button`]. Use [`Button::with_text_color()`]
    /// to avoid making a `mut` variable.
    pub fn set_text_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.text_color = color.into();
    }

    /// Sets the background color of the [`Button`]. Use
    /// [`Button::with_background_color()`] to avoid making a `mut` variable.
    pub fn set_background_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.background_color = color.into();
    }
}

impl<State> BaseView for Button<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for Button<State>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use cacao::{appkit::App, button::BezelStyle};
        use crate::platform::macos::extensions::ButtonExtensions;

        use crate::platform::macos::{
            resources::ToCacao, state::Event, MacOSDelegate
        };

        let map = std::mem::take(&mut self.event_handler_map);
        let id = tree.exchange_events_for_id(map);

        let mut button = self.text.with(|text| {
            cacao::button::Button::new(text)
        });

        if let Some(color) = self.text_color.clone_inner().to_cacao() {
            button.set_text_color(color);
        }

        if let Some(color) = self.background_color.clone_inner().to_cacao() {
            button.set_bezel_color(color);
            button.set_bezel_style(BezelStyle::Rounded);
        }

        crate::platform::macos::state::attach_button_state(&self, &button);

        button.set_action(move || {
            App::<MacOSDelegate<Delegate, State>, Event>::dispatch_main(Event::ButtonClicked(id));
        });
        button.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        _parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinView, WinViewKind};

        _ = &self.text;
        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Empty)
    }
}

impl<Delegate, State> From<Button<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: Button<State>) -> Self {
        Box::new(value)
    }
}
