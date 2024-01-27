// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use euclid::Size2D;

use crate::Number;


#[derive(Default)]
pub struct WindowConfiguration {
    pub(crate) title: TitleWrapper,
    pub(crate) width: Number,
    pub(crate) height: Number,
}

impl WindowConfiguration {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_size<T>(self, size: Size2D<Number, T>) -> Self {
        Self {
            width: size.width,
            height: size.height,
            ..self
        }
    }

    #[must_use]
    pub fn with_title(self, title: impl Into<Cow<'static, str>>) -> Self {
        Self {
            title: TitleWrapper(title.into()),
            ..self
        }
    }
}

/// A simple wrapper, just to allow us to add a [`Default`] value.
#[derive(Debug)]
pub(crate) struct TitleWrapper(Cow<'static, str>);

impl Default for TitleWrapper {
    fn default() -> Self {
        Self("My Application".into())
    }
}

impl TitleWrapper {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
