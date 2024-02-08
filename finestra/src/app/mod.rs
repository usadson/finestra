// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod platform;

use crate::{View, Window, WindowConfiguration};

pub use self::platform::UIBackend;

/// This is the main entrypoint to the framework. You have to implement the
/// [`AppDelegate`] to get notified of specific app lifecycle events.
///
/// ## Example
/// ```no_run
/// # use finestra::App;
/// let app = App::new(());
/// app.run();
/// ```
///
/// ## App Lifecycle
/// The app lifecycle is initiated by the [`App::run()`] function, and after
/// that, the platform takes over. Finestra ensures the order of the delegate
/// invocations are consistent:
/// 1. Instantiate an [`App`] and call the [`run()`](App::run) method.
/// 2. The platform will make preparations for your app, and after loading whats
///    necessary, it will call [`AppDelegate::did_launch()`]
/// 3. Finestra ensures the creation of a main window, and will call
///    [`AppDelegate::configure_main_window()`]. You can override the default
///    implementation to e.g. set a title by using
///    [`WindowConfiguration::with_title()`].
/// 4. After configuring the window, the content of the window must be
///    determined. This is done by calling [`AppDelegate::make_content_view()`].
///    Use [`Labels`](crate::Label), [`Stacks`](crate::Stack) and other views to
///    build your user interface.
/// 5. Finestra will translate those views into the platform-dependent
///    primitives. Before the window is shown, you get a chance to do some
///    last-minute preparations by overriding
///    [`AppDelegate::will_show_window()`]. You also get a reference to the
///    [`Window`].
/// 6. After that, you're done! Respond to events and such by using
///    the [`State<T>`](crate::State) type, or specific-events like
///    [`Button::with_on_click()`](crate::Button::with_on_click()).
pub struct App<Delegate, State=()> {
    backend: UIBackend,
    pub(crate) delegate: Delegate,
    pub(crate) state: State,
}

impl<Delegate, State: 'static> App<Delegate, State>
        where Delegate: AppDelegate<State> + 'static {
    /// This is the main entrypoint to the framework.
    ///
    /// ```no_run
    /// # use finestra::App;
    /// let app = App::new(());
    /// app.run();
    /// ```
    pub fn new(delegate: Delegate) -> App<Delegate, ()> {
        App {
            backend: Default::default(),
            delegate,
            state: (),
        }
    }

    /// With this function, you can specify a `State` parameter that gets passed
    /// to invocation by the platform to your delegate.
    ///
    /// ```no_run
    /// # use finestra::{App, AppDelegate, State};
    /// #[derive(Default)]
    /// struct AppState {
    ///     label: State<String>,
    /// }
    ///
    /// let app = App::with_state((), AppState::default());
    /// app.run();
    /// ```
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
    /// Get the delegate passed by [`Self::new()`] or [`Self::with_state()`].
    pub fn delegate(&self) -> &Delegate {
        &self.delegate
    }

    /// Start the application. At this point, you can no longer make changes to
    /// the application configuration. The next thing that gets called is the
    /// [`AppDelegate::did_launch()`] to signify the application was made aware
    /// of to the platform.
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
