use serde_json::{from_str,Result, Value};
use std::sync::{Arc,RwLock};

pub struct LicenseEntry {
  pub name: String,
  pub license: String,
  pub url: String
}

pub type LockedLicenseEntryList = Arc<RwLock<Vec<LicenseEntry>>>;

pub fn get_repo(v: Value) -> String {
  if v.is_string() {
    return String::from(v.as_str().unwrap())
  }
  if v.is_object() {
    let o = v.as_object().unwrap();
    let url = &o["url"];
    if url.is_string() {
      return String::from(url.as_str().unwrap())
    }
  }
  return String::new();
}

pub fn process_pkg_json(list: LockedLicenseEntryList, name: String, content: String) {
  let v: Result<Value> = from_str(&content);
  if v.is_ok() {
    let c = v.unwrap();
    let license_type = &c["license"];
    if license_type.is_string() {
      let license_type_str = license_type.as_str().unwrap();
      let lock = list.write();
      if lock.is_ok() {
        let mut guard = lock.unwrap();
        guard.push(LicenseEntry {
          name: name.clone(),
          license: String::from(license_type_str),
          url: get_repo(c["repository"].to_owned())
        })
      }
    }
  }
}