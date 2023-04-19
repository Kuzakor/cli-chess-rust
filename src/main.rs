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
            //If there are no possible moves, its checkmate and ends the game.
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
            println!("Any special move?, 10 for long castle, 20 for short castle, 30 for surrender or select location of the piece");
            //let special = get_int();
            let mut special = get_string();
            special.pop();
            //Special moves
            match special.as_str() {
                //Long castle for the player depending whose move is now. If succefull, changes the move to second player and reruns the loop
                "10" => match lowercase_now {
                    true => {
                        match board.long_castle_lowercase(&mut pieces) {
                            true => lowercase_now = !lowercase_now,
                            false => println!("Move is not possible"),
                        }
                        nice_print(board);
                        continue;
                    }
                    false => {
                        match board.long_castle_uppercase(&mut pieces) {
                            true => lowercase_now = !lowercase_now,
                            false => println!("Move is not possible"),
                        }
                        nice_print(board);
                        continue;
                    }
                },
                //Same but short castle
                "20" => match lowercase_now {
                    true => {
                        match board.short_castle_lowercase(&mut pieces) {
                            true => lowercase_now = !lowercase_now,
                            false => println!("Move is not possible"),
                        }
                        nice_print(board);
                        continue;
                    }
                    false => {
                        match board.short_castle_uppercase(&mut pieces) {
                            true => lowercase_now = !lowercase_now,
                            false => println!("Move is not possible"),
                        }
                        nice_print(board);
                        continue;
                    }
                },
                //Player surrender
                "30" => {
                    println!("Player has surrended");
                    break;
                }
                //Normal moves
                _ => {
                    //Converts human readable format to program readable format
                    let loc = convert_boardloc_to_loc(special);
                    //Gets the index of the selected piece
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
                                    //Changes the board if move was done correctly
                                    match moved_board {
                                        Some(x) => board = x,
                                        None => continue,
                                    }
                                    //Pawn promotion
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

                                    //Sets the kick_piece to a piece that was in the position the piece was moved to.
                                    let mut kick_piece =
                                        board::get_piece_from_loc(pieces.clone(), piece.loc);
                                    //En passant is a bit diffrent so it selects the piece it should be, so the piece "behind" the moved piece
                                    if piece.en_passant_was_made {
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
                                        //Resets en passant
                                        piece.en_passant = 0;
                                    }

                                    match kick_piece {
                                        //If piece was kicked, removes it from pieces vector. Always pushes the new piece to the array.
                                        Some(y) => {
                                            pieces.remove(y);
                                            pieces.push(piece);
                                        }
                                        None => pieces.push(piece),
                                    }
                                    //Gives the move to second player
                                    lowercase_now = !lowercase_now;
                                    break;
                                }
                                //Informs player that she/he choose not their piece
                                false => println!("Not your piece"),
                            }
                        }
                        //Informs player that she/he choose an empty space.
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
    pieces: &mut [board::Piece],
) -> Option<board::Board> {
    println!(
        "You have chosen {} on location {:?}",
        piece.piece, piece.loc
    );
    println!("Highlithing possible moves: ");
    //Gets the possible moves, filter them and highlights.
    let unfiltered_moves = piece.get_possible_moves();
    let possible_moves = piece.filter_moves(unfiltered_moves, board);

    let board_2 = piece.highlight_moves(possible_moves, board);

    nice_print(board_2);

    loop {
        //Gets the possible moves
        let unfiltered_moves = piece.get_possible_moves();
        let possible_moves = piece.filter_moves(unfiltered_moves, board);
        //Tells the player there is no moves possible
        if possible_moves.is_empty() {
            println!("No legal moves possible");
            return None;
        }
        //Gets the move from player
        let mov = {
            println!("Insert move: ");
            get_string()
        };
        //Convets the move form human-readable format to program-readable format
        let loc = convert_boardloc_to_loc(mov);

        match possible_moves.contains(&loc) {
            //If move is legal
            true => {
                //En passant implementation
                if piece.piece == 'P' && loc.x == piece.loc.x + 2 {
                    let offset = [1, -1];
                    for i in offset {
                        let sel_piece = board::get_piece_from_loc(
                            pieces.to_owned(),
                            board::Loc {
                                x: loc.x,
                                y: loc.y + i,
                            },
                        );
                        if let Some(piece_in) = sel_piece {
                            if pieces[piece_in].piece == 'p' {
                                pieces[piece_in].en_passant = i;
                            }
                        }
                    }
                }
                if piece.piece == 'p' && loc.x == piece.loc.x - 2 {
                    let offset = [1, -1];
                    for i in offset {
                        let sel_piece = board::get_piece_from_loc(
                            pieces.to_owned(),
                            board::Loc {
                                x: loc.x,
                                y: loc.y + i,
                            },
                        );
                        if let Some(piece_in) = sel_piece {
                            if pieces[piece_in].piece == 'P' {
                                pieces[piece_in].en_passant = i;
                            }
                        }
                    }
                }
                //Moves the piece
                piece.move_piece(loc, &mut board);
                //If en passant was made, removes the kicked piece form board, 'cause en passant does not removes it automatically like other moves.
                if piece.piece == 'p' && piece.en_passant != 0 && loc == piece.loc {
                    piece.en_passant_was_made = true;
                    board.remove_piece(board::Loc {
                        x: piece.loc.x + 1,
                        y: piece.loc.y,
                    });
                }
                if piece.piece == 'P' && piece.en_passant != 0 && loc == piece.loc {
                    piece.en_passant_was_made = true;
                    board.remove_piece(board::Loc {
                        x: piece.loc.x + 1,
                        y: piece.loc.y,
                    });
                }

                //Prints and returns the board
                nice_print(board);
                return Some(board);
            }
            //Player have choosen wrong location
            false => {
                println!("You have chosen wrong location - try again");
                continue;
            }
        }
    }
}

//Funtion prinint given Board in human-readable format
fn nice_print(what: board::Board) {
    println!("    A B C D E F G H");
    for i in 0..8 {
        print!(" {} ", i);
        print!("|");
        for a in 0..8 {
            let print_char: &'static str = match what.layout[i][a] {
                'k' => "♚",
                'K' => "♔",
                'r' => "♜",
                'R' => "♖",
                'b' => "♝",
                'B' => "♗",
                'n' => "♞",
                'N' => "♘",
                'q' => "♛",
                'Q' => "♕",
                'p' => "♟︎",
                'P' => "♙",
                '*' => "*",
                _ => " ",
            };
            print!("{}|", print_char);
        }
        println!();
    }
}
//Funtions getting input from user

fn get_char() -> char {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: char = num.trim().parse().expect("convert error");
    num
}

fn get_string() -> String {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    num
}
//Converts human readable format to program readable format
fn convert_boardloc_to_loc(what: String) -> board::Loc {
    let x = what.chars().nth(1).unwrap();
    let x = x as u32 - '0' as u32;
    board::Loc {
        x: x as i32,
        y: {
            match what.as_bytes()[0] as char {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => 8,
            }
        },
    }
}
