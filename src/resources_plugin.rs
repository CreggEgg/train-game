use std::collections::HashMap;

use bevy::prelude::*;

use crate::ImageAssets;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Item {
    Food,
    Wood,
    Clay,
    Brick,
    Stone,
    Metal,
    Glass,
    Bullet,
    Money,
}
impl Item {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Item::Food => "Food",
            Item::Wood => "Wood",
            Item::Clay => "Clay",
            Item::Brick => "Brick",
            Item::Stone => "Stone",
            Item::Metal => "Metal",
            Item::Glass => "Glass",
            Item::Bullet => "Bullets",
            Item::Money => "Money",
        }
    }

    pub fn get_image(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            Item::Metal => image_assets.item_metal.clone(),
            Item::Wood => image_assets.item_wood.clone(),
            Item::Stone => image_assets.item_stone.clone(),
            _ => image_assets.item_metal.clone(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Fluid {
    Water,
    Fuel,
}

impl Fluid {
    pub fn name(&self) -> &'static str {
        match self {
            Fluid::Water => "Water",
            Fluid::Fuel => "Fuel",
        }
    }
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: HashMap<Item, usize>,
}

impl Inventory {
    pub fn is_empty(&self) -> bool {
        self.items.keys().len() == 0 || self.items.values().all(|it| *it == 0)
    }

    // pub fn add_other(&mut self, mut other: Self, max_stack_size: usize) -> Self {
    //     let other_items = other.items.keys().cloned().collect::<Vec<_>>();
    //     for item in other_items {
    //         let capacity = max_stack_size - self.items.get(&item).cloned().unwrap_or(0);
    //         let slot = other.items.get_mut(&item).unwrap();
    //         let amount = capacity.min(*slot);
    //         *slot -= amount;
    //         *self.items.entry(item).or_insert(0) += amount;
    //     }
    //     other
    // }
}

pub fn resources_plugin(_app: &mut App) {
    // app;
}
