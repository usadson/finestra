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
    pub fn kind(self, kind: DialogKind) -> Self {
        self.inner.set_kind(kind);
        self
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

#[derive(Clone, Debug, Default)]
pub enum DialogKind {
    /// A message box without a specific icon or accessibility information.
    #[default]
    Normal,

    /// A message box that tells the user about an event that occurred, or shows
    /// them information about a specific action.
    Informational,

    /// A message box that tells the user about a possible mistake the user or
    /// application made.
    Warning,

    /// A message box that tells the user about an (unrecoverable) error that
    /// the application encountered.
    Error,
}

pub(crate) trait DialogApi {
    fn set_kind(&self, kind: DialogKind);
    fn set_text(&self, text: Cow<'static, str>);
    fn set_title(&self, title: Cow<'static, str>);
    fn show(&self);
}
