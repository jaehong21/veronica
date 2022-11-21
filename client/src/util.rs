pub mod args {
    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    pub struct Opt {
        #[structopt(short,long)]
        pub host: String,

        #[structopt(short,long, conflicts_with="download")]
        pub upload: Option<String>,
        #[structopt(short, long, conflicts_with="upload")]
        pub download: Option<String>,

        #[structopt(short,long)]
        pub path: Option<String>,
    }

    pub fn file_path(path: &Option<String>) -> String {
        match path {
            Some(path) => {
                let mut file_path = path.clone();
                file_path.push_str("/");
                file_path
            },
            None => String::from("")
        }
    }
    pub fn host(addr: &str) -> (String, String) {
        let mut upload_addr = addr.to_string();
        let mut download_addr = addr.to_string();
        upload_addr.push_str(":8000");
        download_addr.push_str(":8080");
        (upload_addr, download_addr)
    }
}
