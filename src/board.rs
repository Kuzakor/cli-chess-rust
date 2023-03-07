use extend::ext;

//New struct - loc
#[derive(Copy, Clone, Debug)]
pub struct loc {
    pub x: i32,
    pub y: i32,
}
//New struct - board
#[derive(Copy, Clone, Debug)]
pub struct board {
    pub layout: [[char; 7]; 7],
}

impl board {
    pub fn is_check() -> bool {
        todo!()
    }
    //Function insering a char in desired location
    pub fn insert_piece(&mut self, loc: loc, piece: char) {
        self.layout[loc.x as usize][loc.y as usize] = piece;
    }
    //Function insering a ' ' in desired location
    pub fn remove_piece(&mut self, loc: loc) {
        self.layout[loc.x as usize][loc.y as usize] = ' ';
    }
}

//New struct - piece
#[derive(Copy, Clone, Debug)] 
pub struct piece {
    pub piece: char,
    pub loc: loc,
}

impl piece {
    //Funtion removing current piece and adding a new one in desired location, so basically moving a piece.
    pub fn move_piece(&mut self, loc: loc, layout: &mut board) {
        layout.remove_piece(loc {
            x: self.loc.x,
            y: self.loc.y,
        });
        layout.insert_piece(loc { x: loc.x, y: loc.y }, self.piece);
        //changing piece loc to new one
        self.loc.x = loc.x;
        self.loc.y = loc.y;
    }
    //Function getting all posible moves including invalid ones
    pub fn get_possible_moves(self) -> Vec<loc> {
        let mut moves: Vec<loc> = Vec::new();
        //Diffrent move depending on piece
        match self.piece {
            'K' => {
                //Declaring array with values to add to the i instead of doing it manulally:
                /*moves.push(loc{
                    x: self.loc.x + 0
                    y: self.loc.x + 1
                }
                etc.
                )*/
                let a = [0, 1, 1, 1, 1, 0, 1, -1, 0, -1, -1, -1, -1, 0, -1, 1];
                for i in (0..a.len()).step_by(2) {
                    moves.push(loc {
                        x: self.loc.x + a[i],
                        y: self.loc.y + a[i + 1],
                    });
                }
            }
            _ => todo!(),
        }
        moves
    }
    //Funcion creating * in all possible places piece can move
    pub fn filter_and_highlight_moves(self, moves: Vec<loc>, layout: &mut board) {
        let mut moves = moves;
        //Removes invalid moves that are outside the board
        for i in 0..moves.len(){
            if moves[i].x < 0 || moves[i].y < 0 || moves[i].x > 7 || moves[i].y > 7{
                moves.remove(i);
            }
        }
        //Need to add old highlite removal
        //Inserts * on all posible moves
        for i in moves {
            layout.insert_piece(i, '*');
        }
    }
}
