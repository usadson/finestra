// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{fmt::Debug, sync::{Arc, RwLock}};

use crate::{Color, SystemColor, Theme, ViewId};

/// A [`State`]ful [`Color`].
pub type ColorValue = State<Color>;

/// A [`State`]ful [`String`].
pub type TextValue = State<String>;

/// A [`State`]ful [`Theme`](Theme).
pub type ThemeValue = State<Theme>;

type Callback<T> = dyn Fn(&T);

/// The [`State`] primitive is the tool to modify the characteristics of
/// [`Views`](crate::View), by e.g. modifying a [`Label`](crate::Label)
/// when a [`Button`](crate::Button) is clicked:
///
/// ```no_run
/// # use finestra::*;
///
/// struct Application;
///
/// impl AppDelegate<AppState> for Application {
///     fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState> {
///         state.label.set("Clicked: 0");
///
///         Stack::vertical()
///             .with(Label::new(&state.label))
///             .with(Button::new("Press")
///                 .with_on_click(|state: &mut AppState, _| {
///                     state.count += 1;
///                     state.label.set(format!("Clicked: {}", state.count));
///                 }))
///     }
/// }
///
/// #[derive(Debug, Default)]
/// struct AppState {
///     count: usize,
///     label: TextValue,
/// }
/// ```
///
/// For more information, see the [crate documentation](https://github.com/usadson/finestra).
pub struct State<T> {
    inner: Arc<RwLock<StateInner<T>>>,
}

impl<T> State<T> {
    /// Create a new [`State`] with the given value. Note: you can use
    /// [`State::default()`] if your wrapped value supports it :)
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(StateInner {
                value,
                callbacks: Vec::new(),
            })),
        }
    }

    /// Set the new value of the state, which will notify the
    /// [`listeners`](Self::add_listener).
    pub fn set(&self, value: impl Into<T>) {
        self.set_with_origin(value, StateChangeOrigin::User);
    }

    /// Get the value within the state by using a visitor method.
    ///
    /// ```
    /// # use finestra::State;
    /// let state = State::new("Hello, world!".to_string());
    ///
    /// state.with(|value| {
    ///     println!("Value is: {value}");
    /// });
    /// ```
    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        let v = self.inner.as_ref().read().unwrap();
        f(&v.value)
    }

    /// Clone the inner value. Note: you can also use [`State::with()`] if you
    /// only need a temporary reference.
    ///
    /// ```
    /// # use finestra::State;
    /// let state = State::new("Hello, world!".to_string());
    /// let cloned: String = state.clone_inner();
    /// ```
    pub fn clone_inner(&self) -> T
            where T: Clone {
        self.with(Clone::clone)
    }

    /// Get a mutable reference to the value within the state by using a
    /// visitor method.
    ///
    /// ```
    /// # use finestra::State;
    /// let state = State::new("Hello, world!".to_string());
    ///
    /// state.with(|value| {
    ///     println!("Value is: {value}");
    /// });
    /// ```
    pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        let mut v = self.inner.as_ref().write().unwrap();
        f(&mut v.value)
    }

    /// Get subscribed by changes to the State. Note: the callback won't get
    /// invoked for the initial value, only for changes.
    pub fn add_listener<F: Fn(&T) + 'static>(&self, callback: F) {
        self.add_listener_with_origin(callback, StateChangeOrigin::User);
    }

    pub(crate) fn add_listener_with_origin<F: Fn(&T) + 'static>(&self, callback: F, origin: StateChangeOrigin) {
        self.inner.as_ref().write().unwrap().callbacks.push(StateCallback {
            callback: Box::new(callback),
            origin,
        });
    }

    pub(crate) fn set_with_origin(&self, value: impl Into<T>, origin: StateChangeOrigin) {
        let value = value.into();
        let mut inner = self.inner.write().unwrap();
        for callback in &inner.callbacks {
            if callback.origin != origin || origin == StateChangeOrigin::User {
                (callback.callback)(&value);
            }
        }
        inner.value = value;
    }
}

unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}

/// A convenient wrapper for [`State`] or the "raw" value. A bunch of APIs let
/// you call them with either of them, and this wrapper provides easy [`Into`]
/// implementations.
#[derive(Debug)]
pub enum StateOrRaw<T> {
    /// A raw value is a value that is applied once, but can't be updated using
    /// the [State] pattern.
    Raw(T),

    /// This value is reacted by Finestra, and potentially you, by updating the
    /// used value (such as the text of a [Label](crate::Label)).
    State(State<T>),
}

impl<T> StateOrRaw<T> {
    /// Get the value within, by using a visitor method.
    ///
    /// ```
    /// # use finestra::StateOrRaw;
    /// let state = StateOrRaw::Raw("Hello, world!".to_string());
    ///
    /// state.with(|value| {
    ///     println!("Value is: {value}");
    /// });
    /// ```
    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        match self {
            Self::Raw(t) => f(t),
            Self::State(t) => t.with(f),
        }
    }

    /// Clone the inner value. Note: you can also use [`StateOrRaw::with()`] if
    /// you only need a temporary reference.
    ///
    /// ```
    /// # use finestra::StateOrRaw;
    /// let value = StateOrRaw::Raw("Hello, world!".to_string());
    /// let cloned: String = value.clone_inner();
    /// ```
    pub fn clone_inner(&self) -> T
            where T: Clone {
        self.with(Clone::clone)
    }

    /// Get the [`State`] if applicable, otherwise [`None`].
    pub fn as_state(&self) -> Option<State<T>> {
        if let Self::State(state) = &self {
            Some(state.clone())
        } else {
            None
        }
    }
}

impl<T> Default for StateOrRaw<T>
       where T: Default {
    fn default() -> Self {
        Self::Raw(T::default())
    }
}

impl<T> From<T> for StateOrRaw<T> {
    fn from(value: T) -> Self {
        Self::Raw(value)
    }
}

impl From<&str> for State<String> {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl From<&str> for StateOrRaw<String> {
    fn from(value: &str) -> Self {
        Self::Raw(value.to_owned())
    }
}

impl<T> From<&State<T>> for StateOrRaw<T> {
    fn from(value: &State<T>) -> Self {
        Self::State(value.clone())
    }
}

impl<T> From<State<T>> for StateOrRaw<T> {
    fn from(value: State<T>) -> Self {
        Self::State(value)
    }
}

impl<T> Default for State<T>
        where T: Default {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(unused)]
pub(crate) enum StateChangeOrigin {
    User,
    Owner(ViewId),
    System,
}

struct StateInner<T> {
    value: T,
    callbacks: Vec<StateCallback<T>>,
}

impl From<SystemColor> for StateOrRaw<Color> {
    fn from(value: SystemColor) -> Self {
        Self::Raw(Color::system(value))
    }
}

struct StateCallback<T> {
    callback: Box<Callback<T>>,
    origin: StateChangeOrigin,
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn ensure_value_is_set_from_into() {
        let state = State::from("Hello");
        state.with(|val| {
            assert_eq!(val, "Hello");
        });

        state.with_mut(|val| {
            assert_eq!(val, "Hello");
        });
    }

    #[test]
    fn ensure_value_is_set_from_set() {
        let state = State::from("World");
        state.with(|val| {
            assert_eq!(val, "World");
        });

        state.with_mut(|val| {
            assert_eq!(val, "World");
        });

        state.set("Finestra");
        state.with(|val| {
            assert_eq!(val, "Finestra");
        });

        state.with_mut(|val| {
            assert_eq!(val, "Finestra");
        });
    }

    #[test]
    fn ensure_listener() {
        let state = State::new(1);

        let usages = Arc::new(AtomicUsize::new(0));

        let listener_usage = Arc::clone(&usages);
        state.add_listener(move |_| {
            listener_usage.fetch_add(1, Ordering::AcqRel);
        });

        state.set(59);
        state.set(3);

        state.with(|value| assert_eq!(*value, 3));
        assert_eq!(usages.load(Ordering::Acquire), 2);
    }
}
