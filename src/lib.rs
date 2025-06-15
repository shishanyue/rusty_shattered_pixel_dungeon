pub mod actors;
pub mod assets;
pub mod bevy_ext;
pub mod dungeon;
pub mod levels;
pub mod scenes;
pub mod ui;
pub mod utils;
use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    assets::AssetsPlugin, bevy_ext::system::SystemPlugin, scenes::ScenePlugin, ui::UiPlugin,
    utils::dungeon_seed::DugeonSeed,
};

pub struct RustyPixelDungeonPlugins;

impl PluginGroup for RustyPixelDungeonPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(ScenePlugin)
            .add(SystemPlugin)
            .add(AssetsPlugin)
            .add(UiPlugin);
        group
    }
}

pub struct RustyPixelDungeonPlugin;

impl Plugin for RustyPixelDungeonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DugeonSeed>()
            .add_plugins(RustyPixelDungeonPlugins);
    }
}
