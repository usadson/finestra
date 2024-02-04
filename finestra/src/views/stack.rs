// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::marker::PhantomData;

use crate::{AppDelegate, View, ViewBase};

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
    _phantom: PhantomData<Delegate>,

    pub(crate) direction: StackDirection,
    pub(crate) children: Vec<Box<dyn View<Delegate, State>>>,
}

impl<State, Delegate> Stack<State, Delegate> {
    #[must_use]
    pub fn new(direction: StackDirection) -> Self {
        Self {
            base: ViewBase::default(),
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

impl<State: 'static, Delegate> Stack<State, Delegate>
        where Delegate: AppDelegate<State> + 'static {
    pub fn with(mut self, view: impl Into<Box<dyn View<Delegate, State>>>) -> Self {
        self.children.push(view.into());
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

impl<Delegate, State> From<Stack<State, Delegate>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: Stack<State, Delegate>) -> Self {
        Box::new(value)
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for Stack<State, Delegate>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use crate::platform::macos::nsstackview::NSStackView;

        let mut view = NSStackView::new(self.direction);
        for child in &mut self.children {
            let comp = child.build_native(tree);
            view.add_view(comp);
        }

        view.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::win32::view::WinView {
        <() as super::View<Delegate, State>>::build_native(&mut (), tree)
    }
}