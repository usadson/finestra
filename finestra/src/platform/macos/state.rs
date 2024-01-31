// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::event::ViewId;

pub enum Event {
    ButtonClicked(ViewId),
}
