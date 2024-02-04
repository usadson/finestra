// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    button::Button, image::ImageView, input::TextField, layout::{Layout, LayoutAnchorX, LayoutAnchorY}, listview::{ListView, ListViewRow}, progress::ProgressIndicator, scrollview::ScrollView, select::Select, switch::Switch, text::Label, utils::properties::ObjcProperty, view::View
};

use super::{cacao_delegates::MacOSTextFieldDelegate, nsstackview::NSStackView};


/// This internal type, represents [`Layout`](cacao::layout::Layout) in an
/// object-safe manner.
pub enum DynamicViewWrapper {
    Button(Button),
    ImageView(ImageView),
    Label(Label),
    ListView(ListView),
    ListViewRow(ListViewRow),
    ProgressIndicator(ProgressIndicator),
    ScrollView(ScrollView),
    Select(Select),
    StackView(NSStackView),
    Switch(Switch),
    #[allow(private_interfaces)]
    TextField(TextField<MacOSTextFieldDelegate>),
    View(View),
}

impl DynamicViewWrapper {
    /// The inverse of [`Layout::add_subview()`]
    pub(crate) fn add_to_view<V: Layout>(&self, view: &V) {
        match self {
            Self::Button(subview) => view.add_subview(subview),
            Self::ImageView(subview) => view.add_subview(subview),
            Self::Label(subview) => view.add_subview(subview),
            Self::ListView(subview) => view.add_subview(subview),
            Self::ListViewRow(subview) => view.add_subview(subview),
            Self::ProgressIndicator(subview) => view.add_subview(subview),
            Self::ScrollView(subview) => view.add_subview(subview),
            Self::Select(subview) => view.add_subview(subview),
            Self::StackView(subview) => subview.add_as_subview(view),
            Self::Switch(subview) => view.add_subview(subview),
            Self::TextField(subview) => view.add_subview(subview),
            Self::View(subview) => view.add_subview(subview),
        }
    }

    pub(crate) fn layout_constraint_center_x(&self) -> &LayoutAnchorX {
        match self {
            Self::Button(subview) => &subview.center_x,
            Self::ImageView(subview) => &subview.center_x,
            Self::Label(subview) => &subview.center_x,
            Self::ListView(subview) => &subview.center_x,
            Self::ListViewRow(subview) => &subview.center_x,
            Self::ProgressIndicator(subview) => &subview.center_x,
            Self::ScrollView(subview) => &subview.center_x,
            Self::Select(subview) => &subview.center_x,
            Self::StackView(subview) => &subview.center_x,
            Self::Switch(subview) => &subview.center_x,
            Self::TextField(subview) => &subview.center_x,
            Self::View(subview) => &subview.center_x,
        }
    }

    pub(crate) fn layout_constraint_center_y(&self) -> &LayoutAnchorY {
        match self {
            Self::Button(subview) => &subview.center_y,
            Self::ImageView(subview) => &subview.center_y,
            Self::Label(subview) => &subview.center_y,
            Self::ListView(subview) => &subview.center_y,
            Self::ListViewRow(subview) => &subview.center_y,
            Self::ProgressIndicator(subview) => &subview.center_y,
            Self::ScrollView(subview) => &subview.center_y,
            Self::Select(subview) => &subview.center_y,
            Self::StackView(subview) => &subview.center_y,
            Self::Switch(subview) => &subview.center_y,
            Self::TextField(subview) => &subview.center_y,
            Self::View(subview) => &subview.center_y,
        }
    }

    pub(crate) fn objc(&self) -> &ObjcProperty {
        match self {
            Self::Button(subview) => &subview.objc,
            Self::ImageView(subview) => &subview.objc,
            Self::Label(subview) => &subview.objc,
            Self::ListView(subview) => &subview.objc,
            Self::ListViewRow(subview) => &subview.objc,
            Self::ProgressIndicator(subview) => &subview.objc,
            Self::ScrollView(subview) => &subview.objc,
            Self::Select(subview) => &subview.objc,
            Self::StackView(subview) => &subview.objc,
            Self::Switch(subview) => &subview.objc,
            Self::TextField(subview) => &subview.objc,
            Self::View(subview) => &subview.objc,
        }
    }
}

impl From<Button> for DynamicViewWrapper {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<ImageView> for DynamicViewWrapper {
    fn from(value: ImageView) -> Self {
        Self::ImageView(value)
    }
}

impl From<Label> for DynamicViewWrapper {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<ListView> for DynamicViewWrapper {
    fn from(value: ListView) -> Self {
        Self::ListView(value)
    }
}

impl From<ListViewRow> for DynamicViewWrapper {
    fn from(value: ListViewRow) -> Self {
        Self::ListViewRow(value)
    }
}

impl From<NSStackView> for DynamicViewWrapper {
    fn from(value: NSStackView) -> Self {
        Self::StackView(value)
    }
}

impl From<ProgressIndicator> for DynamicViewWrapper {
    fn from(value: ProgressIndicator) -> Self {
        Self::ProgressIndicator(value)
    }
}

impl From<ScrollView> for DynamicViewWrapper {
    fn from(value: ScrollView) -> Self {
        Self::ScrollView(value)
    }
}

impl From<Select> for DynamicViewWrapper {
    fn from(value: Select) -> Self {
        Self::Select(value)
    }
}

impl From<Switch> for DynamicViewWrapper {
    fn from(value: Switch) -> Self {
        Self::Switch(value)
    }
}

impl From<TextField<MacOSTextFieldDelegate>> for DynamicViewWrapper {
    fn from(value: TextField<MacOSTextFieldDelegate>) -> Self {
        Self::TextField(value)
    }
}

impl From<View> for DynamicViewWrapper {
    fn from(value: View) -> Self {
        Self::View(value)
    }
}
