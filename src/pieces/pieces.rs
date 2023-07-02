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
pub enum ColorOfPiece {
    White,
    Black,
}

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

pub struct Piece {
    pub color:    ColorOfPiece,
    pub type_of:  TypeOfPiece,
    pub position: Position,
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

pub struct Pieces {
    pub pawn:   [Position; 8],
    pub rook:   [Position; 2],
    pub bishop: [Position; 2],
    pub knight: [Position; 2],
    pub queen:  [Position; 1],
    pub king:   [Position; 1],
    color: Option<ColorOfPiece>,
}

impl Default for Pieces {
    fn default() -> Self {
        Pieces{
            pawn:   [Position::default(); 8],
            rook:   [Position::default(); 2],
            bishop: [Position::default(); 2],
            knight: [Position::default(); 2],
            queen:  [Position::default(); 1],
            king:   [Position::default(); 1],
            color:  None,
        }
    }
}

impl Pieces {

    pub fn setup(color: ColorOfPiece) -> Self {

        let mut pieces = Pieces::default();
        pieces.setup_color(color);
        pieces.setup_position();

        pieces
    }

    fn setup_color(&mut self, color: ColorOfPiece){
        self.color = Some(color);
    }

    fn setup_position(&mut self){

        let position_y: usize = match self.color {
            Some(ColorOfPiece::White) => 7,
            Some(ColorOfPiece::Black) => 0,
            _ => panic!("Couldn't create pieces color"),
        };
        let color_correction: i32 = match self.color {
            Some(ColorOfPiece::White) => -1,
            Some(ColorOfPiece::Black) => 1,
            _ => panic!("Couldn't create pieces color"),
        };

        //Pawn
        for i in 0..self.pawn.len() {
            self.pawn[i].x = (position_y as i32 + color_correction) as usize;
            self.pawn[i].y = i;
        }
        //rook
        for i in 0..self.rook.len() {
            self.rook[i].x = position_y;
            self.rook[i].y =  i*7;
        }
        //Knight
        for i in 0..self.knight.len() {
            self.knight[i].x = position_y;
            self.knight[i].y =  i*5+1;
        }
        //Bishop
        for i in 0..self.bishop.len(){
            self.bishop[i].x = position_y;
            self.bishop[i].y =  i*3+2;
        }
        //queen
        self.queen[0].x = position_y;
        self.queen[0].y = 3;
        //King
        self.king[0].x = position_y;
        self.king[0].y = 4;
    }

    pub fn get_choosen_piece_ptr(&mut self, converted: &ConvertedPositions) -> Option<&mut Position>{
        
        for pawn_position in &mut self.pawn {
            let is_pawn = pawn_position.x == converted.choosen_piece.x && pawn_position.y == converted.choosen_piece.y;
            if  is_pawn {
                return Some(pawn_position);
            }
        }

        for rook_position in &mut self.rook {
            let is_rook = rook_position.x == converted.choosen_piece.x && rook_position.y == converted.choosen_piece.y;
            if is_rook {
                return Some(rook_position);
            }
        }

        for knight_position in &mut self.knight {
            let is_knight = knight_position.x == converted.choosen_piece.x && knight_position.y == converted.choosen_piece.y;
            if is_knight {
                return Some(knight_position);
            }
        }

        for bishop_position in &mut self.bishop {
            let is_bishop = bishop_position.x == converted.choosen_piece.x && bishop_position.y == converted.choosen_piece.y;
            if is_bishop {
                return Some(bishop_position);
            }
        }
        
        let is_queen = self.queen[0].x == converted.choosen_piece.x && self.queen[0].y == converted.choosen_piece.y;
        if  is_queen {
            return Some(&mut self.queen[0]);
        }
        
        let is_king = self.king[0].x == converted.choosen_piece.x && self.king[0].y == converted.choosen_piece.y;
        if is_king {
            return Some(&mut self.king[0]);
        }
        println!("pinto2");
        return None;
    }
}

pub fn get_position_of(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}
