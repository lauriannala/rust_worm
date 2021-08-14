#[derive(Debug, PartialEq)]
pub enum MoveDirection {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Debug)]
pub struct Worm {
    pub x: u32,
    pub y: u32,
    pub length: u32,

    coordinates: Vec<(u32, u32)>,
    direction: MoveDirection,
    should_grow: bool
}

impl Worm {
    pub fn new(x: u32, y: u32, length: u32) -> Worm {
        let mut coordinates = Vec::new();

        for i in 0..length {
            coordinates.push((x, y + i));
        }

        Worm {
            x,
            y,
            length,
            coordinates,
            direction: MoveDirection::UP,
            should_grow: false
        }
    }

    pub fn is_set(&self, requested_x: &u32, requested_y: &u32) -> bool {
        self
            .coordinates
            .iter()
            .any(|(x, y)| x == requested_x && y == requested_y)
    }

    pub fn set_direction(&mut self, direction: MoveDirection) {
        if self.direction == MoveDirection::UP && direction == MoveDirection::DOWN {
            return;
        }
        if self.direction == MoveDirection::DOWN && direction == MoveDirection::UP {
            return;
        }
        if self.direction == MoveDirection::RIGHT && direction == MoveDirection::LEFT {
            return;
        }
        if self.direction == MoveDirection::LEFT && direction == MoveDirection::RIGHT {
            return;
        }

        self.direction = direction;
    }

    pub fn move_next(&mut self, width: u32, height: u32) {
        let mut first_coordinate = true;
        let mut previous_x: Option<u32> = None;
        let mut previous_y: Option<u32> = None;
        let mut new_tail: (u32, u32) = (0, 0);

        if self.should_grow {
            match self.coordinates.last() {
                Some(coordinates) => {
                    new_tail = coordinates.clone();
                },
                None => panic!("Can't grow worm self.coordinates.last() is None.")
            }
        }

        for coords in &mut self.coordinates {
            let mut negative_offscreen = (false, false);
            if first_coordinate {
                previous_x = Some(coords.0);
                previous_y = Some(coords.1);

                let mut new_x = coords.0.clone();
                let mut new_y = coords.1.clone();

                match self.direction {
                    MoveDirection::UP => {
                        if previous_y.unwrap() == 0u32 {
                            negative_offscreen.1 = true;
                        } else {
                            new_y = previous_y.unwrap() - 1;
                        }
                    },
                    MoveDirection::DOWN => {
                        new_y = previous_y.unwrap() + 1;
                    },
                    MoveDirection::RIGHT => {
                        new_x = previous_x.unwrap() + 1;
                    },
                    MoveDirection::LEFT => {
                        if previous_x.unwrap() == 0u32 {
                            negative_offscreen.0 = true;
                        } else {
                            new_x = previous_x.unwrap() - 1;
                        }
                    }
                }

                if new_x == width {
                    new_x = 0;
                }
                if new_y == height {
                    new_y = 0;
                }
                if negative_offscreen.0 {
                    new_x = width;
                }
                if negative_offscreen.1 {
                    new_y = height;
                }

                coords.0 = new_x;
                coords.1 = new_y;

                first_coordinate = false;
            } else {
                let new_x = previous_x;
                let new_y = previous_y;

                previous_x = Some(coords.0);
                previous_y = Some(coords.1);

                coords.0 = new_x.unwrap();
                coords.1 = new_y.unwrap();
            }
        }

        if self.should_grow {
            self.coordinates.push(new_tail);
            self.should_grow = false;
        }
    }
}
