use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum InMenu {
    #[default]
    None,
    StopMenu,
    BuildMenu,
}
