use std::io;

pub mod illustration{
    pub const WHITE_PAWN   : char = '♟';
    pub const WHITE_KING   : char = '♚';
    pub const WHITE_TOWER  : char = '♜';
    pub const WHITE_QUEEN  : char = '♛';
    pub const WHITE_KNIGHT : char = '♞';
    pub const WHITE_BISHOP : char = '♝';

    pub const BLACK_PAWN   : char = '♙';//🨣
    pub const BLACK_KING   : char = '♔';//🨞
    pub const BLACK_TOWER  : char = '♖';//🨠
    pub const BLACK_QUEEN  : char = '♕';//🨟
    pub const BLACK_KNIGHT : char = '♘';//🨢
    pub const BLACK_BISHOP : char = '♗';//🨡
}

pub enum ColorOfPiece {
    White,
    Black,
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

pub struct Pieces {
    pub pawn:   [Position; 8],
    pub tower:  [Position; 2],
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
            tower:  [Position::default(); 2],
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
        //Tower
        for i in 0..self.tower.len() {
            self.tower[i].x = position_y;
            self.tower[i].y =  i*7;
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


    
    pub fn move_piece(&mut self, converted: &ConvertedPositions){

        //talvez se eu retornar o endereco da peca movida, eu consiga usar isso para manipular a peca diretamente

        for pawn_position in &mut self.pawn {
            let is_pawn = pawn_position.x == converted.choosen_piece.x && pawn_position.y == converted.choosen_piece.y;
            if  is_pawn {
                *pawn_position = converted.choosen_piece_new_position;
                return;
            }
        }

        for tower_position in &mut self.tower {
            let is_tower = tower_position.x == converted.choosen_piece.x && tower_position.y == converted.choosen_piece.y;
            if is_tower {
                *tower_position = converted.choosen_piece_new_position;
                return;
            }
        }

        for knight_position in &mut self.knight {
            let is_knight = knight_position.x == converted.choosen_piece.x && knight_position.y == converted.choosen_piece.y;
            if is_knight {
                *knight_position = converted.choosen_piece_new_position;
                return;
            }
        }

        for bishop_position in &mut self.bishop {
            let is_bishop = bishop_position.x == converted.choosen_piece.x && bishop_position.y == converted.choosen_piece.y;
            if is_bishop {
                *bishop_position = converted.choosen_piece_new_position;
                return;
            }
        }

        for queen_position in &mut self.queen {
            let is_queen = queen_position.x == converted.choosen_piece.x && queen_position.y == converted.choosen_piece.y;
            if is_queen {
                *queen_position = converted.choosen_piece_new_position;
                return;
            }
        }

        for king_position in &mut self.king {
            let is_king = king_position.x == converted.choosen_piece.x && king_position.y == converted.choosen_piece.y;
            if is_king {
                *king_position = converted.choosen_piece_new_position;
                return;
            }
        }
    }
}

pub fn get_position_of(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}
