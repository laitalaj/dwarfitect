use std::vec::Vec;
use std::ops::Range;
use rand::Rng;
use mapping::shapes::Rect;

/// Genes are rooms represented only by their bounding rectangle. A chromosome
/// is made of these.
pub struct Gene {
  rect: Rect,
  gene_id: i16 //Possibly pointer or something here?
}

/// Chromosomes are possible solutions. They handle the genetic operations.
struct Chromosome {
  genes: Vec<Gene>,
  fitness: f32
}

impl Gene {

  /// Rotates the gene (switches it's rectangles width with its height)
  /// Returns a new, rotated gene
  fn rotate(&self) -> Gene {
    let new_rect = self.rect.rotate();
    Gene{ rect: new_rect, gene_id: self.gene_id }
  }
  /// Rotates the gene in place (switches it's rectangles width with its
  /// height). Only works for mutable genes.
  fn rot_in_place(&mut self) {
    self.rect = self.rect.rotate();
  }
  fn get_x(&self) -> i16 {
    self.rect.x
  }
  fn get_y(&self) -> i16 {
    self.rect.y
  }
  fn get_w(&self) -> i16 {
    self.rect.w
  }
  fn get_h(&self) -> i16 {
    self.rect.h
  }
  /// Creates a new gene that's a copy of this one but with a differing
  /// position.
  fn set_pos(&self, new_x: i16, new_y: i16) -> Gene {
    let new_rect = Rect { x: new_x, y: new_y,
      w: self.rect.w, h: self.rect.h };
    Gene { rect: new_rect, gene_id: self.gene_id }
  }
  /// Sets the X position of the gene. Only works if the gene is mutable
  fn set_x(&mut self, x: i16) {
    self.rect.x = x;
  }
  /// Sets the Y position of the gene. Only works if the gene is mutable
  fn set_y(&mut self, y: i16) {
    self.rect.y = y;
  }

}

impl Chromosome {
	/// The mating function: Two children are created by swapping half of this 
	/// chromosomes genes with the partner chromosome's genes
	/// # Panics
	/// Panics if trying to mate two genes of different lengths!
	/// (Doing that is contrary to both natural evolution AND the word of God!)
	fn mate<R: Rng>(&self, partner: &Chromosome, rng: &mut R) 
	-> [Option<Chromosome>; 2] {
		if self.genes.len() != partner.genes.len() {
			panic!("Tried to mate chromosomes with different lengths! 
			Shame on you!");
		}
		let swap_len = self.genes.len() / 2;
		let swap_pos = Range::new(0, self.genes.len() - swap_len - 1)
		.ind_sample(rng);
		// Fill some stacks that contain the gene order of chromosomes for
		// efficient child creation
		let my_stack = Vec::new();
		let partners_stack = Vec::new();
		for i in (1..self.genes.len()).rev() { //Goes from len-1 to 0
			my_stack.push(self.genes.get(i));
			partners_stack.push(partner.genes.get(i));
		}
		let output: [Option<Chromosome>; 2] = [None, None];
		for _ in 0..1 {
			//Tästä jatka (-:
		}
	}
}



#[cfg(test)]
mod tests {

  use super::*;
  use mapping::shapes::Rect;

  #[test]
  fn gene_rotates_correctly() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let gene1 = Gene { rect: rect1, gene_id: 0 };
    let gene2 = gene1.rotate();
    assert_eq!(7, gene2.get_w());
    assert_eq!(5, gene2.get_h());
    let rect2 = Rect { x: -2, y: -3, w: 1, h:9 };
    let mut gene3 = Gene { rect: rect2, gene_id: 0 };
    gene3.rot_in_place();
    assert_eq!(9, gene3.get_w());
    assert_eq!(1, gene3.get_h());
  }

  #[test]
  fn position_changing_ok() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let mut gene1 = Gene { rect: rect1, gene_id: 0 };
    gene1.set_x(9);
    gene1.set_y(10);
    assert_eq!(9, gene1.get_x());
    assert_eq!(10, gene1.get_y());
  }
}
