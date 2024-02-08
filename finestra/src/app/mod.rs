// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod platform;

use crate::{View, Window, WindowConfiguration};

pub use self::platform::UIBackend;

/// This is the main entrypoint to the framework.
///
/// ```no_run
/// # use finestra::App;
/// let app = App::new(());
/// app.run();
/// ```
pub struct App<Delegate, State=()> {
    backend: UIBackend,
    pub(crate) delegate: Delegate,
    pub(crate) state: State,
}

impl<Delegate, State: 'static> App<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    pub fn new(delegate: Delegate) -> App<Delegate, ()> {
        App {
            backend: Default::default(),
            delegate,
            state: (),
        }
    }

    pub fn with_state<NewState>(delegate: Delegate, state: NewState) -> App<Delegate, NewState> {
        App {
            backend: Default::default(),
            delegate,
            state,
        }
    }

    /// Override the [`UIBackend`] for this application. Note that the default
    /// is already the most appropriate for the platform, but you can still
    /// override that behavior if you'd like.
    pub fn with_backend(mut self, backend: UIBackend) -> Self {
        self.backend = backend;
        self
    }
}

impl<Delegate, State> App<Delegate, State>
        where Delegate: AppDelegate<State> + 'static,
              State: 'static {
    pub fn delegate(&self) -> &Delegate {
        &self.delegate
    }

    pub fn run(self) -> ! {
        match self.backend {
            UIBackend::AppKit => {
                #[cfg(target_os = "macos")]
                { crate::platform::macos::run_app(self) }

                #[cfg(not(target_os = "macos"))]
                { panic!("The UIBackend::AppKit is not supported on this platform.") }
            }

            UIBackend::Win32 => {
                #[cfg(target_os = "windows")]
                { crate::platform::win32::run_app(self) }

                #[cfg(not(target_os = "windows"))]
                { panic!("The UIBackend::Win32 is not supported on this platform.") }
            }
        }
    }
}

/// A platform-agnostic delegation trait, which will be used as a facade for
/// these native frameworks.
#[allow(unused_variables)]
pub trait AppDelegate<State = ()>
        where State: 'static {
    /// Called when the underlying native framework finished launching.
    fn did_launch(&mut self, state: &mut State) { _ = state }

    /// Called when the underlying native framework finished launching.
    fn will_show_window(&mut self, window: Window, state: &mut State) { _ = state }

    /// Called when the main window must be configured. By overloading this
    /// function, you can - for example - give the window a name.
    fn configure_main_window(&mut self, state: &mut State) -> WindowConfiguration {
        _ = state;
        Default::default()
    }

    fn make_content_view(&mut self, state: &mut State, window: Window) -> impl View<Self, State>
            where Self: Sized {}
}

impl AppDelegate<()> for () {}
