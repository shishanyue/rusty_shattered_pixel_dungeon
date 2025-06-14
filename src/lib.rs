pub mod assets;
pub mod utils;
pub mod levels;
pub mod actors;
pub mod dungeon;
pub mod scenes;
pub mod bevy_ext;
pub mod ui;
use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{assets::AssetsPlugin, bevy_ext::system::SystemPlugin, scenes::ScenePlugin};

pub struct RustyPixelDungeonPlugins;

impl PluginGroup for RustyPixelDungeonPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(ScenePlugin)
            .add(SystemPlugin)
            .add(AssetsPlugin);
        group
    }
}
