mod board;
use std::io;
fn main() {
    let mut board = board::Board { layout: [[' ';8];8]};
    let mut uppercase_pieces: Vec<board::Piece> = Vec::new();
    for i in 0..8{
        uppercase_pieces.push(board::Piece::new('P', board::Loc { x: 1, y: i }))
    }
    
    uppercase_pieces.push(board::Piece::new('R', board::Loc { x: 0, y: 0 }));
    uppercase_pieces.push(board::Piece::new('R', board::Loc { x: 0, y: 7 }));
    uppercase_pieces.push(board::Piece::new('N', board::Loc { x: 0, y: 1 }));
    uppercase_pieces.push(board::Piece::new('N', board::Loc { x: 0, y: 6 }));
    uppercase_pieces.push(board::Piece::new('B', board::Loc { x: 0, y: 2 }));
    uppercase_pieces.push(board::Piece::new('B', board::Loc { x: 0, y: 5 }));
    uppercase_pieces.push(board::Piece::new('Q', board::Loc { x: 0, y: 3 }));
    uppercase_pieces.push(board::Piece::new('K', board::Loc { x: 0, y: 4 }));

    for i in uppercase_pieces.iter(){
        board.insert_piece(i.loc, i.piece);
    }

    loop{
        nice_print(board);
        println!("Choose piece: ");
        let loc = board::Loc {
            //Here what you normally expect to by x is y because its 2d array not a map nor other stuff
            x: {
                println!("y: ");
                get_int()
            },
            y: {
                println!("x: ");
                get_int()
            },
        };
        let piece_index = get_piece_from_loc(uppercase_pieces.clone(), loc);
        match piece_index {
            Some(x) => {
                board = user_move(board, &mut uppercase_pieces[x]);
            }
            None => {
                println!("No piece there");
                continue;
            }
        };
        
    }
    //loop {
    //board = user_move(board, &mut queen);
    //nice_print(board);
    //}
}

fn get_piece_from_loc(pieces_vec: Vec<board::Piece>, loc:board::Loc) -> Option<usize> {
    for i in 0..pieces_vec.len(){
        if pieces_vec[i].loc == loc {
            return Some(i);
        }
    }
    None
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
        if possible_moves.len() == 0 {
            println!("No legal moves possible");
            break;
        }
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
    board

}

//Funtion prinint given Board in human-readable format
fn nice_print(what: board::Board) {
    println!("    0    1    2    3    4    5    6    7");
    for i in 0..8 {
        println!("{} {:?}",i, what.layout[i])
    }

}
//Funtion getting i32 input from user
fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}
