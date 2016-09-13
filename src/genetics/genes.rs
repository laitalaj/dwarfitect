extern crate mapping;

use std::vec::Vec;
use mapping::shapes::Rect;

struct Gene {
	rect: Rect,
	room_id: i16
}

struct Chromosome {
	genes: Vec<Gene>,
	
}