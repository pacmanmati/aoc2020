use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::fmt;
use std::error;

type MyResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct NoMatch;

impl fmt::Display for NoMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "no match could be found")
    }
}

impl error::Error for NoMatch {} // ?

// TODO: fix the return type
fn read(filepath: &str) -> MyResult<Vec<i32>> {
    // unholy abomination
    BufReader::new(File::open(filepath)?).lines().map(|line| line.unwrap()).map(|line| line.parse::<i32>().unwrap()).collect()
}

pub fn solve1(filepath: &str) -> MyResult<i32> {
    let vec = read(filepath)?;
    // println!("{:?}", vec);
    let mut result: Option<i32> = None;
    for fst in vec.iter() {
	let x = 2020 - fst;
	for scd in vec.iter() {
	    if *scd == x {
		result = Some(fst * scd);
	    }
	}
    }

    match result {
	Some(val) => Ok(val),
	None => Err(()),
    }
}
