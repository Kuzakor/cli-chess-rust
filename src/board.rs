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
                     x: self.loc.x + 0,
                     y: (self.loc.y + 1),
                    });
                if self.loc.y == 1 {
                    moves.push(Loc {
                        x: self.loc.x + 0,
                        y: (self.loc.y + 2),
                       });
                }
            }
            //Pawn moves
            'p' => {
                moves.push(Loc {
                    x: self.loc.x + 0,
                    y: (self.loc.y - 1),
                   });
               if self.loc.y == 5 {
                   moves.push(Loc {
                       x: self.loc.x + 0,
                       y: (self.loc.y - 2),
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
        moves
    }

    pub fn filter_moves(self, moves: Vec<Loc>, mut layout: Board) -> Vec<Loc>{
        let mut moves = moves;
        #[derive(PartialEq, Debug)]
        enum Sign {
            Plus,
            Minus,
            Zero,
        }
        //Removes invalid moves that are outside the board
        let mut comeback = 0;
        let mut is_block_initialized = false;
        let mut block = (0,0);
        for mut i in 0..moves.len(){
            i = i - comeback;
            
            if moves[i].x < 0 || moves[i].y < 0 || moves[i].x > 7 || moves[i].y > 7 {
                moves.remove(i);
                comeback += 1;
                continue;
            } 
            /*if  ((self.loc.x - moves[i].x) >= 0 && (self.loc.y - moves[i].y) >= 0) && (block[0] >= Some(0) && block[1] >= Some(0)) ||
                ((self.loc.x - moves[i].x) >= 0 && (self.loc.y - moves[i].y) <= 0) && (block[0] >= Some(0) && block[1] <= Some(0)) ||
                ((self.loc.x - moves[i].x) <= 0 && (self.loc.y - moves[i].y) >= 0) && (block[0] <= Some(0) && block[1] >= Some(0)) ||
                ((self.loc.x - moves[i].x) >= 0 && (self.loc.y - moves[i].y) <= 0) && (block[0] >= Some(0) && block[1] <= Some(0))  {
                moves.remove(i);
                comeback += 1;
                continue;
            }*/
           
            //println!("{:?}", block);
            if is_block_initialized {
                let x_sign = {
                    match (self.loc.x - moves[i].x).cmp(&0) {
                        Ordering::Less => Sign::Minus,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Plus
                    }
                };
                let y_sign = {
                    match (self.loc.y - moves[i].y).cmp(&0) {
                        Ordering::Less => Sign::Minus,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Plus
                    }
                };
                let x_sign_block = {
                    match block.0.cmp(&0) {
                        Ordering::Less => Sign::Minus,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Plus
                    }
                };
                let y_sign_block = {
                    match block.1.cmp(&0) {
                        Ordering::Less => Sign::Minus,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Plus
                    }
                };
                println!("{:?}, {:?}, {:?}, {:?}", x_sign, y_sign, x_sign_block, y_sign_block);
                if x_sign == x_sign_block && y_sign == y_sign_block {
                    moves.remove(i);
                    comeback += 1;
                    continue;
                }
            }
            

                    

            //Removes possibility to kick own pieces
            let place = layout.layout[moves[i].x as usize][moves[i].y as usize];
            if place != ' ' {
                block.0 = self.loc.x - moves[i].x;
                block.1 = self.loc.y - moves[i].y;
                is_block_initialized = true;
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
