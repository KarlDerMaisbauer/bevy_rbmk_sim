use bevy::prelude::*;

use crate::components::timer::ReactorTimer;

pub struct ReactorTimePlugin;

impl Plugin for ReactorTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReactorTimer(Timer::from_seconds(
            0.5f32,
            TimerMode::Repeating,
        )))
        .add_systems(Update, update_reactor_timer);
    }
}

fn update_reactor_timer(time: Res<Time>, mut timer: ResMut<ReactorTimer>) {
    timer.0.tick(time.delta());
}
