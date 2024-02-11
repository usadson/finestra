// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod config;
mod dialog;

use std::borrow::Cow;
use std::sync::Arc;

pub use self::config::*;
pub use self::dialog::*;

// Internal: this is a wrapper for invoking methods or making changes to the
//           window by user code.
/// A reference to a Window. Use this to perform certain actions at runtime.
/// If you want to modify the look and feel, use]
/// [WindowConfiguration](super::WindowConfiguration).
#[derive(Clone)]
pub struct Window {
    delegator: Arc<dyn WindowDelegator>,
}

impl Window {
    pub(crate) fn new(delegator: Arc<dyn WindowDelegator>) -> Self {
        Self {
            delegator,
        }
    }

    /// Creates a new dialog. Use the [`DialogBuilder`] to set additional
    /// properties, before calling [`DialogBuilder::show()`].
    ///
    /// ## Example
    /// ```rust,no_run,ignore
    /// # use finestra::Window;
    /// let window: Window;
    ///
    /// window.create_dialog("My important message")
    ///         .title("Important App")
    ///         .show();
    /// ```
    #[inline]
    pub fn create_dialog(&self, text: impl Into<Cow<'static, str>>) -> DialogBuilder {
        self.delegator.create_dialog(text.into())
    }
}

pub(crate) trait WindowDelegator {
    fn create_dialog(&self, text: Cow<'static, str>) -> DialogBuilder;
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
