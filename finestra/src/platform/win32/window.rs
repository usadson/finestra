// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::size_of_val;
use std::rc::Rc;
use std::{mem::size_of, sync::Once};

use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError, BOOL};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE};
use windows::Win32::Graphics::Gdi::{GetStockObject, UpdateWindow, BLACK_BRUSH, HDC, HOLLOW_BRUSH};
use windows::Win32::UI::WindowsAndMessaging::*;
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

use crate::event::{EventHandlerMapRegistry, ViewTree};
use crate::{AppDelegate, DialogBuilder, View, ViewId, WindowConfiguration};

use super::dialog::Win32Dialog;
use super::view::WinView;
use super::wrapper::{ControlId, Hwnd};

static CLASS_NAME: &str = "FinestraWindow\0";

#[derive(Clone)]
pub struct Window<Delegate, State> {
    hwnd: HWND,
    _phantom_delegate: PhantomData<Delegate>,
    _phantom_state: PhantomData<State>,
}

impl<Delegate, State> Window<Delegate, State>
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    pub fn new(config: WindowConfiguration, delegate: Delegate, state: State) -> Self {
        let hwnd = create_window::<Delegate, State>(config);
        let this = Self::from(hwnd);

        this.set_data(WindowData {
            hwnd,
            delegate,
            state,
            delegator: crate::Window::new(Rc::new(Self::from(hwnd))),
            view: None,
            registry: Default::default(),
        });
        this
    }

    pub fn show(&self) {
        let data = self.get_data().unwrap();
        data.delegate.will_show_window(data.delegator.clone(), &mut data.state);
        unsafe {
            ShowWindow(self.hwnd, SW_NORMAL);
        }
    }

    pub fn update(&self) {
        unsafe {
            UpdateWindow(self.hwnd);
        }
    }

    fn set_data(&self, data: WindowData<Delegate, State>) {
        let ptr = Box::leak(Box::new(data));
        let ptr = ptr as *mut _;
        let result = unsafe {
            SetWindowLongPtrA(self.hwnd, GWLP_USERDATA, ptr as isize)
        };

        if result == 0 {
            if let Err(e) = unsafe { GetLastError() } {
                panic!("Failed to set Window GWLP_USERDATA: {e}");
            }
        }
    }

    fn get_data(&self) -> Option<&'static mut WindowData<Delegate, State>> {
        let ptr = unsafe { GetWindowLongPtrA(self.hwnd, GWLP_USERDATA) };
        let ptr = ptr as *mut WindowData<Delegate, State>;

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }
}

impl<Delegate, State> From<HWND> for Window<Delegate, State> {
    fn from(value: HWND) -> Self {
        Self {
            hwnd: value,
            _phantom_delegate: Default::default(),
            _phantom_state: Default::default(),
        }
    }
}

pub struct WindowData<Delegate, State: 'static>
        where Delegate: AppDelegate<State> {
    hwnd: HWND,
    delegate: Delegate,
    state: State,
    delegator: crate::Window,
    view: Option<WinView>,
    registry: EventHandlerMapRegistry<State>,
}

impl<Delegate, State: 'static> WindowData<Delegate, State>
        where Delegate: AppDelegate<State> {
    fn make_content_view(&mut self) {
        let view = {
            let mut view = self.delegate.make_content_view(&mut self.state, self.delegator.clone());
            let registry = EventHandlerMapRegistry::<State>::default();
            let mut tree = ViewTree::<State>::new(registry.clone());
            self.registry = registry;
            view.build_native(&mut tree, self.hwnd)
        };

        view.install(&self);
        self.view = Some(view);
    }

    fn handle_control_notification(&mut self, notification: u16, control_id: ControlId, hwnd: HWND) {
        let id = ViewId(control_id.0 as _);
        let Some(control) = self.registry.map.get(&id) else {
            return;
        };

        match notification as u32 {
            BN_CLICKED => if let Some(callback) = &control.click {
                callback(&mut self.state, self.delegator.clone());
            }

            _ => println!("WM_COMMAND: Unknown notification: {notification:x}"),
        }
    }
}

impl<Delegate, State> crate::WindowDelegator for Window<Delegate, State> {
    fn create_dialog(&self, text: std::borrow::Cow<'static, str>) -> crate::DialogBuilder {
        DialogBuilder::new(Box::new(Win32Dialog::new(text.into_owned(), Hwnd::from(self.hwnd).get_text())))
    }
}

fn register_class<Delegate, State: 'static>()
        where Delegate: AppDelegate<State> + 'static {
    static REGISTRATION: Once = Once::new();
    REGISTRATION.call_once(register_class_impl::<Delegate, State>);
}

fn register_class_impl<Delegate, State: 'static>()
        where Delegate: AppDelegate<State> + 'static {
    let class = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as _,
        style: Default::default(),
        lpfnWndProc: Some(window_procedure::<Delegate, State>),
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

pub fn create_window<Delegate, State: 'static>(config: WindowConfiguration) -> HWND
        where Delegate: AppDelegate<State> + 'static{
    register_class::<Delegate, State>();

    let title = config.title.as_ref().with(|title| {
        format!("{title}\0")
    });
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

    Hwnd::from(hwnd).subscribe_text_update(config.title.as_ref().as_state());
    let value = BOOL(1);
    unsafe {
        let ptr = &value as *const BOOL;
        let ptr = ptr as *const c_void;
        DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, ptr, size_of_val(&value) as _)
            .unwrap();
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

unsafe extern "system" fn window_procedure<Delegate, State>(hwnd: HWND, message: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
        where Delegate: AppDelegate<State> + 'static, State: 'static {
    if message == WM_NCDESTROY {
        // Safety: WM_DESTROY already free'd the pointer, so no handling now please.
        return DefWindowProcA(hwnd, message, w_param, l_param);
    }

    let window = Window::from(hwnd);

    if message != WM_DESTROY {
        if let Some(result) = handle_procedure::<Delegate, State>(window, message, w_param, l_param) {
            return result;
        }
    }

    DefWindowProcA(hwnd, message, w_param, l_param)
}

fn handle_procedure<Delegate, State: 'static>(window: Window<Delegate, State>, message: u32, w_param: WPARAM, l_param: LPARAM) -> Option<LRESULT>
        where Delegate: AppDelegate<State> + 'static {
    let Some(data) = window.get_data() else {
        // There are some messages sent between CreateWindowEx and
        // SetWindowLongPtr, notably WM_GETMINMAXINFO, WM_NCCREATE,
        // WM_NCCALCSIZE, and WM_CREATE. We cannot really do anything
        // here without the data, so we just DefWindowProcA here.

        return None;
    };

    let w_param_high = ((w_param.0 as u32) >> 16) as u16;
    let w_param_low = w_param.0 as u16;

    match message {
        // <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close>
        WM_CLOSE => {
            unsafe { PostQuitMessage(0) }
            return Some(LRESULT(0));
        }

        // <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command>
        WM_COMMAND => {
            if l_param.0 != 0 {
                data.handle_control_notification(w_param_high, ControlId(w_param_low as _), HWND(l_param.0))
            } else {
                match w_param_high {
                    // Menu
                    0 => println!("  WM_COMMAND MENU {w_param_low:x}"),

                    // Accelerator
                    1 => println!("  WM_COMMAND ACCEL {w_param_low:x}"),

                    _ => unreachable!(),
                }
            }
        }

        WM_CTLCOLORSTATIC => {
            let hdc = HDC(w_param.0 as _);
            let hctl = HWND(l_param.0);
            return Some(LRESULT(unsafe { GetStockObject(BLACK_BRUSH) }.0));
        }

        WM_SHOWWINDOW => if data.view.is_none() {
            data.make_content_view();
        },

        _ => ()
    }

    None
}
