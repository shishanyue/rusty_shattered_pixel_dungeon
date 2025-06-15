use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{assets::GameAssets, bevy_ext::math::URectExt, ui::UiTextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum ChromeKind {
    Toast,
    ToastTr,
    ToastTrHeavy,
    ToastWhite,
    Window,
    WindowSilver,
    RedButton,
    GreyButton,
    GreyButtonTr,
    Tag,
    Gem,
    Scroll,
    TabSet,
    TabSelected,
    TabUnselected,
    Blank,
}

#[derive(Default, Resource)]
pub struct ChromeTextureSlicer {
    pub grey_button_tr_slicer: TextureSlicer,
    pub window_slicer: TextureSlicer,
}

pub(super) fn process(
    game_assets: &Res<GameAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    ui_texture_atlas_layout_handles: &mut ResMut<UiTextureAtlasLayoutHandles>,
    images: &ResMut<Assets<Image>>,
    chrome_texture_slicer: &mut ResMut<ChromeTextureSlicer>,
) {
    if let Some(banners) = images.get(game_assets.interfaces.banners.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(banners.size());

        texture_atlas.add_texture(URect::from_size(0, 0, 139, 100));
        texture_atlas.add_texture(URect::from_size(139, 0, 278, 100));
        texture_atlas.add_texture(URect::from_size(0, 100, 240, 157));
        texture_atlas.add_texture(URect::from_size(240, 100, 480, 157));
        texture_atlas.add_texture(URect::from_size(0, 157, 128, 192));
        texture_atlas.add_texture(URect::from_size(0, 192, 128, 227));
        texture_atlas.add_texture(URect::from_size(0, 227, 128, 248));

        chrome_texture_slicer.grey_button_tr_slicer = TextureSlicer {
            border: BorderRect::all(4.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 3.0,
        };
        chrome_texture_slicer.window_slicer = TextureSlicer {
            border: BorderRect::all(6.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 2.0,
        };
        ui_texture_atlas_layout_handles.banner_sprites = texture_atlases.add(texture_atlas);
    }
}
