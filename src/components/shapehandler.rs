use bevy::prelude::*;

#[derive(Component)]
pub struct ShapeHandler {
    pub neutron_shape_handle: Handle<Mesh>,
    pub atom_shape_handle: Handle<Mesh>,
    pub water_shape_handle: Handle<Mesh>,
}
