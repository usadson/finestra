// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, Color, StateOrRaw, View, ViewBase};

use super::base::BaseView;

/// A [`View`] that displays text.
///
/// ```
/// # // This is usually used in a context where the `State` generic parameter
/// # // is inferred by the compiler.
/// # type Label = finestra::Label<()>;
/// let label = Label::new("This is some text");
/// let label = Label::new(format!("What is 2 + 2? Correct, {}!", 2 + 2));
/// ```
pub struct Label<State=()> {
    base: ViewBase,
    pub(crate) text: StateOrRaw<String>,
    pub(crate) text_color: StateOrRaw<Color>,
    pub(crate) background_color: StateOrRaw<Color>,

    #[allow(unused)]
    event_handler_map: EventHandlerMap<State>,
}

impl<State> Label<State> {
    /// Creates a new [`Label`] with the associated string.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            base: ViewBase::default(),
            text: text.into().into(),
            text_color: StateOrRaw::Raw(Color::default()),
            background_color: StateOrRaw::Raw(Color::default()),
            event_handler_map: Default::default(),
        }
    }

    /// Returns `Self` with the given `color`.
    #[must_use]
    pub fn with_color(self, color: impl Into<StateOrRaw<Color>>) -> Self {
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

    /// Sets the color of the [`Label`]. Use [`Label::with_color()`] to avoid
    /// making a `mut` variable.
    pub fn set_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.text_color = color.into();
    }

    /// Sets the background color of the [`Label`]. Use
    /// [`Label::with_background_color()`] to avoid making a `mut` variable.
    pub fn set_background_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.background_color = color.into();
    }
}

impl<State> BaseView for Label<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate, State> View<Delegate, State> for Label<State>
        where Delegate: AppDelegate<State> {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, _tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use crate::platform::macos::resources::ToCacao;

        let label = cacao::text::Label::new();
        label.set_text(&self.text.clone_inner());
        label.set_font(&cacao::text::Font::system(30.));

        if let Some(color) = self.text_color.clone_inner().to_cacao() {
            label.set_text_color(color);
        }

        if let Some(color) = self.background_color.clone_inner().to_cacao() {
            label.set_background_color(color);
        }

        crate::platform::macos::state::attach_label_state(&self, &label);
        label.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinView, WinViewKind};

        _ = &self.text;
        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Empty)
    }
}

impl<Delegate, State> From<Label<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: Label<State>) -> Self {
        Box::new(value)
    }
}
