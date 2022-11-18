use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
}
impl File {
    pub fn new(name: String, content: Vec<u8>) -> File {
        File { name, content }
    }
    pub fn from(path: String) -> File {
        let file_name: &OsStr;
        let content: Vec<u8> = match fs::read(&path) {
            Ok(content) => {
                file_name = Path::new(&path).file_name().unwrap();
                content
            }
            Err(err) => { panic!("{}", err.to_string()); }
        };
        File { name: file_name.to_str().unwrap().to_string(), content }
    }

    pub fn json_encode(&self) -> Vec<u8> {
        let result = serde_json::to_vec(&self);
        match result {
            Ok(result) => result,
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub address: String,
    pub file_name: String,
}