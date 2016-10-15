//! Module for genetic structs

use std::vec::Vec;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use mapping::shapes::{Point, Rect, Direction};
use mapping::shapes::Direction::{Left, Right, Up, Down};
use mapping::rooms::{Room, Layout};
use self::Mutation::{RotationMutation, PositionMutation};

// TODO: Get rid of this hardcoding
/// The chance that, during mating, two genes will be switched
pub const CROSSOVER_CHANCE: f32 = 0.5;
/// The chance that, during mutation, a gene will mutate
pub const MUTATION_CHANCE: f32 = 0.04;
/// A list of all possible mutation types
pub const MUTATIONS: [Mutation; 2] = [RotationMutation, PositionMutation];

/// Possible mutation types; just plain old enums
pub enum Mutation {
    RotationMutation,
    PositionMutation,
}

/// Genes are rooms represented only by their bounding rectangle. A chromosome
/// is made of these.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Gene {
    rect: Rect,
    gene_id: isize,
}

/// Chromosomes are possible solutions. They handle the genetic operations.
#[derive(PartialEq, Clone)]
pub struct Chromosome {
    pub genes: Vec<Gene>, //TODO: Instead of pub, getters / setters?
    total_area: isize,
    pub fitness: f32,
    bounding_box: Rect,
    bounding_box_fresh: bool,
}

// Implement methods that manipulate the rectangle inside the gene
impl_rect_methods!(Gene, rect);

impl PartialOrd for Gene {
    /// Gives an ordering the genes by gene ID
    /// Uses Ord-trait's cmp() to do the comparison
    fn partial_cmp(&self, other: &Gene) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Gene {
    /// Gives an ordering for the genes by gene ID
    fn cmp(&self, other: &Gene) -> Ordering {
        self.gene_id.cmp(&other.gene_id)
    }
}

impl Gene {
	/// Constructor for gene
	pub fn new(rect: Rect, gene_id: isize) -> Gene{
		Gene{ rect: rect, gene_id: gene_id }
	}
    /// Mutates the gene: Selects a mutation type randomly and modifies the gene
    /// accordingly.
    /// # Panics
    /// Panics if, for some reason, there's no available choices in MUTATIONS
    fn mutate<R: Rng>(&mut self, allowed_area: Rect, rng: &mut R) {
        let allowed_end = allowed_area.bottom_right();
        let allowed_x = Range::new(allowed_area.x, allowed_end.x);
        // TODO: make sure the whole gene stays in the area (not just the topleft corner)
        let allowed_y = Range::new(allowed_area.y, allowed_end.y);
        match rng.choose(&MUTATIONS) {
            Some(m) => {
                match *m {
                    RotationMutation => self.rot_in_place(),
                    PositionMutation => {
                        // TODO: Transition amount by chromosome fitness?
                        self.set_x(allowed_x.ind_sample(rng));
                        self.set_y(allowed_y.ind_sample(rng));
                    }
                    // _ => panic!("Got a mutation type that's not yet implemented!")
                    // Having a mutation type that's not implemented and still being able to compile
                    // seems to be impossible. Rust <3
                }
            }
            None => panic!("For some reason the mutation list was empty!"),
        };
    }
    /// Rotates the gene (switches it's rectangles width with its height)
    /// Returns a new, rotated gene
    fn rotate(&self) -> Gene {
        let new_rect = self.rect.rotate();
        Gene {
            rect: new_rect,
            gene_id: self.gene_id,
        }
    }
    /// Creates a new gene that's a copy of this one but with a differing
    /// position.
    fn set_pos(&self, new_x: isize, new_y: isize) -> Gene {
        let new_rect = Rect {
            x: new_x,
            y: new_y,
            w: self.rect.w,
            h: self.rect.h,
        };
        Gene {
            rect: new_rect,
            gene_id: self.gene_id,
        }
    }
    /// Converts the gene into a room (shrinks it down a bit)
    pub fn as_room(&self) -> Room {
    	let mut new_rect = self.rect;
    	new_rect.w -= 1;
    	new_rect.h -= 1;
    	Room::new(new_rect)
    }
}

impl PartialOrd for Chromosome {
	/// Partial comparison to other chromosome based on chromosome's fitness
    fn partial_cmp(&self, other: &Chromosome) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl Debug for Chromosome {
	/// Debug output formatting for chromosome
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Chromosome with fitness {}, bounding box area {}, \
		total area {}", self.fitness, self.bounding_box.area(), self.total_area)
	}
}

impl Chromosome {
    /// A constructor for the chromosome. Creates the chromosome and relaxes
    /// it.
    pub fn new(genes: Vec<Gene>) -> Chromosome {
        let mut total_area = 0;
        for i in 0..genes.len() {
            total_area += genes[i].area();
        }
        let mut new_chromosome = Chromosome {
            genes: genes,
            total_area: total_area,
            fitness: 0.0,
            bounding_box: Rect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            bounding_box_fresh: false,
        };
        new_chromosome.relax();
        new_chromosome
    }
    /// Generates a more randomized (and perhaps more valid) initial chromosome
    /// from given genes.
    pub fn generate_initial<R: Rng>(genes: Vec<Gene>, rng: &mut R) -> Chromosome {
        let mut shuffled_genes = genes.to_vec();
        shuffled_genes[0].set_center(0, 0);
        rng.shuffle(&mut shuffled_genes[1..]);
//        shuffled_genes[0].set_x(0);
//        shuffled_genes[0].set_y(0);
        let mut places_to_go: Vec<(isize, isize, Direction)> = Vec::new();
        let top_left_0 = shuffled_genes[0].top_left();
        let bottom_right_0 = shuffled_genes[0].bottom_right();
        places_to_go.push((top_left_0.x, top_left_0.y, Left));
        places_to_go.push((top_left_0.x, top_left_0.y, Up));
        places_to_go.push((bottom_right_0.x, top_left_0.y, Right));
        places_to_go.push((top_left_0.x, bottom_right_0.y, Down));
        for i in 1..shuffled_genes.len() {
            let place = places_to_go.remove(0); //TODO: Create an efficient queue
            let mut x = place.0;
            let mut y = place.1;
            if rng.next_f32() > 0.5 {
                // 50% chance to rotate
                shuffled_genes[i].rot_in_place();
            }
            let x_variance = shuffled_genes[i].get_w() / 2;
            let y_variance = shuffled_genes[i].get_h() / 2;
            let x_var_range = Range::new(-x_variance, x_variance);
            let y_var_range = Range::new(-y_variance, y_variance);
            match place.2 { //TODO: Split this to different functions
                Left => {
                    x -= shuffled_genes[i].get_w();
                    y += y_var_range.ind_sample(rng);
                    places_to_go.push((x, y, Left));
                    places_to_go.push((x, y, Up));
                    places_to_go.push((x, y + shuffled_genes[i].get_h(), Down));
                },
                Up => {
                	x += x_var_range.ind_sample(rng);
                    y -= shuffled_genes[i].get_h();
                    places_to_go.push((x, y, Up));
                    places_to_go.push((x, y, Left));
                    places_to_go.push((x + shuffled_genes[i].get_w(), y, Right));
                },
                Right => {
                	//x += shuffled_genes[i].get_w();
                	y += y_var_range.ind_sample(rng);
                	places_to_go.push((x + shuffled_genes[i].get_w(), y, Right));
                	places_to_go.push((x, y + shuffled_genes[i].get_h(), Down));
                	places_to_go.push((x, y, Up));
                },
                Down => {
                	x += x_var_range.ind_sample(rng);
                	//y += shuffled_genes[i].get_h();
                	places_to_go.push((x, y + shuffled_genes[i].get_h(), Down));
                	places_to_go.push((x + shuffled_genes[i].get_w(), y, Right));
                	places_to_go.push((x, y, Left));
                }
            }
            shuffled_genes[i].set_x(x);
            shuffled_genes[i].set_y(y);
        }
        shuffled_genes.sort();
//        shuffled_genes.insert(0, genes[0].set_pos(0, 0));
        Chromosome::new(shuffled_genes)
    }
    /// Relaxes the chromosome: moves every gene so that no two genes collide.
    /// Recalculates fitness when done.
    fn relax(&mut self) {
    	//Leave gene 0 as first gene
        self.genes[1..].sort_by(|a, b| a.origo_cmp(b));
        for i in 1..self.genes.len() {
        	let mut j = 0;
        	let center_i = self.genes[i].center();
        	let top_left_i = self.genes[i].top_left();
        	let bottom_right_i = self.genes[i].bottom_right();
        	let x_dir = sign(center_i.x);
        	let y_dir = sign(center_i.y);
            while j < i {
                if self.genes[i].collides_with(self.genes[j]) {
                    let top_left_j = self.genes[j].top_left();
                    let bottom_right_j = self.genes[j].bottom_right();
                    let mut diff = Point::new(0, 0);
                    if x_dir < 0 {
                    	diff.x = top_left_j.x - bottom_right_i.x;
                    } else {
                    	diff.x = bottom_right_j.x - top_left_i.x;
                    }
                    if y_dir < 0 {
                    	diff.y = top_left_j.y - bottom_right_i.y;
                    } else {
                    	diff.y = bottom_right_j.y - top_left_i.y;
                    }
                    if diff.x.abs() < diff.y.abs() {
                        let new_x = self.genes[i].get_x() + diff.x;
                        self.genes[i].set_x(new_x);
                    } else {
                        let new_y = self.genes[i].get_y() + diff.y;
                        self.genes[i].set_y(new_y);
                    }
                    j = 0;
                } else {
	                j += 1;
                }
            }
        }
        self.genes.sort();
        self.bounding_box_fresh = false;
        self.calculate_fitness();
    }
    /// Calculates the smallest bounding box for this chromosome's genes
    fn calculate_bounding_box(&mut self) {
        // TODO:Store this in struct, make
        let mut min_x = isize::max_value(); //this update as part of other fns
        let mut min_y = isize::max_value();
        let mut max_x = isize::min_value();
        let mut max_y = isize::min_value();
        for i in 0..self.genes.len() {
            let top_left = self.genes[i].top_left();
            let bottom_right = self.genes[i].bottom_right();
            if top_left.x < min_x {
                min_x = top_left.x;
            }
            if top_left.y < min_y {
                min_y = top_left.y;
            }
            if bottom_right.x > max_x {
                max_x = bottom_right.x;
            }
            if bottom_right.y > max_y {
                max_y = bottom_right.y;
            }
        }
        self.bounding_box = Rect {
            x: min_x,
            y: min_y,
            w: max_x - min_x,
            h: max_y - min_y,
        };
        self.bounding_box_fresh = true;
    }
    /// Calculates this chromosome's fitness. Currently does this by comparing
    /// area  used up by the genes to the area of the minimum bounding box
    /// (so compact, rectangular designs flourish at the moment).
    /// Calls calculate_bounding_box in the beginning if the bounding box is not
    /// fresh.
    pub fn calculate_fitness(&mut self) {
        // TODO: Actual fitness calculation
        if !self.bounding_box_fresh {
            self.calculate_bounding_box();
        }
        let raw_fitness = self.total_area as f32 / self.bounding_box.area() as f32;
        self.fitness = 1.0 / raw_fitness.log10().abs(); //TODO: What if raw_fitness = 1?
    }
    /// The mating function: Two children are created by swapping this
    /// chromosomes genes with the partner chromosome's genes (aka. crossover).
    /// The probability of a swap happening per gene is equal to
    /// CROSSOVER_CHANCE
    /// # Panics
    /// Panics if trying to mate two genes of different lengths!
    /// (Doing that is contrary to both natural evolution AND the word of God!)
    pub fn mate<R: Rng>(&self, partner: &Chromosome, rng: &mut R) -> (Chromosome, Chromosome) {
        if self.genes.len() != partner.genes.len() {
            panic!("Tried to mate chromosomes with different lengths!
      Shame on you!");
        }
        let mut my_childs_genes: Vec<Gene> = Vec::new();
        let mut partners_childs_genes: Vec<Gene> = Vec::new();
        for i in 0..self.genes.len() {
            if rng.next_f32() < CROSSOVER_CHANCE {
                partners_childs_genes.push(self.genes[i]);
                my_childs_genes.push(partner.genes[i]);
            } else {
                my_childs_genes.push(self.genes[i]);
                partners_childs_genes.push(partner.genes[i]);
            }
        }
        let my_child = Chromosome::new(my_childs_genes);
        let partners_child = Chromosome::new(partners_childs_genes);
        // 		my_child.relax();
        // 		partners_child.relax();
        (my_child, partners_child)
    }
    /// Mutates the chromosome: Calls Gene::mutate for each gene with
    /// probability equal to MUTATION_CHANCE. Relaxes the gene at the end.
    pub fn mutate<R: Rng>(&mut self, rng: &mut R) {
        if !self.bounding_box_fresh {
            self.calculate_bounding_box();
        }
        for i in 1..self.genes.len() {
            if rng.next_f32() < MUTATION_CHANCE {
                self.genes[i].mutate(self.bounding_box, rng);
            }
        }
        self.relax();
    }
    /// Converts the chromosome into a layout; converts all the genes into rooms
    /// and returns a new layout
    pub fn as_layout(&self) -> Layout {
    	let mut rooms = Vec::new();
    	for i in 0..self.genes.len() {
    		rooms.push(self.genes[i].as_room());
    	}
    	Layout::new(rooms)
    }
}

fn sign(n: isize) -> isize {
	if n < 0 {
		-1
	} else {
		1
	}
}



#[cfg(test)]
mod tests {

    use super::*;
    use mapping::shapes::Rect;
    use rand::Rng;

    /// Test random number generatror - gives back numbers that were given to it
    /// one by one.
    struct TestRng {
        numbers: Vec<f32>,
        indexes: Vec<usize>,
    }

    impl Rng for TestRng {
        /// Should return an unsigned 32 bit integer. Not supported as of yet by
        /// this RNG.
        /// If I understood correctly, this function is, by default, used by other
        /// random functions that have not been overwritten - that means they will
        /// panic too.
        /// # Panics
        /// Always
        fn next_u32(&mut self) -> u32 {
            panic!("Not supported!");
        }
        /// Returns the next f32 in the numbers-vector. Uses the numbers-vector as
        /// a stack, so always retuns and removes from the vector the last number.
        /// # Panics
        /// Panics when runs out of numbers
        fn next_f32(&mut self) -> f32 {
            match self.numbers.pop() {
                Some(n) => n,
                None => panic!("Ran out of numbers!"),
            }
        }
        /// Returns a choice from the given sized by taking a index from the
        /// indexes-vector, modulo len, and returning corresponding value.
        /// Uses the indexes-vector as a stack, so always retuns and removes from
        /// the vector the last number.
        /// # Panics
        /// Panics when runs out of numbers
        fn choose<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T>
            where Self: Sized
        {
            if values.is_empty() {
                None
            } else {
                match self.indexes.pop() {
                    Some(n) => Some(&values[n % values.len()]),
                    None => panic!("Ran out of numbers!"),
                }
            }
        }
    }

    #[test]
    fn gene_rotates_correctly() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let gene1 = Gene {
            rect: rect1,
            gene_id: 0,
        };
        let gene2 = gene1.rotate();
        assert_eq!(7, gene2.get_w());
        assert_eq!(5, gene2.get_h());
        let rect2 = Rect {
            x: -2,
            y: -3,
            w: 1,
            h: 9,
        };
        let mut gene3 = Gene {
            rect: rect2,
            gene_id: 0,
        };
        gene3.rot_in_place();
        assert_eq!(9, gene3.get_w());
        assert_eq!(1, gene3.get_h());
    }

    #[test]
    fn position_changing_ok() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let mut gene1 = Gene {
            rect: rect1,
            gene_id: 0,
        };
        gene1.set_x(9);
        gene1.set_y(10);
        assert_eq!(9, gene1.get_x());
        assert_eq!(10, gene1.get_y());
    }

    #[test]
    fn mating_works_correctly() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let gene1 = Gene {
            rect: rect1,
            gene_id: 0,
        };
        let rect2 = Rect {
            x: -2,
            y: -2,
            w: 3,
            h: 3,
        };
        let gene2 = Gene {
            rect: rect2,
            gene_id: 1,
        };
        let mut gene3 = gene1;
        gene3.set_x(4);
        let mut gene4 = gene2;
        gene4.set_y(5);
        let genes1 = vec![gene1, gene2];
        let genes2 = vec![gene3, gene4];
        let chrom1 = Chromosome::new(genes1);
        let chrom2 = Chromosome::new(genes2);
        let crossover_delta = CROSSOVER_CHANCE * 0.1;
        let random_numbers = vec![CROSSOVER_CHANCE - crossover_delta,
                                  CROSSOVER_CHANCE + crossover_delta];
        let mut rng = TestRng {
            numbers: random_numbers,
            indexes: vec![],
        };
        let (child1, child2) = chrom1.mate(&chrom2, &mut rng);
        assert_eq!(gene1, child1.genes[0]);
        assert_eq!(gene4, child1.genes[1]);
        assert_eq!(gene3, child2.genes[0]);
        assert_eq!(gene2, child2.genes[1]);
    }

    #[test]
    fn no_intersections_after_relaxing() {
    	let mut gene_vec = Vec::new();
    	gene_vec.push(Gene::new(Rect { x: -2, y: -2, w: 5, h: 5 }, 0));
        for i in 1..100 {
        	let x = (i*13)%7 - 4;
        	let y = (i*17)%7 - 4;
        	let w = (i*5)%11 + 3;
        	let h = (i*7)%11 + 3;
        	gene_vec.push(Gene::new(Rect { x: x, y: y, w: w, h: h }, i));
        }
        let mut genes = Chromosome::new(gene_vec);
        genes.relax();
        for i in 0..genes.genes.len() {
            for j in i + 1..genes.genes.len() {
                assert!(!genes.genes[i].collides_with(genes.genes[j]),
                        "{:?} collides with {:?}!",
                        genes.genes[i],
                        genes.genes[j]);
            }
        }
    }

    #[test]
    fn relax_keeps_gene_order() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let gene1 = Gene {
            rect: rect1,
            gene_id: 0,
        };
        let rect2 = Rect {
            x: 1,
            y: 0,
            w: 3,
            h: 3,
        };
        let gene2 = Gene {
            rect: rect2,
            gene_id: 1,
        };
        let rect3 = Rect {
            x: -2,
            y: 5,
            w: 5,
            h: 10,
        };
        let gene3 = Gene {
            rect: rect3,
            gene_id: 2,
        };
        let rect4 = Rect {
            x: 0,
            y: -7,
            w: 12,
            h: 8,
        };
        let gene4 = Gene {
            rect: rect4,
            gene_id: 3,
        };
        let mut genes = Chromosome::new(vec![gene1, gene2, gene3, gene4]);
        genes.relax();
        for i in 0..genes.genes.len() {
            assert_eq!(i as isize, genes.genes[i].gene_id);
        }
    }

    #[test]
    fn bounding_box_calculation_ok() {
        let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let gene1 = Gene {
            rect: rect1,
            gene_id: 0,
        };
        let rect2 = Rect {
            x: 1,
            y: 0,
            w: 3,
            h: 3,
        };
        let gene2 = Gene {
            rect: rect2,
            gene_id: 1,
        };
        let rect3 = Rect {
            x: -2,
            y: 5,
            w: 5,
            h: 10,
        };
        let gene3 = Gene {
            rect: rect3,
            gene_id: 2,
        };
        let rect4 = Rect {
            x: 0,
            y: -7,
            w: 12,
            h: 8,
        };
        let gene4 = Gene {
            rect: rect4,
            gene_id: 3,
        };
        let mut genes = Chromosome {
		    genes: vec![gene1, gene2, gene3, gene4],
		    total_area: gene1.area() + gene2.area() + gene3.area() + gene4.area(),
		    fitness: 0.0,
		    bounding_box: Rect::new(0, 0, 0, 0),
		    bounding_box_fresh: false,
		};
        genes.calculate_bounding_box();
        assert_eq!(Rect {
                       x: -2,
                       y: -7,
                       w: 14,
                       h: 22,
                   },
                   genes.bounding_box);
    }

}
