use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Id<T>(u64, PhantomData<T>);

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Id<T> {
    pub const fn new(value: u64) -> Self {
        Self(value, PhantomData)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Component)]
pub struct IdComponent<T: 'static + Send + Sync> {
    pub id: Id<T>,
}

#[derive(Resource, Default)]
pub struct IdAllocator<T> {
    next: u64,
    _marker: PhantomData<T>,
}

impl<T> IdAllocator<T> {
    pub fn next(&mut self) -> Id<T> {
        self.next += 1;
        Id::new(self.next)
    }

    pub fn current(&self) -> Id<T> {
        Id::new(self.next)
    }
}

#[derive(Resource, Default)]
pub struct EntityMap<T> {
    map: HashMap<Id<T>, Entity>,
    _marker: PhantomData<T>,
}

impl<T> EntityMap<T> {
    pub fn insert(&mut self, id: Id<T>, entity: Entity) {
        self.map.insert(id, entity);
    }

    pub fn remove(&mut self, id: Id<T>) {
        self.map.remove(&id);
    }

    pub fn get(&self, id: Id<T>) -> Option<Entity> {
        self.map.get(&id).copied()
    }

    pub fn entity(&self, id: Id<T>) -> Option<Entity> {
        self.get(id)
    }
}
