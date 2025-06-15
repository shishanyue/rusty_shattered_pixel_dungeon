use crate::{
    levels::{
        level::Level,
        painters::painter::{BaiscPainter, Painter},
        rooms::{room::Room, standard::standard_room::StandardRoom},
        terrain::Terrain,
    },
    utils::math::Rect,
};

pub struct EmptyRoom {
    pub rect: Rect,
}

impl Room for EmptyRoom {
    fn paint<T: Level>(&self, level: &mut T) {
        BaiscPainter::fill(level, self.rect, Terrain::Wall);
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.rect.w = width;
        self.rect.h = height
    }
}

impl StandardRoom for EmptyRoom {}
