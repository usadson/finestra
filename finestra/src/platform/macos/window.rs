// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cacao::appkit::window::{Window as CacaoWindow, WindowDelegate};
use cacao::layout::LayoutConstraint;

use crate::event::{EventHandlerMapRegistry, ViewTree};
use super::cacao_delegates::StatefulEventDispatcher;
use crate::{AppDelegate, DialogBuilder, View, Window, WindowDelegator};
use super::dynamic_wrapper::DynamicViewWrapper;
use super::dynamic_wrapper::LayoutExt;
use super::extensions::WindowExtensions;
use super::appkit::NSAlert;

pub(super) struct MacOSWindowDelegate<Delegate, State>
        where State: 'static {
    delegate: Rc<RefCell<Delegate>>,
    window: Option<Rc<CacaoWindow>>,

    #[allow(unused)]
    delegator: Option<Window>,
    view: cacao::view::View,
    content: Option<DynamicViewWrapper>,
    event_registry: EventHandlerMapRegistry<State>,
    state: Arc<Mutex<State>>,
}

impl<Delegate: AppDelegate<State>, State> MacOSWindowDelegate<Delegate, State> {
    pub(super) fn new(
        delegate: Rc<RefCell<Delegate>>,
        event_registry: EventHandlerMapRegistry<State>,
        state: Arc<Mutex<State>>,
    ) -> Self {
        Self {
            delegate,
            delegator: None,
            window: None,
            view: Default::default(),
            content: Default::default(),
            event_registry,
            state,
        }
    }
}

impl<Delegate, State> WindowDelegate for MacOSWindowDelegate<Delegate, State>
        where Delegate: AppDelegate<State> {
    const NAME: &'static str = "finestra";

    fn did_load(&mut self, window: CacaoWindow) {
        let window = Rc::new(window);
        debug_assert!(self.window.is_none());

        let user_delegator = Window::new(Arc::new(MacOSWindowDelegator {
            window: Rc::clone(&window),
        }));

        let dispatcher = StatefulEventDispatcher {
            state: self.state.clone(),
            event_registry: self.event_registry.clone(),
            window: user_delegator.clone(),
        };

        let mut delegate = self.delegate.as_ref().borrow_mut();

        let mut state = self.state.lock().unwrap();
        let mut content_view = delegate.make_content_view(&mut state, user_delegator);

        let mut tree = ViewTree::new(self.event_registry.clone(), dispatcher);
        let parent_id = tree.exchange_events_for_id(Default::default());
        tree.set_parent_id(parent_id);
        let content_view = content_view.build_native(&mut tree);

        content_view.add_to_view(&self.view);

        window.set_content_view(&self.view);

        if content_view.constraints().is_empty() {
            LayoutConstraint::activate(&[
                content_view.layout_constraint_center_x().constraint_equal_to(&self.view.center_x),
                content_view.layout_constraint_center_y().constraint_equal_to(&self.view.center_y),
            ]);
        } else {
            for constraint in content_view.constraints() {
                let constraint = constraint.convert(&content_view, &self.view);
                constraint.set_active(true);
            }
        }

        self.content = Some(content_view);
        self.window = Some(window);
    }
}

pub(super) struct MacOSWindowDelegator<CacaoDelegate> {
    pub(super) window: Rc<CacaoWindow<CacaoDelegate>>,
}

impl<CacaoDelegate> WindowDelegator for MacOSWindowDelegator<CacaoDelegate> {
    fn create_dialog(&self, text: Cow<'static, str>) -> crate::DialogBuilder {
        DialogBuilder::new(Box::new(NSAlert::with(text, self.window.get_title())))
    }
}
