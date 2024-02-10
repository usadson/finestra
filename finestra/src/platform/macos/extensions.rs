// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{
    appkit::window::Window as CacaoWindow,
    color::Color,
    foundation::{id, NSString, NO, YES},
    layout::Layout,
    objc::{class, msg_send, sel, sel_impl}
};

use crate::Theme;

extern "C" {
    static NSAppearanceNameAqua: id;
    static NSAppearanceNameDarkAqua: id;
}

pub trait WindowExtensions {
    fn get_title(&self) -> String;
    fn set_appearance(&self, theme: Theme);

    #[allow(unused)]
    fn get_appearance(&self) -> Theme;
}

impl<T> WindowExtensions for CacaoWindow<T> {
    fn get_title(&self) -> String {
        let title: NSString = unsafe {
            msg_send![&*self.objc, title]
        };

        title.to_string()
    }

    // https://developer.apple.com/documentation/appkit/nsappearancecustomization/1533925-appearance
    fn set_appearance(&self, theme: Theme) {
        let appearance = theme_to_ns_appearance(theme);

        let _: () = unsafe {
            msg_send![&*self.objc, setAppearance:appearance]
        };
    }

    fn get_appearance(&self) -> Theme {
        let appearance: id = unsafe {
            msg_send![&*self.objc, effectiveAppearance]
        };

        if appearance == unsafe { NSAppearanceNameAqua } {
            return Theme::Light;
        }

        if appearance == unsafe { NSAppearanceNameDarkAqua } {
            return Theme::Dark;
        }

        Theme::Automatic // todo
    }
}

// https://developer.apple.com/documentation/appkit/nsappearance/1529612-appearancenamed?
fn theme_to_ns_appearance(theme: Theme) -> id {
    let name = match theme {
        Theme::Automatic => return 0 as _,

        Theme::Light => unsafe { NSAppearanceNameAqua },
        Theme::Dark => unsafe { NSAppearanceNameDarkAqua },
    };

    unsafe {
        msg_send![class!(NSAppearance), appearanceNamed:name]
    }
}

pub(crate) trait ViewExtensions: Layout {
    fn set_needs_layer(&self, value: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setNeedsDisplay:match value {
                true => YES,
                false => NO
            }];
        });
    }
}

impl<T> ViewExtensions for T where T: Layout {}

pub trait ButtonExtensions: Layout {
    fn set_bezel_color(&self, color: Color) {
        let color: id = color.as_ref().into();

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBezelColor: color];
        });
    }

    fn set_button_type(&self, type_: NSButtonType) {
        let type_ = type_ as u32;

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setButtonType:type_];
        });
    }
}

impl ButtonExtensions for cacao::button::Button {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSButtonType {
    Switch = 3,
}
