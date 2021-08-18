
#[derive(Debug)]
pub struct Apple {
    coordinates: (u32, u32)
}

impl Apple {
    pub fn new(x: u32, y: u32) -> Apple {
        Apple {
            coordinates: (x, y)
        }
    }

    pub fn reset(&mut self, x: u32, y: u32) {
        self.coordinates = (x, y);
    }

    pub fn is_set(&self, x: u32, y: u32) -> bool {
        let (actual_x, actual_y) = self.coordinates;
        actual_x == x && actual_y == y
    }
}
