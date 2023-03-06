mod board;

fn main() {
    let mut a = board::board { layout: [[' ';7];7]};
    let mut horse = board::piece { 
        piece: 'H',
        x: 1,
        y: 1,
    };
    nice_print(a);
}


fn nice_print(what: board::board) {
    for i in 0..7 {
        println!("{:?}", what.layout[6-i])
    }

}