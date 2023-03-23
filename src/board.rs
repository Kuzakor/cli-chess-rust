use std::cmp::Ordering;

//New struct - loc
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}
//New struct - board
#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub layout: [[char; 8]; 8],
}

impl Board {
    pub fn new(layout: [[char; 8]; 8]) -> Self {
        Board { layout }
    }
    //Function insering a char in desired location
    pub fn insert_piece(&mut self, loc: Loc, piece: char) {
        self.layout[loc.x as usize][loc.y as usize] = piece;
    }
    //Function insering a ' ' in desired location
    pub fn remove_piece(&mut self, loc: Loc) {
        self.layout[loc.x as usize][loc.y as usize] = ' ';
    }

    pub fn short_castle_uppercase(&mut self, pieces: &mut [Piece]) -> bool {
        if self.layout[0][4] == 'K'
            && self.layout[0][7] == 'R'
            && self.layout[0][5] == ' '
            && self.layout[0][6] == ' '
        {
            let test = {
                let mut test_board = *self;
                test_board.remove_piece(Loc { x: 0, y: 4 });
                test_board.insert_piece(Loc { x: 0, y: 5 }, 'K');
                match test_board.is_check_of_uppercase() {
                    true => false,
                    false => {
                        test_board.remove_piece(Loc { x: 0, y: 5 });
                        test_board.insert_piece(Loc { x: 0, y: 6 }, 'K');
                        match test_board.is_check_of_uppercase() {
                            true => false,
                            false => true,
                        }
                    }
                }
            };
            if test {
                let piece = get_piece_from_loc(pieces.to_owned(), Loc { x: 0, y: 4 });
                let piece_2 = get_piece_from_loc(pieces.to_owned(), Loc { x: 0, y: 7 });
                match piece {
                    Some(x) => match piece_2 {
                        Some(y) => {
                            if !pieces[x].has_moved && !pieces[y].has_moved {
                                pieces[x].move_piece(Loc { x: 0, y: 6 }, self);
                                pieces[y].move_piece(Loc { x: 0, y: 5 }, self);
                            }
                        }
                        None => println!("error"),
                    },
                    None => println!("error"),
                }
            }
        }
        false
    }
    pub fn long_castle_uppercase(&mut self, pieces: &mut [Piece]) -> bool {
        if self.layout[0][4] == 'K'
            && self.layout[0][0] == 'R'
            && self.layout[0][3] == ' '
            && self.layout[0][2] == ' '
            && self.layout[0][1] == ' '
        {
            let test = {
                let mut test_board = *self;
                test_board.remove_piece(Loc { x: 0, y: 4 });
                test_board.insert_piece(Loc { x: 0, y: 3 }, 'K');
                match test_board.is_check_of_uppercase() {
                    true => false,
                    false => {
                        test_board.remove_piece(Loc { x: 0, y: 3 });
                        test_board.insert_piece(Loc { x: 0, y: 2 }, 'K');
                        match test_board.is_check_of_uppercase() {
                            true => false,
                            false => {
                                test_board.remove_piece(Loc { x: 0, y: 2 });
                                test_board.insert_piece(Loc { x: 0, y: 1 }, 'K');
                                match test_board.is_check_of_uppercase() {
                                    true => false,
                                    false => true,
                                }
                            }
                        }
                    }
                }
            };

            if test {
                let piece = get_piece_from_loc(pieces.to_owned(), Loc { x: 0, y: 4 });
                let piece_2 = get_piece_from_loc(pieces.to_owned(), Loc { x: 0, y: 0 });
                match piece {
                    Some(x) => match piece_2 {
                        Some(y) => {
                            if !pieces[x].has_moved && !pieces[y].has_moved {
                                pieces[x].move_piece(Loc { x: 0, y: 2 }, self);
                                pieces[y].move_piece(Loc { x: 0, y: 3 }, self);
                            }
                        }
                        None => println!("error"),
                    },
                    None => println!("error"),
                }
            }
        }
        false
    }

    pub fn short_castle_lowercase(&mut self, pieces: &mut [Piece]) -> bool {
        if self.layout[7][4] == 'k'
            && self.layout[7][7] == 'r'
            && self.layout[7][5] == ' '
            && self.layout[7][6] == ' '
        {
            let test = {
                let mut test_board = *self;
                test_board.remove_piece(Loc { x: 7, y: 4 });
                test_board.insert_piece(Loc { x: 7, y: 5 }, 'k');
                match test_board.is_check_of_lowercase() {
                    true => false,
                    false => {
                        test_board.remove_piece(Loc { x: 7, y: 5 });
                        test_board.insert_piece(Loc { x: 7, y: 6 }, 'k');
                        match test_board.is_check_of_lowercase() {
                            true => false,
                            false => true,
                        }
                    }
                }
            };

            if test {
                let piece = get_piece_from_loc(pieces.to_owned(), Loc { x: 7, y: 4 });
                let piece_2 = get_piece_from_loc(pieces.to_owned(), Loc { x: 7, y: 7 });
                match piece {
                    Some(x) => match piece_2 {
                        Some(y) => {
                            if !pieces[x].has_moved && !pieces[y].has_moved {
                                pieces[x].move_piece(Loc { x: 7, y: 6 }, self);
                                pieces[y].move_piece(Loc { x: 7, y: 5 }, self);
                            }
                        }
                        None => println!("error"),
                    },
                    None => println!("error"),
                }
            }
        }
        false
    }
    pub fn long_castle_lowercase(&mut self, pieces: &mut [Piece]) -> bool {
        if self.layout[7][4] == 'k'
            && self.layout[7][0] == 'r'
            && self.layout[7][3] == ' '
            && self.layout[7][2] == ' '
            && self.layout[7][1] == ' '
        {
            let test = {
                let mut test_board = *self;
                test_board.remove_piece(Loc { x: 7, y: 4 });
                test_board.insert_piece(Loc { x: 7, y: 3 }, 'k');
                match test_board.is_check_of_lowercase() {
                    true => false,
                    false => {
                        test_board.remove_piece(Loc { x: 7, y: 3 });
                        test_board.insert_piece(Loc { x: 7, y: 2 }, 'k');
                        match test_board.is_check_of_lowercase() {
                            true => false,
                            false => {
                                test_board.remove_piece(Loc { x: 7, y: 2 });
                                test_board.insert_piece(Loc { x: 7, y: 1 }, 'k');
                                match test_board.is_check_of_lowercase() {
                                    true => false,
                                    false => true,
                                }
                            }
                        }
                    }
                }
            };
            if test {
                let piece = get_piece_from_loc(pieces.to_owned(), Loc { x: 7, y: 4 });
                let piece_2 = get_piece_from_loc(pieces.to_owned(), Loc { x: 7, y: 0 });
                match piece {
                    Some(x) => match piece_2 {
                        Some(y) => {
                            if !pieces[x].has_moved && !pieces[y].has_moved {
                                pieces[x].move_piece(Loc { x: 7, y: 2 }, self);
                                pieces[y].move_piece(Loc { x: 7, y: 3 }, self);
                            }
                        }
                        None => println!("error"),
                    },
                    None => println!("error"),
                }
            }
        }
        false
    }

    //Funtions checking for king check
    pub fn is_check_of_uppercase(self) -> bool {
        //Gets localization of uppercae King.
        let loc = get_piece_loc_from_char(self, 'K');
        match loc {
            Some(x) => {
                /* Puts every possible type of piece in the place of king on testing board
                if there is the same type of piece but the oponent's in possible moves of putted piece
                ,it means that there is a check. */
                let pieces = ['Q', 'P', 'R', 'N', 'B'];
                for i in pieces {
                    let piece = Piece::new(i, x);
                    let mut test_layout = self;
                    test_layout.insert_piece(x, i);
                    //println!("{:?}", test_layout);
                    let possible_moves =
                        piece.filter_moves(piece.get_possible_moves(), test_layout);
                    let a = get_piece_loc_from_char(self, i.to_ascii_lowercase());
                    match a {
                        Some(y) => match possible_moves.contains(&y) {
                            true => return true,
                            false => continue,
                        },
                        None => continue,
                    }
                }
                false
            }
            None => false,
        }

        //Same for lowercase
    }
    pub fn is_check_of_lowercase(self) -> bool {
        let loc = get_piece_loc_from_char(self, 'k');
        match loc {
            Some(x) => {
                let pieces = ['q', 'p', 'r', 'n', 'b'];
                for mut i in pieces {
                    let piece = Piece::new(i, x);
                    let mut test_layout = self;
                    test_layout.insert_piece(x, i);
                    let possible_moves =
                        piece.filter_moves(piece.get_possible_moves(), test_layout);
                    i.make_ascii_uppercase();
                    let a = get_piece_loc_from_char(self, i);
                    match a {
                        Some(y) => match possible_moves.contains(&y) {
                            true => return true,
                            false => continue,
                        },
                        None => continue,
                    }
                }
                false
            }
            None => false,
        }
    }
}

//New struct - piece
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece: char,
    pub loc: Loc,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece: char, loc: Loc) -> Self {
        Piece {
            piece,
            loc,
            has_moved: false,
        }
    }
    //Funtion removing current piece and adding a new one in desired location, so basically moving a piece.
    pub fn move_piece(&mut self, loc: Loc, layout: &mut Board) {
        layout.remove_piece(Loc {
            x: self.loc.x,
            y: self.loc.y,
        });
        layout.insert_piece(Loc { x: loc.x, y: loc.y }, self.piece);
        //changing piece loc to new one
        self.loc.x = loc.x;
        self.loc.y = loc.y;
        self.has_moved = true;
    }
    //Function getting all posible moves including invalid ones
    pub fn get_possible_moves(self) -> Vec<Loc> {
        let mut moves: Vec<Loc> = Vec::new();
        //Diffrent move depending on piece
        match self.piece {
            //King moves
            //Declaring array with values to add to the index instead of doing it manulally:
            /*moves.push(loc{
                x: self.loc.x + 0
                y: self.loc.x + 1
            }
            etc.
            //Doing this for every piece
            )*/
            'K' | 'k' => {
                let change = [0, 1, 1, 1, 1, 0, 1, -1, 0, -1, -1, -1, -1, 0, -1, 1];
                for index in (0..change.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + change[index],
                        y: self.loc.y + change[index + 1],
                    });
                }
            }
            //Knight moves
            'N' | 'n' => {
                let change = [-1, 2, 1, 2, 2, 1, 2, -1, 1, -2, -1, -2, -2, -1, -2, 1];
                for index in (0..change.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + change[index],
                        y: self.loc.y + change[index + 1],
                    });
                }
            }
            //Pawm moves
            'P' => {
                let change = [1, 0, 1, 1, 1, -1];
                for index in (0..change.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + change[index],
                        y: self.loc.y + change[index + 1],
                    })
                }
                if self.loc.x == 1 {
                    moves.push(Loc {
                        x: self.loc.x + 2,
                        y: self.loc.y,
                    });
                }
            }
            //Pawn moves
            'p' => {
                let change = [-1, 0, -1, -1, -1, 1];
                for index in (0..change.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + change[index],
                        y: self.loc.y + change[index + 1],
                    })
                }
                if self.loc.x == 6 {
                    moves.push(Loc {
                        x: self.loc.x - 2,
                        y: self.loc.y,
                    });
                }
            }
            //Bishop moves
            'B' | 'b' => {
                for i in 1..8 {
                    let change = [i, i, -i, -i, i, -i, -i, i];
                    for index in (0..change.len()).step_by(2) {
                        moves.push(Loc {
                            x: self.loc.x + change[index],
                            y: self.loc.y + change[index + 1],
                        });
                    }
                }
            }
            //Rook moves
            'R' | 'r' => {
                for i in 1..8 {
                    let change = [0, i, i, 0, 0, -i, -i, 0];
                    for index in (0..change.len()).step_by(2) {
                        moves.push(Loc {
                            x: self.loc.x + change[index],
                            y: self.loc.y + change[index + 1],
                        });
                    }
                }
            }
            //Queen Moves
            'Q' | 'q' => {
                for i in 1..64 {
                    let change = [i, i, -i, -i, i, -i, -i, i, 0, i, i, 0, 0, -i, -i, 0];
                    for index in (0..change.len()).step_by(2) {
                        moves.push(Loc {
                            x: self.loc.x + change[index],
                            y: self.loc.y + change[index + 1],
                        });
                    }
                }
            }
            _ => todo!(),
        }
        moves
    }

    pub fn filter_moves(self, moves: Vec<Loc>, layout: Board) -> Vec<Loc> {
        let mut moves = moves;
        //Removes invalid moves

        let mut comeback = 0;
        let mut block: Vec<i32> = Vec::new();
        for mut move_index in 0..moves.len() {
            //Comeback is needed 'cause when removing value, indexes are changing.
            let mut need_contunue = false;
            move_index -= comeback;

            //Removes moves that are outside the board

            if moves[move_index].x < 0
                || moves[move_index].y < 0
                || moves[move_index].x > 7
                || moves[move_index].y > 7
            {
                moves.remove(move_index);
                comeback += 1;
                continue;
            }

            if self.piece != 'N' && self.piece != 'n' {
                /*
                Cheks sign of self.loc - moves[i] and the same for blocking piece.
                If blocking piece has the same signs as the move for eg. --, 0- ++ etc,
                it means that the move is "behind" the blocking piece so it removes that move.
                For more in-depth in exmplanation reach me on reddit : u/Kuzakor.
                */
                let x_sign = sign_check(self.loc.x - moves[move_index].x);
                let y_sign = sign_check(self.loc.y - moves[move_index].y);

                //Checks signs of every piece that may be blockig the move and compares them with signs of the move
                for block_localization in (0..block.len()).step_by(2) {
                    let x_sign_block = sign_check(block[block_localization]);
                    let y_sign_block = sign_check(block[block_localization + 1]);
                    if x_sign == x_sign_block && y_sign == y_sign_block {
                        moves.remove(move_index);
                        comeback += 1;
                        need_contunue = true;
                    }
                }
                //resets the for loop if move was removed
                if need_contunue {
                    continue;
                }
            }

            let place = layout.layout[moves[move_index].x as usize][moves[move_index].y as usize];

            if self.piece == 'P' || self.piece == 'p' {
                //If piece is pawn removes kckng place forward
                if (moves[move_index].y == self.loc.y && moves[move_index].x != self.loc.x)
                    && place != ' '
                {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                }
                //If piece is pawn removes moving by axis if there is no piece
                if (moves[move_index].y != self.loc.y) && place == ' ' {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                }
            }
            //Removes possibility to kick own pieces

            if place != ' ' {
                block.push(self.loc.x - moves[move_index].x);
                block.push(self.loc.y - moves[move_index].y);
                if (place.is_ascii_lowercase() && self.piece.is_ascii_lowercase())
                    || (place.is_ascii_uppercase() && self.piece.is_ascii_uppercase())
                {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                }
            }
            /*Prevents possibity to move when its check, can move to prevent check though
            Creates test piece the same as the selected piece and test board, checks if move is blocking current
            by moving the test piece to the location that is being currnetly being checked. When there is still check, removes the move            */
            if self.piece.is_ascii_lowercase() && layout.is_check_of_lowercase() {
                let mut test_board = layout;
                let mut test_piece = self;
                test_piece.move_piece(moves[move_index], &mut test_board);
                if test_board.is_check_of_lowercase() {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                }
            }
            if self.piece.is_ascii_uppercase() && layout.is_check_of_uppercase() {
                let mut test_board = layout;
                let mut test_piece = self;
                test_piece.move_piece(moves[move_index], &mut test_board);

                if test_board.is_check_of_uppercase() {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                }
            }
        }
        moves
    }

    //Funcion creating * in all possible places piece can move
    pub fn highlight_moves(self, moves: Vec<Loc>, mut layout: Board) -> Board {
        //Inserts * on all posible moves
        for i in moves {
            layout.insert_piece(i, '*');
        }
        layout
    }
}

//New type of variable = Sign
#[derive(PartialEq, Debug)]
enum Sign {
    Plus,
    Minus,
    Zero,
}

//Reurns sign of given number (Minus, Zero, One)
fn sign_check(number: i32) -> Sign {
    match (number).cmp(&0) {
        Ordering::Less => Sign::Minus,
        Ordering::Equal => Sign::Zero,
        Ordering::Greater => Sign::Plus,
    }
}

//Funtion getting piece location based on this char.
fn get_piece_loc_from_char(layout: Board, piece: char) -> Option<Loc> {
    for x in 0..layout.layout.len() {
        for y in 0..layout.layout[x].len() {
            if layout.layout[x][y] == piece {
                return Some(Loc {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    None
}

//Returns piece index in pieces vector based on its location.
pub fn get_piece_from_loc(pieces_vec: Vec<Piece>, loc: Loc) -> Option<usize> {
    (0..pieces_vec.len()).find(|&i| pieces_vec[i].loc == loc)
}
