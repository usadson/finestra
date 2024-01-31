// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;
use dashmap::DashMap;

use crate::Window;

type EventHandlerCallback<State> = dyn Fn(&mut State, Window);
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

pub struct ViewTree<State> {
    id_generator: IdGenerator,
    registry: EventHandlerMapRegistry<State>,
}

impl<State> ViewTree<State> {
    pub(crate) fn new(registry: EventHandlerMapRegistry<State>) -> Self {
        Self {
            id_generator: Default::default(),
            registry,
        }
    }

    pub(crate) fn exchange_events_for_id(&mut self, map: EventHandlerMap<State>) -> ViewId {
        let id = self.id_generator.next();
        self.registry.map.insert(id, map);
        id
    }
}

#[derive(Debug, Default)]
pub(crate) struct IdGenerator {
    current_id: usize,
}

impl IdGenerator {
    pub(crate) fn next(&mut self) -> ViewId {
        let id = ViewId(self.current_id);
        self.current_id += 1;
        id
    }
}
