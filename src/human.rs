use crate::resources_plugin::ResourceType;
use bevy::{platform::collections::HashMap, prelude::*};
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Request {
    Store(usize, Entity, ResourceType),
    Get(usize, Entity, ResourceType),
}

#[derive(Resource)]
struct Requests {
    requests: Vec<Request>,
}

#[derive(Resource)]
struct ResourceStorage {
    total_storage: HashMap<ResourceType, usize>,
    current: HashMap<ResourceType, usize>,
    requests_adding: HashMap<ResourceType, usize>,
}

#[derive(Resource)]
struct HumanLocations {
    columns: Vec<BTreeMap<usize, Vec<Entity>>>,
}

const DISTANCE_BETWEEN_STACKS: usize = 20;
const MOVE_SPEED: f32 = 20.0;

impl HumanLocations {
    // gets the closet human to the target location, and removes them from the btree
    fn get_closest(&mut self, column: usize, height: usize) -> Option<Entity> {
        let mut lowest_distance = usize::MAX;
        let mut closest_location: Option<(usize, usize)> = None;

        if let Some(c) = self.columns.get(column) {
            if let Some((h, _)) = c.range(0..=height).next_back() {
                closest_location = Some((column, *h));
                lowest_distance = h.abs_diff(height);
            }

            if let Some((h, _)) = c.range(height + 1..).next() {
                let d = h.abs_diff(height);

                if lowest_distance >= d {
                    closest_location = Some((column, *h));
                    lowest_distance = d;
                }
            }
        }

        let columns_right =
            (column + 1..self.columns.len()).map(|a| (a, self.columns.get(a).unwrap()));
        let columns_left = (0..column).map(|a| (a, self.columns.get(a).unwrap()));

        let columns = columns_left.interleave(columns_right);

        for (i, c) in columns {
            // early exit

            let from_column = i.abs_diff(column) * DISTANCE_BETWEEN_STACKS + height;

            if from_column > lowest_distance {
                break;
            }

            if let Some((h, _)) = c.range(0..).next() {
                let d = h + from_column;

                if lowest_distance >= d {
                    closest_location = Some((i, *h));
                    lowest_distance = d;
                }
            }
        }

        closest_location.map(|(c, h)| {
            let c: &mut BTreeMap<usize, Vec<_>> = self.columns.get_mut(c).unwrap();

            let height: &mut Vec<Entity> = c.get_mut(&h).unwrap();

            let e: Entity = height.pop().unwrap();

            if height.is_empty() {
                c.remove(&h);
            }

            e
        })
    }
}

#[derive(Component)]
struct Mover {
    at: Entity,
    height: f32,
    column: usize,
}

#[derive(Component)]
struct HasRequest {
    to_move: Timer,
    going_to: Option<Entity>,
    current_request: Option<Request>,
    going_to_start: bool,
    resources: usize,
    resource_type: ResourceType,
}

// fn assign_requests(
//     storage: ResMut<ResourceStorage>,
//     mut requests: ResMut<Requests>,
//     mut human_locations: ResMut<HumanLocations>,
//     mover: Query<&Mover, Without<HasRequest>>,
// ) {
//     requests.requests.retain(|request| {
//     });
// }
