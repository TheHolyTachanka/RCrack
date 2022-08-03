mod algorithms;

use rayon::prelude::*;
use std::fs;
use std::time::Instant;
use clap::Parser;
use std::process::exit;
use colored::Colorize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    dic: String,

    /// Number of times to greet
    #[clap(short, long)]
    hash: String,
}

fn main() {
    let args = Args::parse();

   
    let hash = args.hash;
    let dic = args.dic;

    let dic_str = fs::read_to_string(&dic).unwrap();

    let dic_vec: Vec<&str> = dic_str.split("\n").collect();

    match hash.len() {
        32 => println!("Loaded in MD5!"),
        40 => println!("loaded in SHA-1!"),
        80 => println!("Loaded in RipeMD320!"),
        64 => println!("Loaded in SHA-256!"),
        128 => println!("Loaded in SHA-512!"),
        _ => {
            panic("Hash not supported! Quitting...");
            exit(1);
        }

    }
    let now = Instant::now();
    dic_vec.par_iter().for_each(|key| {
        if crack(key, hash.len(), &hash) {
            anounce(format!("Cracked has {} -> \"{}\" in {}", hash, key, now.elapsed().as_secs()));
              
        }else {

        }
    });

}

fn crack(txt: &str, hash_len: usize, hash: &str) -> bool {
    if algorithms::create_hash(txt, hash_len) == hash {
        true
    }else {
        false
    }
}


fn panic(msg: &str) {
    println!("{}", msg.red());
    exit(1);
}

fn anounce(msg: String) {
    println!("{}", msg.green());
}
