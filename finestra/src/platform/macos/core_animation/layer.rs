// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{color::Color, foundation::id, objc::{msg_send, sel, sel_impl, runtime::Object}};

pub struct CALayer {
    pub(crate) obj: *mut Object,
}

impl CALayer {
    pub fn set_background_color(&self, color: Color) {
        let color: id = color.as_ref().into();

        unsafe {
            let _: () = msg_send![self.obj, setBackgroundColor: color];
        }
    }
}
