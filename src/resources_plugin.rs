use std::collections::HashMap;

use bevy::prelude::*;

use crate::ImageAssets;

#[derive(Eq, PartialEq, Hash, Clone, Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Eq, PartialEq, Hash, Clone, Debug, serde::Deserialize)]
pub enum Fluid {
    Water,
    AlchemyJuice,
    Fuel,
}

impl Fluid {
    pub fn name(&self) -> &'static str {
        match self {
            Fluid::Water => "Water",
            Fluid::Fuel => "Fuel",
            Fluid::AlchemyJuice => "Alchemy Juice",
        }
    }
}

#[derive(Component, Default, serde::Serialize, serde::Deserialize, Clone, Debug)]
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

#[macro_export]
macro_rules! consume_resource {
    ($cost:expr, $inventories:ident, $on_fail:block, $on_success:block) => {{
        use crate::resources_plugin::Item;
        consume_resource!(Item::Money, $cost, $inventories, $on_fail, $on_success);
    }};
    ($item:expr, $cost:expr, $inventories:ident, $on_fail:block, $on_success:block) => {{
        let cost = $cost;

        let total_owned = {
            let mut total = 0;
            for mut inventory in &mut $inventories {
                let amount = inventory.items.entry($item).or_insert(0);
                total += *amount;
                if total >= cost {
                    break;
                }
            }
            total
        };

        if total_owned < cost {
            $on_fail
        } else {
            {
                let mut cost = cost;
                for mut inventory in &mut $inventories {
                    let amount = inventory.items.entry($item).or_insert(0);
                    if *amount >= cost {
                        *amount -= cost;
                        break;
                    } else {
                        cost -= *amount;
                        *amount = 0;
                    }
                }
            }
            $on_success
        }
    }};
}
