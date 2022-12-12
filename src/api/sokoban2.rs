// TODO Traer is_object
// TODO Traer refresh_map
// TODO Traer get_object
// TODO Traer constantes
// TODO Traer get_dimentions
// TODO Traer get_map

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
    FileError(String),
    GTKError(String),
    CommandError(String),
}

#[derive(Debug)]
pub struct Sokoban {
    pub map: Vec<Vec<u8>>,
    pub user_coords: Coord,
    pub target_coords: Vec<Coord>,
    pub boxes_coords: Vec<Coord>,
    pub rows: usize,
    pub columns: usize,
}

impl Sokoban {
    // TODO OK
    pub fn create_from_path(argv: &String) -> Result<Self, SokobanError> {
        let mut map = read_file(argv)?;
        validate_file(&map)?;

        let mut input = map.to_owned();

        let (rows, columns) = get_dimentions(&input);
        let input = delete_enters(&mut input);
        let mut map = create_map(input.clone(), rows, columns);

        let mut target_coords = get_coords(input.clone(), TARGET_STR, rows, columns)?;
        let boxes_on_target_coords = get_coords(input.clone(), BOX_ON_TARGET_STR, rows, columns)?;
        target_coords.append(&mut boxes_on_target_coords.clone());

        let boxes_coords = get_coords(input.clone(), BOX_STR, rows, columns)?;
        target_coords.append(&mut boxes_on_target_coords.clone());

        let mut vec_user_coords = get_coords(input.clone(), PLAYER_STR, rows, columns)?;

        Ok(Sokoban {
            map: map,
            user_coords: vec_user_coords.remove(0),
            target_coords: target_coords,
            boxes_coords: boxes_coords,
            rows: rows,
            columns: columns,
        })
    }

    // TODO OK
    pub fn print(&self) {
        let mut str_map = String::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                let object = get_object(self.map[row][column]);
                str_map.push(object.parse().unwrap());
            }
            str_map.push_str(ENTER_STR2);
        }

        println!("{}", str_map);
    }

    // TODO OK
    fn refresh_map(&self, coords_from: &mut Coord, coords_to: &Coord, object: u8) {
        self.map[coords_to.y][coords_to.x] = object;

        if self.target_coords.contains(coords_from) {
            self.map[coords_from.y][coords_from.x] = TARGET_U8;
        } else {
            self.map[coords_from.y][coords_from.x] = AIR_U8;
        }
    }

    // TODO OK
    fn move_player(&mut self, coords_to: &Coord) {
        self.refresh_map(self.user_coords, coords_to, PLAYER_U8);
        update_coords(self.user_coords, coords_to);
    }

    // TODO OK
    fn move_box(&mut self, coords_from: &mut Coord, coords_to: &mut Coord) {
        let move_to_target = is_object(&coords_to, TARGET_U8, self.map);
        let move_from_target = is_object(&coords_from, BOX_ON_TARGET_U8, self.map);

        match self
            .boxes_coords
            .iter()
            .position(|b| equals_to(b, coords_from))
        {
            None => {}
            Some(index_to_remove) => {
                self.boxes_coords.remove(index_to_remove);
                self.boxes_coords.push(Coord {
                    x: coords_to.x,
                    y: coords_to.y,
                });
            }
        }
        self.refresh_map(coords_from, coords_to, BOX_U8);
        if move_from_target {
            self.map[coords_to.y][coords_to.x] = BOX_U8;
        }
        if move_to_target {
            self.map[coords_to.y][coords_to.x] = BOX_ON_TARGET_U8;
        }
    }

    // TODO OK
    fn process_move(&mut self, movement: Move) {
        let (delta_x, delta_y) = get_deltas(movement);
        let mut next_coord: Coord = get_next_coord(self.user_coords, delta_x, delta_y);
        let mut next_next_coord = get_next_coord(&next_coord, delta_x, delta_y);

        if self.is_object(&next_coord, WALL_U8) {
            return;
        }

        if self.is_object(&next_coord, BOX_U8)
            || self.is_object(&next_coord, BOX_ON_TARGET_U8)
        {
            if !(self.is_object(&next_next_coord, AIR_U8)
                || self.is_object(&next_next_coord, TARGET_U8))
            {
                return;
            }
            sok.move_box(&mut next_coord, &mut next_next_coord);
        }
        sok.move_player(&next_coord);
    }

    // TODO OK
    pub fn is_object(&self, next_coord: &Coord, object_to_compare: u8) -> bool {
        return self.map[next_coord.y as usize][next_coord.x as usize] == object_to_compare;
    }

    // TODO OK
    pub fn victory(&self) -> bool {
        for box_coords in self.boxes_coords.iter() {
            let mut found = false;
            for target in self.target_coords {
                if equals_to(box_coords, target) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        return true;
    }
}

// TODO OK
pub fn get_coords(
    mut map_string: String,
    object: &str,
    rows: usize,
    columns: usize,
) -> Result<Vec<Coord>, SokobanError> {
    let mut row = 0;
    let mut column = 0;
    let mut coord_vec = Vec::new();

    while row < rows && !map_string.is_empty() {
        // todo refactor
        if map_string.remove(0).to_string() == object.to_string() {
            let new_coord = Coord { x: column, y: row };
            coord_vec.push(new_coord);
        }
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    Ok(coord_vec)
}
