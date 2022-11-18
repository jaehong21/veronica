use std::io::{Read, Write};
use std::net::{TcpStream};
use structopt::StructOpt;

use veronica::File;

// const FILE_PATH: &str = "/Users/jaehong21/";
const FILE_PATH: &str = "";
const UPLOAD_SERVER_ADDRESS: &str = "127.0.0.1:8000";
const DOWNLOAD_SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short,long, conflicts_with="download")]
    pub upload: Option<String>,
    #[structopt(short, long, conflicts_with="upload")]
    pub download: Option<String>,
}

fn main() {
    // const CLIENT_ADDRESS: &str = "127.0.0.1:50000";

    let args = Opt::from_args();
    match args {
        _ if(args.upload != None) => {
            match TcpStream::connect(UPLOAD_SERVER_ADDRESS) {
                Ok(stream) => {
                    upload_file(stream, args.upload.unwrap());
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            };
        },
        _ if(args.download != None) => {
            match TcpStream::connect(DOWNLOAD_SERVER_ADDRESS) {
                Ok(mut stream) => {
                    let file_name: String = args.download.unwrap();
                    stream.write(file_name.as_bytes()).unwrap();

                    let mut buf: Vec<u8> = vec![];
                    stream.read_to_end(&mut buf).unwrap();

                    let file: Option<File> = match serde_json::from_slice(&*buf) {
                        Ok(file) => Some(file),
                        Err(_) => None
                    };
                    if let Some(file) = file {
                        let mut file_name = String::from(FILE_PATH);
                        file_name.push_str(&file.name[..]);
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
    stream.write(&*file.json_encode()).unwrap();
}