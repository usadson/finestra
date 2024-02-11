// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::StateOrRaw;

#[derive(Debug, Default)]
pub(crate) struct ViewBase {
    pub(crate) tooltip: StateOrRaw<String>,
}

pub(crate) trait BaseView {
    fn base(&self) -> &ViewBase;
    fn base_mut(&mut self) -> &mut ViewBase;
}

/// Extensions for views that are quite common, such as setting the tooltip.
pub trait BaseViewExt {
    /// Set the tooltip of the view, which is text that is commonly displayed
    /// when hovering over a component (such as a button). This is used to
    /// provide the user with more context, or to let them know what the action
    /// will do.
    fn set_tooltip(&mut self, tooltip: impl Into<StateOrRaw<String>>);

    /// Set the tooltip of the view, which is text that is commonly displayed
    /// when hovering over a component (such as a button). This is used to
    /// provide the user with more context, or to let them know what the action
    /// will do.
    fn with_tooltip(self, tooltip: impl Into<StateOrRaw<String>>) -> Self;
}

impl<T> BaseViewExt for T
        where T: BaseView {
    fn set_tooltip(&mut self, tooltip: impl Into<StateOrRaw<String>>) {
        self.base_mut().tooltip = tooltip.into();
    }

    fn with_tooltip(mut self, tooltip: impl Into<StateOrRaw<String>>) -> Self {
        self.set_tooltip(tooltip);
        self
    }
}
