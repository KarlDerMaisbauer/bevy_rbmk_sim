use bevy::prelude::*;
use bevy_turborand::prelude::*;
use std::f32::consts::PI;

use crate::components::atom::Atom;
use crate::components::colorhandler::ColorHandler;
use crate::components::configuration::Configuration;
use crate::components::neutron::{NeutronType, Speed};
use crate::components::random::Random;
use crate::components::shapehandler::ShapeHandler;
use crate::components::timer::ReactorTimer;
use crate::utils::constants::{REACTOR_SIZE_X, REACTOR_SIZE_Y};

pub struct ReactorFuelPlugin;

impl Plugin for ReactorFuelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (xenon, enrich, neutron_emitance));
    }
}

#[allow(unused_mut)]
fn enrich(
    mut query_atoms: Query<(&mut Atom, &mut MeshMaterial2d<ColorMaterial>)>,
    mut query_random: Query<&mut RngComponent, With<Random>>,
    query_config: Query<&Configuration>,
    query_color_handler: Query<&ColorHandler>,
    timer: ResMut<ReactorTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut rng = query_random.single_mut();
    let config = query_config.single();
    // let total_fuel = REACTOR_SIZE_X * REACTOR_SIZE_Y;
    // let enrichment_count = query_atoms
    //     .iter()
    //     .filter(|(a, _other)| matches!(*a, Atom::U235))
    //     .count();

    // let max_enrichment =
    //     (total_fuel as f32 * config.uranium_enrichment as f32 / 100f32).ceil() as usize;
    // let mut remain_to_be_filled = if enrichment_count >= max_enrichment {
    //     4
    // } else {
    //     max_enrichment - enrichment_count
    // };

    let color_handler = query_color_handler.single();
    // let mut atoms: Vec<(Mut<Atom>, Mut<MeshMaterial2d<ColorMaterial>>)> =
    // query_atoms.iter_mut().collect();
    // let number_atoms = atoms.len();

    // Select a random item
    // while remain_to_be_filled > 0 {
    //     let mut atom = &mut atoms[rng.usize(0..number_atoms)];
    //     if !matches!(*atom.0, Atom::Other | Atom::Start) {
    //         continue;
    //     }
    //
    //     if config.uranium_enrichment < rng.u32(0..100) {
    //         continue;
    //     }
    //
    //     *atom.0 = Atom::U235;
    //     *atom.1 = MeshMaterial2d(color_handler.u235_color_handle.clone());
    //     remain_to_be_filled -= 1;
    // }
    for (mut atom_type, mut color) in &mut query_atoms {
        if !matches!(*atom_type, Atom::Other | Atom::Start) {
            continue;
        }
        if config.uranium_enrichment < rng.u32(0..200) {
            continue;
        }
        *atom_type = Atom::U235;
        *color = MeshMaterial2d(color_handler.u235_color_handle.clone());
    }
}

fn xenon(
    mut query_atoms: Query<(&mut Atom, &mut MeshMaterial2d<ColorMaterial>)>,
    mut query_random: Query<&mut RngComponent, With<Random>>,
    query_config: Query<&Configuration>,
    query_color_handler: Query<&ColorHandler>,
    timer: ResMut<ReactorTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut rng = query_random.single_mut();
    let config = query_config.single();
    let color_handler = query_color_handler.single();
    for (mut atom_type, mut color) in &mut query_atoms {
        if !matches!(*atom_type, Atom::Other) {
            continue;
        }
        if config.xenon_probability < rng.u32(0..200) {
            continue;
        }
        *atom_type = Atom::Xenon;
        *color = MeshMaterial2d(color_handler.xenon_color_handle.clone());
    }
}

fn neutron_emitance(
    mut commands: Commands,
    mut query_atoms: Query<(&Atom, &Transform)>,
    mut query_random: Query<&mut RngComponent, With<Random>>,
    query_config: Query<&Configuration>,
    query_color_handler: Query<&ColorHandler>,
    query_shape_handler: Query<&ShapeHandler>,
    timer: ResMut<ReactorTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut rng = query_random.single_mut();
    let config = query_config.single();
    let color_handler = query_color_handler.single();
    let shape_handler = query_shape_handler.single();
    let mut atoms: Vec<_> = query_atoms.iter_mut().collect();
    let number_atoms = atoms.len();
    let mut neutrons_emitted = rng.usize(0..5);
    // Select a random item
    while neutrons_emitted > 0 {
        let (atom, translation) = &mut atoms[rng.usize(0..number_atoms)];
        if !matches!(*atom, Atom::Other | Atom::Start) {
            continue;
        }

        if config.uranium_enrichment < rng.u32(0..200) {
            continue;
        }
        commands.spawn((
            NeutronType::Terminal,
            Speed::new(rng.u32(0..=360) as f32 * PI / 180f32),
            Mesh2d(shape_handler.neutron_shape_handle.clone()),
            MeshMaterial2d(color_handler.terminal_neutron_color_handle.clone()),
            Transform::from_xyz(translation.translation.x, translation.translation.y, 0.0),
        ));
        neutrons_emitted -= 1;
    }
    // for (mut atom_type, mut color) in &mut query_atoms {
    //     if !matches!(*atom_type, Atom::Other | Atom::Start) {
    //         continue;
    //     }
    //     if config.uranium_enrichment < rng.u32(0..100) {
    //         continue;
    //     }
    //     *atom_type = Atom::U235;
    //     *color = MeshMaterial2d(color_handler.u235_color_handle.clone());
    // }
}
