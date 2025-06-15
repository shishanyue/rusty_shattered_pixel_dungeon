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
    
    fn width(&self) -> i32 {
        todo!()
    }
    
    fn height(&self) -> i32 {
        todo!()
    }
    
    fn min_width(&self) -> i32 {
        todo!()
    }
    
    fn max_width(&self) -> i32 {
        todo!()
    }
    
    fn min_height(&self) -> i32 {
        todo!()
    }
    
    fn max_height(&self) -> i32 {
        todo!()
    }
}

impl StandardRoom for EmptyRoom {}
