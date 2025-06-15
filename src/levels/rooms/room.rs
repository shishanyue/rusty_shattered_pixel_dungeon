use std::cmp::min;

use indexmap::IndexMap;

use crate::{
    levels::level::Level,
    utils::{math::Point, ui::Rect},
};

pub enum Direction {
    ALL = 0,
    LEFT = 1,
    TOP = 2,
    RIGHT = 3,
    BOTTOM = 4,
}
pub trait Room
{
    // **** Spatial logic ****

    //Note: when overriding these YOU MUST store any randomly decided values.
    //With the same room and the same parameters these should always return
    //the same value over multiple calls, even if there's some randomness initially.
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn min_width(&self) -> i32;
    fn max_width(&self) -> i32;
    fn min_height(&self) -> i32;
    fn max_height(&self) -> i32;
    fn get_connected(&self) -> IndexMap<Box<dyn Room>, Door>;
    fn get_rect(&self) -> &Rect;

    fn set_size(
        &mut self,
        min_width: i32,
        max_width: i32,
        min_height: i32,
        max_height: i32,
    ) -> bool {
        if min_width < self.min_width()
            || max_width > self.max_width()
            || min_height < self.min_height()
            || max_height > self.max_height()
            || min_width > max_width
            || min_height > max_height
        {
            false
        } else {
            self.resize(
                rand::random_range(min_width..max_width) - 1,
                rand::random_range(min_height..max_height) - 1,
            );
            true
        }
    }
    fn set_size_with_limit(&mut self, width: i32, height: i32) -> bool {
        if width < self.min_width() || height < self.min_height() {
            false
        } else {
            self.set_size(
                self.min_width(),
                self.max_width(),
                self.min_height(),
                self.max_height(),
            );
            if self.width() > width || self.height() > height {
                self.resize(min(self.width(), width) - 1, min(self.height(), height) - 1);
            }
            true
        }
    }
    fn resize(&mut self, width: i32, height: i32);
    fn paint(&self, level:Box<dyn Level>);
    fn poin_inside(&self, from: Point, n: i32) -> Point {
        let mut step = from;
        let Rect {
            left,
            right,
            top,
            bottom,
        } = *self.get_rect();
        if from.x == left {
            step.offset(n, 0);
        } else if from.x == right {
            step.offset(-n, 0);
        } else if from.y == top {
            step.offset(0, n);
        } else if from.y == bottom {
            step.offset(0, -n);
        }
        step
    }
    fn current_connections(&self, direction: Direction) -> i32 {
        match direction {
            Direction::ALL => self.get_connected().len() as i32,
            _ => {
                let mut total = 0;
                for (room, _) in self.get_connected() {
                    let i = self.get_rect().intersect(*room.get_rect());
                    
                }

                total
            }
        }
    }

    fn random(&self, m: i32) -> Point {
        let Rect {
            left,
            right,
            top,
            bottom,
        } = *self.get_rect();
        Point::new(
            rand::random_range((left + m)..(right - m)),
            rand::random_range((top + m)..(bottom - m)),
        )
    }
}

pub struct Door {}
