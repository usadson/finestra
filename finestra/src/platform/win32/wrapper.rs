// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA, TranslateMessage, MSG};

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
