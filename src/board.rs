

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
            //Pawm moves
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
            'B' | 'b' => {
                for i in 1..8 {
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
            'R' | 'r' => 
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
            'Q' | 'q' => 
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
            _ => todo!(),
        }
        moves
    }
    //Funcion creating * in all possible places piece can move
    pub fn filter_and_highlight_moves(self, moves: Vec<Loc>, mut layout: Board) -> Board {
        let mut moves = moves;
        //Removes invalid moves that are outside the board
        let mut comeback = 0;
        for mut i in 0..moves.len(){
            i = i - comeback;
            if moves[i].x < 0 || moves[i].y < 0 || moves[i].x > 7 || moves[i].y > 7 {
                moves.remove(i);
                comeback += 1;
            } 
        }

        //Inserts * on all posible moves
        for i in moves {
            layout.insert_piece(i, '*');
        }
        layout
    }
}
