// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

/// Builds and presents dialog boxes. To create a dialog box, use `window.create_dialog()`.
///
/// ## Example
/// ```rust,ignore
/// # use finestra::Window;
/// let window: Window;
///
/// window.create_dialog("My important message")
///         .title("Important App")
///         .show();
/// ```
pub struct DialogBuilder {
    inner: Box<dyn DialogApi>,
}

impl DialogBuilder {
    pub(crate) fn new(inner: Box<dyn DialogApi>) -> Self {
        Self {
            inner,
        }
    }

    /// Sets the title of the dialog. This is initially set to the title of the
    /// window.
    pub fn title(self, title: impl Into<Cow<'static, str>>) -> Self {
        self.inner.set_title(title.into());
        self
    }

    /// Shows the dialog box. Does not block.
    pub fn show(self) {
        self.inner.show()
    }
}

pub(crate) trait DialogApi {
    fn set_title(&self, title: Cow<'static, str>);
    fn show(&self);
}
