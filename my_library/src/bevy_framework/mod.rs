use bevy::{prelude::*, state::state::FreelyMutableState};

pub struct GameStatePlugin<T> {
    menu_state: T,
    game_start_state: T,
    game_end_state: T,
}

impl<T> GameStatePlugin<T> {
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: T, game_start_state: T, game_end_state: T) -> Self {
        Self {
            menu_state,
            game_start_state,
            game_end_state,
        }
    }
}

impl<T: States + FromWorld + FreelyMutableState> Plugin for GameStatePlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_state::<T>();
    }
}

pub fn cleanup<T>(query: Query<(Entity, &T)>, mut commands: Commands)
where
    T: Component,
{
    query.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn();
    });
}
