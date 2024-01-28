// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod button;
mod label;

use crate::AppDelegate;

pub use self::button::Button;
pub use self::label::Label;

/// A generic graphical component.
///
/// Common components are:
/// 1. [`Label`] can be used to display text.
pub trait View<Delegate, State=()>
        where Delegate: AppDelegate<State> {
    /// Internal API: creates a native view (for macOS).
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::platform::macos::state::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper;
}

impl<Delegate, State> View<Delegate, State> for ()
        where Delegate: AppDelegate<State> {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, _tree: &mut crate::platform::macos::state::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        cacao::text::Label::new().into()
    }
}
