use crossterm::style::Color;
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Shape {
    pub array: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub row: isize,
    pub col: isize,
    pub color: Color,
}

impl Shape {
    pub fn new(shape_data: &[&[u8]], color: Color) -> Self {
        let array = shape_data
            .iter()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let height = shape_data.len();
        let width = shape_data[0].len();
        Self {
            array,
            width,
            height,
            row: 0,
            col: 5 - width as isize / 2,
            color,
        }
    }

    pub fn rotate(&mut self) {
        let mut new_array = vec![vec![0; self.height]; self.width];
        for i in 0..self.height {
            for j in 0..self.width {
                new_array[j][self.height - 1 - i] = self.array[i][j];
            }
        }

        self.array = new_array;
        std::mem::swap(&mut self.width, &mut self.height);
    }
}

pub struct ShapeGenerator {
    shapes: Vec<Shape>,
    permutation: Vec<Shape>,
}

impl ShapeGenerator {
    pub fn new(shapes: Vec<Shape>) -> Self {
        ShapeGenerator {
            shapes,
            permutation: Vec::new(),
        }
    }

    pub fn generate_permutation(&mut self) -> Vec<Shape> {
        let mut rng = rand::rng();
        let mut shapes = self.shapes.clone();
        shapes.shuffle(&mut rng);
        self.permutation = shapes;
        self.permutation.clone()
    }

    pub fn next_shape(&mut self) -> Option<Shape> {
        self.permutation.pop()
    }
}
