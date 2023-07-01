use crate::pieces::pieces::{Pieces, Position, ConvertedPositions,illustration::*};

pub const WHITE_SQUARE: char = '⚈';
pub const BLACK_SQUARE: char = '⚆';

pub fn add_pieces_to_board(board: &mut [[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces) {

    //todo refatorar struct Pieces com Enum pra essa poha virar funcoes menores
    //spawn pawn
    for i in 0..white_pieces.pawn.len() {
        board[white_pieces.pawn[i].x][white_pieces.pawn[i].y] =       WHITE_PAWN;
        board[black_pieces.pawn[i].x][black_pieces.pawn[i].y] =       BLACK_PAWN;
    }

    //spawn rook
    for i in 0..white_pieces.rook.len() {
        board[white_pieces.rook[i].x][white_pieces.rook[i].y] =    WHITE_ROOK;
        board[black_pieces.rook[i].x][black_pieces.rook[i].y] =    BLACK_ROOK;
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

pub fn move_piece(board: &mut [[char; 8]; 8], old_piece_position: &mut Position, new_piece_position: &Position) {

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

    board[new_piece_position.x][new_piece_position.y] = board[old_piece_position.x][old_piece_position.y];
    board[old_piece_position.x][old_piece_position.y] = board_without_pieces[old_piece_position.x][old_piece_position.y];

    *old_piece_position = *new_piece_position;
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