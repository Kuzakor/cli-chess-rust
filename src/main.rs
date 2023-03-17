mod board;
use std::io;
fn main() {
    let mut board = board::Board { layout: [[' ';8];8]};
    let mut queen = board::Piece { 
        piece: 'P',
        loc: board::Loc {
            x: 1,
            y: 1,
        },
    };
    let mut horse = board::Piece { 
        piece: 'n',
        loc: board::Loc {
            x: 2,
            y: 0,
        },
    };
    let mut rook = board::Piece { 
        piece: 'R',
        loc: board::Loc {
            x: 2,
            y: 4,
        },
    };
    board.insert_piece(horse.loc, horse.piece);
    board.insert_piece(queen.loc, queen.piece);
    board.insert_piece(rook.loc, rook.piece);
    loop {
    board = user_move(board, &mut queen);
    nice_print(board);
    }
}


fn nice_print(what: board::Board) {
    println!("    0    1    2    3    4    5    6    7");
    for i in 0..8 {
        println!("{} {:?}",i, what.layout[i])
    }

}

fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}

fn user_move(mut board: board::Board, piece: &mut board::Piece) -> board::Board {
    println!("You have chosen {} on location {:?}", piece.piece, piece.loc);
    println!("Highlithing possible moves: ");
    let unfiltered_moves = piece.get_possible_moves();
    let possible_moves = piece.filter_moves(unfiltered_moves, board);
    let board_2 = piece.highlight_moves(possible_moves, board);
    nice_print(board_2);
    loop {
        let unfiltered_moves = piece.get_possible_moves();
        let possible_moves = piece.filter_moves(unfiltered_moves, board);
        let loc = board::Loc {
            //Here what you normally expect to by x is y because its 2d array not a map nor other stuff
            x: {
                println!("Select your move(y): ");
                get_int()
            },
            y: {
                println!("Select your move(x): ");
                get_int()
            },
        };
        if possible_moves.contains(&loc){
            piece.move_piece(loc, &mut board);
            return board;
        }else{
            println!("You have chosen wrong location - try again");
            continue;
      }
    }

}

