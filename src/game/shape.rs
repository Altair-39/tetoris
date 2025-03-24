use crossterm::style::Color;
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Shape {
    pub array: Vec<Vec<u8>>,
    pub width: usize,
    pub row: isize,
    pub col: isize,
    pub color: Color,
}

impl Shape {
    pub fn new(shape_data: &[&[u8]], color: Color) -> Self {
        let array = shape_data.iter().map(|row| row.to_vec()).collect();
        let width = shape_data.len();
        Self {
            array,
            width,
            row: 0,
            col: 5 - width as isize / 2,
            color,
        }
    }

    pub fn rotate(&mut self) {
        let mut new_array = vec![vec![0; self.width]; self.width];
        for i in 0..self.width {
            for j in 0..self.width {
                new_array[i][j] = self.array[self.width - j - 1][i];
            }
        }
        self.array = new_array;
    }
}

pub struct ShapeGenerator {
    shapes: Vec<Shape>,
    permutation: Vec<Shape>,
    rng: rand::rngs::ThreadRng,
}

impl ShapeGenerator {
    pub fn new(shapes: Vec<Shape>) -> Self {
        ShapeGenerator {
            shapes,
            permutation: Vec::new(),
            rng: rand::rng(),
        }
    }

    // Generate the permutation using Fisher-Yates Shuffle
    pub fn generate_permutation(&mut self) -> Vec<Shape> {
        let mut shapes = self.shapes.clone();
        shapes.shuffle(&mut self.rng); // Shuffle the shapes vector in place
        self.permutation = shapes;
        self.permutation.clone()
    }

    // Public method to get the next shape from the permutation
    pub fn next_shape(&mut self) -> Option<Shape> {
        self.permutation.pop() // Pop the last shape from the permutation
    }
}
