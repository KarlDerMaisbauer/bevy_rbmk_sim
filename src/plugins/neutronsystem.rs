use bevy::prelude::*;

use crate::components::configuration::Configuration;
use crate::components::controllrod::ControllRod;
use crate::components::neutron::{NeutronType, Speed};

pub struct NeutronSystemPlugin;

impl Plugin for NeutronSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_particle)
            .add_systems(PostUpdate, despawn);
    }
}

fn move_particle(
    mut query: Query<(&mut Transform, &Speed, &NeutronType), Without<ControllRod>>,
    time: Res<Time>,
    query_configuration: Query<&Configuration>,
) {
    let config = query_configuration.single();
    for (mut transform, speed, p_type) in &mut query {
        let multiplier = match p_type {
            NeutronType::Terminal => config.terminal_speed,
            _ => config.fast_speed,
        };
        transform.translation.x += multiplier * speed.vx * time.delta_secs();
        transform.translation.y += multiplier * speed.vy * time.delta_secs();
    }
}

fn despawn(query: Query<(Entity, &Transform), With<NeutronType>>, mut commands: Commands) {
    for (id, transform) in &query {
        if transform.translation.x > 900f32 || transform.translation.x < -900f32 {
            commands.entity(id).despawn();
        } else if transform.translation.y > 500f32 || transform.translation.y < -500f32 {
            commands.entity(id).despawn();
        }
    }
}
