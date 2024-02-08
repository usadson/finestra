// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod base;
mod button;
mod checkbox;
mod image;
mod label;
mod stack;
mod text_block;
mod text_field;

use crate::AppDelegate;

pub use self::base::*;
pub use self::button::Button;
pub use self::checkbox::Checkbox;
pub use self::image::*;
pub use self::label::Label;
pub use self::stack::{Stack, StackDirection};
pub use self::text_block::TextBlock;
pub use self::text_field::TextField;

/// A generic graphical component.
///
/// Common components are:
/// 1. [`Label`] can be used to display text.
pub trait View<Delegate, State=()>
        where Delegate: AppDelegate<State>, State: 'static {
    /// Internal API: creates a native view (for macOS).
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper;

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView;
}

impl<Delegate, State: 'static> View<Delegate, State> for ()
        where Delegate: AppDelegate<State> {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, _tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        cacao::text::Label::new().into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(
        &mut self,
        tree: &mut crate::event::ViewTree<State>,
        _parent: windows::Win32::Foundation::HWND,
    ) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinView, WinViewKind};

        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Empty)
    }
}
