mod chapter_1;
use chapter_1::simple_binary_search;

fn main() {
    let slice = &[1, 2, 3, 4, 5];
    let target = 5;

    println!("{}", simple_binary_search(slice, target).unwrap());
}

