extern crate lib;

use std::io::{Read, Write};
use std::net::TcpStream;

use structopt::StructOpt;

use lib::{File, Json};
use util::args;

mod util;

fn main() {
    let args = args::Opt::from_args();
    let file_path = args::file_path(&args.path);

    let (upload_addr, download_addr) = args::host(&args.host);

    match args {
        _ if(args.upload != None) => {
            match TcpStream::connect(upload_addr) {
                Ok(stream) => {
                    upload_file(stream, args.upload.unwrap());
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            };
        },
        _ if(args.download != None) => {
            match TcpStream::connect(download_addr) {
                Ok(mut stream) => {
                    let file_name: String = args.download.unwrap();
                    stream.write(file_name.as_bytes()).unwrap();

                    let mut buf: Vec<u8> = vec![];
                    stream.read_to_end(&mut buf).unwrap();

                    let file: Option<File> = match serde_json::from_slice(&buf) {
                        Ok(file) => Some(file),
                        Err(_) => None
                    };
                    if let Some(file) = file {
                        let mut file_name = String::from(file_path);
                        file_name.push_str(&file.name);
                        println!("{}", file_name);
                        match std::fs::write(file_name, file.content) {
                            Ok(_) => {},
                            Err(_) => {}
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            };
        },
        _ => { }
    };
}

fn upload_file(mut stream: TcpStream, file_name: String) {
    let file: File = File::from(file_name);
    stream.write(&file.json_encode()).unwrap();
}