#![allow(clippy::too_many_arguments, clippy::type_complexity)]
mod camera;
mod state;
mod tilemap;

use bevy::prelude::*;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(state::StatePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(tilemap::TilemapPlugin);

    app.run();
}
