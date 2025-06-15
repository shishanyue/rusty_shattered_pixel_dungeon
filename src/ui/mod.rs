use crate::{assets::GameAssets, bevy_ext::system::SystemState, ui::chrome::ChromeTextureSlicer};
use bevy::prelude::*;
pub mod banner_sprites;
pub mod chrome;
pub mod icons;
pub struct UiPlugin;

#[derive(Default, Resource)]
pub struct UiTextureAtlasLayoutHandles {
    pub icons: Handle<TextureAtlasLayout>,
    pub banner_sprites: Handle<TextureAtlasLayout>,
    pub chrome: Handle<TextureAtlasLayout>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChromeTextureSlicer>()
            .init_resource::<UiTextureAtlasLayoutHandles>()
            .add_systems(OnEnter(SystemState::AssetsLoaded), process_assets);
    }
}

fn process_assets(
    mut system_state: ResMut<NextState<SystemState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut ui_texture_atlas_layout_handles: ResMut<UiTextureAtlasLayoutHandles>,
    mut game_assets: Res<GameAssets>,
    images: ResMut<Assets<Image>>,
    mut chrome_texture_slicer: ResMut<ChromeTextureSlicer>,
) {
    icons::process(
        &mut game_assets,
        &mut texture_atlases,
        &mut ui_texture_atlas_layout_handles,
        &images,
    );
    banner_sprites::process(
        &game_assets,
        &mut texture_atlases,
        &mut ui_texture_atlas_layout_handles,
        &images,
    );
    chrome::process(
        &game_assets,
        &mut texture_atlases,
        &mut ui_texture_atlas_layout_handles,
        &images,
        &mut chrome_texture_slicer,
    );
    system_state.set(SystemState::AssetsProcessed);
}
