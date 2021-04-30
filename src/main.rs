extern crate randseqsum;

use std::env;
use std::process;

fn parse_arg(arg: Option<String>) -> Result<i32, &'static str> {
    Ok(match arg {
        Some(arg) => {
            match arg.parse::<i32>() {
                Ok(i) => i,
                Err(_) => return Err("Can't parse arguments to integer")
            }
        },
        None => return Err("Usage: seqsum <nums> <total>")
    })
}

fn main() {
    let mut args = env::args();
    args.next();

    let nums: i32 = match parse_arg(args.next()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let total: i32 = match parse_arg(args.next()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("{:?}", randseqsum::seqsum(nums as usize, total));
}
