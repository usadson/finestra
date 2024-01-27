// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{View, WindowConfiguration};

/// This is the main entrypoint to the framework.
///
/// ```no_run
/// # use finestra::App;
/// let app = App::new(());
/// app.run();
/// ```
pub struct App<Delegate> {
    pub(crate) delegate: Delegate,
}

impl<Delegate> App<Delegate>
        where Delegate: AppDelegate + 'static {
    pub fn new(delegate: Delegate) -> Self {
        Self {
            delegate,
        }
    }

    pub fn delegate(&self) -> &Delegate {
        &self.delegate
    }

    pub fn run(self) -> ! {
        #[cfg(target_os = "macos")]
        { crate::platform::macos::run_app(self) }

        #[cfg(not(target_os = "macos"))]
        { panic!("Invalid platform") }
    }
}

/// A platform-agnostic delegation trait, which will be used as a facade for
/// these native frameworks.
pub trait AppDelegate {
    /// Called when the underlying native framework finished launching.
    fn did_launch(&mut self) {}

    /// Called when the underlying native framework finished launching.
    fn will_show_window(&mut self) {}

    /// Called when the main window must be configured. By overloading this
    /// function, you can - for example - give the window a name.
    fn configure_main_window(&mut self) -> WindowConfiguration {
        Default::default()
    }

    fn make_content_view(&mut self) -> impl View {}
}

impl AppDelegate for () {}
