use crate::pieces::{pieces::{Pieces, Position, ConvertedPositions,illustration::*, Piece, TypeOfPiece, ColorOfPiece}, self};

pub const WHITE_SQUARE: char = '⚈';
pub const BLACK_SQUARE: char = '⚆';

pub fn add_pieces_to_board(board: &mut [[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces) {

    //todo refatorar struct Pieces com Enum pra essa poha virar funcoes menores
    //spawn pawn
    for i in 0..white_pieces.pawn.len() {
        board[white_pieces.pawn[i].position.x][white_pieces.pawn[i].position.y] =     WHITE_PAWN;
        board[black_pieces.pawn[i].position.x][black_pieces.pawn[i].position.y] =     BLACK_PAWN;
    }

    //spawn rook
    for i in 0..white_pieces.rook.len() {
        board[white_pieces.rook[i].position.x][white_pieces.rook[i].position.y] =     WHITE_ROOK;
        board[black_pieces.rook[i].position.x][black_pieces.rook[i].position.y] =     BLACK_ROOK;
    }

    //spawn bishop
    for i in 0..white_pieces.bishop.len() {
        board[white_pieces.bishop[i].position.x][white_pieces.bishop[i].position.y] = WHITE_BISHOP;
        board[black_pieces.bishop[i].position.x][black_pieces.bishop[i].position.y] = BLACK_BISHOP;
    }

    //spawn horse
    for i in 0..white_pieces.knight.len() {
        board[white_pieces.knight[i].position.x][white_pieces.knight[i].position.y] = WHITE_KNIGHT;
        board[black_pieces.knight[i].position.x][black_pieces.knight[i].position.y] = BLACK_KNIGHT;
    }

    //spawn queen
    board[white_pieces.queen[0].position.x][white_pieces.queen[0].position.y] = WHITE_QUEEN;
    board[black_pieces.queen[0].position.x][black_pieces.queen[0].position.y] = BLACK_QUEEN;
    //spawn king
    board[white_pieces.king[0].position.x][white_pieces.king[0].position.y] = WHITE_KING;
    board[black_pieces.king[0].position.x][black_pieces.king[0].position.y] = BLACK_KING;
}

pub fn move_piece(board: &mut [[char; 8]; 8], piece: &mut Piece, new_piece_position: &Position) {

    let board_without_pieces = [
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
        [WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE],
        [BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE, BLACK_SQUARE, WHITE_SQUARE],
    ];

    board[new_piece_position.x][new_piece_position.y] = match piece.type_of.unwrap() {
        TypeOfPiece::Pawn => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_PAWN
            },
            ColorOfPiece::White => {
                WHITE_PAWN
            },
        },
        TypeOfPiece::Rook => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_ROOK
            },
            ColorOfPiece::White => {
                WHITE_ROOK
            },
        },
        TypeOfPiece::Bishop => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_BISHOP
            },
            ColorOfPiece::White => {
                WHITE_BISHOP
            },
        },
        TypeOfPiece::Knight => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_KNIGHT
            },
            ColorOfPiece::White => {
                WHITE_KNIGHT
            },
        },
        TypeOfPiece::Queen => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_QUEEN
            },
            ColorOfPiece::White => {
                WHITE_QUEEN
            },
        },
        TypeOfPiece::King => match piece.color.unwrap() {
            ColorOfPiece::Black => {
                BLACK_KING
            },
            ColorOfPiece::White => {
                WHITE_KING
            },
        },
    };

    board[piece.position.x][piece.position.y] = match piece.position.x + piece.position.y % 2{
        0 => BLACK_SQUARE,
        _ => WHITE_SQUARE,
    };
}

pub fn print_board(board: &mut [[char; 8]; 8]){
    for i in board{
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

pub fn verify_new_position(converted: &ConvertedPositions) -> bool{
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

pub fn is_move_valid(choosen_piece_ptr: &Piece, new_piece_position: &Position) -> bool {

    match choosen_piece_ptr.type_of.unwrap() {
        TypeOfPiece::Pawn => {
            match choosen_piece_ptr.color.unwrap() {
                ColorOfPiece::Black => {
                    if choosen_piece_ptr.position.x == 1 && new_piece_position.x - choosen_piece_ptr.position.x == 2 && new_piece_position.y == choosen_piece_ptr.position.y {
                        return true;
                    }

                    if new_piece_position.x - choosen_piece_ptr.position.x == 1 && new_piece_position.y == choosen_piece_ptr.position.y {
                        return true;
                    }
                    return false;
                },
                ColorOfPiece::White => {
                    if choosen_piece_ptr.position.x == 6 && choosen_piece_ptr.position.x - new_piece_position.x == 2 && new_piece_position.y == choosen_piece_ptr.position.y {
                        return true;
                    }
                    if choosen_piece_ptr.position.x - new_piece_position.x == 1 && new_piece_position.y == choosen_piece_ptr.position.y {
                        return true;
                    }
                    return false;
                },
            }
        }

        TypeOfPiece::Rook => {
            if choosen_piece_ptr.position.x == new_piece_position.x || choosen_piece_ptr.position.y == new_piece_position.y{
                return true;
            }
            return false;
        }

        TypeOfPiece::Bishop => {
            if (new_piece_position.x as i32 - choosen_piece_ptr.position.x as i32).abs() == (new_piece_position.y as i32 - choosen_piece_ptr.position.y as i32).abs(){
                return true;
            }
            return false;
        }

        TypeOfPiece::Knight => {
            if  (new_piece_position.x as i32 - choosen_piece_ptr.position.x as i32).abs() == 2 && 
                (new_piece_position.y as i32 - choosen_piece_ptr.position.y as i32).abs() == 1 {
                return true;
            }
            if (choosen_piece_ptr.position.x as i32 - new_piece_position.x as i32).abs()  == 1 && (choosen_piece_ptr.position.x as i32 - new_piece_position.x as i32).abs() == 2{
                return true;
            }
            return false;
        }

        TypeOfPiece::Queen => {
            if choosen_piece_ptr.position.x == new_piece_position.x || choosen_piece_ptr.position.y == new_piece_position.y{
                return true;
            }
            if (new_piece_position.x as i32 - choosen_piece_ptr.position.x as i32).abs() == (new_piece_position.y as i32 - choosen_piece_ptr.position.y as i32).abs(){
                return true;
            }
            return false;
        }

        TypeOfPiece::King => {
            if  (choosen_piece_ptr.position.x as i32 - new_piece_position.x as i32).pow(2) + (choosen_piece_ptr.position.y as i32 - new_piece_position.y as i32).pow(2) <= 2 {
                    return true;
                }
            return false;
        }
    }
}