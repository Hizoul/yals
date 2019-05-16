use std::fs::{read_dir,DirEntry};
use std::env;
use std::io::{Error};
use rayon::prelude::*;

fn main() {
    let to_scan: String;
    let arg: Vec<String> = env::args().collect();
    if arg.len() > 1 {
      to_scan = arg[1].to_owned();
    } else {
      to_scan = "./".to_owned();
    }
    println!("Scanning directory: {}", to_scan);
    let paths = read_dir(to_scan).unwrap();
    let mut collected: Vec<Result<DirEntry, Error>> = paths.collect();
    let parrallel_iterator = collected.into_par_iter();
    parrallel_iterator.for_each(|path| {
      println!("HI THIS IS {}", path.unwrap().path().display())
    });

}
