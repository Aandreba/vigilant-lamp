use std::{time::Duration, rc::Rc};
use crate::{graph::Renderer};
use super::{scene::Scene};

pub type StartFunction<R> = fn(&mut Scene<R>);
pub type UpdateFunction<R: Renderer> = fn(&mut Scene<R>, &R::KeyboardListenerType, &R::MouseListenerType, &Duration);

/// Element containing all the actions an element may execute at variuos stages og the execution process
pub struct Script<R: Renderer> {
    pub start: Option<StartFunction<R>>,
    pub update: Option<UpdateFunction<R>>
}

impl<R: Renderer> Script<R> {
    pub fn new (start: StartFunction<R>, update: UpdateFunction<R>) -> Script<R> {
        Script { start: Some(start), update: Some(update) }
    }

    pub fn of_start (start: StartFunction<R>) -> Script<R> {
        Script { start: Some(start), update: None }
    }

    pub fn of_update (update: UpdateFunction<R>) -> Script<R> {
        Script { start: None, update: Some(update) }
    }

    pub fn empty () -> Script<R> {
        Script { start: None, update: None }
    }
}