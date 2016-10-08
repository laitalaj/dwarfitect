#![feature(alloc, heap_api, unique)] //Unstable features for data structure programming
#![allow(dead_code)] //Allowing dead code to avoid console spam on test
extern crate rand;	 //TODO: Get rid of the allow when actually building
pub mod genetics;
pub mod mapping;
pub mod io;
pub mod collections;