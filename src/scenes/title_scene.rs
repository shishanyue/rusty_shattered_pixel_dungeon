use super::{Scene, SceneState};
use crate::{
    assets::GameAssets,
    bevy_ext::{app::AppExt, ui::create_button},
};
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

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_white_style = TextFont {
        font_size: 30.,
        ..Default::default()
    };
    let text_gold_style = TextFont {
        font_size: 30.,
        ..Default::default()
    };

    let button_image = game_assets.environment.caves_boss.1.clone();
    let slicer = TextureSlicer {
        border: BorderRect::all(4.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 3.0,
    };
    let icons = game_assets.environment.caves_boss.1.clone();
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

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent
                .spawn((
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
                    ImageNode::new(game_assets.environment.caves_boss.1.clone()),
                    PixelDungeon,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::new(game_assets.environment.caves_boss.1.clone()),
                        PixelDungeonSigns,
                    ));
                });

            parent
                .spawn(Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceEvenly,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            create_button(
                                parent,
                                button_image.clone(),
                                two_button_style.clone(),
                                ButtonLabel::EnterDungeon,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "进入地牢",
                                text_white_style.clone(),
                                Color::BLACK,
                            );
                            create_button(
                                parent,
                                button_image.clone(),
                                two_button_style.clone(),
                                ButtonLabel::Supporter,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "支持游戏开发",
                                text_gold_style.clone(),
                                Color::BLACK,
                            );
                        });

                    parent
                        .spawn(Node {
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::SpaceEvenly,
                                ..Default::default()
                        })
                        .with_children(|parent| {
                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Rankings,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "排行榜",
                                text_white_style.clone(),
                                Color::BLACK,
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::News,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "游戏新闻",
                                text_white_style.clone(),
                                Color::BLACK,
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Prefs,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "设置",
                                text_white_style.clone(),
                                Color::BLACK,
                            );
                        });

                    parent
                        .spawn(Node {
                            
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::SpaceEvenly,
                                ..Default::default()
                        })
                        .with_children(|parent| {
                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Badges,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "徽章",
                                text_white_style.clone(),
                                Color::BLACK,
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Changes,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "改动",
                                text_white_style.clone(),
                                Color::BLACK,
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::About,
                                slicer.clone(),
                                Some(icons.clone()),
                                Some(icon_style.clone()),
                                "关于",
                                text_white_style.clone(),
                                Color::BLACK,
                            );
                        });
                });
        });
}

fn check_interaction(
    interaction_query: Query<(&Interaction, &ButtonLabel), Changed<Interaction>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    for (interaction, label) in interaction_query.iter() {
        match label {
            ButtonLabel::EnterDungeon => {
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
