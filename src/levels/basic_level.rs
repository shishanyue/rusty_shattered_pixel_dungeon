use crate::levels::{level::Level, terrain::Terrain};

#[derive(Debug, Default)]
pub struct BasicLevel {
    pub width: i32,
    pub height: i32,
    pub length: i32,

    pub map: Vec<Terrain>,
}

impl Level for BasicLevel {
    fn get_map_mut(&mut self) -> &mut Vec<Terrain> {
        &mut self.map
    }

    fn get_map(&self) -> &Vec<Terrain> {
        &self.map
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn length(&self) -> i32 {
        self.length
    }
}
