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
        loop {
            println!("Any special move?, 10 for long castle, 20 for short castle, 30 for surrender or select row of the piece you want to select: ");
            let special = get_int();
            //Special moves
            match special {
                10 => match lowercase_now {
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
                20 => match lowercase_now {
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
                30 => {
                    println!("Player has surrended");
                    break;
                }
                //Normal moves
                _ => {
                    let loc = board::Loc {
                        //Here what you normally expect to by x is y because its 2d array not a map nor other stuff
                        x: special,
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
                                    let moved_board = user_move(board, &mut piece, &mut pieces);
                                    match moved_board {
                                        Some(x) => board = x,
                                        None => continue,
                                    }
                                    if piece.piece == 'p' && piece.loc.x == 0 {
                                        pieces[x].piece = {
                                            println!("To what you want to promote: ");
                                            let promotion = get_char();
                                            board.layout[piece.loc.x as usize]
                                                [piece.loc.y as usize] = promotion;
                                            promotion
                                        }
                                    }
                                    if piece.piece == 'P' && piece.loc.x == 7 {
                                        pieces[x].piece = {
                                            println!("To what you want to promote: ");
                                            let promotion = get_char();
                                            board.layout[piece.loc.x as usize]
                                                [piece.loc.y as usize] = promotion;
                                            promotion
                                        }
                                    }
                                    let mut kick_piece =
                                        board::get_piece_from_loc(pieces.clone(), piece.loc);
                                    /*if piece.en_passant_was_made {
                                        match piece.piece.is_ascii_lowercase() {
                                            true => {
                                                kick_piece = board::get_piece_from_loc(
                                                    pieces.clone(),
                                                    board::Loc {
                                                        x: piece.loc.x + 1,
                                                        y: piece.loc.y,
                                                    },
                                                )
                                            }
                                            false => {
                                                kick_piece = board::get_piece_from_loc(
                                                    pieces.clone(),
                                                    board::Loc {
                                                        x: piece.loc.x - 1,
                                                        y: piece.loc.y,
                                                    },
                                                )
                                            }
                                        }
                                        piece.en_passant = 0;
                                    }*/
                                    println!("{:?}", kick_piece);

                                    match kick_piece {
                                        Some(y) => {
                                            pieces.remove(y);
                                            pieces.push(piece);
                                        }
                                        None => pieces.push(piece),
                                    }
                                    lowercase_now = !lowercase_now;
                                    break;
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
}

//User move
fn user_move(
    mut board: board::Board,
    piece: &mut board::Piece,
    pieces: &mut Vec<board::Piece>,
) -> Option<board::Board> {
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
            return None;
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
                if piece.piece == 'P' && loc.x == piece.loc.x + 2 {
                    let offset = [1, -1];
                    for i in offset {
                        let a = board::get_piece_from_loc(
                            pieces.clone(),
                            board::Loc {
                                x: loc.x,
                                y: loc.y + i,
                            },
                        );
                        if a != None {
                            if pieces[a.unwrap()].piece == 'p' {
                                pieces[a.unwrap()].en_passant = i
                            }
                        }
                    }
                }
                if piece.piece == 'p' && loc.x == piece.loc.x - 2 {
                    let offset = [1, -1];
                    for i in offset {
                        let a = board::get_piece_from_loc(
                            pieces.clone(),
                            board::Loc {
                                x: loc.x,
                                y: loc.y + i,
                            },
                        );
                        if a != None {
                            if pieces[a.unwrap()].piece == 'P' {
                                pieces[a.unwrap()].en_passant = i;
                            }
                        }
                    }
                }
                
                
                piece.move_piece(loc, &mut board);
                
                if piece.piece == 'p' && piece.en_passant != 0  && loc == (board::Loc {x: piece.loc.x -1, y: piece.loc.y - piece.en_passant}){
                    piece.en_passant_was_made = true;
                    board.remove_piece(board::Loc { x: piece.loc.x + 1, y: piece.loc.y });
                }
                if piece.piece == 'P' && piece.en_passant != 0  && loc == (board::Loc {x: piece.loc.x +1, y: piece.loc.y - piece.en_passant}){
                    piece.en_passant_was_made = true;
                    board.remove_piece(board::Loc { x: piece.loc.x + 1, y: piece.loc.y });
                }
                
                return Some(board);
            }
            false => {
                println!("You have chosen wrong location - try again");
                continue;
            }
        }
    }
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

fn get_char() -> char {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: char = num.trim().parse().expect("convert error");
    num
}
