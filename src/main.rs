mod board;
use std::{io, alloc::Layout};
fn main() {
    let mut board = board::board { layout: [[' ';7];7]};
    let mut horse = board::piece { 
        piece: 'N',
        loc: board::loc {
            x: 0,
            y: 0,
        },
    };
    board.insert_piece(horse.loc, horse.piece);
    loop {
    board = user_move(board, &mut horse);
    nice_print(board);
    }
}


fn nice_print(what: board::board) {
    for i in 0..7 {
        println!("{:?}", what.layout[i])
    }

}

fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}

fn user_move(mut board: board::board, mut piece: &mut board::piece) -> board::board {
    println!("You have chosen {} on location {:?}", piece.piece, piece.loc);
    println!("Highlithing possible moves: ");
    let board_2 = piece.filter_and_highlight_moves(piece.get_possible_moves(), board);
    nice_print(board_2);
    let loc = board::loc {
        x: {
            println!("Select your move(x): ");
            get_int()
        },
        y: {
            println!("Select your move(y): ");
            get_int()
        },
    };
    piece.move_piece(loc, &mut board);
    board
}