// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// This enumeration specifies the UI backend for Finestra.
///
/// ## Usage
/// You can override the backend for [`Apps`][super::App] using the
/// [`App::with_backend`](super::App::with_backend) function:
/// ```
/// # use finestra::*;
/// # #[derive(Default)]
/// # struct MyApplication;
/// # impl AppDelegate for MyApplication {};
/// App::new(MyApplication::default())
///     .with_backend(UIBackend::Win32);
/// ```
///
/// ## Remarks
/// The [`Default`] value of [`UIBackend`] is already the most appropriate for
/// the given platform. For advanced users, you can override this value, but in
/// most cases the given [`UIBackend`] is not available for that platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIBackend {
    /// [AppKit](https://developer.apple.com/documentation/appkit) is the native
    /// backend for macOS applications and part of the
    /// [Cocoa](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/WhatIsCocoa/WhatIsCocoa.html)
    /// SDK.
    AppKit,

    /// The [Win32](https://learn.microsoft.com/en-us/windows/win32/learnwin32/learn-to-program-for-windows)
    /// API is the low-level native Windows API, which is supported on every
    /// Windows version since Windows NT.
    Win32,
}

impl UIBackend {
    /// Use this function to check if the UI backend is available for the
    /// current platform.
    #[inline]
    #[must_use]
    pub fn is_available(&self) -> bool {
        match self {
            Self::AppKit => cfg!(target_os = "macos"),
            Self::Win32 => cfg!(target_os = "windows"),
        }
    }
}

#[cfg(target_os = "macos")]
impl Default for UIBackend {
    fn default() -> Self {
        Self::AppKit
    }
}

#[cfg(target_os = "windows")]
impl Default for UIBackend {
    fn default() -> Self {
        Self::Win32
    }
}
