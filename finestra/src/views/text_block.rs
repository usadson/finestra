// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, Color, StateOrRaw, TextAlignment, View, ViewBase};

use super::base::BaseView;

/// A text block is a view that can display multiple lines of text.
pub struct TextBlock<State=()> {
    base: ViewBase,
    pub(crate) text: StateOrRaw<String>,
    pub(crate) text_color: StateOrRaw<Color>,
    pub(crate) background_color: StateOrRaw<Color>,
    pub(crate) alignment: StateOrRaw<TextAlignment>,

    #[allow(unused)]
    event_handler_map: EventHandlerMap<State>,
}

impl<State> TextBlock<State> {
    /// A text block is a view that can display multiple lines of text.
    #[must_use]
    pub fn new(text: impl Into<StateOrRaw<String>>) -> Self {
        Self {
            base: ViewBase::default(),
            text: text.into(),
            text_color: StateOrRaw::Raw(Color::default()),
            background_color: StateOrRaw::Raw(Color::default()),
            alignment: Default::default(),
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

    /// Sets the color of the [`TextBlock`]. Use [`TextBlock::with_color()`] to
    /// avoid making a `mut` variable.
    pub fn set_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.text_color = color.into();
    }

    /// Sets the background color of the [`TextBlock`]. Use
    /// [`TextBlock::with_background_color()`] to avoid making a `mut` variable.
    pub fn set_background_color(&mut self, color: impl Into<StateOrRaw<Color>>) {
        self.background_color = color.into();
    }

    /// Set the alignment/justification of the lines.
    #[must_use]
    pub fn with_text_alignment(mut self, alignment: impl Into<StateOrRaw<TextAlignment>>) -> Self {
        self.set_text_alignment(alignment);
        self
    }

    /// Set the alignment/justification of the lines.
    pub fn set_text_alignment(&mut self, alignment: impl Into<StateOrRaw<TextAlignment>>) {
        self.alignment = alignment.into();
    }

    /// Align the lines to the center of this text block.
    #[must_use]
    pub fn centered(mut self) -> Self {
        self.alignment = StateOrRaw::Raw(TextAlignment::Center);
        self
    }
}

impl<State> BaseView for TextBlock<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate, State: 'static> View<Delegate, State> for TextBlock<State>
        where Delegate: AppDelegate<State> {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use crate::platform::macos::ToCacao;

        let view_id = tree.exchange_events_for_id(Default::default());

        let label = cacao::text::Label::new();
        label.set_text(self.text.clone_inner());

        label.set_line_break_mode(cacao::text::LineBreakMode::WrapWords);
        label.set_text_alignment(self.alignment.clone_inner().to_cacao());

        if let Some(color) = self.text_color.clone_inner().to_cacao() {
            label.set_text_color(color);
        }

        if let Some(color) = self.background_color.clone_inner().to_cacao() {
            label.set_background_color(color);
        }

        crate::platform::macos::state::attach_text_block_state(view_id, self, &label);
        label.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinLabel, WinView, WinViewKind};

        let label = self.text.with(|text| {
            WinLabel::new(parent, text)
        });

        label.as_ref().subscribe_text_update(self.text.as_state());

        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Label(label))
    }
}

impl<Delegate, State> From<TextBlock<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: TextBlock<State>) -> Self {
        Box::new(value)
    }
}
