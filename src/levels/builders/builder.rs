use bevy::prelude::*;

use crate::levels::rooms::room::Room;

pub trait Builder {
    //If builders require additional parameters, they should
    // request them in their constructor or other methods

    //builders take a list of rooms and returns them as a connected map
    //returns null on failure
    fn build<T: Room>(rooms: &Vec<T>);

    fn find_neighbours<T: Room>(rooms: &Vec<T>){
        for i in rooms{
            
        }
    }
}
