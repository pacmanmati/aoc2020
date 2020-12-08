use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// use std::error;

// TODO: handle both types of error

pub fn solve(filepath: &str) -> Result<u32> {
    let file = File::open(filepath)?;
    let file = BufReader::new(file);

    let mut vec = Vec::new();
    
    for line in file.lines() {
	vec.push(line?.parse::<u32>()?);
    }

    let mut result: Option<u32> = None;
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
