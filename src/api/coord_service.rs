#[derive(Debug, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn update_coords(coords_from: &mut Coord, coords_to: &Coord){
    coords_from.x = coords_to.x;
    coords_from.y = coords_to.y;
}