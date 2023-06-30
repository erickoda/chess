use std::io;

const WHITE_PAWN   : char = '♟';
const WHITE_KING   : char = '♚';
const WHITE_TOWER  : char = '♜';
const WHITE_QUEEN  : char = '♛';
const WHITE_KNIGHT : char = '♞';
const WHITE_BISHOP : char = '♝';

const BLACK_PAWN   : char = '🨣';
const BLACK_KING   : char = '🨞';
const BLACK_TOWER  : char = '🨠';
const BLACK_QUEEN  : char = '🨟';
const BLACK_KNIGHT : char = '🨢';
const BLACK_BISHOP : char = '🨡';


#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn convert(&mut self, next_input: &String){
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

#[derive(Copy, Clone)]
enum ColorOfPiece {
    White,
    Black,
}

struct Pieces {
    pawn:   [Position; 8],
    tower:  [Position; 2],
    bishop: [Position; 2],
    knight: [Position; 2],
    queen:  [Position; 1],
    king:   [Position; 1],
    color: Option<ColorOfPiece>,
}

impl Default for Pieces {
    fn default() -> Self {
        Pieces{
            pawn:   [Position { x: 0, y: 0 }; 8],
            tower:  [Position { x: 0, y: 0 }; 2],
            bishop: [Position { x: 0, y: 0 }; 2],
            knight: [Position { x: 0, y: 0 }; 2],
            queen:  [Position { x: 0, y: 0 }; 1],
            king:   [Position { x: 0, y: 0 }; 1],
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

    fn is_pawn(&mut self, next_position: &Position) -> bool{
        for pawn_position in &mut self.pawn {
            let is_pawn = pawn_position.x == next_position.x && pawn_position.y == next_position.y;
            if  is_pawn {
                return true;
            }
        }
        false
    }

    fn move_piece(&mut self, choosen_piece: &Position, next_position: &Position){

        for pawn_position in &mut self.pawn {
            let is_pawn = pawn_position.x == choosen_piece.x && pawn_position.y == choosen_piece.y;
            if  is_pawn {
                println!("pinto");
                *pawn_position = *next_position;
                return;
            }
        }

        for tower_position in &mut self.tower {
            let is_tower = tower_position.x == choosen_piece.x && tower_position.y == choosen_piece.y;
            if is_tower {
                *tower_position = *next_position;
                return;
            }
        }

        for knight_position in &mut self.knight {
            let is_knight = knight_position.x == choosen_piece.x && knight_position.y == choosen_piece.y;
            if is_knight {
                *knight_position = *next_position;
                return;
            }
        }

        for bishop_position in &mut self.bishop {
            let is_bishop = bishop_position.x == choosen_piece.x && bishop_position.y == choosen_piece.y;
            if is_bishop {
                *bishop_position = *next_position;
                return;
            }
        }

        for queen_position in &mut self.queen {
            let is_queen = queen_position.x == choosen_piece.x && queen_position.y == choosen_piece.y;
            if is_queen {
                *queen_position = *next_position;
                return;
            }
        }

        for king_position in &mut self.king {
            let is_king = king_position.x == choosen_piece.x && king_position.y == choosen_piece.y;
            if is_king {
                *king_position = *next_position;
                return;
            }
        }
    }
}

fn setup_board(board: &mut [[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces) {

    //todo refatorar struct Pieces com Enum pra essa poha virar funcoes menores
    //spawn pawn
    for i in 0..white_pieces.pawn.len() {
        board[white_pieces.pawn[i].x][white_pieces.pawn[i].y] = WHITE_PAWN;
        board[black_pieces.pawn[i].x][black_pieces.pawn[i].y] = BLACK_PAWN;
    }

    //spawn tower
    for i in 0..white_pieces.tower.len() {
        board[white_pieces.tower[i].x][white_pieces.tower[i].y] = WHITE_TOWER;
        board[black_pieces.tower[i].x][black_pieces.tower[i].y] = BLACK_TOWER;
    }

    //spawn bishop
    for i in 0..white_pieces.bishop.len() {
        board[white_pieces.bishop[i].x][white_pieces.bishop[i].y] = WHITE_BISHOP;
        board[black_pieces.bishop[i].x][black_pieces.bishop[i].y] = BLACK_BISHOP;
    }

    //spawn horse
    for i in 0..white_pieces.knight.len() {
        board[white_pieces.knight[i].x][white_pieces.knight[i].y] = WHITE_KNIGHT;
        board[black_pieces.knight[i].x][black_pieces.knight[i].y] = BLACK_KNIGHT;
    }

    //spawn queen
    board[white_pieces.queen[0].x][white_pieces.queen[0].y] = WHITE_QUEEN;
    board[black_pieces.queen[0].x][black_pieces.queen[0].y] = BLACK_QUEEN;
    //spawn king
    board[white_pieces.king[0].x][white_pieces.king[0].y] = WHITE_KING;
    board[black_pieces.king[0].x][black_pieces.king[0].y] = BLACK_KING;
}

fn update_board(board: &mut [[char; 8]; 8], board_without_pieces: &[[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces){

        for i in 0..board.len(){
            for j in 0..board[i].len(){
                board[i][j] = board_without_pieces[i][j];
            }
        }
    
        setup_board(board, white_pieces, black_pieces);
}

fn print_board(board: &mut [[char; 8]; 8]){
    for i in board{
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

fn get_position_of(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}

fn main() {
    let     won = false;
    let mut board = [
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
    ];

    let board_without_pieces = board.clone();

    let mut white_pieces = Pieces::setup(ColorOfPiece::White);
    let mut black_pieces = Pieces::setup(ColorOfPiece::Black);
    
    let mut choosen_piece    = String::with_capacity(2);
    let mut choosen_position = String::with_capacity(2);

    let mut converted_choosen_piece    = Position{x: 0, y: 0};
    let mut converted_choosen_position = Position{x: 0, y: 0};
    
    setup_board(&mut board, &white_pieces, &black_pieces);

    let mut is_white_turn = true;

    while !won {
        print_board(&mut board);

        println!("What Piece do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_piece);
        converted_choosen_piece.convert(&choosen_piece);

        println!("To where do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_position);
        converted_choosen_position.convert(&choosen_position);

        if converted_choosen_piece.x == 8 || converted_choosen_piece.y == 8 || converted_choosen_position.x == 8 || converted_choosen_position.y == 8 {
            println!("Out of the board!!!");
            continue;
        }

        if converted_choosen_piece.x == converted_choosen_position.x && converted_choosen_piece.y == converted_choosen_position.y {
            println!("You can't move to the same position!!!");
            continue;
        }

        println!("{}", board[converted_choosen_piece.x][converted_choosen_piece.y]);

        match is_white_turn{
            true  => white_pieces.move_piece(&converted_choosen_piece, &converted_choosen_position),
            false => black_pieces.move_piece(&converted_choosen_piece, &converted_choosen_position),
        }
        
        for i in white_pieces.pawn.iter() {
            println!("{} {}", i.x, i.y);
        }

        update_board(&mut board, &board_without_pieces, &white_pieces, &black_pieces);

        if is_white_turn{
            is_white_turn = false;
        } else {
            is_white_turn = true;
        }
    }
}
