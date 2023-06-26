use std::{io, string};

struct Pieces{
    pawn:   Position,
    tower:  Position,
    bisp:   Position,
    knight: Position,
    queen:  Position,
    king:   Position,
}

struct Position{
    x: usize,
    y: usize,
}

impl Position {
    fn new(&mut self, next_input: &String){
        self.x = match next_input.chars().nth(1).unwrap() {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _   => 0,
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

fn print_board(board: &[[char; 8]; 8] ){
    for i in board{
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

fn get_position_of(input: &mut String){
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}

fn main() {
    let board_whithout_pieces = [
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
    ];

    let mut board = [
        ['🨠', '🨡', '🨢', '🨟', '🨞', '🨢', '🨡', '🨠'],
        ['🨣', '🨣', '🨣', '🨣', '🨣', '🨣', '🨣', '🨣'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆'],
        ['⚆', '⚈', '⚆', '⚈', '⚆', '⚈', '⚆', '⚈'],
        ['🨅', '🨅', '🨅', '🨅', '🨅', '🨅', '🨅', '🨅'],
        ['🨂', '🨃', '🨄', '🨁', '🨀', '🨄', '🨃', '🨂'],
    ];
    
    let mut choosen_piece = String::new();
    let mut choosen_position = String::new();
    let mut won = false;
    let mut converted_choosen_piece = Position{x: 0, y: 0};
    let mut converted_choosen_position = Position{x: 0, y: 0};
    
    while !won {
        print_board(&board);

        println!("What Piece do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_piece);
        converted_choosen_piece.new(&choosen_piece);

        println!("To where do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_position);
        converted_choosen_position.new(&choosen_position);

        println!("{} {}", converted_choosen_piece.x   , converted_choosen_piece.y);
        println!("{} {}", converted_choosen_position.x, converted_choosen_position.y);

        board[converted_choosen_position.x][converted_choosen_position.y] = board[converted_choosen_piece.x][converted_choosen_piece.y];
        board[converted_choosen_piece.x][converted_choosen_piece.y]       = board_whithout_pieces[converted_choosen_piece.x][converted_choosen_piece.y];
    }
}
