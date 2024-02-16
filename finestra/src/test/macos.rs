// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::path::Path;

use crate::platform::macos::XCUIApplication;

pub struct UIProcess {
    xc_app: XCUIApplication,
}

impl UIProcess {
    pub fn new(path: &Path) -> Self {
        Self {
            xc_app: XCUIApplication::init_with_url(path.to_str().unwrap()),
        }
    }

    pub fn wait_until_ready(&self) {
        self.xc_app.activate();
    }
}
