use crate::components::atom::Atom;
use crate::components::colorhandler::ColorHandler;
use crate::components::configuration::Configuration;
use crate::components::neutron::NeutronType;
use crate::components::neutron::Speed;
use crate::components::random::Random;
use crate::components::shapehandler::ShapeHandler;
use bevy::prelude::*;
use bevy_turborand::prelude::*;
use std::f32::consts::PI;

pub struct CollisionSystemPlugin;

impl Plugin for CollisionSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_checker);
    }
}

fn collision_checker(
    mut commands: Commands,
    mut query_atoms: Query<(&Transform, &mut Atom, &mut MeshMaterial2d<ColorMaterial>)>,
    mut query_neutrons: Query<(Entity, &Transform, &NeutronType)>,
    query_configuration: Query<&Configuration>,
    query_color_handler: Query<&ColorHandler>,
    mut rng_component: Query<&mut RngComponent, With<Random>>,
    query_shape_handler: Query<&ShapeHandler>,
) {
    let conf = query_configuration.single();
    let color_handler = query_color_handler.single();
    let mut rng = rng_component.single_mut();
    let shape_handler = query_shape_handler.single();
    println!(
        "num neutrons {:?}",
        query_neutrons.iter().collect::<Vec<_>>().len()
    );
    for (transform_a, mut atom_type, mut color) in &mut query_atoms {
        if matches!(*atom_type, Atom::Other | Atom::Start) {
            continue;
        }
        for (e_id_n, transform_n, p_type_n) in &mut query_neutrons {
            if !matches!(p_type_n, NeutronType::Terminal) {
                continue;
            }
            if !collision(
                &transform_a.translation.x,
                &transform_a.translation.y,
                &conf.atom_size,
                &transform_n.translation.x,
                &transform_n.translation.y,
                &conf.neutron_size,
            ) {
                continue;
            }
            commands.entity(e_id_n).despawn();

            if *atom_type == Atom::U235 {
                color.0 = color_handler.other_color_handle.clone();
                *atom_type = Atom::Other;
                let x = transform_a.translation.x;
                let y = transform_a.translation.y;
                for _ in 0..3 {
                    commands.spawn((
                        NeutronType::Terminal,
                        Speed::new(rng.u32(0..=360) as f32 * PI / 180f32),
                        Mesh2d(shape_handler.neutron_shape_handle.clone()),
                        MeshMaterial2d(color_handler.terminal_neutron_color_handle.clone()),
                        Transform::from_xyz(x, y, 0.0),
                    ));
                }
            } else if *atom_type == Atom::Xenon {
                color.0 = color_handler.other_color_handle.clone();
                *atom_type = Atom::Other;
            }
        }
    }
}

fn collision(x1: &f32, y1: &f32, size1: &f32, x2: &f32, y2: &f32, size2: &f32) -> bool {
    let x = (x1 - x2).abs();
    let y = (y1 - y2).abs();
    (x * x + y * y) < (size1 * size1 + size2 * size2)
}
