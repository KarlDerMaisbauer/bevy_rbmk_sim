use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent {
    neutron_id: Entity,
    atom_id: Entity,
}
