mod board;
use std::io;
fn main() {
    let mut a = board::board { layout: [[' ';7];7]};
    let mut horse = board::piece { 
        piece: 'K',
        loc: board::loc {
            x: 1,
            y: 1,
        },
    };
    
}


fn nice_print(what: board::board) {
    for i in 0..7 {
        println!("{:?}", what.layout[i])
    }

}

fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}