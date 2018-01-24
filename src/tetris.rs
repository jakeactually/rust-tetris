use data::*;
use piston_window::*;

pub struct Tetris {
    width: usize,
    height: usize,
    tile_size: usize,
    grid: TetrisGrid,
    colors: [Color; COLORS_AMOUNT],
    pieces: [Piece; PIECES_AMOUNT],
    piece: Piece
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            width: WIDTH,
            height: HEIGHT,
            tile_size: 40,
            grid: [[0; WIDTH]; HEIGHT],
            colors: get_colors(),
            pieces: get_pieces(),
            piece: Piece::new(&get_pieces(), &[[0; WIDTH]; HEIGHT])
        }
    }

    // Plot

    pub fn plot<F>(&self, mut plotter: F) where F: FnMut(usize, usize) {
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
                let block = piece.grid[y][x];
                if block != 0 {
                    self.grid[piece.y + y][piece.x + x] = block;
                }
            }
        }

        self.piece = piece;
        self
    }

    pub fn eval_grid(mut self) -> Tetris {
        let mut next_grid = [[0; WIDTH]; HEIGHT];
        let reverse = self.height - 1;
        let mut i = 0;

        for y in 0..self.height {
            if self.grid[reverse - y].iter().any(|x| x == &0) {
                next_grid[reverse - i] = self.grid[reverse - y];
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

    pub fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        clear(self.get_color(0), graphics);

        self.plot(|x, y| {
            let piece = &self.piece;

            let color = if piece.has_block(x, y) {
                self.get_color(piece.grid[y - piece.y][x - piece.x])
            } else {
                self.get_color(self.grid[y][x])
            };

            rectangle(color, self.rect(x, y), context.transform, graphics);
        });
    }

    pub fn get_color(&self, index: usize) -> Color {
        self.colors[index]
    }

    pub fn rect(&self, x: usize, y: usize) -> [f64; 4] {
        let tile_size = self.tile_size;
        [(x * tile_size) as f64, (y * tile_size) as f64, (tile_size - 1) as f64, (tile_size - 1) as f64]
    }
}
