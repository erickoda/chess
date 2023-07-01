mod board;
use board::board::*;

mod pieces;
use pieces::pieces::*;

const WHITE_SQUARE: char = '⚈';
const BLACK_SQUARE: char = '⚆';

fn verify_new_position(converted: &ConvertedPositions) -> bool{
    let piece_is_out_of_board = converted.choosen_piece.x == 8 || converted.choosen_piece.y == 8 || converted.choosen_piece_new_position.x == 8 || converted.choosen_piece_new_position.y == 8;
    if  piece_is_out_of_board {
        println!("Out of the board!!!");
        return false;
    }

    let piece_is_in_same_position = converted.choosen_piece.x == converted.choosen_piece_new_position.x && converted.choosen_piece.y == converted.choosen_piece_new_position.y;
    if  piece_is_in_same_position {
        println!("You can't move to the same position!!!");
        return false;
    }
    true
}

fn main() {
    let     won             = false;
    let mut is_white_turn   = true;
    let mut board = [
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
    ];

    let board_without_pieces = board.clone();

    let mut white_pieces = Pieces::setup(ColorOfPiece::White);
    let mut black_pieces = Pieces::setup(ColorOfPiece::Black);
    
    let mut choosen_piece    = String::with_capacity(2);
    let mut choosen_position = String::with_capacity(2);
    let mut converted = ConvertedPositions::default();

    
    setup_board(&mut board, &white_pieces, &black_pieces);

    while !won {
        print_board(&mut board);

        println!("What Piece do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_piece);
        converted.choosen_piece.convert(&choosen_piece);

        println!("To where do you want to move? (ex: a1 or A1) ");
        get_position_of(&mut choosen_position);
        
        converted.choosen_piece_new_position.convert(&choosen_position);

        let next_position_is_valid = verify_new_position(&converted);
        if !next_position_is_valid{
            continue;
        }

        println!("{}", board[converted.choosen_piece.x][converted.choosen_piece.y]);

        match is_white_turn{
            true  => white_pieces.move_piece(&converted),
            false => black_pieces.move_piece(&converted),
        }

        update_board(&mut board, &board_without_pieces, &white_pieces, &black_pieces);

        if is_white_turn{
            is_white_turn = false;
        } else {
            is_white_turn = true;
        }
    }
}
