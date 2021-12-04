use wasm_bindgen::{prelude::*, convert::IntoWasmAbi, describe::WasmDescribe};
use std::{ptr::NonNull, time::Duration};
use crate::{graph::{renderer::{Renderer, GenericRenderer}, window::Window, shaders::program::{Program, GenericProgram}, mesh::Mesh}, renderers::webgl::WebGL};
use super::{scene::Scene, input::{keyboard::KeyboardListener, mouse::MouseListener}};

pub struct Script<R: Renderer> {
    pub start: Option<fn(&mut Scene<R>)>,
    pub update: Option<fn(&mut Scene<R>, &R::KeyboardListenerType, &R::MouseListenerType, &Duration)>
}

impl<R: Renderer> Script<R> {
    pub fn new (start: fn(&mut Scene<R>), update: fn(&mut Scene<R>, &R::KeyboardListenerType, &R::MouseListenerType, &Duration)) -> Script<R> {
        Script { start: Some(start), update: Some(update) }
    }

    pub fn of_start (start: fn(&mut Scene<R>)) -> Script<R> {
        Script { start: Some(start), update: None }
    }

    pub fn of_update (update: fn(&mut Scene<R>, &R::KeyboardListenerType, &R::MouseListenerType, &Duration)) -> Script<R> {
        Script { start: None, update: Some(update) }
    }

    pub fn empty () -> Script<R> {
        Script { start: None, update: None }
    }
}