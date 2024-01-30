// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use windows::Win32::UI::WindowsAndMessaging::WM_QUIT;

use super::wrapper::{get_next_message, MsgExtensions};

pub fn run_message_pump() -> ! {
    loop {
        let message = get_next_message();

        if message.message == WM_QUIT {
            std::process::exit(message.wParam.0 as _);
        }

        message.translate();
        message.dispatch();
    }
}
