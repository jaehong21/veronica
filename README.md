# Veronica

Veronica is a simple, lightweight, and file server written in Rust.
It is self-hosting application, which needs custom server.

## How to use
### Server-side
```shell
# in case of linux/ubuntu
$ sudo apt install build-essential

$ cargo build --package veronica-server --release
```
```shell
# port 8000 and 8080 must be open
$ ./target/release/veronica-server --path ./data
```
veronica-server uses port 8000 for uploading and 
8080 for downloading. Max length of file name is 512 characters.

```shell
veronica-server 0.1.0

USAGE:
    veronica-server --path <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>
```

### Client-side
```shell
$ cargo build --package veronica --release
```
```shell
veronica 0.1.0

USAGE:
    veronica [OPTIONS] --host <host>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --download <file_name>    
    -h, --host <host>            
    -p, --path <path>            
    -u, --upload <file_name>     
```