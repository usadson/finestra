// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use block::ConcreteBlock;
use cacao::{foundation::NO, objc::{class, msg_send, runtime::Object, sel, sel_impl}};
use objc_id::Id;

use crate::Timer;

pub struct NSTimer {
    obj: Id<Object>,
}

impl NSTimer {
    pub fn run(self) {
        _ = self.obj;
    }
}

impl<F> From<Timer<F>> for NSTimer
        where F: FnOnce() + 'static {
    fn from(timer: Timer<F>) -> Self {
        let interval = convert_ns_time_interval(timer.delay);
        let action = timer.action;

        let action: *mut dyn FnOnce() = {
            Box::into_raw(action)
        };

        let block = ConcreteBlock::new(
            move |_: *mut Object| {
                unsafe {
                    let action: *mut dyn Fn() = std::mem::transmute(action);
                    (*action)();
                }

                _ = unsafe { Box::from_raw(action) };
            }
        );

        let obj = unsafe {
            let alert: cacao::foundation::id = msg_send![
                class!(NSTimer),
                scheduledTimerWithTimeInterval:interval
                repeats:NO
                block:block
            ];
            Id::from_ptr(alert)
        };

        Self {
            obj,
        }
    }
}

fn convert_ns_time_interval(duration: Duration) -> f64 {
    duration.as_secs_f64()
}
