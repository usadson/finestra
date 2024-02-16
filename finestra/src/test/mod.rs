// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains various testing facilities to aid User Interface Tests.

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::*;

#[cfg(not(target_os = "macos"))]
mod stub;

#[cfg(not(target_os = "macos"))]
use stub::*;

mod application;
