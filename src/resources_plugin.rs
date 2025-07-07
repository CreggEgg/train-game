use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Item {
    Food,
    Water,
    Wood,
    Clay,
    Brick,
    Metal,
    Glass,
    Bullet,
    Money,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: HashMap<Item, usize>,
}

impl Inventory {
    fn is_empty(&self) -> bool {
        self.items.keys().len() == 0 || self.items.values().all(|it| *it == 0)
    }

    fn add_other(&mut self, mut other: Self, max_stack_size: usize) -> Self {
        let other_items = other.items.keys().cloned().collect::<Vec<_>>();
        for item in other_items {
            let capacity = max_stack_size - self.items.get(&item).cloned().unwrap_or(0);
            let slot = other.items.get_mut(&item).unwrap();
            let amount = capacity.min(*slot);
            *slot -= amount;
            *self.items.entry(item).or_insert(0) += amount;
        }
        other
    }
}

pub fn resources_plugin(app: &mut App) {
    app;
}
