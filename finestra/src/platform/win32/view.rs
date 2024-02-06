// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use windows::{core::PCSTR, Win32::{Foundation::{GetLastError, HWND}, System::SystemServices::SS_CENTER, UI::WindowsAndMessaging::{CreateWindowExA, ShowWindow, SW_SHOWDEFAULT, WINDOW_STYLE, WS_CHILD, WS_TABSTOP, WS_VISIBLE}}};
use windows::Win32::System::SystemServices::SS_SIMPLE;
use crate::event::ViewId;
use windows::Win32::UI::WindowsAndMessaging::BS_DEFPUSHBUTTON;

use super::{window::WindowData, wrapper::Hwnd};

#[derive(Debug)]
pub struct WinView {
    pub(crate) id: ViewId,
    pub(crate) kind: WinViewKind,
}

impl WinView {
    pub(crate) fn new(id: ViewId, kind: WinViewKind) -> Self {
        Self {
            id,
            kind,
        }
    }

    pub(crate) fn install<Delegate, State>(&self, window: &WindowData<Delegate, State>)
            where Delegate: crate::AppDelegate<State> {
        _ = window;
        if let Some(hwnd) = self.kind.hwnd() {
            hwnd.show(SW_SHOWDEFAULT);
        }
    }
}

#[derive(Debug)]
pub enum WinViewKind {
    Empty,
    Button(WinButton),
    Label(WinLabel),
}

impl WinViewKind {
    pub fn hwnd(&self) -> Option<Hwnd> {
        match self {
            Self::Empty => None,
            Self::Button(button) => Some(button.hwnd),
            Self::Label(label) => Some(label.hwnd),
        }
    }
}

#[derive(Debug)]
pub struct WinButton {
    hwnd: Hwnd,
}

impl WinButton {
    pub fn new(parent: HWND, text: &str) -> Self {
        let class_name = PCSTR::from_raw("BUTTON\0".as_ptr());
        let text = PCSTR::from_raw(format!("{text}\0").as_ptr());

        let hwnd = unsafe {
            CreateWindowExA(
                Default::default(),
                class_name,
                text,
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_DEFPUSHBUTTON as _),
                10, 10,
                100, 100,
                parent,
                None,
                None,
                None
            )
        };

        debug_assert!(hwnd.0 != 0, "{:#?}", unsafe{GetLastError()});

        Self {
            hwnd: hwnd.into(),
        }
    }
}

impl AsRef<Hwnd> for WinButton {
    fn as_ref(&self) -> &Hwnd {
        &self.hwnd
    }
}

#[derive(Debug)]
pub struct WinLabel {
    hwnd: Hwnd,
}

impl WinLabel {
    pub fn new(parent: HWND, text: &str) -> Self {
        let class_name = PCSTR::from_raw("STATIC\0".as_ptr());
        let text = PCSTR::from_raw(format!("{text}\0").as_ptr());

        let hwnd = unsafe {
            CreateWindowExA(
                Default::default(),
                class_name,
                text,
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(SS_SIMPLE.0),
                10, 10,
                100, 100,
                parent,
                None,
                None,
                None
            )
        };

        debug_assert!(hwnd.0 != 0, "{:#?}", unsafe{GetLastError()});

        Self {
            hwnd: hwnd.into(),
        }
    }
}

impl AsRef<Hwnd> for WinLabel {
    fn as_ref(&self) -> &Hwnd {
        &self.hwnd
    }
}
