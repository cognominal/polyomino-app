use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            grid: vec![false; width * height],
        }
    }

    pub fn place_polyomino(&mut self, x: usize, y: usize, polyomino: &Polyomino) -> bool {
        if self.can_place(x, y, polyomino) {
            for &(dx, dy) in polyomino.blocks.iter() {
                self.grid[(y + dy) * self.width + (x + dx)] = true;
            }
            true
        } else {
            false
        }
    }

    pub fn can_place(&self, x: usize, y: usize, polyomino: &Polyomino) -> bool {
        for &(dx, dy) in polyomino.blocks.iter() {
            if x + dx >= self.width || y + dy >= self.height || self.grid[(y + dy) * self.width + (x + dx)] {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            *cell = false;
        }
    }
}

#[wasm_bindgen]
pub struct Polyomino {
    blocks: Vec<(usize, usize)>,  // Coordinates relative to the top-left corner
}

#[wasm_bindgen]
impl Polyomino {
    #[wasm_bindgen(constructor)]
    pub fn new(blocks: Vec<(usize, usize)>) -> Polyomino {
        Polyomino { blocks }
    }

    pub fn rotate(&mut self) {
        self.blocks = self.blocks.iter().map(|&(x, y)| (-y as usize, x as usize)).collect();
    }

    pub fn get_blocks(&self) -> Vec<(usize, usize)> {
        self.blocks.clone()
    }
}
