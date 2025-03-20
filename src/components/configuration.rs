use crate::utils::constants::{ATOM_SIZE, PERCENT_URANIUM, PROBABILITY_XENON, SIZE};
use bevy::prelude::*;

#[derive(Component)]
pub struct Configuration {
    pub terminal_speed: f32,
    pub fast_speed: f32,
    pub neutron_size: f32,
    pub atom_size: f32,
    pub uranium_enrichment: u32,
    pub xenon_probability: u32,
    pub reactor_power: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            terminal_speed: 200f32,
            fast_speed: 400f32,
            neutron_size: SIZE,
            atom_size: ATOM_SIZE as f32,
            uranium_enrichment: PERCENT_URANIUM as u32,
            xenon_probability: PROBABILITY_XENON as u32,
            reactor_power: 50,
        }
    }
}
