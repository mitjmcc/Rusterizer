#[derive(Debug, Clone, Copy)]
struct Square {
    pub pos: (f32, f32),
    pub size: f32,
    pub color: [f32; 3],
}

// A cube is a pile of infinitely (as continuum) many squares
// This data stucture is finite, so we call it “pseudo”
#[derive(Debug)]
struct Pseudocube {
    squares: Vec<Square>,
    ratio: f32,
}

impl Pseudocube {
    pub fn new() -> Self {
        Pseudocube {
            squares: vec![],
            ratio: 1.0,
        }
    }

    pub fn add_square(&mut self, x: f32, y: f32, size: f32, color: [f32; 3]) {
        let sq = Square {
            pos: (x, y),
            size,
            color,
        };
        self.squares.push(sq);
    }

    pub fn get_vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let (mut vs, mut is) = (vec![], vec![]);
        for (i, sq) in self.squares.iter().enumerate() {
            let (pos, half) = (sq.pos, 0.5 * sq.size);
            let i = i as u16;

            let (hx, hy);
            if self.ratio > 1.0 {
                hx = half / self.ratio;
                hy = half;
            } else {
                hx = half;
                hy = half * self.ratio;
            }

            vs.extend(&[
                Vertex {
                    pos: [pos.0 + hx, pos.1 - hy],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 - hx, pos.1 - hy],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 - hx, pos.1 + hy],
                    color: sq.color,
                },
                Vertex {
                    pos: [pos.0 + hx, pos.1 + hy],
                    color: sq.color,
                },
            ]);
            is.extend(&[4 * i, 4 * i + 1, 4 * i + 2, 4 * i + 2, 4 * i + 3, 4 * i]);
        }

        (vs, is)
    }

    pub fn update_ratio(&mut self, ratio: f32) {
        self.ratio = ratio
    }
}
