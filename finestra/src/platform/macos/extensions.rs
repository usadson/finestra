// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    appkit::window::Window as CacaoWindow,
    color::Color,
    foundation::{id, NSString, NO, YES},
    layout::Layout,
    objc::{msg_send, sel, sel_impl}
};

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
    fn set_needs_layer(&self, value: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setNeedsDisplay:match value {
                true => YES,
                false => NO
            }];
        });
    }
}

impl<T> ViewExtensions for T where T: Layout {}

pub trait ButtonExtensions: Layout {
    fn set_bezel_color(&self, color: Color) {
        let color: id = color.as_ref().into();

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBezelColor: color];
        });
    }
}

impl ButtonExtensions for cacao::button::Button {}
