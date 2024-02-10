// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The theme of the window.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    /// The default theme of the system, dependent on the user preferences.
    #[default]
    Automatic,

    /// A light/white mode.
    Light,

    /// A dark/black mode.
    Dark,
}
