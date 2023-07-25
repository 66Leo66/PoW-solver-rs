use ring::digest::{Context, SHA256};
use std::env;
use std::time::Instant;
use byteorder::{ByteOrder, BigEndian};
use rayon::prelude::*;

/// This function returns an integer, for which the decimal representation as a string is the solution to the PoW:
///  binstr(sha256(prefix + str(i))).startswith("0"*difficulty)
/// @arg prefix The prefix for hashing
/// @arg difficulty The number of leading 0 bits wanted
/// @ret The solution to the PoW.
fn solve(prefix: &str, difficulty: u32) -> u64 {
    let mut i: u64 = 0;
    let mut found = false;
    let chunk_size: u64 = 1_000_000; // define a suitable chunk size

    while !found {
        let result = (i..i+chunk_size).into_par_iter().find_any(|&i| {
            let mut context = Context::new(&SHA256);
            let data = format!("{}{}", prefix, i);
            context.update(data.as_bytes());
            let digest = context.finish();
            let leading_zeros = BigEndian::read_u64(&digest.as_ref()[..8]).leading_zeros();
            leading_zeros >= difficulty
        });

        match result {
            Some(val) => {
                i = val;
                found = true;
            },
            None => {
                i += chunk_size;
            },
        }
    }
    i
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <prefix> <difficulty>", args[0]);
        return;
    }
    let difficulty = match args[2].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Difficulty is not a valid integer: {}", args[2]);
            return;
        }
    };
    let start = Instant::now();
    
    let i = solve(&args[1], difficulty);
    println!("Answer is {}", i);
    
    let duration = start.elapsed();
    println!("Time elapsed in solve() is: {:?}", duration);

}
