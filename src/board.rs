use std::{cmp::Ordering};

//New struct - loc
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}
//New struct - board
#[derive(Copy, Clone, Debug)]
pub struct Board{
    pub layout: [[char; 8]; 8]

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
            'K' | 'k' => {
                //Declaring array with values to add to the i instead of doing it manulally:
                /*moves.push(loc{
                    x: self.loc.x + 0
                    y: self.loc.x + 1
                }
                etc.
                )*/
                let a = [0, 1, 1, 1, 1, 0, 1, -1, 0, -1, -1, -1, -1, 0, -1, 1];
                for i in (0..a.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + a[i],
                        y: self.loc.y + a[i + 1],
                    });
                }
            }
            //Knight moves
            'N' | 'n' => {
                let a = [-1,2,1,2,2,1,2,-1,1,-2,-1,-2,-2,-1,-2,1];
                for i in (0..a.len()).step_by(2) {
                    moves.push(Loc {
                        x: self.loc.x + a[i],
                        y: self.loc.y + a[i + 1],
                    });
                }
            } 
            //Pawm moves
            'P' => {
                moves.push(Loc {
                     x: self.loc.x + 1,
                     y: self.loc.y,
                    });
                moves.push(Loc {
                    x: self.loc.x + 1,
                    y: self.loc.y + 1,
                });
                moves.push(Loc {
                    x: self.loc.x + 1,
                    y: self.loc.y - 1,
                });
                if self.loc.x == 1 {
                    moves.push(Loc {
                        x: self.loc.x + 2,
                        y: self.loc.y,
                       });
                }
                
            }
            //Pawn moves
            'p' => {
                moves.push(Loc {
                    x: self.loc.x - 1,
                    y: self.loc.y,
                   });
                moves.push(Loc {
                    x: self.loc.x - 1,
                    y: self.loc.x - 1,
                });
                moves.push(Loc {
                    x: self.loc.x - 1,
                    y: self.loc.x + 1,
                });
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
                    let a = [i, i, -i, -i, i, -i, -i, i];
                    for b in (0..a.len()).step_by(2){
                        moves.push(Loc{
                            x: self.loc.x + a[b],
                            y: self.loc.y + a[b + 1],
                    });
                    }
                    
                }
            }
            //Rook moves
            'R' | 'r' => {
                for i in 1..8 {
                    moves.push(Loc{
                        x: self.loc.x,
                        y: self.loc.y + i,
                    });
                    moves.push(Loc{
                        x: self.loc.x + i,
                        y: self.loc.y,
                    });
                    moves.push(Loc{
                        x: self.loc.x,
                        y: self.loc.y - i,
                    });
                    moves.push(Loc{
                        x: self.loc.x - i,
                        y: self.loc.y,
                    });
                }
            }
            //Queen Moves
            'Q' | 'q' => {
            for i in 1..8 {
                moves.push(Loc{
                    x: self.loc.x,
                    y: self.loc.y + i,
                });
                moves.push(Loc{
                    x: self.loc.x + i,
                    y: self.loc.y,
                });
                moves.push(Loc{
                    x: self.loc.x,
                    y: self.loc.y - i,
                });
                moves.push(Loc{
                    x: self.loc.x - i,
                    y: self.loc.y,
                });
                moves.push(Loc{
                    x: self.loc.x + i,
                    y: self.loc.y + i,
                });
                moves.push(Loc{
                    x: self.loc.x - i,
                    y: self.loc.y - i,
                });
                moves.push(Loc{
                    x: self.loc.x + i,
                    y: self.loc.y - i,
                });
                moves.push(Loc{
                    x: self.loc.x - i,
                    y: self.loc.y + i,
                });
            }
        }
            _ => todo!(),
        }
        println!("{:?}", moves);
        moves
    }

    pub fn filter_moves(self, moves: Vec<Loc>, mut layout: Board) -> Vec<Loc>{
        let mut moves = moves;
        
        //Removes invalid moves 
        
        let mut comeback = 0;
        let mut block:Vec<i32> = Vec::new();
        for mut i in 0..moves.len(){

            //Comeback is needed 'cause when removing value, indexes are changing
            let mut need_contunue = false;
            i = i - comeback;
            
            //Moves that are outside the board

            if moves[i].x < 0 || moves[i].y < 0 || moves[i].x > 7 || moves[i].y > 7 {
                moves.remove(i);
                comeback += 1;
                continue;
            } 

            /*
            Cheks sign of self.loc - moves[i] and the same for blocking piece.
            If blocking piece has the same signs as the move for eg. --, 0- ++ etc,
            it means that the move is "behind" the blocking piece so it removes that move.
            For more in-depth in exmplanation reach me on reddit : u/Kuzakor.
             */

            let x_sign = sign_check(self.loc.x - moves[i].x);
            let y_sign = sign_check(self.loc.y - moves[i].y);

            for block_loc in (0..block.len()).step_by(2){
                let x_sign_block = sign_check(block[block_loc]);
                let y_sign_block = sign_check(block[block_loc + 1]);
                if x_sign == x_sign_block && y_sign == y_sign_block {
                    moves.remove(i);
                    comeback += 1;
                    need_contunue = true;
                }
            }
            //resets the for loop if move was removed
            if need_contunue {continue;}
            

            //If piece is pawn removes moving by axis if there is no piece
            let place = layout.layout[moves[i].x as usize][moves[i].y as usize];

            if (self.piece == 'P' || self.piece == 'p') && (moves[i].y != self.loc.y){
                if place == ' ' {
                    moves.remove(i);
                    comeback += 1;
                    continue;
                }
            }      
            //Removes possibility to kick own pieces
            
            if place != ' ' {
                block.push(self.loc.x - moves[i].x);
                block.push(self.loc.y - moves[i].y);
                if (place.is_ascii_lowercase() && self.piece.is_ascii_lowercase()) || (place.is_ascii_uppercase() && self.piece.is_ascii_uppercase()){
                    moves.remove(i);
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


#[derive(PartialEq, Debug)]
        enum Sign {
            Plus,
            Minus,
            Zero,
        }

fn sign_check(number:i32) -> Sign {
    match (number).cmp(&0) {
        Ordering::Less => Sign::Minus,
        Ordering::Equal => Sign::Zero,
        Ordering::Greater => Sign::Plus
     }
}