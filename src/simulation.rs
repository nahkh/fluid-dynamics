pub mod world {
    use std::collections::{HashMap, HashSet};
    use std::mem;
    use std::fmt;

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Position {
        x: i32,
        y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Position {
            Position {x, y}
        }
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[derive(Clone)]
    pub struct Cell {
        velocity: (f64, f64), // in m/s
        pressure: f64, // in Pascal
        temperature: f64, // in Kelvin
    }

    impl Cell {
        pub fn new() -> Cell {
            Cell {
                velocity: (0.0, 0.0),
                pressure: 0.0,
                temperature: 0.0,
            }
        }
    }

    pub struct Grid {
        pub width: i32,
        pub height: i32,
        buffer_a: HashMap<Position, Cell>,
        buffer_b: HashMap<Position, Cell>,
        outside_cell: Cell,
    }

    impl Grid {
        pub fn new(width: i32, height: i32) -> Grid {
            let mut grid = Grid {
                width,
                height,
                buffer_a: HashMap::new(),
                buffer_b: HashMap::new(),
                outside_cell: Cell::new(),
            };
            for x in 0..width {
                for y in 0..height {
                    grid.buffer_a.insert(Position::new(x, y), Cell::new());
                    grid.buffer_b.insert(Position::new(x, y), Cell::new());
                };
            };
            grid
        }

        pub fn swap(&mut self) {
            mem::swap(&mut self.buffer_a, &mut self.buffer_b);
        }

        pub fn get(&self, position: &Position) -> &Cell {
            match self.buffer_a.get(position) {
                Some(cell) => cell,
                None => &self.outside_cell,
            }
        }

        fn set(&mut self, position: Position, cell: Cell) {
            self.buffer_b.insert(position, cell);
        }

        pub fn iter(&self) -> GridIterator {
            GridIterator::new(self.width, self.height)
        }
    }

    pub struct GridIterator {
        width: i32,
        height: i32,
        x: i32,
        y: i32,
    }

    impl GridIterator {
        pub fn new(width: i32, height: i32) -> GridIterator {
            GridIterator {
                width,
                height,
                x: 0,
                y: 0,
            }
        }
    }

    impl Iterator for GridIterator {
        type Item = Position;

        fn next(&mut self) -> Option<Position> {
            self.x += 1;
            if self.x >= self.width {
                self.x = 0;
                self.y += 1;
            };
            if self.y >= self.height {
                None
            } else {
                Some(Position::new(self.x, self.y))
            }
        }
    }

    pub trait Solver {
        fn solve(&self, position: &Position, grid: &Grid) -> Cell;
    }

    pub struct Simulation {
        pub grid: Grid,
        scale: f64,
        blocked_cells: HashSet<Position>,
    }

    impl Simulation {

        pub fn new(width: i32, height: i32, image: Vec<u8>) -> Simulation {
            let grid = Grid::new(width, height);
            let mut blocked_cells = HashSet::new();
            for x in 0..(grid.width) {
                for y in 0..(grid.height) {
                    let index = (4 * (x + y * width)) as usize;
                    if image[index] == 0 {
                        blocked_cells.insert(Position::new(x, y));
                    }
                };
            };
            println!("Total cells: {}", width * height);
            println!("Blocked cells: {}", blocked_cells.len());
            Simulation {
                grid,
                scale: 0.1,
                blocked_cells,
            }
        }

        pub fn evolve(&mut self, solver: impl Solver) {
            for position in self.grid.iter(){
                if !self.is_blocked(&position) {
                    self.grid.set(position, solver.solve(&position, &self.grid));
                }
            };
            self.grid.swap();
        }

        pub fn is_blocked(&self, position: &Position) -> bool {
            self.blocked_cells.contains(position)
        }
    }

    pub fn test_fn() {
        println!("Hello");
    }
}
