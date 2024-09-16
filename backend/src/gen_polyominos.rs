use std::collections::HashSet;

pub struct Polyomino {
    blocks: Vec<(usize, usize)>,  // Coordinates relative to the top-left corner
}

impl Polyomino {
    // #[wasm_bindgen(constructor)]
    pub fn new(blocks: Vec<(usize, usize)>) -> Polyomino {
        Polyomino { blocks }
    }

    /// Rotates the polyomino 90 degrees clockwise
    pub fn rotate(&mut self) {
        self.blocks = self.blocks.iter().map(|&(x, y)| (-y as usize, x as usize)).collect();
    }

    /// Returns the blocks (relative coordinates) of the polyomino
    pub fn get_blocks(&self) -> Vec<(usize, usize)> {
        self.blocks.clone()
    }
}

// We define a normalize function that shifts the pentomino so that its bottom-left corner is at (0,0).
// We define a canonicalize function that rotates the pentomino to find its most "canonical" form (the one that comes first lexicographically).
// In the main generate_pentominoes function, we use a HashSet instead of a Vec to automatically eliminate duplicates.
// Before adding each new pentomino to the set, we canonicalize it.
// This approach ensures that all symmetrical variations of the same pentomino are considered as the same shape, eliminating duplicates based on rotation and reflection.

// You can use this function the same way as before:

// let pentominoes = generate_pentominoes(5);
// println!("Generated {} unique pentominoes:", pentominoes.len());

fn normalize(pento: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let min_x = pento.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = pento.iter().map(|&(_, y)| y).min().unwrap();
    
    let normalized: HashSet<_> = pento.iter().map(|&(x, y)| (x - min_x, y - min_y)).collect();
    normalized.into_iter().collect()
}

fn canonicalize(pento: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut best = pento.to_vec();
    let mut rotated = pento.to_vec();
    
    for _ in 0..3 {
        rotated.rotate_right(1);
        let normalized = normalize(&rotated);
        if normalized < best {
            best = normalized;
        }
    }
    
    best
}

pub fn generate_pentominoes(n: usize) -> Vec<Vec<(usize, usize)>> {
    if n == 1 {
        return vec![vec![(0, 0)]];
    }

    let prev_pentos = generate_pentominoes(n - 1);
    let mut current_pentos = HashSet::new();

    for pento in &prev_pentos {
        for &(dx, dy) in pento.iter() {
            for x in 0..=n {
                for y in 0..=n {
                    if x + dx >= n || y + dy >= n {
                        continue;
                    }
                    let mut new_pento = pento.clone();
                    new_pento.push((x + dx, y + dy));
                    current_pentos.insert(canonicalize(&new_pento));
                }
            }
        }
    }

    current_pentos.into_iter().collect()
}