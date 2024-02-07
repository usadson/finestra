// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{event::EventHandlerMap, AppDelegate, Image, View, ViewBase};

use super::base::BaseView;

pub struct ImageView<State> {
    pub(crate) base: ViewBase,
    image: Image,
    event_handler_map: EventHandlerMap<State>,
}

impl<State> Default for ImageView<State> {
    fn default() -> Self {
        Self::new()
    }
}

impl<State> ImageView<State> {
    /// Creates a new [`ImageView`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: ViewBase::default(),
            image: Image::default(),
            event_handler_map: Default::default(),
        }
    }

    pub fn with(mut self, image: Image) -> Self {
        self.set_image(image);
        self
    }

    pub fn set_image(&mut self, image: Image) {
        self.image = image;
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
        use cacao::core_graphics::geometry::CGPoint;
        use cacao::core_graphics::geometry::CGRect;
        use cacao::core_graphics::geometry::CGSize;
        use cacao::image::Image as CocoaImage;
        use cacao::image::ImageView as CocoaImageView;
        use cacao::layout::Layout;

        use crate::platform::macos::DynamicViewWrapper;
        use crate::ImageKind;

        let map = std::mem::take(&mut self.event_handler_map);
        let id = tree.exchange_events_for_id(map);

        let image_view = CocoaImageView::new();

        let image = match self.image.kind() {
            ImageKind::File(file) => Some(
                CocoaImage::with_contents_of_file(
                    file.as_path().to_str().unwrap()
                )
            ),

            ImageKind::None => None,
        };

        if let Some(image) = image {
            // TODO: don't be lazy and leak the image
            let image = Box::leak(Box::new(image));
            image_view.set_image(&*image);
        }

        image_view.set_frame(CGRect::new(&CGPoint::new(10.0, 10.0), &CGSize::new(200.0, 200.0)));

        crate::platform::macos::state::attach_image_view_state(id, self, &image_view);

        let mut obj: DynamicViewWrapper = image_view.into();

        if let Some(parent) = tree.parent_id() {
            obj.add_constraints_to_parent_box(parent);
        }

        obj
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

impl<Delegate, State> From<ImageView<State>> for Box<dyn View<Delegate, State>>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    fn from(value: ImageView<State>) -> Self {
        Box::new(value)
    }
}
