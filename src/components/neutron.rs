use bevy::prelude::Component;
use bevy::prelude::Mesh2d;

#[derive(Component, Default)]
pub struct Speed {
    pub vx: f32,
    pub vy: f32,
}

impl Speed {
    pub fn new(dir: f32) -> Speed {
        let vx = f32::cos(dir);
        let vy = f32::sin(dir);
        Speed { vx, vy }
    }
}

#[derive(Component, Default, PartialEq)]
#[require(Speed, Mesh2d)]
pub enum NeutronType {
    Fast,
    #[default]
    Terminal,
}
