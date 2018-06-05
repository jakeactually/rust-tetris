use data::*;
use piston_window::*;

pub struct Tetris {
    width: i32,
    height: i32,
    tile_size: i32,
    grid: TetrisGrid,
    colors: [Color; COLORS_AMOUNT as usize],
    pieces: [Piece; PIECES_AMOUNT as usize],
    piece: Piece
}

impl Tetris {
    pub fn new() -> Tetris {
        let grid = [[0; WIDTH as usize]; HEIGHT as usize];
        let pieces = get_pieces();
        let piece = Piece::new(&pieces, &grid);

        Tetris {
            width: WIDTH,
            height: HEIGHT,
            tile_size: 40,
            grid,
            colors: get_colors(),
            pieces,
            piece
        }
    }

    // Plot

    pub fn plot<F>(&self, mut plotter: F) where F: FnMut(i32, i32) {
        for y in 0..self.height {
            for x in 0..self.width {
               plotter(x, y);
            }
        }
    }

    // Events

    pub fn tick(mut self) -> Tetris {
        if self.piece.can_go_down(&self.grid) {
            self.piece = self.piece.try_down(&self.grid);
            self
        } else {
            self
                .save_piece()
                .eval_grid()
                .next_piece()
        }
    }

    pub fn key(mut self, key: Key) -> Tetris {
        let piece = self.piece;

        self.piece = match key {
            Key::Left => piece.try_left(&self.grid),
            Key::Right => piece.try_right(&self.grid),
            Key::Down => piece.try_down(&self.grid),
            Key::Space => piece.try_rotate(&self.grid),
            _ => piece
        };

        self
    }

    pub fn save_piece(mut self) -> Tetris {
        let piece = self.piece;

        for y in 0..piece.height {
            for x in 0..piece.width {
                let block = piece.grid[y as usize][x as usize];
                if block != 0 {
                    self.grid[(piece.y + y) as usize][(piece.x + x) as usize] = block;
                }
            }
        }

        self.piece = piece;
        self
    }

    pub fn eval_grid(mut self) -> Tetris {
        let mut next_grid = [[0; WIDTH as usize]; HEIGHT as usize];
        let reverse = self.height - 1;
        let mut i = 0;

        for y in 0..self.height {
            if self.grid[(reverse - y) as usize].iter().any(|&x| x == 0) {
                next_grid[(reverse - i) as usize] = self.grid[(reverse - y) as usize];
                i += 1;
            }
        }

        self.grid = next_grid;
        self
    }

    pub fn next_piece(mut self) -> Tetris {
        self.piece = Piece::new(&self.pieces, &self.grid);
        self
    }

    // Draw

    pub fn draw<G>(&self, context: &Context, graphics: &mut G) where G: Graphics {
        clear(self.get_color(0), graphics);

        self.plot(|x, y| {
            let piece = &self.piece;

            let color = if piece.has_block(x, y) {
                self.get_color(piece.grid[(y - piece.y) as usize][(x - piece.x) as usize])
            } else {
                self.get_color(self.grid[y as usize][x as usize])
            };

            rectangle(color, self.rect(x, y), context.transform, graphics);
        });
    }

    pub fn get_color(&self, index: i32) -> Color {
        self.colors[index as usize]
    }

    pub fn rect(&self, x: i32, y: i32) -> [f64; 4] {
        let tile_size = self.tile_size;
        [
            (x * tile_size) as f64,
            (y * tile_size) as f64,
            (tile_size - 1) as f64,
            (tile_size - 1) as f64
        ]
    }
}
