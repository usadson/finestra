// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::path::PathBuf;

use super::UIProcess;

/// A wrapper around UI Applications that can be used in test scenarios.
pub struct UITestApplication {
    path: PathBuf,
    process: UIProcess,
}

impl UITestApplication {
    /// Launch
    pub fn launch_by_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let process = UIProcess::new(&path);
        process.wait_until_ready();
        Self {
            path: path,
            process,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::{null, null_mut};

    use cacao::{foundation::NSString, objc::{msg_send, runtime::Object, sel, sel_impl}};

    use super::*;

    #[test]
    fn launch_example() {
        let test = || {
            let app = UITestApplication::launch_by_path("/Users/tager/Developer/Public/Finestra/showcase/showcase.app");
            _ = app;
        };

        if let Err(e) = unsafe { objc_exception::r#try(test) } {
            println!("EXCEPTION: {e:p}\n");

            if std::ptr::null() == e {
                return;
            }

            unsafe {
                let e = e as *mut Object;

                let name = msg_send![e, name];
                if name != null_mut() {
                    println!("   Name: {}", NSString::retain(name).to_string());
                }

                let reason = msg_send![e, reason];
                if reason != null_mut() {
                    println!("   Reason: {}", NSString::retain(reason).to_string());
                }
            }
        }
    }
}
