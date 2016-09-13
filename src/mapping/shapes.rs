//! Module for all kinds of geometry

/// A simple rectangle struct
/// (x, y) is the top left corner, w is width, h is height
pub struct Rect {
	x: i16,
	y: i16,
	w: i16,
	h: i16,
}

impl Rect {
	///"Rotates" the rectangle: switches width with height
	fn rotate(&self) -> Rect {
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
	}
}