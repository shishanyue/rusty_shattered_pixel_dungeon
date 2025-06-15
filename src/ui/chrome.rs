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
    if let Some(chrome) = images.get(game_assets.interfaces.chrome.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(chrome.size());

        texture_atlas.add_texture(URect::from_size(20, 0, 9, 9)); // Toast
        texture_atlas.add_texture(URect::from_size(20, 9, 9, 9)); // ToastTr
        texture_atlas.add_texture(URect::from_size(29, 9, 9, 9)); // ToastTrHeavy
        texture_atlas.add_texture(URect::from_size(29, 0, 9, 9)); // ToastWhite
        texture_atlas.add_texture(URect::from_size(0, 0, 20, 20)); // Window
        texture_atlas.add_texture(URect::from_size(86, 0, 22, 22)); // WindowSilver
        texture_atlas.add_texture(URect::from_size(38, 0, 6, 6)); // RedButton
        texture_atlas.add_texture(URect::from_size(38, 6, 6, 6)); // GreyButton
        texture_atlas.add_texture(URect::from_size(20, 9, 9, 9)); // GreyButtonTr (与 ToastTr 相同)
        texture_atlas.add_texture(URect::from_size(22, 18, 16, 14)); // Tag
        texture_atlas.add_texture(URect::from_size(0, 32, 32, 32)); // Gem
        texture_atlas.add_texture(URect::from_size(32, 32, 32, 32)); // Scroll
        texture_atlas.add_texture(URect::from_size(64, 0, 20, 20)); // TabSet
        texture_atlas.add_texture(URect::from_size(65, 22, 8, 13)); // TabSelected
        texture_atlas.add_texture(URect::from_size(75, 22, 8, 13)); // TabUnselected
        texture_atlas.add_texture(URect::from_size(45, 0, 1, 1)); // Blank
        
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
        ui_texture_atlas_layout_handles.chrome = texture_atlases.add(texture_atlas);
    }
}
