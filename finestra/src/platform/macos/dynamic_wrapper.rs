// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    button::Button, image::ImageView, input::TextField, layout::{Layout, LayoutAnchorX, LayoutAnchorY}, listview::{ListView, ListViewRow}, progress::ProgressIndicator, scrollview::ScrollView, select::Select, switch::Switch, text::Label, utils::properties::ObjcProperty, view::View
};

use crate::{Constraint, ConstraintAlignment, ViewId};

use super::{cacao_delegates::MacOSTextFieldDelegate, NSStackView};

/// This internal type, represents [`Layout`](cacao::layout::Layout) in an
/// object-safe manner.
pub struct DynamicViewWrapper {
    kind: DynamicViewWrapperKind,
    constraints: Vec<Constraint>,
}

impl DynamicViewWrapper {
    pub(crate) fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    pub(crate) fn add_constraints_to_parent_box(&mut self, parent_id: ViewId) {
        self.add_constraint(Constraint {
            alignment: ConstraintAlignment::Top,
            reference_id: parent_id,
            reference_alignment: ConstraintAlignment::Top,
        });

        self.add_constraint(Constraint {
            alignment: ConstraintAlignment::Bottom,
            reference_id: parent_id,
            reference_alignment: ConstraintAlignment::Bottom,
        });

        self.add_constraint(Constraint {
            alignment: ConstraintAlignment::Left,
            reference_id: parent_id,
            reference_alignment: ConstraintAlignment::Left,
        });

        self.add_constraint(Constraint {
            alignment: ConstraintAlignment::Right,
            reference_id: parent_id,
            reference_alignment: ConstraintAlignment::Right,
        });
    }

    pub(crate) fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}

impl DynamicViewWrapper {
    pub(crate) fn add_to_view<V: Layout>(&self, view: &V) {
        self.kind.add_to_view(view)
    }

    pub(crate) fn objc(&self) -> &ObjcProperty {
        self.kind.objc()
    }
}

pub enum DynamicViewWrapperKind {
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

impl DynamicViewWrapperKind {
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

    pub(crate) fn layout_constraint_top(&self) -> &LayoutAnchorY {
        match self {
            Self::Button(subview) => &subview.top,
            Self::ImageView(subview) => &subview.top,
            Self::Label(subview) => &subview.top,
            Self::ListView(subview) => &subview.top,
            Self::ListViewRow(subview) => &subview.top,
            Self::ProgressIndicator(subview) => &subview.top,
            Self::ScrollView(subview) => &subview.top,
            Self::Select(subview) => &subview.top,
            Self::StackView(..) => todo!(),
            Self::Switch(subview) => &subview.top,
            Self::TextField(subview) => &subview.top,
            Self::View(subview) => &subview.top,
        }
    }

    pub(crate) fn layout_constraint_bottom(&self) -> &LayoutAnchorY {
        match self {
            Self::Button(subview) => &subview.bottom,
            Self::ImageView(subview) => &subview.bottom,
            Self::Label(subview) => &subview.bottom,
            Self::ListView(subview) => &subview.bottom,
            Self::ListViewRow(subview) => &subview.bottom,
            Self::ProgressIndicator(subview) => &subview.bottom,
            Self::ScrollView(subview) => &subview.bottom,
            Self::Select(subview) => &subview.bottom,
            Self::StackView(..) => todo!(),
            Self::Switch(subview) => &subview.bottom,
            Self::TextField(subview) => &subview.bottom,
            Self::View(subview) => &subview.bottom,
        }
    }

    pub(crate) fn layout_constraint_left(&self) -> &LayoutAnchorX {
        match self {
            Self::Button(subview) => &subview.left,
            Self::ImageView(subview) => &subview.left,
            Self::Label(subview) => &subview.left,
            Self::ListView(subview) => &subview.left,
            Self::ListViewRow(subview) => &subview.left,
            Self::ProgressIndicator(subview) => &subview.left,
            Self::ScrollView(subview) => &subview.left,
            Self::Select(subview) => &subview.left,
            Self::StackView(..) => todo!(),
            Self::Switch(subview) => &subview.left,
            Self::TextField(subview) => &subview.left,
            Self::View(subview) => &subview.left,
        }
    }

    pub(crate) fn layout_constraint_right(&self) -> &LayoutAnchorX {
        match self {
            Self::Button(subview) => &subview.right,
            Self::ImageView(subview) => &subview.right,
            Self::Label(subview) => &subview.right,
            Self::ListView(subview) => &subview.right,
            Self::ListViewRow(subview) => &subview.right,
            Self::ProgressIndicator(subview) => &subview.right,
            Self::ScrollView(subview) => &subview.right,
            Self::Select(subview) => &subview.right,
            Self::StackView(..) => todo!(),
            Self::Switch(subview) => &subview.right,
            Self::TextField(subview) => &subview.right,
            Self::View(subview) => &subview.right,
        }
    }
}

impl From<Button> for DynamicViewWrapperKind {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<ImageView> for DynamicViewWrapperKind {
    fn from(value: ImageView) -> Self {
        Self::ImageView(value)
    }
}

impl From<Label> for DynamicViewWrapperKind {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<ListView> for DynamicViewWrapperKind {
    fn from(value: ListView) -> Self {
        Self::ListView(value)
    }
}

impl From<ListViewRow> for DynamicViewWrapperKind {
    fn from(value: ListViewRow) -> Self {
        Self::ListViewRow(value)
    }
}

impl From<NSStackView> for DynamicViewWrapperKind {
    fn from(value: NSStackView) -> Self {
        Self::StackView(value)
    }
}

impl From<ProgressIndicator> for DynamicViewWrapperKind {
    fn from(value: ProgressIndicator) -> Self {
        Self::ProgressIndicator(value)
    }
}

impl From<ScrollView> for DynamicViewWrapperKind {
    fn from(value: ScrollView) -> Self {
        Self::ScrollView(value)
    }
}

impl From<Select> for DynamicViewWrapperKind {
    fn from(value: Select) -> Self {
        Self::Select(value)
    }
}

impl From<Switch> for DynamicViewWrapperKind {
    fn from(value: Switch) -> Self {
        Self::Switch(value)
    }
}

impl From<TextField<MacOSTextFieldDelegate>> for DynamicViewWrapperKind {
    fn from(value: TextField<MacOSTextFieldDelegate>) -> Self {
        Self::TextField(value)
    }
}

impl From<View> for DynamicViewWrapperKind {
    fn from(value: View) -> Self {
        Self::View(value)
    }
}

impl<T> From<T> for DynamicViewWrapper
        where T: Into<DynamicViewWrapperKind> {
    fn from(value: T) -> Self {
        Self {
            kind: value.into(),
            constraints: Vec::new(),
        }
    }
}

pub(crate) trait LayoutExt {
    fn layout_constraint_center_x(&self) -> &LayoutAnchorX;

    fn layout_constraint_center_y(&self) -> &LayoutAnchorY;

    fn layout_constraint_left(&self) -> &LayoutAnchorX;

    fn layout_constraint_right(&self) -> &LayoutAnchorX;

    fn layout_constraint_top(&self) -> &LayoutAnchorY;

    fn layout_constraint_bottom(&self) -> &LayoutAnchorY;
}

impl LayoutExt for DynamicViewWrapper {
    fn layout_constraint_center_x(&self) -> &LayoutAnchorX {
        self.kind.layout_constraint_center_x()
    }

    fn layout_constraint_center_y(&self) -> &LayoutAnchorY {
        self.kind.layout_constraint_center_y()
    }

    fn layout_constraint_left(&self) -> &LayoutAnchorX {
        self.kind.layout_constraint_left()
    }

    fn layout_constraint_right(&self) -> &LayoutAnchorX {
        self.kind.layout_constraint_right()
    }

    fn layout_constraint_top(&self) -> &LayoutAnchorY {
        self.kind.layout_constraint_top()
    }

    fn layout_constraint_bottom(&self) -> &LayoutAnchorY {
        self.kind.layout_constraint_bottom()
    }
}

impl<S> LayoutExt for View<S> {
    fn layout_constraint_center_x(&self) -> &LayoutAnchorX {
        &self.center_x
    }

    fn layout_constraint_center_y(&self) -> &LayoutAnchorY {
        &self.center_y
    }

    fn layout_constraint_left(&self) -> &LayoutAnchorX {
        &self.left
    }

    fn layout_constraint_right(&self) -> &LayoutAnchorX {
        &self.right
    }

    fn layout_constraint_top(&self) -> &LayoutAnchorY {
        &self.top
    }

    fn layout_constraint_bottom(&self) -> &LayoutAnchorY {
        &self.bottom
    }
}
