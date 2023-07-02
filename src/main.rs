mod board;
use board::board::*;

mod pieces;
use pieces::pieces::*;

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
    
    let mut choosen_piece_ptr: &mut Position;
    let mut white_pieces = Pieces::setup(ColorOfPiece::White);
    let mut black_pieces = Pieces::setup(ColorOfPiece::Black);

    
    let mut choosen_piece    = String::with_capacity(2);
    let mut choosen_position = String::with_capacity(2);
    let mut converted = ConvertedPositions::default();
    
    add_pieces_to_board(&mut board, &white_pieces, &black_pieces);

    while !won {
        print_board(&mut board);

        println!("What Piece do you want to move? (ex: a1 or A1)");
        get_position_of(&mut choosen_piece);
        
        println!("To where do you want to move? (ex: a1 or A1)");
        get_position_of(&mut choosen_position);
        
        converted.choosen_piece.convert(&choosen_piece);
        converted.choosen_piece_new_position.convert(&choosen_position);

        let next_position_is_valid = verify_new_position(&converted);
        if !next_position_is_valid{
            continue;
        }

        match is_white_turn{
            true => {
                choosen_piece_ptr = match white_pieces.get_choosen_piece_ptr(&converted) {
                    Some(piece) => piece,
                    None => {
                        println!("None white piece selected!!!");
                        continue;
                    },
                };
                move_piece(&mut board, choosen_piece_ptr, &converted.choosen_piece_new_position);
    
                is_white_turn = false;
            },
            false => {
                choosen_piece_ptr = match black_pieces.get_choosen_piece_ptr(&converted) {
                    Some(piece) => piece,
                    None => {
                        println!("None black piece selected!!!");
                        continue;
                    },
                };
                move_piece(&mut board, &mut choosen_piece_ptr, &converted.choosen_piece_new_position);
    
                is_white_turn = true
            },
        }
    }
}
