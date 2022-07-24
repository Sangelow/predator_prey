use std::fs::File;

use cairo::{ ImageSurface, Format, Context };
use rand::Rng;

#[derive(Clone, PartialEq)]
enum CellType {
    NOTHING,
    PREY { energy: u8 },
    PRED { energy: u8 },
}

struct Grid {
    width  : i64,
    height : i64,
    cells  : Vec<CellType>,
    played : Vec<bool>,
}


impl Grid {
    pub fn new ( w: i64, h: i64 ) -> Self {
        // Initialize a random number generator
        let mut rng = rand::thread_rng();
        let mut random_number: f64;
        // Create a grid
        let mut cells : Vec<CellType> = Vec::new();
        for _j in 0..h {
            for _i in 0..w {
                random_number = rng.gen_range(0.0..1.0);
                if random_number < 0.8 {
                    cells.push(CellType::NOTHING);
                } else if random_number < 0.95 {
                    cells.push(CellType::PREY{ energy:  1 });
                } else {
                    cells.push(CellType::PRED{ energy: 10 });
                }
            }
        }
        // Create player vec
        let played = vec![false; cells.len()];
        // Return 
        return Self { width: w, height: h, cells: cells, played: played};
    }

    fn index ( &self, i: i64, j:i64 ) -> usize {
        (j + (self.height-1) * i).try_into().unwrap()
    }

    fn wrapped_index ( &self, i:i64, j:i64) -> usize {
        // Wrap i and j
        let iw = i.rem_euclid(self.width);
        let jw = j.rem_euclid(self.height);
        // Return wrapped index
        self.index(iw, jw)
    }

    fn find_nothing_neigs (&self, neigs: &mut Vec<usize>, i: &i64, j: &i64) {
        // Declare an index for the neigs
        let mut idx_neig : usize;
        // Reset the neigs
        neigs.clear();
        // Iterate through the neigs (and the current cell)
        for i_off in -1..2 {
            for j_off in -1..2 {
                // Continue when reaching the current cell
                if i_off==0 && j_off==0 {
                    continue;
                }
                // Compute neighbor index
                idx_neig = self.wrapped_index(i+i_off, j+j_off);
                // Check if the cell is nothing
                if self.cells[self.wrapped_index(i+i_off, j+j_off)] == CellType::NOTHING {
                    neigs.push(idx_neig);
                }
            }
        }
    }

    fn find_prey_neigs (&self, neigs: &mut Vec<usize>, i: &i64, j: &i64) {
        // Declare an index for the neigs
        let mut idx_neig : usize;
        // Reset the neigs
        neigs.clear();
        // Iterate through the neigs (and the current cell)
        for i_off in -1..2 {
            for j_off in -1..2 {
                // Continue when reaching the current cell
                if i_off==0 && j_off==0 {
                    continue;
                }
                // Compute neighbor index
                idx_neig = self.wrapped_index(i+i_off, j+j_off);
                // Check if the cell is nothing
                match self.cells[self.wrapped_index(i+i_off, j+j_off)] {
                    CellType::PREY { energy: _ } => { neigs.push(idx_neig); },
                    _ => {},
                }
            }
        }
    }

    pub fn update( &mut self ) {
        // Define temporary variables
        let mut idx_cell : usize;
        let mut idx_neig : usize;
        let mut idx_prey : usize;
        let mut nothing_neigs : Vec<usize> = Vec::new();
        let mut prey_neigs    : Vec<usize> = Vec::new();
        // Reset played
        self.played = vec![false; self.cells.len()];
        // Update the cells
        for j in 0..self.height {
            for i in 0..self.width {
                // Set the index of current cell
                idx_cell = self.index(i,j);
                // Continue if the cell has already benn player
                if self.played[idx_cell] {
                    continue;
                }
                // Update the cell
                match self.cells[ idx_cell ] {
                    CellType::PRED { energy: e } => {
                        // Find nothing neifs
                        self.find_nothing_neigs(&mut nothing_neigs, &i, &j);
                        // Find prey neigs
                        self.find_prey_neigs(&mut prey_neigs, &i, &j);
                        // Check if the pred should die (e-1<=0 and can not eat)
                        if e-1<=0 && prey_neigs.len()==0 {
                            self.cells[idx_cell] = CellType::NOTHING;
                        } else {
                            if nothing_neigs.len()==0 && prey_neigs.len()==0 {
                                // Can not move and can not eat
                                self.cells[idx_cell] = CellType::PRED { energy: e-1 };
                                self.played[idx_cell] = true;
                            } else if nothing_neigs.len()!=0 && prey_neigs.len()==0 { 
                                // Can move and can not eat
                                idx_neig = nothing_neigs[ rand::thread_rng().gen_range(0..nothing_neigs.len()) ];
                                self.cells[idx_neig] = CellType::PRED { energy: e-1 };
                                self.played[idx_neig] = true;
                                self.cells[idx_cell] = CellType::NOTHING;

                            } else if nothing_neigs.len()==0 && prey_neigs.len()!=0 {
                                // Can not move and can eat
                                // Set as played
                                self.played[idx_cell] = true;
                                // Select a random prey
                                idx_prey = prey_neigs[ rand::thread_rng().gen_range(0..prey_neigs.len()) ];
                                // Get its energy
                                if let CellType::PREY {energy: e_prey} = self.cells[idx_prey] {
                                    // Transform it into a pred
                                    self.cells[idx_prey] = CellType::PRED { energy: 10 };
                                    self.played[idx_prey] = true;
                                    // Update energy
                                    self.cells[idx_cell] = CellType::PRED { energy: e-1+e_prey };
                                } else {
                                    panic!("Error in prey energy !");
                                }
                            } else if nothing_neigs.len()!=0 && prey_neigs.len()!=0 {
                                // Can move and can eat
                                // Select a random prey neig
                                idx_prey = prey_neigs[ rand::thread_rng().gen_range(0..prey_neigs.len()) ];
                                // Select a random nothing neig
                                idx_neig = prey_neigs[ rand::thread_rng().gen_range(0..prey_neigs.len()) ];
                                // Get its energy
                                if let CellType::PREY {energy: e_prey} = self.cells[idx_prey] {
                                    // Transform it into a pred
                                    self.cells[idx_prey] = CellType::PRED { energy: 10 };
                                    self.played[idx_prey] = true;
                                    // Move the pred
                                    self.cells[idx_neig] = CellType::PRED { energy: e-1+e_prey };
                                    self.played[idx_neig] = true;
                                    self.cells[idx_cell] = CellType::NOTHING;

                                } else { 
                                    panic!("Error in prey energy !");
                                }
                            }
                        }
                    },
                    CellType::PREY { energy: e } => {
                        //// Find neighbor cell that contains nothing
                        // Iterate throug neighbors
                        self.find_nothing_neigs(&mut nothing_neigs, &i, &j);
                        // If there is no neig cells containing NOTHING
                        if nothing_neigs.len() == 0 { // Do not move and reproduce
                            self.cells[idx_cell] = CellType::PREY { energy: if e+1>=10 {9} else {e+1} };
                            self.played[idx_cell] = true;
                        } else { // Move and reproduce (if needed)
                            // Choose a random neig cell
                            idx_neig = nothing_neigs[ rand::thread_rng().gen_range(0..nothing_neigs.len()) ];
                            if e+1 < 10 { // Move
                                // Move the prey to a nearby cell and increase energy
                                self.cells[idx_neig] = CellType::PREY { energy : e+1 };
                                self.played[idx_neig] = true;
                                // Remove the current cell from its previous cell
                                self.cells[idx_cell] = CellType::NOTHING;
                            } else {
                                // Move the prey to a nearby cell
                                self.cells[idx_neig] = CellType::PREY { energy : 1 };
                                self.played[idx_neig] = true;
                                // Create a child prey at the previous place
                                self.cells[idx_cell] = CellType::PREY { energy : 1 };
                                self.played[idx_cell] = true;
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn display( &self, t:u64 ) {
        // Create the sufarce to draw
        let surface = ImageSurface::create(
            Format::Rgb24,
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap())
            .expect("Could not create a surface !");
        let context = Context::new(&surface)
            .expect("Could not create context !");
        
        // Fill with black
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.paint().expect("Could not fill the background !");

        // Draw the cells
        for j in 0..self.height {
            for i in 0..self.width {
                // Set the color
                match self.cells[ self.index(i,j) ] {
                    CellType::NOTHING            => context.set_source_rgb(0.0, 0.0, 0.0),
                    CellType::PREY { energy: _ } => context.set_source_rgb(0.0, 1.0, 0.0),
                    CellType::PRED { energy: _ } => context.set_source_rgb(1.0, 0.0, 0.0),
                }
                // Create and fill the rectangle
                let x:f64 = i as f64;
                let y:f64 = j as f64;
                context.rectangle(x,y, 1., 1.);
                context.fill().expect("Could not fill the particles !");
            }
        }

        // Export into a file
        let mut file = File::create( format!("img/{:08}.png", t) )
            .expect("Could not create the png file !");
        surface.write_to_png(&mut file)
            .expect("Could not write the png file !");
    }
}

fn main() {
    // Parameters
    let width  : i64 = 500;
    let height : i64 = 500;
    let t_f    : u64   = 1001;
    // Create a grid
    let mut grid = Grid::new(width, height);
    // Display the initial state
    grid.display(0);
    // Iterate through time step
    for t in 1..t_f {
        // Apply the rule
        grid.update();
        // Display the grid
        grid.display(t);
    }
}
