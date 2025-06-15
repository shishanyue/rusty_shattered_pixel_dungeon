use crate::levels::level::Level;

pub trait Room {
    // **** Spatial logic ****

    //Note: when overriding these YOU MUST store any randomly decided values.
    //With the same room and the same parameters these should always return
    //the same value over multiple calls, even if there's some randomness initially.
    fn min_width(&self) -> i32 {
        -1
    }
    fn max_width(&self) -> i32 {
        -1
    }
    fn min_height(&self) -> i32 {
        -1
    }
    fn max_height(&self) -> i32 {
        -1
    }
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
    fn resize(&mut self, width: i32, height: i32);
    fn paint<T: Level>(&self, level: &mut T);
}
