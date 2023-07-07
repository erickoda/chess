use std::cmp::Ordering;

use crate::pieces::pieces::{Pieces, Position, ConvertedPositions,illustration::*, Piece, TypeOfPiece, ColorOfPiece};

pub const WHITE_SQUARE: char = '⚈';
pub const BLACK_SQUARE: char = '⚆';

pub fn add_pieces_to_board(board: &mut [[char; 8]; 8], pieces: &Pieces) {

    for pawn in pieces.pawn {
        match pawn.color.unwrap() {
            ColorOfPiece::White => board[pawn.position.x][pawn.position.y] = WHITE_PAWN,
            ColorOfPiece::Black => board[pawn.position.x][pawn.position.y] = BLACK_PAWN,
        };
    }

    //spawn rook
    for rook in pieces.rook {
        match rook.color.unwrap() {
            ColorOfPiece::White => board[rook.position.x][rook.position.y] = WHITE_ROOK,
            ColorOfPiece::Black => board[rook.position.x][rook.position.y] = BLACK_ROOK,
        };
    }

    //spawn bishop
    for bishop in pieces.bishop {
        match bishop.color.unwrap() {
            ColorOfPiece::White => board[bishop.position.x][bishop.position.y] = WHITE_BISHOP,
            ColorOfPiece::Black => board[bishop.position.x][bishop.position.y] = BLACK_BISHOP,
        };
    }

    //spawn knight
    for knight in pieces.knight {
        match knight.color.unwrap() {
            ColorOfPiece::White => board[knight.position.x][knight.position.y] = WHITE_KNIGHT,
            ColorOfPiece::Black => board[knight.position.x][knight.position.y] = BLACK_KNIGHT,
        };
    }

    //spawn queen
    board[pieces.queen[0].position.x][pieces.queen[0].position.y] = WHITE_QUEEN;
    board[pieces.queen[0].position.x][pieces.queen[0].position.y] = BLACK_QUEEN;
    //spawn king
    board[pieces.king[0].position.x][pieces.king[0].position.y] = WHITE_KING;
    board[pieces.king[0].position.x][pieces.king[0].position.y] = BLACK_KING;
}

pub fn move_piece(board: &mut [[char; 8]; 8], piece: &mut Piece, new_piece_position: &Position) {

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
        0 => match piece.position.x % 2 {
            0 => BLACK_SQUARE,
            _ => WHITE_SQUARE,
        },
        _ => match piece.position.x % 2 {
            0 => WHITE_SQUARE,
            _ => BLACK_SQUARE,
        }
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


pub fn has_piece_in_path(board: &[[char; 8]; 8], piece: &Piece, new_piece_position: &Position) -> bool{

    let lines_iter = match piece.position.x.cmp(&new_piece_position.x) {
        Ordering::Greater  => Some(new_piece_position.x..piece.position.x),
        Ordering::Less     => Some(piece.position.x..new_piece_position.x),
        Ordering::Equal    => None,
    };

    let column_iter = match piece.position.y.cmp(&new_piece_position.x) {
        Ordering::Greater => Some(new_piece_position.y..piece.position.y),
        Ordering::Less    => Some(piece.position.y..new_piece_position.y),
        Ordering::Equal   => None,
    };

    dbg!(&lines_iter);
    dbg!(&column_iter);

    match lines_iter {
        Some(_) => for line in lines_iter.unwrap(){
                        match column_iter {
                            Some(_) => for column in column_iter.clone().unwrap() {
                                            if dbg!(board[line][column] != WHITE_SQUARE && board[line][column] != BLACK_SQUARE) {
                                                return true;
                                            } 
                                    },
                
                            None => if dbg!(board[line][piece.position.y] != WHITE_SQUARE && board[line][piece.position.y] != BLACK_SQUARE) {
                                        return true;
                                    } 
                        }
                    }
        None => for column in column_iter.unwrap() {
                    if board[piece.position.x][column] != WHITE_SQUARE && board[piece.position.x][column] != BLACK_SQUARE{
                        return true;
                    }
                }
    }

    false    
}