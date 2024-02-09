// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use crate::Timer;

/// The cursor is the pointer that the user can move around with the mouse.
///
/// ```
/// # use finestra::{Cursor, SystemCursor};
///
/// let curosr = Cursor::system(SystemCursor::IBeam);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    pub(crate) kind: CursorKind,
}

impl Cursor {
    /// Creates a [`Cursor`] object using the cross-platform [`SystemCursor`]
    /// enumeration.
    pub fn system(cursor: SystemCursor) -> Self {
        Self {
            kind: CursorKind::System(cursor)
        }
    }

    /// Creates a [`Cursor`] object using a cursor that might not be available
    /// on the current platform.
    pub fn unstable(cursor: UnstableCursor, alternative: SystemCursor) -> Self {
        Self {
            kind: CursorKind::Unstable {
                cursor,
                alternative,
            }
        }
    }

    /// Show the given timer for a certain duration, starting from this
    /// function call.
    pub fn show_for(&self, duration: Duration) {
        self.push_internal();

        let cursor = self.clone();
        Timer::delayed_action(duration, move || {
            cursor.pop_internal();
        }).schedule();
    }
}

#[cfg(target_os = "macos")]
impl Cursor {
    fn push_internal(&self) {
        use cacao::appkit::Cursor as CacaoCursor;
        CacaoCursor::push(self.clone().into());
    }

    fn pop_internal(&self) {
        use cacao::appkit::Cursor as CacaoCursor;
        CacaoCursor::pop();
    }
}

#[cfg(not(target_os = "macos"))]
impl Cursor {
    fn push_internal(&self) {
        todo!();
    }

    fn pop_internal(&self) {
        todo!();
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::system(SystemCursor::Default)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CursorKind {
    Unstable {
        cursor: UnstableCursor,
        alternative: SystemCursor,
    },
    System(SystemCursor),
}

/// These cursor are guaranteed to be available on all platforms, or appropriate
/// alternatives that carry the meaning of the cursor exist.
///
/// ## Notes:
/// * On macOS, there is no way to set the wait (spinning) cursor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SystemCursor {
    /// The platform-dependent default cursor (most likely the
    /// [`SystemCursor::Arrow`]).
    #[default]
    Default,

    /// An arrow-like cursor, which is most likely the default cursor.
    ///
    /// * macOS: [arrowCursor](https://developer.apple.com/documentation/appkit/nscursor/1527160-arrowcursor)
    /// * Windows: [IDC_ARROW](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    Arrow,

    /// Used for indicating insertion points/high precision operations.
    ///
    /// * macOS: [crosshairCursor](https://developer.apple.com/documentation/appkit/nscursor/1525359-crosshaircursor)
    /// * Windows: [IDC_CROSS](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    CrossHair,

    /// Used for indicating that the action is not allowed, e.g. drag-and-drop,
    /// hover over a disabled element.
    ///
    /// * macOS: [operationNotAllowedCursor](https://developer.apple.com/documentation/appkit/nscursor/1525180-operationnotallowedcursor)
    /// * Windows: [IDC_NO](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    NotAllowed,

    /// The text selection cursor.
    ///
    /// * macOS: [IBeamCursor](https://developer.apple.com/documentation/appkit/nscursor/1526109-ibeamcursor)
    /// * Windows: [IDC_IBEAM](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    IBeam,

    /// The (open) hand cursor, often used for clickable components, like links
    /// and buttons.
    ///
    /// * macOS: [pointingHandCursor](https://developer.apple.com/documentation/appkit/nscursor/1531896-pointinghandcursor)
    /// * Windows: [IDC_HAND](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    Hand,

    /// Used for resizing the window or view from the bottom of the object.
    ///
    /// * macOS: [resizeDownCursor](https://developer.apple.com/documentation/appkit/nscursor/1531340-resizedowncursor)
    /// * Windows: [IDC_SIZENS](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    ResizeDown,

    /// Used for resizing the window or view from the left side of the object.
    ///
    /// * macOS: [resizeLeftCursor](https://developer.apple.com/documentation/appkit/nscursor/1535416-resizeleftcursor)
    /// * Windows: [IDC_SIZEWE](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    ResizeLeft,

    /// Used for resizing the window or view from the right side of the object.
    ///
    /// * macOS: [resizeRightCursor](https://developer.apple.com/documentation/appkit/nscursor/1526314-resizerightcursor)
    /// * Windows: [IDC_SIZEWE](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    ResizeRight,

    /// Used for resizing the window or view from the top of the object.
    ///
    /// * macOS: [resizeUpCursor](https://developer.apple.com/documentation/appkit/nscursor/1532226-resizeupcursor)
    /// * Windows: [IDC_SIZENS](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    ResizeUp,
}

/// The following cursors are available, that might be most appropriate for some
/// platforms, but don't have proper alternatives on other platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnstableCursor {
    /// A spinning cursor indicating that the application is busy processing,
    /// and cannot accept interactions from the user.
    ///
    /// * macOS: Not available.
    /// * Windows: [IDC_WAIT](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    Busy,

    /// A spinning cursor indicating that the application is busy processing,
    /// but can accept interactions from the user (where
    /// [`UnstableCursor::Busy`] can't).
    ///
    /// * macOS: Not available.
    /// * Windows: [IDC_APPSTARTING](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    BusyInBackground,

    /// A cursor that indicates that the item will disappear if the action is
    /// continued.
    ///
    /// * macOS: [disappearingItemCursor](https://developer.apple.com/documentation/appkit/nscursor/1534280-disappearingitemcursor?language=objc)
    /// * Windows: Not available.
    DisappearingItem,

    /// A cursor that indicates that the current context allows help from the
    /// application for the action.
    ///
    /// * macOS: Not available.
    /// * Windows: [IDC_HELP](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
    Help,
}

impl From<SystemCursor> for Cursor {
    fn from(value: SystemCursor) -> Self {
        Self::system(value)
    }
}

#[cfg(target_os = "macos")]
impl From<SystemCursor> for cacao::appkit::CursorType {
    fn from(value: SystemCursor) -> Self {
        match value {
            SystemCursor::Default => Self::Arrow,

            SystemCursor::Arrow => Self::Arrow,
            SystemCursor::CrossHair => Self::Crosshair,
            SystemCursor::Hand => Self::OpenHand,
            SystemCursor::IBeam => Self::IBeam,
            SystemCursor::NotAllowed => Self::OperationNotAllowed,
            SystemCursor::ResizeDown => Self::ResizeDown,
            SystemCursor::ResizeLeft => Self::ResizeLeft,
            SystemCursor::ResizeRight => Self::ResizeRight,
            SystemCursor::ResizeUp => Self::ResizeUp,
        }
    }
}

#[cfg(target_os = "macos")]
impl From<UnstableCursor> for Option<cacao::appkit::CursorType> {
    fn from(value: UnstableCursor) -> Self {
        use cacao::appkit::CursorType as Type;
        match value {
            UnstableCursor::Busy => None,
            UnstableCursor::BusyInBackground => None,
            UnstableCursor::DisappearingItem => Some(Type::DisappearingItem),
            UnstableCursor::Help => None,
        }
    }
}

#[cfg(target_os = "macos")]
impl From<Cursor> for cacao::appkit::CursorType {
    fn from(value: Cursor) -> Self {
        match value.kind {
            CursorKind::Unstable { cursor, alternative } => {
                let unstable: Option<_> = cursor.into();
                unstable.unwrap_or_else(|| alternative.into())
            }
            CursorKind::System(system) => system.into(),
        }
    }
}
