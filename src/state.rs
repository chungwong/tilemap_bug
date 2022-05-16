use bevy::prelude::*;
pub(crate) use iyes_loopless::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum GameState {
    LoadingLevel,
}

pub(crate) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::LoadingLevel);
    }
}
