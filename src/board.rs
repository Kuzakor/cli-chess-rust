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
    //Function insering a char in desired location
    pub fn insert_piece(&mut self, loc: Loc, piece: char) {
        self.layout[loc.x as usize][loc.y as usize] = piece;
    }
    //Function insering a ' ' in desired location
    pub fn remove_piece(&mut self, loc: Loc) {
        self.layout[loc.x as usize][loc.y as usize] = ' ';
    }
}

//New struct - piece
#[derive(Copy, Clone, Debug)]
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
                if self.loc.x == 5 {
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
                continue;
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
