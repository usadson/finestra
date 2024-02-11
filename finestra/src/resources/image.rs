// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::path::PathBuf;

/// An image that can be used in an [ImageView](crate::ImageView).
#[derive(Debug, Default, PartialEq)]
pub struct Image {
    kind: ImageKind,
}

impl Image {
    /// An image that is loaded from the contents of the file, which is pointed
    /// to by the given [path](PathBuf).
    #[must_use]
    pub fn with_contents_of_file(path: impl Into<PathBuf>) -> Self {
        Self {
            kind: ImageKind::File(path.into()),
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn kind(&self) -> &ImageKind {
        &self.kind
    }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) enum ImageKind {
    #[default]
    None,
    File(PathBuf),
}
