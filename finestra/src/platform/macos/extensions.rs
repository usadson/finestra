// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    appkit::window::Window as CacaoWindow, color::Color, foundation::{id, NSString, NO, YES}, layout::Layout, objc::{msg_send, runtime::Object, sel, sel_impl}
};

use super::core_animation::CALayer;

pub trait WindowExtensions {
    fn get_title(&self) -> String;
}

impl<T> WindowExtensions for CacaoWindow<T> {
    fn get_title(&self) -> String {
        let title: NSString = unsafe {
            msg_send![&*self.objc, title]
        };

        title.to_string()
    }
}

pub(crate) trait ViewExtensions: Layout {
    fn layer(&self) -> Option<CALayer> {
        let layer: *mut Object = self.get_from_backing_obj(|obj| unsafe { msg_send![obj, layer] });
        if layer.is_null() {
            return None;
        }

        Some(CALayer { obj: layer })
    }

    fn set_needs_layer(&self, value: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setNeedsDisplay:match value {
                true => YES,
                false => NO
            }];
        });
    }

    fn set_content_hugging_priority(
        &self,
        priority: NSLayoutPriority,
        for_orientation: NSLayoutConstraintOrientation,
    ) {
        let priority = priority as i32 as f32;
        let orientation = for_orientation as i32;

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () =
                msg_send![obj, setContentHuggingPriority:priority forOrientation:orientation];
        });
    }

    /// <https://developer.apple.com/documentation/appkit/nsview/1524974-setcontentcompressionresistancep?language=objc>
    fn set_content_compression_resistance_priority(
        &self,
        priority: NSLayoutPriority,
        for_orientation: NSLayoutConstraintOrientation,
    ) {
        let priority = priority as i32 as f32;
        let orientation = for_orientation as i32;

        self.with_backing_obj_mut(|obj| unsafe {
                let _: () = msg_send![obj, setContentCompressionResistancePriority:priority forOrientation:orientation];
            });
    }
}

impl<T> ViewExtensions for T where T: Layout {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum NSLayoutConstraintOrientation {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum NSLayoutPriority {
    Required = 1000,
    DefaultHigh = 750,
    DragThatCanResizeWindow = 510,
    WindowSizeStayPut = 500,
    DragThatCannotResizeWindow = 490,
    DefaultLow = 250,
    FittingSizeCompression = 50,
}

pub trait ButtonExtensions: Layout {
    fn set_bezel_color(&self, color: Color) {
        let color: id = color.as_ref().into();

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBezelColor: color];
        });
    }
}

impl ButtonExtensions for cacao::button::Button {}
