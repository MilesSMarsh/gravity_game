use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .run()
}