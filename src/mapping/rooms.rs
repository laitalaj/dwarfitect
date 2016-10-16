use mapping::shapes::{Rect, Point};
use std::cmp::Ordering;
use collections::{Matrix, Vector};

pub struct Room {
    rect: Rect,
}

pub struct Layout {
    rooms: Vector<Room>,
}

// Implement methods that manipulate the rectangle inside the room
impl_rect_methods!(Room, rect);

impl Room {
    pub fn new(rect: Rect) -> Self {
        Room { rect: rect }
    }
}

impl Layout {
    pub fn new(rooms: Vector<Room>) -> Self {
        Layout { rooms: rooms }
    }
    fn calculate_bounding_box(&self) -> Rect {
        let mut min_x = isize::max_value();
        let mut min_y = isize::max_value();
        let mut max_x = isize::min_value();
        let mut max_y = isize::min_value();
        for i in 0..self.rooms.len() {
            let top_left = self.rooms[i].top_left();
            let bottom_right = self.rooms[i].bottom_right();
            if top_left.x < min_x {
                min_x = top_left.x;
            }
            if top_left.y < min_y {
                min_y = top_left.y;
            }
            if bottom_right.x > max_x {
                max_x = bottom_right.x;
            }
            if bottom_right.y > max_y {
                max_y = bottom_right.y;
            }
        }
        Rect {
            x: min_x,
            y: min_y,
            w: max_x - min_x,
            h: max_y - min_y,
        }
    }
    pub fn as_char_matrix(&self) -> Matrix<char> {
        let bounding_box = self.calculate_bounding_box();
        let top_left = bounding_box.top_left();
        let mut matrix = Matrix::new(bounding_box.w as usize, bounding_box.h as usize);
        for i in 0..self.rooms.len() {
            let w = self.rooms[i].get_w();
            let h = self.rooms[i].get_h();
            let room_top_left = self.rooms[i].top_left();
            let base_x = room_top_left.x - top_left.x;
            let base_y = room_top_left.y - top_left.y;
            for x in 0..w {
                for y in 0..h {
                    if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                        matrix.set((x + base_x) as usize, (y + base_y) as usize, '#');
                        // TODO: Different chars
                    } else {
                        matrix.set((x + base_x) as usize, (y + base_y) as usize, '.');
                    }
                }
            }
        }
        matrix
    }
}
