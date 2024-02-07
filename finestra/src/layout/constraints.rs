// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::ViewId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Constraint {
    pub alignment: ConstraintAlignment,

    pub reference_id: ViewId,
    pub reference_alignment: ConstraintAlignment,
}

#[cfg(target_os = "macos")]
impl Constraint {
    pub fn convert(
            &self,
            this: &impl crate::platform::macos::LayoutExt,
            reference: &impl crate::platform::macos::LayoutExt,
        ) -> cacao::layout::LayoutConstraint {
        _ = self.reference_alignment; // TODO

        match self.alignment {
            ConstraintAlignment::Top => {
                this.layout_constraint_top().constraint_less_than_or_equal_to(reference.layout_constraint_top())
            }

            ConstraintAlignment::Bottom => {
                this.layout_constraint_bottom().constraint_less_than_or_equal_to(reference.layout_constraint_bottom())
            }

            ConstraintAlignment::Left => {
                this.layout_constraint_left().constraint_less_than_or_equal_to(reference.layout_constraint_left())
            }

            ConstraintAlignment::Right => {
                this.layout_constraint_right().constraint_less_than_or_equal_to(reference.layout_constraint_right())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConstraintAlignment {
    Left,
    Right,
    Top,
    Bottom,
}
