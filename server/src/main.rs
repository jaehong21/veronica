extern crate lib;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use::serde_json;
use structopt::StructOpt;

use lib::{File, Json};

mod util;

const FILE_NAME_MAX_LEN: usize = 216;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short,long)]
    pub path: String,
}

fn main() {
    let args = Opt::from_args();
    let (upload_file_path, download_file_path) = util::path::exist_dir(args.path);

    let upload_listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let download_listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let upload_thread = thread::spawn(move || {
        for stream in upload_listener.incoming() {
            let file_path = upload_file_path.clone();
            match stream {
                Ok(stream) => {
                    // println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_upload(stream, file_path);
                    });
                },
                Err(e) => { println!("Error: {}", e); }
            }
        }
    });

    let download_thread = thread::spawn(move || {
        for stream in download_listener.incoming() {
            let file_path = download_file_path.clone();
            match stream {
                Ok(stream) => {
                    // println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_download(stream, file_path);
                    });
                },
                Err(e) => { println!("Error: {}", e); }
            }
        }
    });

    upload_thread.join().unwrap();
    download_thread.join().unwrap();
}

fn handle_download(mut stream: TcpStream, file_path: String) {
    let mut buf = [0 as u8; FILE_NAME_MAX_LEN];
    // socket.read_to_end(&mut buf).unwrap();
    match stream.read(&mut buf) {
        Ok(_size) => {
            let file_name = match std::str::from_utf8(&buf) {
                Ok(file_name) => { file_name.trim_matches(char::from(0)) }
                Err(_) => { "" }
            };

            let mut saved_path = String::new();
            saved_path.push_str(&file_path);
            saved_path.push_str(file_name);

            let file: File = File::from(saved_path);

            stream.write(&*file.json_encode()).unwrap();
            stream.flush().unwrap();
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn handle_upload(mut stream: TcpStream, file_path: String) {
    let mut buf: Vec<u8> = vec![];
    match stream.read_to_end(&mut buf) {
        Ok(_size) => {
            let file: Option<File> = match serde_json::from_slice(&*buf) {
                Ok(file) => Some(file),
                Err(_) => None
            };
            if let Some(file) = file {
                let mut saved_path = file_path.to_owned();
                saved_path.push_str(file.name.as_str());

                match std::fs::write(saved_path, file.content) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            }
            stream.flush().unwrap();
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    };
}

