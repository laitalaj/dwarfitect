//Unstable features for data structure programming and benchmark testing
#![feature(alloc, heap_api, unique, test)] 
#![allow(dead_code)] //Allowing dead code to avoid console spam on test
extern crate rand;	 //TODO: Get rid of the allow when actually building
extern crate rustc_serialize;
#[macro_use]
pub mod mapping;
pub mod genetics;
pub mod io;
pub mod collections;
mod tests;