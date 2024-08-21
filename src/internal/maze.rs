use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;
use std::sync::Arc;

use super::color::Color;
use super::texture::Texture;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(
    || Arc::new(Texture::new("assets/walls.png")));
pub struct Maze {
    structure : Vec<Vec<char>>
}

impl Maze {
    pub fn new(structure : Vec<Vec<char>>) -> Maze {
        Maze{structure: structure}
    }
    
    pub fn rows(&self) -> usize{
        self.structure.len()
    } 
    pub fn column_len(&self, row : usize) -> usize{
        self.structure[row].len()
    } 

    pub fn char_at(&self,  x:usize, y:usize) -> char {
        self.structure[x][y]
    }

    pub fn color_for_cell(&self, row:usize, column:usize) -> Color {
        let cell = self.char_at(row, column);
        match cell {
            '-' => Color::new(40, 120, 40),
            '+' => Color::new(40, 200, 60),
            '|' => Color::new(40, 200, 60),
            'g' => Color::new(230, 50, 60),
            _ => Color::new(10, 10, 10)
        }
    }

    pub fn texture_for_cell(&self, impact_char: char, tx: u32, ty: u32) -> Color {
        match impact_char {
            '-' | '+' | '|' => WALL1.get_pixel_color(tx, ty),
            'g' => WALL1.get_pixel_color(tx, ty),
            _ => Color::new(100, 150, 50),
        }
    }
}

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let structure = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Maze::new(structure)
}