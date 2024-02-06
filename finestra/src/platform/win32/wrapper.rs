// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use windows::{core::PCSTR, Win32::{Foundation::HWND, UI::WindowsAndMessaging::{DispatchMessageA, GetDlgCtrlID, GetMessageA, SetWindowTextA, ShowWindow, TranslateMessage, MSG}}};
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

use crate::{State, StateChangeOrigin};

pub fn get_next_message() -> MSG {
    let mut msg = MSG::default();
    unsafe {
        _ = GetMessageA(&mut msg, None, 0, 0);
    }

    msg
}

pub trait MsgExtensions {
    /// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage>
    fn translate(&self);

    /// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessage>
    fn dispatch(&self);
}

impl MsgExtensions for MSG {
    fn translate(&self) {
        unsafe {
            TranslateMessage(self as *const MSG);
        }
    }

    fn dispatch(&self) {
        // Important: GetMessageA and DispatchMessageA should be used together,
        //            and not GetMessageW with DispatchMessageA or the other way
        //            around.
        // <https://devblogs.microsoft.com/oldnewthing/20181101-00/?p=100105>
        unsafe {
            DispatchMessageA(self);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlId(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Hwnd {
    inner: HWND,
}

impl Hwnd {
    pub fn get_control_id(&self) -> ControlId {
        ControlId(unsafe { GetDlgCtrlID(self.inner) })
    }

    pub fn show(&self, command: SHOW_WINDOW_CMD) {
        unsafe { ShowWindow(self.inner, command) };
    }

    pub fn set_text(&self, text: impl Into<String>) {
        let mut text = text.into();
        text += "\0";

        let text = PCSTR::from_raw(text.as_ptr());
        unsafe {
            SetWindowTextA(self.inner, text).unwrap();
        }
    }

    pub fn subscribe_text_update(&self, state: Option<State<String>>) {
        let Some(state) = state else {
            return
        };

        let obj = self.clone();
        state.add_listener_with_origin(move |val| {
            obj.set_text(val);
        }, StateChangeOrigin::System);
    }
}

impl From<HWND> for Hwnd {
    fn from(value: HWND) -> Self {
        Self {
            inner: value,
        }
    }
}

impl AsRef<HWND> for Hwnd {
    fn as_ref(&self) -> &HWND {
        &self.inner
    }
}

impl Deref for Hwnd {
    type Target = HWND;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
