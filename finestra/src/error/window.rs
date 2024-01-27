// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WindowShowError {
    #[error("Window is already shown")]
    AlreadyShowing,

    #[error("An I/O error occurred: {error}")]
    OsError { error: std::io::Error },
}
