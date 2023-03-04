mod board;

fn main() {
    let mut a = board::board { layout: [[' ';7];7]};
    println!("{:#?}", a);
}
