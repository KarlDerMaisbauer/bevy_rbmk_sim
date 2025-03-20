use bevy::prelude::*;
use bevy_turborand::prelude::*;
use plugins::RBMKPlugin;

mod components;
mod events;
mod plugins;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RBMKPlugin)
        .add_plugins(RngPlugin::default())
        .insert_resource(ClearColor(Color::srgba(1.0, 1.0, 1.0, 1.0)))
        // .insert_resource(SpawnTimer(Timer::from_seconds(1f32, TimerMode::Repeating)))
        // .add_systems(Startup, setup)
        // .add_systems(Update, move_particle)
        // .add_systems(PostUpdate, (/*spawn_entity, bounce,*/ xenon, enrich))
        .run();
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut global_rng: ResMut<GlobalRng>,
// ) {
//     let neutron_shape_handle = meshes.add(Circle::new(SIZE));
//     let rectangle = meshes.add(Rectangle::new(25.0, 600.0));
//     let terminal_neutron_color_handle = materials.add(Color::srgba(0.5, 0.5, 0.5, 1f32));
//     let fast_neutron_color_handle = materials.add(Color::srgba(0.5, 0.5, 0.5, 1f32));
//     let color2 = Color::srgba(0f32, 0f32, 0f32, 1f32);
//     commands.spawn((Camera2d, Camera { ..default() }));
//     commands.spawn((
//         Particle::Fast,
//         Speed::new(0.125f32),
//         Mesh2d(neutron_shape_handle.clone()),
//         MeshMaterial2d(terminal_neutron_color_handle.clone()),
//         Transform::from_xyz(0.0, 0.0, 0.0),
//     ));
//     commands.spawn((
//         Mesh2d(rectangle),
//         MeshMaterial2d(materials.add(color2)),
//         Transform::from_xyz(400.0, 0.0, 0.0),
//         ControllRod,
//     ));
//
//     commands.spawn((Random, RngComponent::from(&mut global_rng)));
//     let water_shape_handle = meshes.add(Rectangle::new(WATER_SIZE as f32, WATER_SIZE as f32));
//     let atom_shape_handle = meshes.add(Circle::new(ATOM_SIZE as f32));
//     let color_water = Color::srgba(0f32, 0f32, 1f32, 0.65f32);
//     let water_color_handle = materials.add(color_water);
//     let start_val_x = (REACTOR_SIZE_X * WATER_SPACE) as f32 / 2f32;
//     let start_val_y = (REACTOR_SIZE_Y * WATER_SPACE) as f32 / 2f32;
//     let color_u235 = Color::srgba(0f32, 0.5f32, 0.9375f32, 1f32);
//     let color_xenon = Color::srgba(0f32, 0f32, 0f32, 1f32);
//     let color_other = Color::srgba(0.65f32, 0.65f32, 0.65f32, 1f32);
//     let u235_color_handle = materials.add(color_u235);
//     let xenon_color_handle = materials.add(color_xenon);
//     let other_color_handle = materials.add(color_other);
//     commands.spawn(ShapeHandler {
//         neutron_shape_handle: neutron_shape_handle.clone(),
//         atom_shape_handle: atom_shape_handle.clone(),
//         water_shape_handle: water_shape_handle.clone(),
//     });
//     commands.spawn(ColorHandler {
//         u235_color_handle: u235_color_handle.clone(),
//         xenon_color_handle: xenon_color_handle.clone(),
//         other_color_handle: other_color_handle.clone(),
//         terminal_neutron_color_handle,
//         fast_neutron_color_handle,
//     });
//     commands.spawn(Configuration::default());
//
//     let mut rng = RngComponent::from(&mut global_rng);
//     for x in 0..REACTOR_SIZE_X {
//         for y in 0..REACTOR_SIZE_Y {
//             commands.spawn((
//                 Water::default(),
//                 MeshMaterial2d(water_color_handle.clone()),
//                 Mesh2d(water_shape_handle.clone()),
//                 Transform::from_xyz(
//                     start_val_x - (WATER_SPACE * x) as f32,
//                     start_val_y - (WATER_SPACE * y) as f32,
//                     WATER_Z_VALUE,
//                 ),
//             ));
//             let prob = rng.u32(0..1000);
//             let (atom_val, atom_color) = if prob < PERCENT_URANIUM {
//                 (Atom::U235, u235_color_handle.clone())
//             } else {
//                 (Atom::Start, other_color_handle.clone())
//             };
//             commands.spawn((
//                 MeshMaterial2d(atom_color),
//                 Mesh2d(atom_shape_handle.clone()),
//                 Transform::from_xyz(
//                     start_val_x - (WATER_SPACE * x) as f32,
//                     start_val_y - (WATER_SPACE * y) as f32,
//                     ATOM_Z_VALUE,
//                 ),
//                 atom_val,
//             ));
//         }
//     }
// }

// fn spawn_entity(
//     mut commands: Commands,
//     timer: ResMut<ReactorTimer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut rng: Query<&mut RngComponent, With<Random>>,
// ) {
//     if timer.0.just_finished() {
//         let circle = meshes.add(Circle::new(SIZE));
//         let color = Color::srgba(0.5, 0.5, 0.5, 1f32);
//         let mut rngg = rng.single_mut();
//         commands.spawn((
//             Particle::Terminal,
//             Speed::new(rngg.u32(0..=360) as f32 * PI / 180f32),
//             Mesh2d(circle),
//             MeshMaterial2d(materials.add(color)),
//             Transform::from_xyz(0.0, 0.0, 0.0),
//         ));
//         // commands.spawn((
//         //     Player::new(rngg.u32(0..=360) as f32 * PI / 180f32, TERMINAL_SPEED),
//         //     Mesh2d(circle),
//         //     MeshMaterial2d(materials.add(color)),
//         //     Transform::from_xyz(0.0, 0.0, 0.0),
//         // ));
//     }
// }

//
// fn bounce(
//     mut query_particles: Query<
//         (&mut Transform, &mut Speed, &Particle),
//         (With<Particle>, Without<Water>, Without<ControllRod>),
//     >,
//     query_controll_rods: Query<
//         &Transform,
//         (
//             With<ControllRod>,
//             Without<Particle>,
//             Without<Particle>,
//             Without<Water>,
//         ),
//     >,
//     query_configuration: Query<&Configuration>,
// ) {
//     let conf = query_configuration.single();
//     for rect in &query_controll_rods {
//         for (t, mut p, p_type) in &mut query_particles {
//             if *p_type == Particle::Terminal
//                 && (t.translation.y < rect.translation.y + 300f32
//                     || t.translation.y > rect.translation.y - 300f32)
//             {
//                 if ((t.translation.x + conf.atom_size / 2f32) - (rect.translation.x - 25f32 / 2f32))
//                     .abs()
//                     < EPSILON
//                 {
//                     p.vx = -p.vx
//                 } else if ((t.translation.x - conf.atom_size / 2f32)
//                     - (rect.translation.x + 25f32 / 2f32))
//                     .abs()
//                     < EPSILON
//                 {
//                     p.vx = -p.vx
//                 }
//             }
//         }
//     }
// }
