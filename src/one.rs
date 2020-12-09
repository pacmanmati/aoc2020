use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::fmt;
use std::error;

#[derive(Debug)]
struct NoMatch;

impl fmt::Display for NoMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "no match could be found")
    }
}

impl error::Error for NoMatch {}

fn read(filepath: &str) -> Vec<i32> {
    // unholy abomination
    BufReader::new(File::open(filepath).unwrap()).lines().map(|line| line.unwrap()).map(|line| line.parse::<i32>().unwrap()).collect()
}

pub fn solve1(filepath: &str) -> Result<i32, Box<dyn error::Error>> {
    let vec = read(filepath);
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
	None => Err(Box::new(NoMatch)),
    }
}

pub fn solve2(filepath: &str) -> Result<i32, Box<dyn error::Error>> {
    let vec = read(filepath);
    let mut result: Option<i32> = None;
    for fst in vec.iter() {
	let x = 2020 - fst;
	for scd in vec.iter() {
	    let y = x - scd;
	    for thd in vec.iter() {
		if *thd == y {
		    result = Some(fst * scd * thd);
		}
	    }
	}
    }

    match result {
	Some(val) => Ok(val),
	None => Err(Box::new(NoMatch)),
    }
}
