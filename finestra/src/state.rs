// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{fmt::Debug, sync::{Arc, RwLock}};

use crate::{Color, SystemColor};

pub type ColorValue = State<Color>;
pub type TextValue = State<String>;
type Callback<T> = dyn Fn(&T);

pub struct State<T> {
    inner: Arc<RwLock<StateInner<T>>>,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(StateInner {
                value,
                callbacks: Vec::new(),
            })),
        }
    }

    pub fn set(&self, value: impl Into<T>) {
        let value = value.into();

        let mut inner = self.inner.write().unwrap();
        for callback in &inner.callbacks {
            (callback)(&value);
        }
        inner.value = value;
    }

    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        let v = self.inner.as_ref().read().unwrap();
        f(&v.value)
    }

    pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        let mut v = self.inner.as_ref().write().unwrap();
        f(&mut v.value)
    }

    pub fn add_listener<F: Fn(&T) + 'static>(&self, callback: F) {
        self.inner.as_ref().write().unwrap().callbacks.push(Box::new(callback));
    }
}

#[derive(Debug)]
pub enum StateOrRaw<T> {
    Raw(T),
    State(State<T>),
}

impl<T> StateOrRaw<T> {
    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        match self {
            Self::Raw(t) => f(t),
            Self::State(t) => t.with(f),
        }
    }

    pub fn clone_inner(&self) -> T
            where T: Clone {
        self.with(Clone::clone)
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

struct StateInner<T> {
    value: T,
    callbacks: Vec<Box<Callback<T>>>,
}

impl From<SystemColor> for StateOrRaw<Color> {
    fn from(value: SystemColor) -> Self {
        Self::Raw(Color::system(value))
    }
}
