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
    if let Some(banners) = images.get(game_assets.interfaces.banners.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(banners.size());

        texture_atlas.add_texture(URect::from_size(0, 0, 139, 100)); // TitlePort
        texture_atlas.add_texture(URect::from_size(139, 0, 139, 100)); // TitleGlowPort
        texture_atlas.add_texture(URect::from_size(0, 100, 240, 57)); // TitleLand
        texture_atlas.add_texture(URect::from_size(240, 100, 240, 57)); // TitleGlowLand
        texture_atlas.add_texture(URect::from_size(0, 157, 128, 35)); // BossSlain
        texture_atlas.add_texture(URect::from_size(0, 192, 128, 35)); // GameOver
        texture_atlas.add_texture(URect::from_size(0, 227, 128, 21)); // SelectYourHero

        ui_texture_atlas_layout_handles.banner_sprites = texture_atlases.add(texture_atlas);
    }
}
