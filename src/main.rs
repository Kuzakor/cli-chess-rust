mod board;
use std::io;

fn main() {
    //Declaring main board and vec with all the pieces
    let mut board = board::Board::new([[' '; 8]; 8]);
    let mut pieces: Vec<board::Piece> = Vec::new();
    for i in 0..8 {
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
    pieces.push(board::Piece::new('b', board::Loc { x: 7, y: 5 }));
    pieces.push(board::Piece::new('k', board::Loc { x: 7, y: 4 }));
    pieces.push(board::Piece::new('q', board::Loc { x: 7, y: 3 }));

    for i in pieces.iter() {
        board.insert_piece(i.loc, i.piece);
    }
    let mut lowercase_now = true;
    loop {
        nice_print(board);

        //Checks if there is a checkmate for each player by testing if there is a check AND there is no possiblity to move.
        if board.is_check_of_uppercase() {
            println!("Its check");
            let mut valid_pieces = 0;
            let mut no_moves = 0;
            for i in pieces.clone() {
                if i.piece.is_ascii_uppercase() {
                    valid_pieces += 1;
                    match i.filter_moves(i.get_possible_moves(), board).is_empty() {
                        true => {
                            no_moves += 1;
                            continue;
                        }
                        false => break,
                    }
                }
            }
            if valid_pieces == no_moves {
                println!("its checkmate, white wins");
                break;
            }
        }
        //Same for lowercase
        if board.is_check_of_lowercase() {
            println!("Its check");
            let mut valid_pieces = 0;
            let mut no_moves = 0;
            for i in pieces.clone() {
                if i.piece.is_ascii_lowercase() {
                    valid_pieces += 1;
                    match i.filter_moves(i.get_possible_moves(), board).is_empty() {
                        true => {
                            no_moves += 1;
                            continue;
                        }
                        false => break,
                    }
                }
            }
            if valid_pieces == no_moves {
                println!("its checkmate, black wins");
                break;
            }
        }
        println!("Any special move?, 1 for long castle, 2 for short castle, 3 for surrender other number for normal move");
        let special = get_int();
        //Special moves
        match special {
            1 => match lowercase_now {
                true => {
                    match board.long_castle_lowercase(&mut pieces) {
                        true => lowercase_now = !lowercase_now,
                        false => println!("Move is not possible"),
                    }
                    continue;
                }
                false => {
                    match board.long_castle_uppercase(&mut pieces) {
                        true => lowercase_now = !lowercase_now,
                        false => println!("Move is not possible"),
                    }
                    continue;
                }
            },
            2 => match lowercase_now {
                true => {
                    match board.short_castle_lowercase(&mut pieces) {
                        true => lowercase_now = !lowercase_now,
                        false => println!("Move is not possible"),
                    }
                    continue;
                }
                false => {
                    match board.short_castle_uppercase(&mut pieces) {
                        true => lowercase_now = !lowercase_now,
                        false => println!("Move is not possible"),
                    }
                    continue;
                }
            },
            3 => {
                println!("Player has surrended");
                break;
            }
            //Normal moves
            _ => {
                println!("Choose piece: ");
                let loc = board::Loc {
                    //Here what you normally expect to by x is y because its 2d array not a map nor other stuff
                    x: {
                        println!("row: ");
                        get_int()
                    },
                    y: {
                        println!("collumn: ");
                        get_int()
                    },
                };
                let piece_index = board::get_piece_from_loc(pieces.clone(), loc);
                match piece_index {
                    Some(x) => {
                        let mut piece = pieces[x];
                        //Contiunue only when the piece is owned by current player (both are uppercase or both lowercase)
                        match piece.piece.is_ascii_lowercase() && lowercase_now
                            || piece.piece.is_ascii_uppercase() && !lowercase_now
                        {
                            true => {
                                //Moves the piece by player and kicks piece from Vec on moved location if there is some piece in that location.
                                board = user_move(board, &mut piece);
                                let kick_piece =
                                    board::get_piece_from_loc(pieces.clone(), piece.loc);
                                match kick_piece {
                                    Some(y) => {
                                        pieces.remove(y);
                                        pieces.push(piece);
                                    }
                                    None => pieces.push(piece),
                                }
                                lowercase_now = !lowercase_now;
                            }
                            false => println!("Not your piece"),
                        }
                    }
                    None => {
                        println!("No piece there");
                        continue;
                    }
                };
            }
        }
    }
}

//User move
fn user_move(mut board: board::Board, piece: &mut board::Piece) -> board::Board {
    println!(
        "You have chosen {} on location {:?}",
        piece.piece, piece.loc
    );
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
                println!("Select your move(row): ");
                get_int()
            },
            y: {
                println!("Select your move(collumn): ");
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
        println!("{} {:?}", i, what.layout[i])
    }
}
//Funtion getting i32 input from user
fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}
