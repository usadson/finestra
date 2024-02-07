// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use cacao::foundation::NSString;

use objc_id::Id;
use cacao::objc::runtime::Object;
use cacao::objc::{class, msg_send, sel, sel_impl};

use crate::{DialogApi, DialogKind};

pub(crate) struct NSAlert(Id<Object>);

impl NSAlert {
    pub fn new() -> Self {
        Self(unsafe {
            let alert: cacao::foundation::id = msg_send![class!(NSAlert), new];
            Id::from_ptr(alert)
        })
    }

    pub fn with(text: Cow<'static, str>, title: String) -> Self {
        let this = Self::new();
        this.set_title(title.into());
        this.set_text(text);
        this
    }
}

impl DialogApi for NSAlert {
    fn set_title(&self, title: Cow<'static, str>) {
        let title = NSString::new(&title);
        unsafe {
            let _: () = msg_send![self.0, setMessageText: title];
        }
    }

    fn set_text(&self, text: Cow<'static, str>) {
        let text = NSString::new(&text);
        unsafe {
            let _: () = msg_send![self.0, setInformativeText: text];
        }
    }

    fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, runModal];
        }
    }

    fn set_kind(&self, kind: crate::DialogKind) {
        let style: NSAlertStyle = kind.into();
        let style = style as u32;
        unsafe {
            let _: () = msg_send![self.0, setAlertStyle: style];
        }
    }
}

enum NSAlertStyle {
    Warning = 0,
    Informational = 1,
    Critical = 2,
}

impl From<DialogKind> for NSAlertStyle {
    fn from(value: DialogKind) -> Self {
        match value {
            DialogKind::Normal => Self::Informational,
            DialogKind::Error => Self::Critical,
            DialogKind::Informational => Self::Informational,
            DialogKind::Warning => Self::Warning,
        }
    }
}
