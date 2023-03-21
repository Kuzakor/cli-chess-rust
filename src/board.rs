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
    pub fn new(layout: [[char; 8]; 8]) -> Self{
        Board {
            layout,
        }
    }
    //Function insering a char in desired location
    pub fn insert_piece(&mut self, loc: Loc, piece: char) {
        self.layout[loc.x as usize][loc.y as usize] = piece;
    }
    //Function insering a ' ' in desired location
    pub fn remove_piece(&mut self, loc: Loc) {
        self.layout[loc.x as usize][loc.y as usize] = ' ';
    }
    //Funtions checking for king check
    pub fn is_check_of_uppercase(self) -> bool{
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
                    let mut test_layout = self.clone();
                    test_layout.insert_piece(x, i);
                    //println!("{:?}", test_layout);
                    let possible_moves = piece.filter_moves(piece.get_possible_moves(), test_layout);
                    let a = get_piece_loc_from_char(self, i.to_ascii_lowercase());
                    match a {
                        Some(y) => {
                            match possible_moves.contains(&y) {
                                true => return true,
                                false => continue,
                            }
                        }
                        None => continue,
                    }
                    
                }
                false

            }
            None => false,
        }
        
    //Same for lowercase
    }
    pub fn is_check_of_lowercase(self) -> bool{
        let loc = get_piece_loc_from_char(self, 'k');
        match loc {
            Some(x) => {
                let pieces = ['q', 'p', 'r', 'n', 'b'];
                for mut i in pieces {
                    let piece = Piece::new(i, x);
                    let mut test_layout = self.clone();
                    test_layout.insert_piece(x, i);
                    let possible_moves = piece.filter_moves(piece.get_possible_moves(), test_layout);
                    i.make_ascii_uppercase();
                    let a = get_piece_loc_from_char(self, i);
                    match a {
                        Some(y) => {
                            match possible_moves.contains(&y) {
                                true => return true,
                                false => continue,
                            }
                        }
                        None => continue,
                    }
                    
                }
                false

            }
            None => false,
        }
    }
    pub fn is_checkmate_of_lowercase(self) -> bool {
        if self.is_check_of_lowercase() {
            let loc = get_piece_loc_from_char(self, 'k');
            match loc {
                Some(x) => {
                    let test_piece = Piece::new('k', x);
                    let possible_moves = test_piece.filter_moves(test_piece.get_possible_moves(), self);
                    return possible_moves.len() == 0;
                }
                None => return false,
            }
        }
        false
    }
    pub fn is_checkmate_of_uppercase(self) -> bool {
        if self.is_check_of_uppercase() {
            let loc = get_piece_loc_from_char(self, 'K');
            match loc {
                Some(x) => {
                    let test_piece = Piece::new('K', x);
                    let possible_moves = test_piece.filter_moves(test_piece.get_possible_moves(), self);
                    return possible_moves.len() == 0;
                }
                None => return false,
            }
        }
        false
    }
}

//New struct - piece
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece: char,
    pub loc: Loc,
}

impl Piece {
    pub fn new(piece: char, loc: Loc) -> Self {
        Piece {
            piece,
            loc,
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

            //Moves that are outside the board

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
            

            //If piece is pawn removes moving by axis if there is no piece
            let place = layout.layout[moves[move_index].x as usize][moves[move_index].y as usize];

            if (self.piece == 'P' || self.piece == 'p') && (moves[move_index].y != self.loc.y) {
                if place == ' ' {
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
            if self.piece.is_ascii_lowercase() && layout.is_check_of_lowercase() {
                let mut test_board = layout.clone();
                let mut test_piece = self.clone();
                test_piece.move_piece(moves[move_index], &mut test_board);
                if test_board.is_check_of_lowercase() {
                    moves.remove(move_index);
                    comeback += 1;
                    continue;
                } 
            }
            if self.piece.is_ascii_uppercase() && layout.is_check_of_uppercase(){
                let mut test_board = layout.clone();
                let mut test_piece = self.clone();
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

fn get_piece_loc_from_char(layout: Board, piece:char) -> Option<Loc> {
    for i in 0..layout.layout.len(){
        for a in 0..layout.layout[i].len(){
            if layout.layout[i][a] == piece {
                return Some(Loc {
                    x: i as i32,
                    y: a as i32,
                });
            }
        }
        }
        None
    }
    