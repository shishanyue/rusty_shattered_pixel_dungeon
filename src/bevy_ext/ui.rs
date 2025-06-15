use bevy::prelude::*;

pub fn create_button<T>(
    button_image: Handle<Image>,
    button_style: Node,
    button_atlas: TextureAtlas,
    button_label: T,
    slicer: TextureSlicer,
    icon: Handle<Image>,
    icon_style: Node,
    icon_atlas: TextureAtlas,
    text: &str,
    text_style: TextFont,
    text_color: Color,
) -> impl Bundle
where
    T: Component,
{
    (
        Button,
        button_style,
        ImageNode::from_atlas_image(button_image, button_atlas)
            .with_mode(NodeImageMode::Sliced(slicer)),
        button_label,
        children![
            (icon_style, ImageNode::from_atlas_image(icon, icon_atlas)),
            (Text::new(text), text_style, TextColor(text_color))
        ],
    )
}
