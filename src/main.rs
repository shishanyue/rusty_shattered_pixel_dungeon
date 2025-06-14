use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use rusty_shattered_pixel_dungeon::{utils::dungeon_seed::DugeonSeed, RustyPixelDungeonPlugins};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(RustyPixelDungeonPlugins)
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
    .add_plugins(WorldInspectorPlugin::new())
    .insert_resource(DugeonSeed)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
