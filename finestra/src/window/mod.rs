// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod config;
mod dialog;

use std::borrow::Cow;
use std::rc::Rc;

pub use self::config::*;
pub use self::dialog::*;

// Internal: this is a wrapper for invoking methods or making changes to the
//           window by user code.
#[derive(Clone)]
pub struct Window {
    delegator: Rc<dyn WindowDelegator>,
}

impl Window {
    pub(crate) fn new(delegator: Rc<dyn WindowDelegator>) -> Self {
        Self {
            delegator,
        }
    }

    /// Creates a new dialog. Use the [`DialogBuilder`] to set additional
    /// properties, before calling [`DialogBuilder::show()`].
    ///
    /// ## Example
    /// ```no_run
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
