use bevy::prelude::*;

#[derive(Component)]
pub struct Water {
    pub temperature: u8,
}

impl Default for Water {
    fn default() -> Self {
        Water { temperature: 20 }
    }
}
