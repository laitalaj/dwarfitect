//! Module for all kinds of geometry
use std::cmp::Ordering;

/// Direction enums
#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A simple point struct
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

/// A simple rectangle struct
/// (x, y) is the top left corner, w is width, h is height
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Rect {
    pub x: isize,
    pub y: isize,
    pub w: isize,
    pub h: isize,
}

impl Point {
	pub fn new(x: isize, y: isize) -> Self {
		Point { x: x, y: y }
	}
	/// Returns distance between two points
	pub fn dist(&self, other: Point) -> f32 {
		let diff = self.diff(other);
		((diff.x as f32).powi(2) + (diff.y as f32).powi(2)).sqrt()
	}
    /// Returns a vector (represented by a Point) from this point to the other
    /// point
    pub fn diff(&self, point: Point) -> Point {
        Point {
            x: point.x - self.x,
            y: point.y - self.y,
        }
    }
}

impl Rect {
	pub fn new(x: isize, y: isize, w: isize, h: isize) -> Rect {
		Rect{ x: x, y: y, w: w, h: h}
	}
    /// "Rotates" the rectangle: returns a new rectangle with switched width and
    /// height.
    pub fn rotate(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.h,
            h: self.w,
        }
    }
    /// Checks if this rect collides (overlaps) with another rectangle.
    pub fn collides_with(&self, rect: Rect) -> bool {
        self.x < rect.x + rect.w && rect.x < self.x + self.w && self.y < rect.y + rect.h &&
        rect.y < self.y + self.h
    }
    /// Returns this rect's area (w*h)
    pub fn area(&self) -> isize {
        self.w * self.h
    }
    /// Gets the center point (x+w/2, y+h/2)
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.w / 2,
            y: self.y + self.h / 2,
        }
    }
    /// Gets the top left corner (x, y)
    pub fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    /// Gets the bottom right corner (x+w, y+h)
    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }
    /// Sets the center of the rect to given value
    pub fn set_center(&mut self, x: isize, y: isize) {
    	let current_center = self.center();
    	let desired_center = Point::new(x, y);
    	let move_vector = current_center.diff(desired_center);
    	self.x += move_vector.x;
    	self.y += move_vector.y;
    } 
    /// Compare based on distance from origo
    pub fn origo_cmp(&self, other: &Rect) -> Ordering {
    	let origo = Point::new(0, 0);
    	let my_dist = origo.dist(self.center());
    	let other_dist = origo.dist(other.center());
    	my_dist.partial_cmp(&other_dist).unwrap_or(Ordering::Equal)
    }
}

/// Implement methods that interact with an enclosed Rect in a struct
#[macro_export]
macro_rules! impl_rect_methods {
	($Struct:ty, $rect:ident) => {
		impl $Struct {
			/// Gives an ordering according to the rect's ordering
		    pub fn rect_cmp(&self, other: &$Struct) -> Ordering {
		        self.$rect.cmp(&other.$rect)
		    }
		    /// Gives an ordering based on distance from origo
		    pub fn origo_cmp(&self, other: &$Struct) -> Ordering {
		    	self.$rect.origo_cmp(&other.$rect)
		    }
		    /// Rotates the struct's rect in place (switches it's rectangles
		    /// width with its height). Only works if mutable.
		    pub fn rot_in_place(&mut self) {
		        self.$rect = self.$rect.rotate();
		    }
		    /// Checks if this struct collides with another struct. Uses Rect's
		    /// collides_with for this
		    pub fn collides_with(&self, other: $Struct) -> bool {
		        self.$rect.collides_with(other.$rect)
		    }
		    /// Gives the area of this structs's rect
		    pub fn area(&self) -> isize {
		        self.$rect.area()
		    }
		    /// Gets the center point (x+w/2, y+h/2) of this structs's rect
		    pub fn center(&self) -> Point {
		        self.$rect.center()
		    }
		    /// Gets the top left corner (x, y) of this structs's rect
		    pub fn top_left(&self) -> Point {
		        self.$rect.top_left()
		    }
		    /// Gets the bottom right corner (x+w, y+h) of this structs's rect
		    pub fn bottom_right(&self) -> Point {
		        self.$rect.bottom_right()
		    }
		    pub fn get_x(&self) -> isize {
		        self.$rect.x
		    }
		    pub fn get_y(&self) -> isize {
		        self.$rect.y
		    }
		    pub fn get_w(&self) -> isize {
		        self.$rect.w
		    }
		    pub fn get_h(&self) -> isize {
		        self.$rect.h
		    }
		    /// Sets the X position of the struct. 
		    /// Only works if the struct is mutable
		    pub fn set_x(&mut self, x: isize) {
		        self.$rect.x = x;
		    }
		    /// Sets the Y position of the struct. 
		    /// Only works if the struct is mutable
		    pub fn set_y(&mut self, y: isize) {
		        self.$rect.y = y;
		    }
		    /// Sets the center of the struct.
		    pub fn set_center(&mut self, x: isize, y: isize) {
		    	self.$rect.set_center(x, y);
		    }
		}
	}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn point_diff_works_correctly() {
        let point1 = Point { x: 5, y: 7 };
        let point2 = Point { x: -3, y: 10 };
        let diff = point1.diff(point2);
        assert_eq!(Point { x: -8, y: 3 }, diff);
    }

    #[test]
    fn rect_rotates_correctly() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let rect2 = rect1.rotate();
        assert_eq!(7, rect2.w);
        assert_eq!(5, rect2.h);
        assert_eq!(2, rect2.x);
        assert_eq!(3, rect2.y);
    }

    #[test]
    fn rect_copy_testing() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let rect2 = rect1.rotate();
        let mut rect3 = rect2;
        rect3.x = 0;
        assert_eq!(0, rect3.x);
        assert_eq!(2, rect2.x);
    }

    #[test]
    fn rect_collisions_work() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let mut rect2 = rect1.rotate();
        assert!(rect1.collides_with(rect2));
        rect2.x = 7;
        assert!(!rect1.collides_with(rect2));
        rect2.x = -2;
        assert!(rect1.collides_with(rect2));
        rect2.y = -4;
        assert!(!rect1.collides_with(rect2));
    }

    #[test]
    fn rect_area_calculation_works() {
        let rect1 = Rect {
            x: 666,
            y: -1337,
            w: 42,
            h: 42,
        };
        assert_eq!(1764, rect1.area());
    }

    #[test]
    fn rect_top_left_center_bottom_right_work() {
        let rect1 = Rect {
            x: 666,
            y: -1337,
            w: 42,
            h: 42,
        };
        assert_eq!(Point { x: 666, y: -1337 }, rect1.top_left());
        assert_eq!(Point { x: 687, y: -1316 }, rect1.center());
        assert_eq!(Point { x: 708, y: -1295 }, rect1.bottom_right());
    }
}
