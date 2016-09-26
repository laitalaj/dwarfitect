//! Module for all kinds of geometry

/// Direction enums
#[derive(Copy, Clone)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down
}

/// A simple point struct
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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
  /// Returns a vector (represented by a Point) from this point to the other
  /// point
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
  /// Checks if this rect collides (overlaps) with another rectangle.
  pub fn collides_with(&self, rect: Rect) -> bool {
    self.x < rect.x + rect.w && rect.x < self.x + self.w &&
    self.y < rect.y + rect.h && rect.y < self.y + self.h
  }
  /// Returns this rect's area (w*h)
  pub fn area(&self) -> i16 {
    self.w * self.h
  }
  /// Gets the center point (x+w/2, y+h/2)
  pub fn center(&self) -> Point {
    Point { x: self.x + self.w / 2, y: self.y + self.h / 2 }
  }
  /// Gets the top left corner (x, y)
  pub fn top_left(&self) -> Point {
    Point { x: self.x, y: self.y }
  }
  /// Gets the bottom right corner (x+w, y+h)
  pub fn bottom_right(&self) -> Point {
    Point { x: self.x + self.w, y: self.y + self.h }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn point_diff_works_correctly() {
    let point1 = Point{ x: 5, y: 7};
    let point2 = Point{ x: -3, y: 10};
    let diff = point1.diff(point2);
    assert_eq!(Point{ x: -8, y: 3 }, diff);
  }

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
  fn rect_copy_testing() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let rect2 = rect1.rotate();
    let mut rect3 = rect2;
    rect3.x = 0;
    assert_eq!(0, rect3.x);
    assert_eq!(2, rect2.x);
  }

  #[test]
  fn rect_collisions_work() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
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
    let rect1 = Rect {x: 666, y: -1337, w: 42, h: 42 };
    assert_eq!(1764, rect1.area());
  }
  
  #[test]
  fn rect_top_left_center_bottom_right_work() {
  	let rect1 = Rect {x: 666, y: -1337, w: 42, h: 42 };
  	assert_eq!(Point { x: 666, y: -1337 }, rect1.top_left());
  	assert_eq!(Point { x: 687, y: -1316 }, rect1.center());
  	assert_eq!(Point { x: 708, y: -1295 }, rect1.bottom_right());
  }
}
