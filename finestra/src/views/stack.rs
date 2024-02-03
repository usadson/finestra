// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::marker::PhantomData;

use cacao::layout::Layout;

use crate::{event::EventHandlerMap, AppDelegate, StateOrRaw, View, ViewBase};

use super::base::BaseView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackDirection {
    Horizontal,
    Vertical,
}

/// NOTE: We require a [`Delegate`] parameter here, so [`Self::with()`] can
///       infer type. :)
pub struct Stack<State, Delegate> {
    base: ViewBase,
    event_handler_map: EventHandlerMap<State>,
    _phantom: PhantomData<Delegate>,

    pub(crate) direction: StackDirection,
    pub(crate) children: Vec<Box<dyn View<Delegate, State>>>,
}

impl<State, Delegate> Stack<State, Delegate> {
    #[must_use]
    pub fn new(direction: StackDirection) -> Self {
        Self {
            base: ViewBase::default(),
            event_handler_map: EventHandlerMap::default(),
            _phantom: PhantomData::default(),

            direction,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn horizontal() -> Self {
        Self::new(StackDirection::Horizontal)
    }

    #[must_use]
    pub fn vertical() -> Self {
        Self::new(StackDirection::Vertical)
    }
}

impl<State, Delegate> Stack<State, Delegate>
        where Delegate: AppDelegate<State> + 'static {
    pub fn with(self, view: impl Into<Box<dyn View<Delegate, State>>>) -> Self {
        self
    }
}

impl<State, Delegate> BaseView for Stack<State, Delegate> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for Stack<State, Delegate>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use crate::platform::macos::nsstackview::{NSStackView, NSStackViewGravity};

        let view = NSStackView::new();
        for child in &mut self.children {
            let comp = child.build_native(tree);
            view.add_view(comp.objc(), NSStackViewGravity::Center);
        }

        view.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::win32::view::WinView {
        <() as super::View<Delegate, State>>::build_native(&mut (), tree)
    }
}
