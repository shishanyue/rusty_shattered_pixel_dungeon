use rusty_shattered_pixel_dungeon::{
    levels::{basic_level::BasicLevel, rooms::{room::Room, standard::empty_room::EmptyRoom}, terrain::Terrain},
    utils::math::Rect,
};

#[test]
fn empty_room() {
    let empty_room = EmptyRoom {
        rect: Rect {
            x: 1,
            y: 1,
            w: 4,
            h: 4,
        },
    };
    let mut level = BasicLevel::default();
    level.map = vec![Terrain::Empty; 10];
    empty_room.paint(&mut level);

    print!("{:?}",level.map)

}
