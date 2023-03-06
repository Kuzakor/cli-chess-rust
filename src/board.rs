
use extend::ext;

#[derive(Copy, Clone, Debug)]
pub struct board {
   pub layout: [[char;7];7],
}

impl board {
    pub fn is_check() -> bool {
        todo!()
    }
    pub fn insert_piece(self, x: i32, y:i32, piece: char) -> [[char;7];7]{
        let mut layout_output = self.layout.clone();
        layout_output[x as usize][y as usize] = piece;
        layout_output
    }
    pub fn remove_piece(&mut self, x: i32, y:i32) -> [[char;7];7] {
        let mut layout_output = self.layout.clone();
        layout_output[x as usize][y as usize] = ' ';
        layout_output
    }
}

#[derive(Copy, Clone, Debug)]
pub struct piece {   
    pub piece: char,
    pub x: i32,
    pub y: i32,
}

impl piece {
    pub fn move_piece(&mut self, x: i32, y: i32, layout:board) -> board{
        let mut layout_output = layout.clone();
        layout_output.layout = layout_output.remove_piece(self.x, self.y);
        layout_output.layout = layout_output.insert_piece(x, y, self.piece);
        self.x = x;
        self.y = y;
        layout_output
    }
    pub fn highlight_possible_moves(self) -> () {

        todo!()

    }
        
    
}