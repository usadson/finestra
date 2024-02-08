// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;
use dashmap::DashMap;

#[cfg(target_os = "macos")]
use crate::platform::macos::cacao_delegates::{EventDispatcher, StatefulEventDispatcher};

use crate::Window;

type EventHandlerCallback<State> = dyn Fn(&mut State, Window);
type EventHandler<State> = Option<Box<EventHandlerCallback<State>>>;

type BoolEventHandlerCallback<State> = dyn Fn(&mut State, bool, Window);
type BoolEventHandler<State> = Option<Box<BoolEventHandlerCallback<State>>>;

type TextEventHandlerCallback<State> = dyn Fn(&mut State, String, Window);
type TextEventHandler<State> = Option<Box<TextEventHandlerCallback<State>>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ViewId(pub usize);

pub(crate) struct EventHandlerMap<State> {
    pub(crate) click: EventHandler<State>,
    pub(crate) checked: BoolEventHandler<State>,
    pub(crate) text_changed: TextEventHandler<State>,
}

impl<State> Default for EventHandlerMap<State> {
    fn default() -> Self {
        Self {
            click: None,
            checked: None,
            text_changed: None,
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

    #[cfg(target_os = "macos")]
    dispatcher: StatefulEventDispatcher<State>,

    parent_id: Option<ViewId>,
}

impl<State: 'static> ViewTree<State> {
    #[cfg(not(target_os = "macos"))]
    pub(crate) fn new(registry: EventHandlerMapRegistry<State>) -> Self {
        Self {
            id_generator: Default::default(),
            registry,
            parent_id: None,
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(registry: EventHandlerMapRegistry<State>, dispatcher: StatefulEventDispatcher<State>) -> Self {
        Self {
            id_generator: Default::default(),
            registry,
            dispatcher,
            parent_id: None,
        }
    }

    pub(crate) fn exchange_events_for_id(&mut self, map: EventHandlerMap<State>) -> ViewId {
        let id = self.id_generator.next();
        self.registry.map.insert(id, map);
        id
    }

    #[cfg(windows)]
    pub(crate) fn put_event_handlers_with_id(&mut self, id: ViewId, map: EventHandlerMap<State>) -> ViewId {
        self.registry.map.insert(id, map);
        id
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn create_dispatcher(&self) -> Box<dyn EventDispatcher> {
        Box::new(self.dispatcher.clone())
    }

    pub(crate) fn parent_id(&self) -> Option<ViewId> {
        self.parent_id
    }

    pub(crate) fn set_parent_id(&mut self, id: ViewId) {
        self.parent_id = Some(id);
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
