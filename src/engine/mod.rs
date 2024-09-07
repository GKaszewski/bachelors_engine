#[allow(dead_code)]
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use raylib::{RaylibHandle, RaylibThread};

/// Game trait that defines the methods that a game must implement.
pub trait Game {
    fn handle_input(&mut self, rl: &RaylibHandle);
    fn update(&mut self, delta_time: f32);
    fn render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn change_state(&mut self, new_state: GameState);
}

pub enum GameState {
    Playing,
    Paused,
}

/// Entity struct that represents an entity in the game.
/// An entity is a game object that can have components.
/// For example, a player entity can have a Transform, a Sprite, and a Collider component.
pub struct Entity {
    #[allow(unused)]
    id: u32,
    /// Components are stored in a HashMap with the TypeId as the key.
    /// We can store different types of components in the same HashMap.
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|component| component.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|component| component.downcast_mut::<T>())
    }

    #[allow(unused)]
    pub fn remove_component<T: 'static>(&mut self) -> Option<T> {
        self.components
            .remove(&TypeId::of::<T>())
            .and_then(|component| component.downcast::<T>().ok().map(|c| *c))
    }
}

pub struct EntityBuilder {
    entity: Entity,
}

impl EntityBuilder {
    pub fn new(id: u32) -> Self {
        Self {
            entity: Entity {
                id,
                components: HashMap::new(),
            },
        }
    }

    pub fn with_component<T: 'static>(mut self, component: T) -> Self {
        self.entity.add_component(component);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

/// System trait - a system is a game logic unit that processes entities.
/// For example, a PhysicsSystem can update the position of entities based on their velocity.
pub trait System {
    fn update(&mut self, entities: &mut [Entity], delta_time: f32);
}

pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn update(&mut self, entities: &mut [Entity], delta_time: f32) {
        for system in self.systems.iter_mut() {
            system.update(entities, delta_time);
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub event_type: String,
    /// The type of the event. (for now it's a string, but it should be an enum)
    pub data: Box<dyn Any>,
}

/// EventQueue struct that stores events.
pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
}
