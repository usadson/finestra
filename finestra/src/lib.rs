// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#![doc = include_str!("../../README.md")]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::default_trait_access)]
#![warn(missing_docs)]

pub(crate) type Number = f32;
pub use euclid;

mod app;
mod event;
mod layout;
mod platform;
mod property;
mod resources;
mod state;
mod views;
mod window;

pub use self::app::*;
pub(crate) use self::layout::*;
pub use self::property::*;
pub use self::resources::*;
pub use self::state::*;
pub use self::views::*;
pub use self::window::*;

pub(crate) use self::event::*;
