use serde_json::{Result, Value};
use std::collections::HashMap;

pub type LicenseList = HashMap<String, std::sync::RwLock<Vec<String>>>;
pub type LockedLicenseList = std::sync::Arc<std::sync::RwLock<LicenseList>>;

pub fn process_pkg_json(list: LockedLicenseList, name: String, content: String) {
  let v: Result<Value> = serde_json::from_str(&content);
  if v.is_ok() {
    let c = v.unwrap();
    let license_type = &c["license"];
    if license_type.is_string() {
      let license_type_str = license_type.as_str().unwrap();
      let lock = list.write();
      if lock.is_ok() {
        let mut guard: std::sync::RwLockWriteGuard<LicenseList> = lock.unwrap();
        if !guard.contains_key(license_type_str) {
          println!("FOUND {}", String::from(license_type_str));
          guard.insert(String::from(license_type_str), std::sync::RwLock::new(Vec::new()));
        }
        let con = guard.get(&String::from(license_type_str));
        if con.is_some() {
          let mut license_vector = con.unwrap().write().unwrap();
          license_vector.push(name.clone());
        }
      }
    }
  }
}