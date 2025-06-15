use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{assets::GameAssets, bevy_ext::math::URectExt, ui::UiTextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum IconKind {
    //slightly larger title screen icons, spacing for 17x16
    Enter,
    Gold,
    Rankings,
    Badges,
    News,
    Changes,
    Prefs,
    Shpx,
    Journal,
    //grey icons, mainly used for buttons, spacing for 16x16
    Exit,
    Display, //2 separate images, changes based on orientation
    DisplayLand,
    DisplayPort,
    Data,
    Audio,
    Langs,
    Controller,
    Keyboard,
    Stats,
    ChallengeGrey,
    ScrollGrey,
    Seed,
    LeftArrow,
    RightArrow,
    Calendar,
    Chevron,
    //misc larger icons, mainly used for buttons, tabs, and journa
    Target,
    Info,
    Warning,
    Unchecked,
    Checked,
    Close,
    Plus,
    Repeat,
    Arrow,
    ChallengeColor,
    ScrollColor,
    Copy,
    Paste,
    BackpackLrg,
    Talent,
    Magnify,
    Snake,
    Buffs,
    Catalog,
    Alchemy,
    Grass,
    Stairs,
    StairsChasm,
    StairsWater,
    StairsGrass,
    StairsDark,
    StairsLarge,
    StairsTraps,
    StairsSecrets,
    WellHealth,
    WellAwareness,
    SacrificeAltar,
    DistantWell,
    //smaller icons,variable spacing
    Skull,
    Busy,
    Compass,
    Sleep,
    Alert,
    Lost,
    Depth, //depth icons have three variants,for regular se
    DepthChasm,
    DepthWater,
    DepthGrass,
    DepthDark,
    DepthLarge,
    DepthTraps,
    DepthSecrets,
    ChalCount,
    CoinSml,
    EnergySml,
    Backpack,
    SeedPouch,
    ScrollHolder,
    WandHolster,
    PotionBandolier,
    //icons that appear in the about screen,variable spacing
    Libgdx,
    Aleks,
    Wata,
    Celesti,
    Kristjan,
    CubeCode,
    Purigro,
    ARCNOR,
}

pub(super) fn process(
    game_assets: &Res<GameAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    ui_texture_atlas_layout_handles: &mut ResMut<UiTextureAtlasLayoutHandles>,
    images: &ResMut<Assets<Image>>,
) {
    if let Some(icons) = images.get(game_assets.interfaces.icons.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(icons.size());

        texture_atlas.add_texture(URect::from_size(0, 0, 16, 16)); // Enter
        texture_atlas.add_texture(URect::from_size(17, 0, 17, 16)); // Gold
        texture_atlas.add_texture(URect::from_size(34, 0, 17, 16)); // Rankings
        texture_atlas.add_texture(URect::from_size(51, 0, 16, 16)); // Badges
        texture_atlas.add_texture(URect::from_size(68, 0, 16, 15)); // News
        texture_atlas.add_texture(URect::from_size(85, 0, 15, 15)); // Changes
        texture_atlas.add_texture(URect::from_size(102, 0, 14, 14)); // Prefs
        texture_atlas.add_texture(URect::from_size(119, 0, 16, 16)); // Shpx
        texture_atlas.add_texture(URect::from_size(136, 0, 17, 15)); // Journal
        texture_atlas.add_texture(URect::from_size(0, 16, 15, 11)); // Exit

        // Display 有两个变体
        texture_atlas.add_texture(URect::from_size(16, 16, 12, 16)); // DisplayPort
        texture_atlas.add_texture(URect::from_size(32, 16, 16, 12)); // DisplayLand

        texture_atlas.add_texture(URect::from_size(48, 16, 14, 15)); // Data
        texture_atlas.add_texture(URect::from_size(64, 16, 14, 14)); // Audio
        texture_atlas.add_texture(URect::from_size(80, 16, 14, 11)); // Langs
        texture_atlas.add_texture(URect::from_size(96, 16, 16, 12)); // Controller
        texture_atlas.add_texture(URect::from_size(112, 16, 15, 12)); // Keyboard
        texture_atlas.add_texture(URect::from_size(128, 16, 16, 13)); // Stats
        texture_atlas.add_texture(URect::from_size(144, 16, 15, 12)); // ChallengeGrey
        texture_atlas.add_texture(URect::from_size(160, 16, 15, 14)); // ScrollGrey
        texture_atlas.add_texture(URect::from_size(176, 16, 15, 10)); // Seed
        texture_atlas.add_texture(URect::from_size(192, 16, 14, 9)); // LeftArrow
        texture_atlas.add_texture(URect::from_size(208, 16, 14, 9)); // RightArrow
        texture_atlas.add_texture(URect::from_size(224, 16, 15, 12)); // Calendar
        texture_atlas.add_texture(URect::from_size(240, 16, 13, 10)); // Chevron

        texture_atlas.add_texture(URect::from_size(0, 32, 16, 16)); // Target
        texture_atlas.add_texture(URect::from_size(16, 32, 14, 14)); // Info
        texture_atlas.add_texture(URect::from_size(32, 32, 14, 14)); // Warning
        texture_atlas.add_texture(URect::from_size(48, 32, 12, 12)); // Unchecked
        texture_atlas.add_texture(URect::from_size(64, 32, 12, 12)); // Checked
        texture_atlas.add_texture(URect::from_size(80, 32, 11, 11)); // Close
        texture_atlas.add_texture(URect::from_size(96, 32, 11, 11)); // Plus
        texture_atlas.add_texture(URect::from_size(112, 32, 11, 11)); // Repeat
        texture_atlas.add_texture(URect::from_size(128, 32, 11, 11)); // Arrow
        texture_atlas.add_texture(URect::from_size(144, 32, 15, 12)); // ChallengeColor
        texture_atlas.add_texture(URect::from_size(160, 32, 15, 14)); // ScrollColor
        texture_atlas.add_texture(URect::from_size(176, 32, 13, 13)); // Copy
        texture_atlas.add_texture(URect::from_size(192, 32, 13, 13)); // Paste

        texture_atlas.add_texture(URect::from_size(0, 48, 16, 16)); // BackpackLrg
        texture_atlas.add_texture(URect::from_size(16, 48, 13, 13)); // Talent
        texture_atlas.add_texture(URect::from_size(32, 48, 14, 14)); // Magnify
        texture_atlas.add_texture(URect::from_size(48, 48, 9, 13)); // Snake
        texture_atlas.add_texture(URect::from_size(64, 48, 16, 15)); // Buffs
        texture_atlas.add_texture(URect::from_size(80, 48, 13, 16)); // Catalog
        texture_atlas.add_texture(URect::from_size(96, 48, 16, 16)); // Alchemy
        texture_atlas.add_texture(URect::from_size(112, 48, 16, 16)); // Grass

        texture_atlas.add_texture(URect::from_size(0, 64, 15, 16)); // Stairs
        texture_atlas.add_texture(URect::from_size(16, 64, 15, 16)); // StairsChasm
        texture_atlas.add_texture(URect::from_size(32, 64, 15, 16)); // StairsWater
        texture_atlas.add_texture(URect::from_size(48, 64, 15, 16)); // StairsGrass
        texture_atlas.add_texture(URect::from_size(64, 64, 15, 16)); // StairsDark
        texture_atlas.add_texture(URect::from_size(80, 64, 15, 16)); // StairsLarge
        texture_atlas.add_texture(URect::from_size(96, 64, 15, 16)); // StairsTraps
        texture_atlas.add_texture(URect::from_size(112, 64, 15, 16)); // StairsSecrets
        texture_atlas.add_texture(URect::from_size(128, 64, 16, 16)); // WellHealth
        texture_atlas.add_texture(URect::from_size(144, 64, 16, 16)); // WellAwareness
        texture_atlas.add_texture(URect::from_size(160, 64, 16, 16)); // SacrificeAltar
        texture_atlas.add_texture(URect::from_size(176, 64, 16, 16)); // DistantWell

        texture_atlas.add_texture(URect::from_size(0, 80, 8, 8)); // Skull
        texture_atlas.add_texture(URect::from_size(8, 80, 8, 8)); // Busy
        texture_atlas.add_texture(URect::from_size(0, 88, 7, 5)); // Compass
        texture_atlas.add_texture(URect::from_size(16, 80, 9, 8)); // Sleep
        texture_atlas.add_texture(URect::from_size(16, 88, 8, 8)); // Alert
        texture_atlas.add_texture(URect::from_size(24, 88, 8, 8)); // Lost

        // Depth 系列 (使用默认位置)
        texture_atlas.add_texture(URect::from_size(32, 80, 6, 7)); // Depth
        texture_atlas.add_texture(URect::from_size(40, 80, 7, 7)); // DepthChasm
        texture_atlas.add_texture(URect::from_size(48, 80, 7, 7)); // DepthWater
        texture_atlas.add_texture(URect::from_size(56, 80, 7, 7)); // DepthGrass
        texture_atlas.add_texture(URect::from_size(64, 80, 7, 7)); // DepthDark
        texture_atlas.add_texture(URect::from_size(72, 80, 7, 7)); // DepthLarge
        texture_atlas.add_texture(URect::from_size(80, 80, 7, 7)); // DepthTraps
        texture_atlas.add_texture(URect::from_size(88, 80, 7, 7)); // DepthSecrets

        texture_atlas.add_texture(URect::from_size(160, 80, 7, 7)); // ChalCount
        texture_atlas.add_texture(URect::from_size(168, 80, 7, 7)); // CoinSml
        texture_atlas.add_texture(URect::from_size(168, 88, 8, 7)); // EnergySml
        texture_atlas.add_texture(URect::from_size(176, 80, 10, 10)); // Backpack
        texture_atlas.add_texture(URect::from_size(186, 80, 10, 10)); // SeedPouch
        texture_atlas.add_texture(URect::from_size(196, 80, 10, 10)); // ScrollHolder
        texture_atlas.add_texture(URect::from_size(206, 80, 10, 10)); // WandHolster
        texture_atlas.add_texture(URect::from_size(216, 80, 10, 10)); // PotionBandolier

        texture_atlas.add_texture(URect::from_size(0, 96, 16, 13)); // Libgdx
        texture_atlas.add_texture(URect::from_size(16, 96, 16, 13)); // Aleks
        texture_atlas.add_texture(URect::from_size(0, 112, 17, 12)); // Wata
        texture_atlas.add_texture(URect::from_size(32, 96, 32, 32)); // Celesti
        texture_atlas.add_texture(URect::from_size(64, 96, 32, 32)); // Kristjan
        texture_atlas.add_texture(URect::from_size(96, 96, 32, 32)); // Arcnor
        texture_atlas.add_texture(URect::from_size(128, 96, 32, 32)); // Purigro
        texture_atlas.add_texture(URect::from_size(160, 96, 27, 30)); // CubeCode

        ui_texture_atlas_layout_handles.icons = texture_atlases.add(texture_atlas);
    }
}
