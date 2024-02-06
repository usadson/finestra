// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, View, ViewBase};

use super::base::BaseView;

pub struct ImageView<State> {
    pub(crate) base: ViewBase,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> ImageView<State> {
    /// Creates a new [`ImageView`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: ViewBase::default(),
            event_handler_map: Default::default(),
        }
    }
}

impl<State> BaseView for ImageView<State> {
    fn base(&self) -> &ViewBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl<Delegate: AppDelegate<State>, State> View<Delegate, State> for ImageView<State>
        where Delegate: 'static, State: 'static {
    #[cfg(target_os = "macos")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::macos::DynamicViewWrapper {
        use cacao::image::ImageView as CocoaImageView;


        let map = std::mem::take(&mut self.event_handler_map);
        let id = tree.exchange_events_for_id(map);

        let image_view = CocoaImageView::new();

        crate::platform::macos::state::attach_image_view_state(id, &self, &image_view);

        image_view.into()
    }

    /// Internal API: creates a native view (for Win32).
    #[cfg(target_os = "windows")]
    fn build_native(&mut self, tree: &mut crate::event::ViewTree<State>) -> crate::platform::win32::view::WinView {
        use crate::platform::win32::view::{WinView, WinViewKind};
        WinView::new(tree.exchange_events_for_id(Default::default()), WinViewKind::Empty)
    }
}

impl<Delegate, State> From<ImageView<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: ImageView<State>) -> Self {
        Box::new(value)
    }
}
