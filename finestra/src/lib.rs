// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::default_trait_access)]

pub type Number = f32;
pub use euclid;

mod app;
mod error;
mod event;
mod platform;
mod resources;
mod state;
mod views;
mod window;

pub use self::app::*;
pub use self::error::*;
pub use self::resources::*;
pub use self::state::*;
pub use self::views::*;
pub use self::window::*;
