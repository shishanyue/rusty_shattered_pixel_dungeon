use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{assets::GameAssets, bevy_ext::math::URectExt, ui::UiTextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum BannerSpriteKind {
    TitlePort,
    TitleGlowPort,
    TitleLand,
    TitleGlowLand,
    BossSlain,
    GameOver,
    SelectYourHero,
}

pub(super) fn process(
    game_assets: &Res<GameAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    ui_texture_atlas_layout_handles: &mut ResMut<UiTextureAtlasLayoutHandles>,
    images: &ResMut<Assets<Image>>,
) {
    if let Some(chrome) = images.get(game_assets.interfaces.chrome.id()) {
        let mut texture_atlas =
            TextureAtlasLayout::new_empty( chrome.size());

        texture_atlas.add_texture(URect::from_size(0, 0, 20, 20));
        texture_atlas.add_texture(URect::from_size(86, 0, 22, 22));
        texture_atlas.add_texture(URect::from_size(20, 0, 9, 9));
        texture_atlas.add_texture(URect::from_size(20, 9, 9, 9));
        texture_atlas.add_texture(URect::from_size(29, 9, 9, 9));
        texture_atlas.add_texture(URect::from_size(29, 0, 9, 9));
        texture_atlas.add_texture(URect::from_size(38, 0, 6, 6));
        texture_atlas.add_texture(URect::from_size(38, 6, 6, 6));
        texture_atlas.add_texture(URect::from_size(22, 18, 16, 14));
        texture_atlas.add_texture(URect::from_size(0, 32, 32, 32));
        texture_atlas.add_texture(URect::from_size(32, 32, 32, 32));
        texture_atlas.add_texture(URect::from_size(64, 0, 20, 20));
        texture_atlas.add_texture(URect::from_size(65, 22, 8, 13));
        texture_atlas.add_texture(URect::from_size(75, 22, 8, 13));
        texture_atlas.add_texture(URect::from_size(45, 0, 1, 1));

        ui_texture_atlas_layout_handles.chrome = texture_atlases.add(texture_atlas);
    }
}
