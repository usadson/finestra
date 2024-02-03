// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::foundation::{NSArray, NO, YES};
use cacao::layout::{Layout, LayoutAnchorX, LayoutAnchorY};
use cacao::utils::properties::ObjcProperty;
use objc_id::ShareId;
use cacao::objc::{class, msg_send, sel, sel_impl};

use crate::StackDirection;

pub struct NSStackView {
    pub objc: ObjcProperty,

    pub center_x: LayoutAnchorX,
    pub center_y: LayoutAnchorY,
}

impl NSStackView {
    pub fn new(direction: StackDirection) -> Self {
        let views = NSArray::new(&[]);

        let view: cacao::foundation::id = unsafe {
            msg_send![
                class!(NSStackView), stackViewWithViews:&*views
            ]
        };

        let orientation = direction as i32;

        unsafe {
            let _: () = msg_send![view, setWantsLayer: YES];
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];
            let _: () = msg_send![view, setOrientation: orientation]; // NSUserInterfaceLayoutOrientationVertical
            // let _: () = msg_send![view, setAlignment: 7]; // .width
            // let _: () = msg_send![view, setDistribution: 1]; // .fillEqually
        }

        Self {
            objc: ObjcProperty::retain(view),

            center_x: LayoutAnchorX::Center(unsafe { ShareId::from_ptr(msg_send![view, centerXAnchor]) }),
            center_y: LayoutAnchorY::Center(unsafe { ShareId::from_ptr(msg_send![view, centerYAnchor]) })
        }
    }

    pub fn add_view(&self, subview: &ObjcProperty) {
        self.objc.with_mut(|view| {
            subview.with_mut(|subview| {
                unsafe {
                    // let _: () = msg_send![view, addView:subview inGravity:gravity];
                    let _: () = msg_send![view, addArrangedSubview:subview];

                }
            });
        });
    }

    pub(crate) fn add_as_subview<V: Layout>(&self, view: &V) {
        view.with_backing_obj_mut(|backing_node| {
            self.objc.with_mut(|subview| {
                unsafe {
                    let _: () = msg_send![backing_node, addSubview: subview];
                }
            })
        });
    }
}
