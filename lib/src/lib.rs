use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
}

impl File {
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
}

impl Json for File {}

pub trait Json {
    fn json_encode(&self) -> Vec<u8> where Self: Serialize {
        let result = serde_json::to_vec(&self);
        match result {
            Ok(result) => result,
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}