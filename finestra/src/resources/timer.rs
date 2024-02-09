// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use crate::{AppContext, UIBackend};

/// Use this timer to delay a certain action.
///
/// ```
/// # use finestra::Timer;
/// use std::time::Duration;
///
/// let timer = Timer::delayed_action(Duration::from_secs(1), || {
///     println!("Hello, world!");
/// });
///
/// timer.schedule();
/// ```
pub struct Timer {
    pub(crate) delay: Duration,
    pub(crate) action: Box<dyn Fn()>,
}

impl Timer {
    /// Create a new [`Timer`] that will fire after the given delay.
    pub fn delayed_action<F: Fn() + 'static>(delay: Duration, f: F) -> Self {
        Self {
            delay,
            action: Box::new(f),
        }
    }

    pub fn schedule(self) {
        match AppContext::backend() {
            UIBackend::AppKit => schedule_app_kit(self),
            UIBackend::Win32 => schedule_win32(self),
        }
    }
}

#[cfg(target_os = "macos")]
fn schedule_app_kit(timer: Timer) {
    use crate::platform::macos::NSTimer;

    NSTimer::from(timer).run();
}

#[cfg(not(target_os = "macos"))]
fn schedule_app_kit(timer: Timer) {
    _ = timer;
    panic!("AppKit Timers are not available on this platform.");
}

#[cfg(target_os = "windows")]
fn schedule_win32(timer: Timer) {
    _ = timer;
    todo!();
}

#[cfg(not(target_os = "windows"))]
fn schedule_win32(timer: Timer) {
    _ = timer;
    panic!("Win32 Timers are not available on this platform.");
}
