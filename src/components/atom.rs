use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Atom {
    U235,
    Xenon,
    Other,
    Start,
}
