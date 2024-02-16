// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod xcuiapplication;
mod xcuiapplicationstate;

use std::{io::{stdout, Write}, sync::Once};

use cacao::{foundation::NSString, objc::runtime::Class};

pub(crate) use self::{
    xcuiapplication::XCUIApplication,
    xcuiapplicationstate::XCUIApplicationState,
};

const BUNDLE_LOAD: Once = Once::new();

pub(crate) fn load_xc_test_into_bundle() {
    println!("LOADBUNDLE");

    use cacao::objc::{class, msg_send, sel, sel_impl};
    use cacao::objc::runtime::Object;
    BUNDLE_LOAD.call_once(|| {
        let result = unsafe {
            objc_exception::r#try(|| {
                let path = NSString::new("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks/XCTest.framework");
                let xctest_framework: *const Object = msg_send![class!(NSBundle), bundleWithPath: &*path.objc];
                let _: () = msg_send![xctest_framework, load];
            })
        };

        if let Err(e) = result {
            panic!("Failed to load bundle: {e:p}");
        }
    });

    for class in Class::classes().into_iter() {
        if !class.name().starts_with("XCTestConfiguration") { continue }
        eprintln!("\n\nClass: {}", class.name());

        for protocol in class.adopted_protocols().iter() {
            eprintln!("  Protocol: {}", protocol.name());
        }

        for variable in class.instance_variables().iter() {
            eprintln!("  Variable: {}", variable.name());
        }

        for method in class.instance_methods().iter() {
            eprintln!("  Method: {}", method.name().name());
            eprintln!("      Return Type: {}", method.return_type().as_str());
            eprintln!("      Arguments: {}", method.arguments_count());
            for i in 0..method.arguments_count() {
                eprintln!("          * Argument {i}: {:?}", method.argument_type(i).map(|x| x.as_str().to_owned()).unwrap_or_default());
            }
        }
    }

    println!("done load");
}
