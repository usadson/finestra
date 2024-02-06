// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::cell::Cell;

use windows::{
    core::PCSTR,
    Win32::UI::WindowsAndMessaging::{
        MessageBoxA,
        MB_ICONERROR,
        MB_ICONINFORMATION,
        MB_ICONWARNING,
        MB_OK,
    }
};

use crate::{DialogApi, DialogKind};

pub(crate) struct Win32Dialog {
    text: Cell<String>,
    kind: Cell<DialogKind>,
    title: Cell<String>,
}

impl Win32Dialog {
    pub fn new(text: String, title: String) -> Self {
        Self {
            text: Cell::new(text),
            kind: Cell::new(DialogKind::Informational),
            title: Cell::new(title),
        }
    }
}

impl DialogApi for Win32Dialog {
    fn set_kind(&self, kind: crate::DialogKind) {
        self.kind.set(kind);
    }

    fn set_text(&self, text: std::borrow::Cow<'static, str>) {
        self.text.set(text.into_owned());
    }

    fn set_title(&self, title: std::borrow::Cow<'static, str>) {
        self.title.set(title.into_owned());
    }

    fn show(&self) {
        let text = self.text.take();
        let text = PCSTR::from_raw(text.as_ptr());

        let title = self.title.take();
        let title = PCSTR::from_raw(title.as_ptr());

        let mut look_and_feel = MB_OK;

        match self.kind.take() {
            DialogKind::Normal => (),

            DialogKind::Informational => {
                look_and_feel |= MB_ICONINFORMATION;
            }

            DialogKind::Warning => {
                look_and_feel |= MB_ICONWARNING;
            }

            DialogKind::Error => {
                look_and_feel |= MB_ICONERROR;
            }
        }

        unsafe {
            MessageBoxA(None, text, title, look_and_feel);
        }
    }
}
