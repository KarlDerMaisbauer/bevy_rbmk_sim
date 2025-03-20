use bevy::app::Plugin;
use collision_system::CollisionSystemPlugin;
use neutronsystem::NeutronSystemPlugin;
use reactorbuilderplugin::ReactorBuilderPlugin;
use reactorfuel::ReactorFuelPlugin;
use timer::ReactorTimePlugin;

mod collision_system;
mod neutronsystem;
mod reactorbuilderplugin;
mod reactorfuel;
mod timer;

pub struct RBMKPlugin;

impl Plugin for RBMKPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ReactorTimePlugin)
            .add_plugins(ReactorBuilderPlugin)
            .add_plugins(ReactorFuelPlugin)
            .add_plugins(NeutronSystemPlugin)
            .add_plugins(CollisionSystemPlugin);
    }
}
