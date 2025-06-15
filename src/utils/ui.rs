#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn intersect(&self, other: Self) -> Self {
        Self::new(
            max(self.left, other.left),
            max(self.top, other.top),
            max(self.right, other.right),
            max(self.bottom, other.bottom),
        )
    }
}
