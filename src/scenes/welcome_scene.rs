use bevy::prelude::*;

use crate::bevy_ext::{app::AppExt, system::SystemState};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct WelcomeScene;

#[derive(Component)]
struct WelcomeSceneMark;

impl Scene for WelcomeScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<WelcomeSceneMark, _>(SceneState::WelcomeScene, setup)
            .add_systems(
                Update,
                (|mut scene_state: ResMut<NextState<SceneState>>| {
                    scene_state.set(SceneState::TitleScene);
                })
                .run_if(in_state(SystemState::AssetsProcessed).and(run_once)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        WelcomeSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
    ));
}
