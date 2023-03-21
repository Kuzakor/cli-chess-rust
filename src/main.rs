mod board;
use std::io;
fn main() {
    let mut board = board::Board::new([[' ';8];8]);
    let mut pieces: Vec<board::Piece> = Vec::new();
    for i in 0..8{
        pieces.push(board::Piece::new('P', board::Loc { x: 1, y: i }));
        pieces.push(board::Piece::new('p', board::Loc { x: 6, y: i }));
    }
    pieces.push(board::Piece::new('R', board::Loc { x: 0, y: 0 }));
    pieces.push(board::Piece::new('R', board::Loc { x: 0, y: 7 }));
    pieces.push(board::Piece::new('N', board::Loc { x: 0, y: 1 }));
    pieces.push(board::Piece::new('N', board::Loc { x: 0, y: 6 }));
    pieces.push(board::Piece::new('B', board::Loc { x: 0, y: 2 }));
    pieces.push(board::Piece::new('B', board::Loc { x: 0, y: 5 }));
    pieces.push(board::Piece::new('Q', board::Loc { x: 0, y: 3 }));
    pieces.push(board::Piece::new('K', board::Loc { x: 0, y: 4 }));
    pieces.push(board::Piece::new('r', board::Loc { x: 7, y: 0 }));
    pieces.push(board::Piece::new('r', board::Loc { x: 7, y: 7 }));
    pieces.push(board::Piece::new('n', board::Loc { x: 7, y: 1 }));
    pieces.push(board::Piece::new('n', board::Loc { x: 7, y: 6 }));
    pieces.push(board::Piece::new('b', board::Loc { x: 7, y: 2 }));
    pieces.push(board::Piece::new('b', board::Loc { x: 4, y: 2 }));
    pieces.push(board::Piece::new('q', board::Loc { x: 1, y: 5 }));
    pieces.push(board::Piece::new('k', board::Loc { x: 7, y: 4 }));
    for i in pieces.iter(){
        board.insert_piece(i.loc, i.piece);
    }
    
    
    
    

    loop{
        nice_print(board);
        if board.is_checkmate_of_uppercase() {
            println!("Its checkmate!, white wins");
            break;
        }
        if board.is_checkmate_of_lowercase() {
            println!("Its checkmate!, black wins");
            break;
        }
        
        
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
        let piece_index = get_piece_from_loc(pieces.clone(), loc);
        match piece_index {
            Some(x) => {
                let mut piece = pieces[x];
                board = user_move(board, &mut piece);
                let kick_piece = get_piece_from_loc(pieces.clone(), piece.loc);
                match kick_piece {
                    Some(y) => {
                        pieces.remove(y);
                        pieces.push(piece);
                }
                    None => pieces.push(piece),
                }
                
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
        if possible_moves.is_empty() {
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
        match possible_moves.contains(&loc) {
            true => {
                piece.move_piece(loc, &mut board);
                return board;
            }
            false => {
                println!("You have chosen wrong location - try again");
                continue;
            }
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
