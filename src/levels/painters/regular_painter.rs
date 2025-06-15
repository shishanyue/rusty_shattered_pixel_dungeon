use crate::levels::painters::painter::Painter;

pub trait RegularPainter: Painter {
    fn set_water();
}
