// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;
use dashmap::DashMap;

type EventHandlerCallback<State> = dyn Fn(&mut State);
type EventHandler<State> = Option<Box<EventHandlerCallback<State>>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ViewId(pub usize);

pub(crate) struct EventHandlerMap<State> {
    pub(crate) click: EventHandler<State>,
}

impl<State> Default for EventHandlerMap<State> {
    fn default() -> Self {
        Self {
            click: None,
        }
    }
}

pub(crate) struct EventHandlerMapRegistry<State> {
    pub(crate) map: Arc<DashMap<ViewId, EventHandlerMap<State>>>,
}

impl<State> Default for EventHandlerMapRegistry<State> {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}

impl<State> Clone for EventHandlerMapRegistry<State> {
    fn clone(&self) -> Self {
        Self { map: Arc::clone(&self.map) }
    }
}
