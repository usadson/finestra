// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::event::ViewId;

#[derive(Debug)]
pub struct WinView {
    pub(crate) id: ViewId,
    pub(crate) kind: WinViewKind,
}

impl WinView {
    pub(crate) fn new(id: ViewId, kind: WinViewKind) -> Self {
        Self {
            id,
            kind,
        }
    }
}

#[derive(Debug)]
pub enum WinViewKind {
    Empty,
}
