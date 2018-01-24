use data::*;
use rand;

impl Piece {
    pub fn new(pieces: &[Piece; PIECES_AMOUNT], tetris_grid: &TetrisGrid) -> Piece {
        let piece_id = rand::random::<usize>() % PIECES_AMOUNT;
        let rotations = rand::random::<usize>() % 4 + 1;
        let color_id = rand::random::<usize>() % (COLORS_AMOUNT - 1) + 1;

        let mut piece = Piece {
            ..pieces[piece_id]
        };

        for _ in 0..rotations {
            piece = piece.try_rotate(tetris_grid)
        }

        piece.set_color(color_id)
    }

    pub fn set_color(mut self, color_id: usize) -> Piece {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == 1 {
                    self.grid[y][x] = color_id;
                }
            }
        }

        self
    }

    // Plot

    pub fn plot<F>(&self, mut plotter: F) where F: FnMut(usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
               plotter(x, y)
            }
        }
    }

    // Actions

    pub fn can_go_down(&self, tetris_grid: &TetrisGrid) -> bool {
        self.y < HEIGHT - self.height && !self.will_collide(self.x, self.y + 1, tetris_grid)
    }

    pub fn try_down(mut self, tetris_grid: &TetrisGrid) -> Piece {
        if self.can_go_down(tetris_grid) {
            self.y += 1;
        }
        self
    }

    pub fn try_left(mut self, tetris_grid: &TetrisGrid) -> Piece {
        if self.x > 0 && !self.will_collide(self.x - 1, self.y, tetris_grid) {
            self.x -= 1;
        }
        self
    }

    pub fn try_right(mut self, tetris_grid: &TetrisGrid) -> Piece {
        if self.x < WIDTH - self.width && !self.will_collide(self.x + 1, self.y, tetris_grid) {
            self.x += 1;
        }
        self
    }

    pub fn try_rotate(self, tetris_grid: &TetrisGrid) -> Piece  {
        if let Some(piece) = self.maybe_rotate(tetris_grid) {
            piece
        } else {
            self
        }
    }

    pub fn maybe_rotate(&self, tetris_grid: &TetrisGrid) -> Option<Piece> {
        let mut next_piece = Piece {
            ..*self
        };

        let mut next_grid = next_piece.grid;

        next_piece.plot(|x, y| {
            next_grid[x][next_piece.height - 1 - y] = next_piece.grid[y][x];
        });

        let width = next_piece.width;
        let height = next_piece.height;
        next_piece.width = height;
        next_piece.height = width;
        next_piece.grid = next_grid;
        
        if next_piece.is_inside_tetris() && !next_piece.will_collide(next_piece.x, next_piece.y, tetris_grid) {
            Some(next_piece)
        } else {
            None
        }
    }

    pub fn is_inside_tetris(&self) -> bool {
        self.x + self.width <= WIDTH &&
        self.y + self.height <= HEIGHT
    }

    // Math

    pub fn has_block(&self, tetris_x: usize, tetris_y: usize) -> bool {
        self.has_point(tetris_x, tetris_y) &&
        self.grid[tetris_y - self.y][tetris_x - self.x] != 0  
    }

    pub fn has_point(&self, tetris_x: usize, tetris_y: usize) -> bool {
        let &Piece { x, y, width, height, .. } = self;

        tetris_x >= x &&
        tetris_x < x + width &&
        tetris_y >= y &&
        tetris_y < y + height
    }

    pub fn will_collide(&self, next_x: usize, next_y: usize, tetris_grid: &TetrisGrid) -> bool {
        let mut will = false;

        self.plot(|x, y| {
            if self.grid[y][x] != 0 && tetris_grid[next_y + y][next_x + x] != 0 {
                will = true;
            }
        });

        will
    }
}
