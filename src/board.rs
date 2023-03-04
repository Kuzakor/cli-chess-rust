
#[derive(Copy, Clone, Debug)]
pub struct board {
   pub layout: [[char;7];7]

}

impl board {
    pub fn is_check() -> bool {
        todo!()
    }
    pub fn move_piece(&mut self, x: i32, y:i32, piece: char) {
        self.layout[x as usize][y as usize] = 'X';
    }
        
}