use bevy::prelude::*;


#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum SystemState {
    #[default]
    Loading,
    Loaded,
}


pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SystemState>();
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
