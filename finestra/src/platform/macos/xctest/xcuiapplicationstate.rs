// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::foundation::NSUInteger;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum XCUIApplicationState {
    Unknown,
    NotRunning,
    RunningBackgroundSuspended,
    RunningBackground,
    RunningForeground,
}

impl From<NSUInteger> for XCUIApplicationState {
    fn from(value: NSUInteger) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::NotRunning,
            2 => Self::RunningBackgroundSuspended,
            3 => Self::RunningBackground,
            4 => Self::RunningForeground,
            _ => panic!("Invalid value {value} for XCUIApplicationState"),
        }
    }
}
