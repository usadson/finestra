// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cacao::foundation::{id, NSString};
use cacao::objc::{msg_send, sel, sel_impl};
use cacao::{button::Button as CacaoButton, utils::properties::ObjcProperty};
use cacao::text::Label as CacaoLabel;

use crate::{BaseView, Color, StateOrRaw};
use crate::{event::ViewId, Button, Label};

use super::resources::ToCacao;

#[derive(Debug)]
pub enum Event {
    ButtonClicked(ViewId),
}

pub fn attach_base_state(finestra: &dyn BaseView, objc: &ObjcProperty) {
    hook_tooltip_state(objc, &finestra.base().tooltip);
}

pub fn attach_button_state<S>(finestra: &Button<S>, cacao: &CacaoButton) {
    attach_base_state(finestra, &cacao.objc);
    hook_background_color_state(&cacao.objc, &finestra.background_color);
    hook_text_color_state(&cacao.objc, &finestra.text_color);
    hook_title_state(&cacao.objc, &finestra.text);
}

pub fn attach_label_state<S>(finestra: &Label<S>, cacao: &CacaoLabel) {
    attach_base_state(finestra, &cacao.objc);
    hook_background_color_state(&cacao.objc, &finestra.background_color);
    hook_text_color_state(&cacao.objc, &finestra.text_color);
    hook_string_value_state(&cacao.objc, &finestra.text);
}

fn hook_background_color_state(objc: &ObjcProperty, color: &StateOrRaw<Color>) {
    let StateOrRaw::State(color_state) = &color else {
        return;
    };

    let objc = objc.clone();
    color_state.add_listener(move |val| {
        let color = val.to_cacao().unwrap();

        objc.with_mut(|obj| unsafe {
            let color = color.as_ref().cg_color();
            let layer: id = msg_send![obj, layer];
            let _: () = msg_send![layer, setBackgroundColor: color];
        });
    });
}
fn hook_string_value_state(objc: &ObjcProperty, text: &StateOrRaw<String>) {
    let StateOrRaw::State(text_state) = &text else {
        return;
    };

    let objc = objc.clone();
    text_state.add_listener(move |val| {
        let s = NSString::new(val);

        objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setStringValue:&*s];
        });
    });
}

fn hook_text_color_state(objc: &ObjcProperty, color: &StateOrRaw<Color>) {
    let StateOrRaw::State(color_state) = &color else {
        return;
    };

    let objc = objc.clone();
    color_state.add_listener(move |val| {
        let color: id = val.to_cacao().unwrap().as_ref().into();

        objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTextColor: color];
        });
    });
}

fn hook_title_state(objc: &ObjcProperty, text: &StateOrRaw<String>) {
    let StateOrRaw::State(text_state) = &text else {
        return;
    };

    let objc = objc.clone();
    text_state.add_listener(move |val| {
        let s = NSString::new(val);

        objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTitle:&*s];
        });
    });
}

fn hook_tooltip_state(objc: &ObjcProperty, text: &StateOrRaw<String>) {
    let StateOrRaw::State(text_state) = &text else {
        return;
    };

    text_state.with(|val| {
        let s = NSString::new(val);
        objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setToolTip:&*s];
        });
    });

    let objc = objc.clone();
    text_state.add_listener(move |val| {
        let s = NSString::new(val);

        objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setToolTip:&*s];
        });
    });
}
