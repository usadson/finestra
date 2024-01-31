// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{appkit::window::Window as CacaoWindow, foundation::NSString, objc::{msg_send, sel, sel_impl}};

pub(crate) trait WindowExtensions {
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
