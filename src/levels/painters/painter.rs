use crate::{
    levels::{level::Level, terrain::Terrain},
    utils::math::*,
};

pub trait Painter {
    fn set<T: Level>(level: &mut T, point: Point, value: Terrain) {
        let width = level.width();
        level.get_map_mut()[((point.x + point.y) * width) as usize] = value;
    }
    fn fill<T: Level>(level: &mut T, rect: Rect, value: Terrain) {
        let width = level.width();

        let mut pos = rect.y * width + rect.x;

        for _ in rect.y..(rect.y + rect.h) {
            pos += width;
            level.get_map_mut()[pos as usize..(pos + rect.w) as usize].fill(value);
        }
    }
    fn draw_line<T: Level>(level: &mut T, from: Point, to: Point, value: Terrain) {
        let (x, y, mut dx, mut dy) = (
            from.x as f32,
            from.y as f32,
            (to.x - from.x) as f32,
            (to.y - from.y) as f32,
        );
        let movingby_x = dx.abs() >= dy.abs();

        //normalize
        if movingby_x {
            dy /= dx.abs();
            dx /= dx.abs();
        } else {
            dx /= dy.abs();
            dy /= dy.abs();
        }

        Self::set(
            level,
            Point {
                x: x.round() as i32,
                y: y.round() as i32,
            },
            value,
        );
    }
}

#[derive(Default)]
pub struct BaiscPainter;

impl Painter for BaiscPainter {}
