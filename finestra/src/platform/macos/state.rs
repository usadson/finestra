// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::event::{EventHandlerMap, EventHandlerMapRegistry, ViewId};

pub enum Event {
    ButtonClicked(ViewId),
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
