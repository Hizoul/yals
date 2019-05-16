use std::fs::{read_dir,DirEntry,read_to_string};
use std::path::{PathBuf};
use std::env;
use rayon::prelude::*;
mod parse;

fn check_dir(list: parse::LockedLicenseEntryList, mut unwrapped_path: PathBuf, found_at_sign: bool, prefix: String) {
  let last = unwrapped_path.components().last();
  match last {
    Some(c) => {
      let name_end = c.as_os_str().to_string_lossy().into_owned();
      let name_end_clone = name_end.to_owned();
      if !found_at_sign {
        let index = name_end.find("@");
        match index {
          Some(_) => {
            check_dir(list.clone(), unwrapped_path.clone(), true, name_end);
          },
          None => {}
        };
      }
      unwrapped_path.push("package.json");
      let contents = read_to_string(unwrapped_path.as_path());
      match contents {
        Ok(v) => {
          let mut full_name = String::new();
          if prefix.len() > 0 {
            full_name.push_str(prefix.as_str());
            full_name.push_str("/");
          }
          full_name.push_str(name_end_clone.as_str());
          parse::process_pkg_json(list.clone(), full_name,v)
        },
        Err(_) => {}
      }
    },
    None => {}
  };
}

fn main() {
    let to_scan: String;
    let arg: Vec<String> = env::args().collect();
    if arg.len() > 1 {
      to_scan = arg[1].to_owned();
    } else {
      to_scan = "./".to_owned();
    }
    let paths = read_dir(to_scan).unwrap();
    let collected: Vec<Result<DirEntry, std::io::Error>> = paths.collect();
    let parrallel_iterator = collected.into_par_iter();
    let locked_list: parse::LockedLicenseEntryList = std::sync::Arc::new(std::sync::RwLock::new(Vec::new()));
    parrallel_iterator.for_each(|path| check_dir(locked_list.clone(), path.unwrap().path(), false, "".to_owned()));
    let mut readable_list = locked_list.write().unwrap();
    readable_list.sort_by(|a, b| a.name.cmp(&b.name));
    for e in readable_list.iter() {
      println!("{} licensed under {}. Repository: {}", e.name, e.license, e.url)
    }
}
