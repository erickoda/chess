use crate::pieces::pieces::{Pieces, illustration::*};


pub fn setup_board(board: &mut [[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces) {

    //todo refatorar struct Pieces com Enum pra essa poha virar funcoes menores
    //spawn pawn
    for i in 0..white_pieces.pawn.len() {
        board[white_pieces.pawn[i].x][white_pieces.pawn[i].y] =       WHITE_PAWN;
        board[black_pieces.pawn[i].x][black_pieces.pawn[i].y] =       BLACK_PAWN;
    }

    //spawn tower
    for i in 0..white_pieces.tower.len() {
        board[white_pieces.tower[i].x][white_pieces.tower[i].y] =    WHITE_TOWER;
        board[black_pieces.tower[i].x][black_pieces.tower[i].y] =    BLACK_TOWER;
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

pub fn update_board(board: &mut [[char; 8]; 8], board_without_pieces: &[[char; 8]; 8], white_pieces: &Pieces, black_pieces: &Pieces){

    for i in 0..board.len(){
        for j in 0..board[i].len(){
            board[i][j] = board_without_pieces[i][j];
        }
    }

    setup_board(board, white_pieces, black_pieces);
}

pub fn print_board(board: &mut [[char; 8]; 8]){
    for i in board{
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}