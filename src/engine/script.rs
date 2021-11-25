use std::time::Duration;
use crate::graph::renderer::Renderer;
use super::scene::Scene;

pub struct Script<R: Renderer> {
    pub start: Option<fn(&mut Scene<R>)>,
    pub update: Option<fn(&mut Scene<R>, &R::KeyboardListenerType, &Duration)>
}

impl<R: Renderer> Script<R> {
    pub fn new (start: fn(&mut Scene<R>), update: fn(&mut Scene<R>, &R::KeyboardListenerType, &Duration)) -> Script<R> {
        Script { start: Some(start), update: Some(update) }
    }

    pub fn of_start (start: fn(&mut Scene<R>)) -> Script<R> {
        Script { start: Some(start), update: None }
    }

    pub fn of_update (update: fn(&mut Scene<R>, &R::KeyboardListenerType, &Duration)) -> Script<R> {
        Script { start: None, update: Some(update) }
    }

    pub fn empty () -> Script<R> {
        Script { start: None, update: None }
    }
}