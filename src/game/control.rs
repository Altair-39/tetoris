use crate::game::shape::{Shape, ShapeGenerator};
use crossterm::style::Color;
use std::time::{Duration, Instant};

const ROWS: usize = 20;
const COLS: usize = 11;

pub struct Game {
    pub table: [[u8; COLS]; ROWS],
    pub current: Shape,
    pub next_shape: Shape,
    pub score: u32,
    pub running: bool,
    pub paused: bool,
    pub last_tick: Instant,
    pub tick_rate: Duration,
    pub shape_generator: ShapeGenerator, // Add ShapeGenerator here
}

impl Game {
    pub fn new() -> Self {
        // Create the list of shapes (same as before)
        let shapes = vec![
            Shape::new(&[&[0, 1, 1], &[1, 1, 0], &[0, 0, 0]], Color::Green),
            Shape::new(&[&[1, 1, 0], &[0, 1, 1], &[0, 0, 0]], Color::Red),
            Shape::new(&[&[0, 1, 0], &[1, 1, 1], &[0, 0, 0]], Color::Yellow),
            Shape::new(&[&[0, 0, 1], &[1, 1, 1], &[0, 0, 0]], Color::Blue),
            Shape::new(&[&[1, 0, 0], &[1, 1, 1], &[0, 0, 0]], Color::Magenta),
            Shape::new(&[&[1, 1], &[1, 1]], Color::Cyan),
            Shape::new(
                &[&[0, 0, 0, 0], &[1, 1, 1, 1], &[0, 0, 0, 0], &[0, 0, 0, 0]],
                Color::White,
            ),
        ];

        let mut shape_generator = ShapeGenerator::new(shapes);
        let permuted_shapes = shape_generator.generate_permutation(); // Generate the random permutation

        Self {
            table: [[0; COLS]; ROWS],
            current: permuted_shapes[0].clone(),
            next_shape: permuted_shapes[1].clone(),
            score: 0,
            running: true,
            paused: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            shape_generator, // Add the shape generator to the game state
        }
    }

    pub fn is_game_over(&self) -> bool {
        // Get the height and width from the shape's array
        let height = self.current.array.len();
        let width = self.current.array[0].len();

        // Check if the shape cannot be placed on the board (i.e., the table is full)
        for x in 0..width {
            for y in 0..height {
                if self.current.array[x][y] == 1 {
                    let new_row = (self.current.row + x as isize) as usize;
                    let new_col = (self.current.col + y as isize) as usize;

                    // Check bounds and collision with other shapes
                    if new_row >= ROWS || new_col >= COLS || self.table[new_row][new_col] == 1 {
                        return true; // The game is over if there's no place to put the shape
                    }
                }
            }
        }
        false
    }

    pub fn is_valid_position(&self, shape: &Shape) -> bool {
        for i in 0..shape.width {
            for j in 0..shape.width {
                if shape.array[i][j] == 1 {
                    let row = shape.row + i as isize;
                    let col = shape.col + j as isize;

                    if row < 0 || col < 0 || col >= COLS as isize || row >= ROWS as isize {
                        return false;
                    }

                    if self.table[row as usize][col as usize] == 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn move_shape(&mut self, dx: isize, dy: isize) {
        let mut temp = self.current.clone();
        temp.row += dy;
        temp.col += dx;

        if temp.row >= 0 && self.is_valid_position(&temp) {
            self.current = temp;
        } else if dy > 0 {
            self.place_shape();
        }
    }

    pub fn rotate_shape(&mut self) {
        let mut temp = self.current.clone();
        temp.rotate();
        if self.is_valid_position(&temp) {
            self.current = temp;
        }
    }

    fn place_shape(&mut self) {
        // Place the current shape on the board
        for i in 0..self.current.width {
            for j in 0..self.current.width {
                if self.current.array[i][j] == 1 {
                    let row = self.current.row + i as isize;
                    let col = self.current.col + j as isize;

                    // Ensure valid indices before casting
                    if row >= 0 && col >= 0 {
                        self.table[row as usize][col as usize] = 1;
                    }
                }
            }
        }

        // Check and remove completed lines
        self.check_lines();

        // Move to the next shape
        self.current = self.next_shape.clone();
        if let Some(next) = self.shape_generator.next_shape() {
            self.next_shape = next; // Use the next shape from the permutation
        } else {
            // If there are no more shapes, regenerate the permutation
            self.shape_generator.generate_permutation();
            if let Some(next) = self.shape_generator.next_shape() {
                self.next_shape = next;
            }
        }

        // If the new shape cannot be placed, the game is over
        if !self.is_valid_position(&self.current) {
            self.running = false;
        }
    }

    fn check_lines(&mut self) {
        let mut new_table = [[0; COLS]; ROWS];
        let mut new_row = ROWS as isize - 1;

        for i in (0..ROWS).rev() {
            if self.table[i].iter().all(|&x| x == 1) {
                self.score += 100;
            } else if new_row >= 0 {
                new_table[new_row as usize] = self.table[i];
                new_row -= 1;
            }
        }

        self.table = new_table;
    }
}
