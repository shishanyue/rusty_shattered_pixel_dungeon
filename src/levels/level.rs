use crate::levels::terrain::Terrain;

pub enum Feeling {
    NONE,
    CHASM,
    WATER,
    GRASS,
    DARK,
    LARGE,
    TRAPS,
    SECRETS,
}

pub trait Level {
    fn get_map_mut(&mut self) -> &mut Vec<Terrain>;
    fn get_map(&self) -> &Vec<Terrain>;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn length(&self) -> i32;
}
