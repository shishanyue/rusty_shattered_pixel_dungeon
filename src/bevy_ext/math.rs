use bevy::prelude::*;

pub trait URectExt {
    fn from_size(x: u32, y: u32, width: u32, height: u32) -> Self;
}

impl URectExt for URect {
    fn from_size(x: u32, y: u32, width: u32, height: u32) -> Self {
        URect::new(x, y, x + width, y + height)
    }
}
