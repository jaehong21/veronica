use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use::serde_json;

extern crate lib;
use lib::File;

const FILE_NAME_MAX_LEN: usize = 216;
const FILE_PATH: &str = "data/";

fn main() {
    let upload_listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let download_listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let upload_thread = thread::spawn(move || {
        for stream in upload_listener.incoming() {
            match stream {
                Ok(stream) => {
                    // println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_upload(stream)
                    });
                },
                Err(e) => {
                    // Connection Error
                    println!("Error: {}", e);
                }
            }
        }
    });

    let download_thread = thread::spawn(move || {
        for stream in download_listener.incoming() {
            match stream {
                Ok(stream) => {
                    // println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_download(stream)
                    });
                },
                Err(e) => {
                    // Connection Error
                    println!("Error: {}", e);
                }
            }
        }
    });
    upload_thread.join().unwrap();
    download_thread.join().unwrap();
}

fn handle_download(mut stream: TcpStream) {
    let mut buf = [0 as u8; FILE_NAME_MAX_LEN];
    // socket.read_to_end(&mut buf).unwrap();
    match stream.read(&mut buf) {
        Ok(size) => {
            println!("{:?} bytes have been read", size);
            let file_name = match std::str::from_utf8(&buf) {
                Ok(file_name) => { file_name.trim_matches(char::from(0)) }
                Err(_) => { "" }
            };

            let mut file_path = String::new();
            file_path.push_str(FILE_PATH);
            file_path.push_str(file_name);

            let file: File = File::from(file_path);
            println!("{:?}", file);

            stream.write(&*file.json_encode()).unwrap();
            stream.flush().unwrap();
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn handle_upload(mut stream: TcpStream) {
    let mut buf: Vec<u8> = vec![];
    match stream.read_to_end(&mut buf) {
        Ok(size) => {
            println!("{:?} bytes have been read", size);
            let file: Option<File> = match serde_json::from_slice(&*buf) {
                Ok(file) => Some(file),
                Err(_) => None
            };
            if let Some(file) = file {
                let mut saved_path = FILE_PATH.to_owned();
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

