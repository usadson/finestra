// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::StateOrRaw;

#[derive(Debug, Default)]
pub struct ViewBase {
    pub(crate) tooltip: StateOrRaw<String>,
}

pub trait BaseView {
    fn base(&self) -> &ViewBase;
    fn base_mut(&mut self) -> &mut ViewBase;
}

pub trait BaseViewExt {
    fn set_tooltip(&mut self, tooltip: impl Into<StateOrRaw<String>>);
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
