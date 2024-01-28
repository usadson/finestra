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
pub struct App<Delegate, State=()> {
    pub(crate) delegate: Delegate,
    pub(crate) state: State,
}

impl<Delegate, State> App<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    pub fn new(delegate: Delegate) -> App<Delegate, ()> {
        App {
            delegate,
            state: (),
        }
    }

    pub fn with_state<NewState>(delegate: Delegate, state: NewState) -> App<Delegate, NewState> {
        App {
            delegate,
            state,
        }
    }
}

impl<Delegate, State> App<Delegate, State>
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {
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
pub trait AppDelegate<State = ()> {
    /// Called when the underlying native framework finished launching.
    fn did_launch(&mut self) {}

    /// Called when the underlying native framework finished launching.
    fn will_show_window(&mut self) {}

    /// Called when the main window must be configured. By overloading this
    /// function, you can - for example - give the window a name.
    fn configure_main_window(&mut self) -> WindowConfiguration {
        Default::default()
    }

    #[allow(unused_variables)]
    fn make_content_view(&mut self, state: &mut State) -> impl View<Self, State>
            where Self: Sized {}
}

impl AppDelegate<()> for () {}
