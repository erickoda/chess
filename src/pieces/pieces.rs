use std::io;

pub mod illustration{
    pub const WHITE_PAWN   : char = '♟';
    pub const WHITE_KING   : char = '♚';
    pub const WHITE_ROOK   : char = '♜';
    pub const WHITE_QUEEN  : char = '♛';
    pub const WHITE_KNIGHT : char = '♞';
    pub const WHITE_BISHOP : char = '♝';

    pub const BLACK_PAWN   : char = '♙';
    pub const BLACK_KING   : char = '♔';
    pub const BLACK_ROOK   : char = '♖';
    pub const BLACK_QUEEN  : char = '♕';
    pub const BLACK_KNIGHT : char = '♘';
    pub const BLACK_BISHOP : char = '♗';
}

#[derive(Clone, Copy)]
pub enum ColorOfPiece {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum TypeOfPiece {
    Pawn,
    King,
    Rook,
    Queen,
    Knight,
    Bishop,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 8,
            y: 8,
        }
    }
}

impl Position {
    pub fn convert(&mut self, next_input: &String){
        self.x = match next_input.chars().nth(1).unwrap() {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _   => 8,
        };
        self.y = match next_input.chars().nth(0).unwrap() {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _         => 8,
        };
    }
}

pub struct ConvertedPositions{
    pub choosen_piece:              Position,
    pub choosen_piece_new_position: Position,
}

impl Default for ConvertedPositions{
    fn default() -> Self {
        ConvertedPositions{
            choosen_piece:              Position::default(),
            choosen_piece_new_position: Position::default(),
        }
    }
}   

#[derive(Clone, Copy)]
pub struct Piece {
    pub color:    Option<ColorOfPiece>,
    pub type_of:  Option<TypeOfPiece>,
    pub position: Position,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            color:    None,
            type_of:  None,
            position: Position::default(),
        }
    }
}


impl Piece {
    pub fn pawn_setup(&mut self, position: &Position) -> Piece {
        Piece {
            color:    self.color,
            type_of:  Some(TypeOfPiece::Pawn),
            position: Position::default(),
        }
    }
}

pub struct Pieces {
    //refazer com vector
    pub pawn:   [Piece; 8],
    pub rook:   [Piece; 2],
    pub bishop: [Piece; 2],
    pub knight: [Piece; 2],
    pub queen:  [Piece; 1],
    pub king:   [Piece; 1],
}

impl Default for Pieces {
    fn default() -> Self {
        Pieces{
            pawn:   [Piece::default(); 8],
            rook:   [Piece::default(); 2],
            bishop: [Piece::default(); 2],
            knight: [Piece::default(); 2],
            queen:  [Piece::default(); 1],
            king:   [Piece::default(); 1],
        }
    }
}

impl Pieces {

    pub fn setup(color: ColorOfPiece) -> Self {

        let mut pieces = Pieces::default();

        for i in 0..pieces.pawn.len() {
            pieces.pawn[i].color = Some(color);
            pieces.pawn[i].position = match pieces.pawn[i].color {
                Some(ColorOfPiece::White) => Position{x: 6, y: i},
                Some(ColorOfPiece::Black) => Position{x: 1, y: i},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.pawn[i].type_of = match pieces.pawn[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::Pawn),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::Pawn),
                None => panic!("Couldn't create pieces color"),
            };
        }

        for i in 0..pieces.rook.len() {
            pieces.rook[i].color = Some(color);
            pieces.rook[i].position = match pieces.rook[i].color {
                Some(ColorOfPiece::White) => Position{x: 7, y: i*7},
                Some(ColorOfPiece::Black) => Position{x: 0, y: i*7},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.rook[i].type_of = match pieces.rook[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::Rook),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::Rook),
                None => panic!("Couldn't create pieces color"),
            };
        }

        for i in 0..pieces.bishop.len() {
            pieces.bishop[i].color = Some(color);
            pieces.bishop[i].position = match pieces.bishop[i].color {
                Some(ColorOfPiece::White) => Position{x: 7, y: i*3+2},
                Some(ColorOfPiece::Black) => Position{x: 0, y: i*3+2},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.bishop[i].type_of = match pieces.bishop[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::Bishop),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::Bishop),
                None => panic!("Couldn't create pieces color"),
            };
        }

        for i in 0..pieces.knight.len() {
            pieces.knight[i].color = Some(color);
            pieces.knight[i].position = match pieces.knight[i].color {
                Some(ColorOfPiece::White) => Position{x: 7, y: i*5+1},
                Some(ColorOfPiece::Black) => Position{x: 0, y: i*5+1},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.knight[i].type_of = match pieces.knight[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::Knight),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::Knight),
                None => panic!("Couldn't create pieces color"),
            };
        }

        for i in 0..pieces.queen.len() {
            pieces.queen[i].color = Some(color);
            pieces.queen[i].position = match pieces.queen[i].color {
                Some(ColorOfPiece::White) => Position{x: 7, y: 3},
                Some(ColorOfPiece::Black) => Position{x: 0, y: 3},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.queen[i].type_of = match pieces.queen[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::Queen),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::Queen),
                None => panic!("Couldn't create pieces color"),
            };
        }

        for i in 0..pieces.king.len() {
            pieces.king[i].color = Some(color);
            pieces.king[i].position = match pieces.king[i].color {
                Some(ColorOfPiece::White) => Position{x: 7, y: 4},
                Some(ColorOfPiece::Black) => Position{x: 0, y: 4},
                None => panic!("Couldn't create pieces color"),
            };
            pieces.king[i].type_of = match pieces.king[i].color {
                Some(ColorOfPiece::White) => Some(TypeOfPiece::King),
                Some(ColorOfPiece::Black) => Some(TypeOfPiece::King),
                None => panic!("Couldn't create pieces color"),
            };
        }

        pieces
    }

    pub fn get_choosen_piece_ptr(&mut self, converted: &ConvertedPositions) -> Option<&mut Piece>{
        
        for pawn in &mut self.pawn {
            let is_pawn = pawn.position.x == converted.choosen_piece.x && pawn.position.y == converted.choosen_piece.y;
            if  is_pawn {
                return Some(pawn);
            }
        }

        for rook in &mut self.rook {
            let is_rook = rook.position.x == converted.choosen_piece.x && rook.position.y == converted.choosen_piece.y;
            if is_rook {
                return Some(rook);
            }
        }

        for knight in &mut self.knight {
            let is_knight = knight.position.x == converted.choosen_piece.x && knight.position.y == converted.choosen_piece.y;
            if is_knight {
                return Some(knight);
            }
        }

        for bishop in &mut self.bishop {
            let is_bishop = bishop.position.x == converted.choosen_piece.x && bishop.position.y == converted.choosen_piece.y;
            if is_bishop {
                return Some(bishop);
            }
        }
        
        for queen in &mut self.queen {
            let is_queen = queen.position.x == converted.choosen_piece.x && queen.position.y == converted.choosen_piece.y;
            if  is_queen {
                return Some(queen);
            }
        }

        for king in &mut self.king {
            let is_king = king.position.x == converted.choosen_piece.x && king.position.y == converted.choosen_piece.y;
            if is_king {
                return Some(king);
            }
        }

        return None;
    }
}

pub fn get_position_of(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}
