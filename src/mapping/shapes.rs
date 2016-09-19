//! Module for all kinds of geometry

/// A simple rectangle struct
/// (x, y) is the top left corner, w is width, h is height
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
	pub x: i16,
	pub y: i16,
	pub w: i16,
	pub h: i16,
}

impl Rect {
	///"Rotates" the rectangle: returns a new rectangle with switched width and
	/// height.
	pub fn rotate(&self) -> Rect {
		Rect { x: self.x, y: self.y, w: self.h, h: self.w }
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