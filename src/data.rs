pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 15;
pub const COLORS_AMOUNT: usize = 5;
pub const PIECES_AMOUNT: usize = 7;

pub type Color = [f32; 4];
pub type TetrisGrid = [[usize; WIDTH]; HEIGHT];
pub type PieceGrid = [[usize; 4]; 4];

pub struct Piece {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub grid: PieceGrid,
}

pub fn get_colors() -> [Color; COLORS_AMOUNT] {
    [
        [0.13, 0.13, 0.13, 1.00], // Blue gray
        [0.91, 0.12, 0.39, 1.00], // Pink
        [0.01, 0.66, 0.96, 1.00], // Cyan
        [1.00, 0.92, 0.23, 1.00], // Yellow
        [0.55, 0.76, 0.29, 1.00] // Green
    ]
}

pub fn get_pieces() -> [Piece; PIECES_AMOUNT] {
    [
        Piece {
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            grid: [
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
            grid: [
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
            grid: [
                [1, 1, 0, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            grid: [
                [1, 1, 1, 0],
                [1, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            grid: [
                [1, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            grid: [
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        },
        Piece {
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            grid: [
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        }
    ]
}
