use bevy::prelude::*;

pub fn create_button<T>(
    parent: &mut ChildSpawnerCommands,
    button_image: Handle<Image>,
    button_style: Node,
    button_label: T,
    slicer: TextureSlicer,
    icon: Option<Handle<Image>>,
    icon_style: Option<Node>,
    text: &str,
    text_style: TextFont,
    text_color: Color,
) where
    T: Component,
{
    parent.spawn((
        button_style,
        ImageNode::new(button_image).with_mode(NodeImageMode::Sliced(slicer)),
        button_label,
        children![(
            icon_style.unwrap_or_default(),
            ImageNode::new(icon.unwrap_or_default()),
            Text2d::new(text),
            text_style,
            TextColor(text_color)
        )],
    ));
}
