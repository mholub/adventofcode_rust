use std::str::FromStr;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[derive(Debug)]
struct Present {
    length : u32,
    width : u32,
    height : u32
}

impl FromStr for Present {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims : Vec<u32> = s.split('x').map(|sv| sv.parse().unwrap_or(0) ).collect();

        if dims.len() == 3 {
            Ok(Present {length:dims[0], width: dims[1], height: dims[2]})
        } else {
            Err("Format should be AxBxC")
        }
    }
}

impl Present {
    fn paper_area(&self) -> u32 {
        let side1 = self.length * self.width;
        let side2 = self.length * self.height;
        let side3 = self.width * self.height;

        let smallest = cmp::min(cmp::min(side1, side2), side3);

        2 * side1 + 2 * side2 + 2 * side3 + smallest
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let b = BufReader::new(f);

    let mut total_area = 0;
    for l in b.lines() {
        let p : Present = l.unwrap().parse().unwrap();
        total_area += p.paper_area();
    }

    println!("Total area {:?}", total_area);
}
