// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{mem::size_of, sync::Once};

use windows::core::PCSTR;
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::{
    Foundation::{
        HWND,
        LPARAM,
        LRESULT,
        WPARAM,
    },
    UI::WindowsAndMessaging::{
        CreateWindowExA,
        DefWindowProcA,
        RegisterClassExA,
        ShowWindow,
        CW_USEDEFAULT,
        SW_NORMAL,
        WNDCLASSEXA,
        WS_EX_LEFT,
        WS_OVERLAPPEDWINDOW
    },
};

use crate::WindowConfiguration;

pub struct Window {
    hwnd: HWND,
}

impl Window {
    pub fn new(config: WindowConfiguration) -> Self {
        Self {
            hwnd: create_window(config),
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_NORMAL);
        }
    }

    pub fn update(&self) {
        unsafe {
            UpdateWindow(self.hwnd);
        }
    }
}

static CLASS_NAME: &str = "FinestraWindow\0";

fn register_class() {
    static REGISTRATION: Once = Once::new();
    REGISTRATION.call_once(register_class_impl);
}

fn register_class_impl() {
    let class = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as _,
        style: Default::default(),
        lpfnWndProc: Some(window_procedure),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: Default::default(),
        hIcon: Default::default(),
        hCursor: Default::default(),
        hbrBackground: Default::default(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: PCSTR::from_raw(CLASS_NAME.as_ptr()),
        hIconSm: Default::default(),
    };

    let atom = unsafe { RegisterClassExA(&class) };

    if atom == 0 {
        panic!("Failed to register Window Class!");
    }
}

pub fn create_window(config: WindowConfiguration) -> HWND {
    register_class();

    let title = format!("{}\0", config.title.as_str());
    let title = PCSTR::from_raw(title.as_ptr());

    let hwnd = unsafe {
        CreateWindowExA(
            WS_EX_LEFT,
            PCSTR::from_raw(CLASS_NAME.as_ptr()),
            title,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            translate_size(config.width),
            translate_size(config.height),
            None,
            None,
            None,
            None
        )
    };

    if hwnd == Default::default() {
        panic!("Failed to create window!")
    }

    hwnd
}

fn translate_size(input: f32) -> i32 {
    let value = input as i32;

    if value == 0 {
        CW_USEDEFAULT
    } else {
        value
    }
}

unsafe extern "system" fn window_procedure(window: HWND, message: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    DefWindowProcA(window, message, w_param, l_param)
}
