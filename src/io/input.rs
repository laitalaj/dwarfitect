//! This module contains functions related to reading data from the hard drive
//! and converting it into a format that can be understood by the genetic
//! algorithmics.

use collections::Vector;
use std::collections::HashMap; 
//Won't be using own implementation as this is basically UI code
use genetics::genes::{Gene, Target};
use mapping::shapes::Rect;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use rustc_serialize::json;

/// A blueprint of a single target that the algorithm will aim for. Will be
/// transformed into Target by Blueprint.compile()
#[derive(Copy, Clone, PartialEq, RustcDecodable, RustcEncodable)]
pub struct TargetBlueprint {
    from_key: usize,
    to_key: usize,
    weight: f32,
}

/// A "blueprint" for a type of room; will be transformed into Gene by 
/// Blueprint.compile()
#[derive(Copy, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct RoomBlueprint {
    key: usize,
    width: isize, //TODO: Refactor to usize
    height: isize,
    amount: usize,
}

/// Blueprint is a collection of blueprints for rooms and targets that can be
/// decoded from a JSON file and transformed into two vectors containing genes
/// and targets.
#[derive(RustcDecodable, RustcEncodable)]
pub struct Blueprint {
    // Using Vec here instead of custom vector in order to not have to rewrite
    // JSON parsing myself. I'm using custom data structures everywhere else
    // except this UI code, honest, gov'nor.
    pub rooms: Vec<RoomBlueprint>,
    pub targets: Vec<TargetBlueprint>,
}

impl Blueprint {
	/// Creates two vectors, one containing genes and one containing targets, as
	/// specified by the gene- and target blueprints inside this blueprint.
	pub fn compile(&self) -> (Vector<Gene>, Vector<Target>) {
		let mut key_to_id = HashMap::new();
		let mut genes = Vector::new(); //TODO: new_with_size?
		let mut targets = Vector::new();
		let mut current_id: usize = 0;
		for i in 0..self.rooms.len() {
			let mut ids = Vector::new();
			let room = self.rooms[i];
			// Gene size is blueprint size + 1 (for room between genes)
			let rect = Rect::new(0, 0, room.width + 1, room.height + 1);
			for _ in 0..self.rooms[i].amount {
				genes.push(Gene::new(rect, current_id as isize));
				ids.push(current_id);
				current_id += 1;
			}
			key_to_id.insert(room.key, ids);
		}
		for i in 0..self.targets.len() {
			let target = self.targets[i];
			let from = key_to_id.get(&target.from_key).unwrap().clone();
			let to = key_to_id.get(&target.to_key).unwrap().clone();
			targets.push(Target::new(from, to, target.weight));
		}
		(genes, targets)
	}
}

/// Read a blueprint from a JSON file with given filename
pub fn read(filename: String) -> Blueprint {
    let path = Path::new(&filename);
    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {:?}", filename, reason),
        Ok(file) => file
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(reason) => panic!("Couldn't read {}: {:?}", filename, reason),
        Ok(_) => {}
    }
    let blueprint: Blueprint = json::decode(&data).unwrap();
    blueprint
}

/// Checks if a file exists
pub fn exists(filename: &String) -> bool {
	let path = Path::new(filename);
	path.exists()
}

#[cfg(test)]
mod tests {

    use super::*;
    use io::output::save;
    use rustc_serialize::json;

    #[test]
    fn read_works() {
        let tbp1 = TargetBlueprint {
            from_key: 0,
            to_key: 1,
            weight: 1.2,
        };
        let tbp2 = TargetBlueprint {
            from_key: 1,
            to_key: 2,
            weight: 1.1,
        };
        let rbp0 = RoomBlueprint {
            key: 0,
            width: 3,
            height: 3,
            amount: 1,
        };
        let rbp1 = RoomBlueprint {
            key: 1,
            width: 5,
            height: 5,
            amount: 3,
        };
        let rbp2 = RoomBlueprint {
            key: 2,
            width: 6,
            height: 2,
            amount: 2,
        };
        let bp = Blueprint {
            rooms: vec![rbp0, rbp1, rbp2],
            targets: vec![tbp1, tbp2],
        };
        let encoded_bp = json::encode(&bp).unwrap();
        match save(encoded_bp, String::from("test.json")) {
        	Ok(_) => {},
        	Err(reason) => panic!("{:?}", reason)
        };
        let read_bp = read(String::from("test.json"));
        assert_eq!(read_bp.rooms.len(), bp.rooms.len());
        assert_eq!(read_bp.targets.len(), bp.targets.len());
        for i in 0..read_bp.rooms.len() {
        	assert!(read_bp.rooms[i] == rbp0 ||
        		read_bp.rooms[i] == rbp1 ||
        		read_bp.rooms[i] == rbp2
        	)
        }
        for i in 0..read_bp.targets.len() {
        	assert!(read_bp.targets[i] == tbp1 ||
        		read_bp.targets[i] == tbp2
        	)
        }
    }
}
