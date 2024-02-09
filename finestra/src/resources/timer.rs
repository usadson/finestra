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
/// timer.schedule_once();
/// ```
pub struct Timer<F: FnOnce()> {
    pub(crate) delay: Duration,
    pub(crate) action: Box<F>,
}

impl<F> Timer<F>
        where F: FnOnce() + 'static {
    /// Create a new [`Timer`] that will fire after the given delay.
    pub fn delayed_action(delay: Duration, f: F) -> Self {
        Self {
            delay,
            action: Box::new(f),
        }
    }

    pub fn schedule_once(self)
            where F: Send {
        match AppContext::backend() {
            UIBackend::AppKit => schedule_app_kit(self),
            UIBackend::Win32 => schedule_win32(self),
        }
    }
}

#[cfg(target_os = "macos")]
fn schedule_app_kit<F: FnOnce() + 'static>(timer: Timer<F>) {
    use crate::platform::macos::NSTimer;

    NSTimer::from(timer).run();
}

#[cfg(not(target_os = "macos"))]
fn schedule_app_kit<F: FnOnce() + Send + 'static>(timer: Timer<F>) {
    default_schedule_timer(timer);
}

#[cfg(target_os = "windows")]
fn schedule_win32<F: FnOnce() + 'static>(timer: Timer<F>) {
    todo!();
}

#[cfg(not(target_os = "windows"))]
fn schedule_win32<F: FnOnce() + Send + 'static>(timer: Timer<F>) {
    default_schedule_timer(timer);
}

fn default_schedule_timer<F: FnOnce() + Send + 'static>(timer: Timer<F>) {
    std::thread::Builder::new()
        .name("Finestra Timer Thread".into())
        .spawn(move || {
            std::thread::sleep(timer.delay);
            (timer.action)();
        })
        .unwrap();
}
