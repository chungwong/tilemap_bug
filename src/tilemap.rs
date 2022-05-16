#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;
pub use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use crate::{
    state::{AppLooplessStateExt, GameState},
};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..default()
            })
            .add_enter_system(GameState::LoadingLevel, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/reckoning.ldtk"),
        ..default()
    });
}
