use bevy::prelude::*;

#[derive(Component)]
pub struct ColorHandler {
    pub u235_color_handle: Handle<ColorMaterial>,
    pub xenon_color_handle: Handle<ColorMaterial>,
    pub other_color_handle: Handle<ColorMaterial>,
    pub terminal_neutron_color_handle: Handle<ColorMaterial>,
    pub fast_neutron_color_handle: Handle<ColorMaterial>,
    // pub  color_handle_water_1: Color,
    // pub  color_handle_water_2: Color,
    // pub  color_handle_water_3: Color,
    // pub  color_handle_water_4: Color,
    // pub  color_handle_water_5: Color,
}
