// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;
use std::cell::Cell;

use cacao::foundation::NSString;

use objc_id::Id;
use cacao::objc::runtime::Object;
use cacao::objc::{class, msg_send, sel, sel_impl};

use crate::DialogApi;

pub(crate) struct DialogImpl {
    pub title: Cell<Cow<'static, str>>,
    pub text: Cow<'static, str>,
}

impl DialogImpl {
    pub fn new(text: Cow<'static, str>, title: String) -> Self {
        Self {
            title: Cell::new(Cow::Owned(title)),
            text,
        }
    }
}

impl DialogApi for DialogImpl {
    fn set_title(&self, title: Cow<'static, str>) {
        self.title.set(title);
    }

    fn show(&self) {
        let alert = NSAlert::new(self.title.take().as_ref(), &self.text);
        alert.show();
    }
}

pub(crate) struct NSAlert(Id<Object>);

impl NSAlert {
    pub fn new(title: &str, message: &str) -> Self {
        let title = NSString::new(title);
        let message = NSString::new(message);
        let ok = NSString::new("OK");

        Self(unsafe {
            let alert: cacao::foundation::id = msg_send![class!(NSAlert), new];
            let _: () = msg_send![alert, setMessageText: title];
            let _: () = msg_send![alert, setInformativeText: message];
            let _: () = msg_send![alert, addButtonWithTitle: ok];
            Id::from_ptr(alert)
        })
    }

    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, runModal];
        }
    }
}
