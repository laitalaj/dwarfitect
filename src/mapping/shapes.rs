//! Module for all kinds of geometry

#[derive(Copy, Clone)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down
}

pub struct Point {
  pub x: i16,
  pub y: i16
}

/// A simple rectangle struct
/// (x, y) is the top left corner, w is width, h is height
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Rect {
  pub x: i16,
  pub y: i16,
  pub w: i16,
  pub h: i16,
}

impl Point {
	/// Returns a vector from this point to the other point
	pub fn diff(&self, point: Point) -> Point {
		Point{ x: point.x - self.x, y: point.y - self.y }
	}
}

impl Rect {
  ///"Rotates" the rectangle: returns a new rectangle with switched width and
  /// height.
  pub fn rotate(&self) -> Rect {
    Rect { x: self.x, y: self.y, w: self.h, h: self.w }
  }
  pub fn collides_with(&self, rect: Rect) -> bool {
    self.x < rect.x + rect.w && rect.x < self.x + self.w &&
    self.y < rect.y + rect.h && rect.y < self.y + self.h
  }
  pub fn area(&self) -> i16 {
  	self.w * self.h
  }
  pub fn center(&self) -> Point {
    Point { x: self.x + self.w / 2, y: self.y + self.h / 2 } 
  }
  pub fn top_left(&self) -> Point {
  	Point { x: self.x, y: self.y }
  }
  pub fn bottom_right(&self) -> Point {
  	Point { x: self.x + self.w, y: self.y + self.h }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn rect_rotates_correctly() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let rect2 = rect1.rotate();
    assert_eq!(7, rect2.w);
    assert_eq!(5, rect2.h);
    assert_eq!(2, rect2.x);
    assert_eq!(3, rect2.y);
  }

  #[test]
  fn copy_testing() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let rect2 = rect1.rotate();
    let mut rect3 = rect2;
    rect3.x = 0;
    assert_eq!(0, rect3.x);
    assert_eq!(2, rect2.x);
  }
}
