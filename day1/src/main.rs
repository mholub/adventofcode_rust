use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    let mut floor = 0i32;
    for c in s.chars() {
        floor += match c {
            ')' => -1,
            '(' => 1,
            _ => 0
        }
    }
    println!("final floor {}", floor);
}
