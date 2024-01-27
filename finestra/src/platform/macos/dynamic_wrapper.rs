// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    layout::{Layout, LayoutAnchorX, LayoutAnchorY}, text::Label, view::View
};

/// This internal type, represents [`Layout`](cacao::layout::Layout) in an
/// object-safe manner.
pub enum DynamicViewWrapper {
    Label(Label),
    View(View),
}

impl DynamicViewWrapper {
    /// The inverse of [`Layout::add_subview()`]
    pub(crate) fn add_to_view<V: Layout>(&self, view: &V) {
        match self {
            Self::Label(subview) => view.add_subview(subview),
            Self::View(subview) => view.add_subview(subview),
        }
    }

    pub(crate) fn layout_constraint_center_x(&self) -> &LayoutAnchorX {
        match self {
            Self::Label(subview) => &subview.center_x,
            Self::View(subview) => &subview.center_x,
        }
    }

    pub(crate) fn layout_constraint_center_y(&self) -> &LayoutAnchorY {
        match self {
            Self::Label(subview) => &subview.center_y,
            Self::View(subview) => &subview.center_y,
        }
    }
}

impl From<Label> for DynamicViewWrapper {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<View> for DynamicViewWrapper {
    fn from(value: View) -> Self {
        Self::View(value)
    }
}
