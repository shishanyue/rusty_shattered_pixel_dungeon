use super::{Scene, SceneState};
use crate::{
    assets::GameAssets,
    bevy_ext::{app::AppExt, ui::create_button},
    ui::{
        banner_sprites::BannerSpriteKind, chrome::{ChromeKind, ChromeTextureSlicer}, icons::IconKind, UiTextureAtlasLayoutHandles
    },
};
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Default)]
pub struct TitleScene;

#[derive(Component)]
struct TitleSceneMark;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum ButtonLabel {
    EnterDungeon,
    Badges,
    Rankings,
    Supporter,
    News,
    Changes,
    Prefs,
    About,
}
#[derive(Component)]
pub struct PixelDungeon;
#[derive(Component)]
pub struct PixelDungeonSigns;
impl Scene for TitleScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<TitleSceneMark, _>(SceneState::TitleScene, setup)
            .add_systems(
                Update,
                check_interaction.run_if(in_state(SceneState::TitleScene)),
            );
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    ui_texture_atlas_layout_handles: Res<UiTextureAtlasLayoutHandles>,
    chrome_texture_slicer: Res<ChromeTextureSlicer>,
) {
    let text_font = TextFont {
        font: game_assets.fonts.pixelfont.clone(),
        font_size: 30.,
        ..Default::default()
    };

    let button_image = game_assets.interfaces.chrome.clone();

    let button_slicer = chrome_texture_slicer.grey_button_tr_slicer.clone();
    let icons = game_assets.interfaces.icons.clone();

    let icons_layout = ui_texture_atlas_layout_handles.icons.clone();
    let banners_layout = ui_texture_atlas_layout_handles.banner_sprites.clone();
    let chrome_layout = ui_texture_atlas_layout_handles.chrome.clone();

    let icon_style = Node {
        width: Val::Px(30.),
        margin: UiRect {
            right: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            ..Default::default()
        },
        ..Default::default()
    };

    let two_button_style = Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(45.),
        padding: UiRect {
            top: Val::Px(5.),
            bottom: Val::Px(5.),
            left: Val::Px(20.),
            right: Val::Px(20.),
        },
        ..Default::default()
    };

    let three_button_style = Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(30.),
        padding: UiRect {
            top: Val::Px(5.),
            bottom: Val::Px(5.),
            left: Val::Px(20.),
            right: Val::Px(20.),
        },
        ..Default::default()
    };

    commands.spawn((
        TitleSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::FlexStart,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(35.),
                    margin: UiRect {
                        top: Val::Percent(1.),
                        bottom: Val::Percent(2.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ImageNode::from_atlas_image(
                    game_assets.interfaces.banners.clone(),
                    TextureAtlas {
                        layout: banners_layout.clone(),
                        index: BannerSpriteKind::TitleLand as usize
                    }
                ),
                PixelDungeon,
                children![(
                    Node {
                        width: Val::Percent(94.),
                        ..Default::default()
                    },
                    ImageNode::from_atlas_image(
                        game_assets.interfaces.banners.clone(),
                        TextureAtlas {
                            layout: banners_layout.clone(),
                            index: BannerSpriteKind::TitleGlowLand as usize
                        }
                    ),
                    PixelDungeonSigns,
                ),]
            ),
            (
                Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                children![
                    (
                        Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceEvenly,
                            ..Default::default()
                        },
                        children![
                            create_button(
                                button_image.clone(),
                                two_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::EnterDungeon,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Enter as usize
                                },
                                "进入地牢",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            ),
                            create_button(
                                button_image.clone(),
                                two_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::Supporter,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Gold as usize
                                },
                                "支持游戏开发",
                                text_font.clone(),
                                Color::Srgba(GOLD),
                            )
                        ]
                    ),
                    (
                        Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceEvenly,
                            ..Default::default()
                        },
                        children![
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::Rankings,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Rankings as usize
                                },
                                "排行榜",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            ),
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::News,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::News as usize
                                },
                                "游戏新闻",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            ),
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::Prefs,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Prefs as usize
                                },
                                "设置",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            )
                        ]
                    ),
                    (
                        Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceEvenly,
                            ..Default::default()
                        },
                        children![
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::Rankings,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Badges as usize
                                },
                                "徽章",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            ),
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::News,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Changes as usize
                                },
                                "改动",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            ),
                            create_button(
                                button_image.clone(),
                                three_button_style.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeKind::GreyButton as usize
                                },
                                ButtonLabel::Prefs,
                                button_slicer.clone(),
                                icons.clone(),
                                icon_style.clone(),
                                TextureAtlas {
                                    layout: icons_layout.clone(),
                                    index: IconKind::Journal as usize
                                },
                                "关于",
                                text_font.clone(),
                                Color::Srgba(WHITE),
                            )
                        ]
                    )
                ]
            )
        ],
    ));
}

fn check_interaction(
    interaction_query: Query<(&Interaction, &ButtonLabel), Changed<Interaction>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    for (interaction, label) in interaction_query.iter() {
        match label {
            ButtonLabel::EnterDungeon => {
                println!("press");
                if *interaction == Interaction::Pressed {
                    scene_state.set(SceneState::StartScene);
                }
            }
            ButtonLabel::Badges => {}
            ButtonLabel::Rankings => {}
            ButtonLabel::Supporter => {}
            ButtonLabel::News => {}
            ButtonLabel::Changes => {}
            ButtonLabel::Prefs => {}
            ButtonLabel::About => {}
        }
    }
}
