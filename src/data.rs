pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 15;
pub const COLORS_AMOUNT: i32 = 5;
pub const PIECES_AMOUNT: i32 = 7;

pub type Color = [f32; 4];
pub type TetrisGrid = [[i32; WIDTH as usize]; HEIGHT as usize];
pub type PieceGrid = [[i32; 4]; 4];

pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub grid: PieceGrid,
}

pub fn get_colors() -> [Color; COLORS_AMOUNT as usize] {
    [
        [0.13, 0.13, 0.13, 1.00], // Blue gray
        [0.91, 0.12, 0.39, 1.00], // Pink
        [0.01, 0.66, 0.96, 1.00], // Cyan
        [1.00, 0.92, 0.23, 1.00], // Yellow
        [0.55, 0.76, 0.29, 1.00]  // Green
    ]
}

pub fn get_pieces() -> [Piece; PIECES_AMOUNT as usize] {
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
