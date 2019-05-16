use std::fs::{read_dir,DirEntry,read_to_string};
use std::path::{PathBuf};
use std::env;
use std::io::{Error};
use rayon::prelude::*;

fn check_dir(mut unwrapped_path: PathBuf, found_at_sign: bool) {
  println!("Reading directory {}", unwrapped_path.display());
  if !found_at_sign {
    let last = unwrapped_path.components().last();
    match last {
      Some(c) => {
        let name_end = c.as_os_str().to_string_lossy().into_owned();
        let index = name_end.find("@");
        match index {
          Some(_) => {
            check_dir(unwrapped_path.clone(), true);
          },
          None => {}
        };
      },
      None => {}
    };
  }
  unwrapped_path.push("package.json");
  println!("Reading File {}", unwrapped_path.display());
  let contents = read_to_string(unwrapped_path.as_path());
  match contents {
    Ok(v) => println!("file is ok"),
    Err(_) => {}
  }
}

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
    let collected: Vec<Result<DirEntry, Error>> = paths.collect();
    let parrallel_iterator = collected.into_par_iter();
    parrallel_iterator.for_each(|path| check_dir(path.unwrap().path(), false));

}
