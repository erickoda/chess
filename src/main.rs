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
    
    let mut chosen_piece_ptr: &mut Piece;
    let mut white_pieces = Pieces::setup(ColorOfPiece::White);
    let mut black_pieces = Pieces::setup(ColorOfPiece::Black);

    
    let mut chosen_piece    = String::with_capacity(2);
    let mut chosen_position = String::with_capacity(2);
    let mut converted = ConvertedPositions::default();
    
    add_pieces_to_board(&mut board, &white_pieces);
    add_pieces_to_board(&mut board, &black_pieces);
    
    while !won {
        print_board(&mut board);

        println!("What Piece do you want to move? (ex: a1 or A1)");
        get_position_of(&mut chosen_piece);
        
        println!("To where do you want to move? (ex: a1 or A1)");
        get_position_of(&mut chosen_position);
        
        converted.chosen_piece.convert(&chosen_piece);
        converted.chosen_piece_new_position.convert(&chosen_position);

        let next_position_is_valid = verify_new_position(&converted);
        if !next_position_is_valid{
            continue;
        }

        match is_white_turn{
            true => {
                chosen_piece_ptr = match white_pieces.get_chosen_piece_ptr(&converted) {
                    Some(piece) => piece,
                    None => {
                        println!("None white piece selected!!!");
                        continue;
                    },
                };

                let can_move = is_move_valid(&mut chosen_piece_ptr, &converted.chosen_piece_new_position);
                if !can_move {
                    println!("INVALID MOVE!");
                    continue;
                }

                let has_piece_in_path = dbg!(has_piece_in_path(&board, &chosen_piece_ptr, &converted.chosen_piece_new_position));
                if  has_piece_in_path {
                    println!("Piece in the path, choose a new position!");
                    continue;
                }

                move_piece(&mut board, &mut chosen_piece_ptr, &converted.chosen_piece_new_position);
                chosen_piece_ptr.position = converted.chosen_piece_new_position;
    
                is_white_turn = false;
            },
            false => {
                chosen_piece_ptr = match black_pieces.get_chosen_piece_ptr(&converted) {
                    Some(piece) => piece,
                    None => {
                        println!("None black piece selected!!!");
                        continue;
                    },
                };

                let valid_move_for_piece = is_move_valid(&mut chosen_piece_ptr, &converted.chosen_piece_new_position);
                if !valid_move_for_piece {
                    println!("INVALID MOVE!");
                    continue;
                }

                let has_piece_in_path = has_piece_in_path(&mut board, &chosen_piece_ptr, &converted.chosen_piece_new_position);
                if  has_piece_in_path {
                    println!("Piece in the path, choose a new position!");
                    continue;
                }
                
                move_piece(&mut board, &mut chosen_piece_ptr, &converted.chosen_piece_new_position);
                chosen_piece_ptr.position = converted.chosen_piece_new_position;
    
                is_white_turn = true
            },
        }
    }
}
