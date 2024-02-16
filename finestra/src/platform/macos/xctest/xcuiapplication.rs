// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::{foundation::{id, NSString, NSUInteger, NSURL}, objc::{class, msg_send, runtime::Class, sel, sel_impl}, utils::properties::ObjcProperty};

use super::XCUIApplicationState;

pub(crate) struct XCUIApplication {
    objc: ObjcProperty,
}

impl XCUIApplication {
    /// <https://developer.apple.com/documentation/xctest/xcuiapplication/2879413-initwithurl?language=objc>
    pub fn init_with_url(uurl: &str) -> Self {
        super::load_xc_test_into_bundle();

        let url = NSString::new(uurl);
        let obj: id = unsafe {
            let url: id = msg_send![class!(NSURL), URLWithString:&*url];
            println!("URL: {url:p} from \"{uurl}\"");

            // let obj: id = msg_send![class!(XCUIApplication), new];
            // let obj: id = msg_send![obj, initWithURL:url];
            let obj: id = msg_send![class!(XCUIApplication), alloc];
            let obj: id = msg_send![obj, initWithURL:url];
            obj
        };

        Self {
            objc: ObjcProperty::retain(obj),
        }
    }

    pub fn activate(&self) {
        let _: () = self.objc.get(|objc| unsafe {
            msg_send![objc, activate]
        });
    }

    pub fn state(&self) -> XCUIApplicationState {
        let id: NSUInteger = self.objc.get(|objc| unsafe {
            msg_send![
                objc, state
            ]
        });

        id.into()
    }
}

impl Drop for XCUIApplication {
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, terminate];
        })
    }
}
